use lz4_flex::frame::{FrameDecoder, FrameEncoder};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

use super::gridpdf::GridArray;
use super::metadata::MetaData;
use std::sync::Arc;

/// Container for GridArray with shared metadata reference.
#[derive(Debug)]
pub struct GridArrayWithMetadata {
    pub grid: GridArray,
    pub metadata: Arc<MetaData>,
}

/// For storing multiple GridArrays with shared metadata.
pub struct GridArrayCollection;

impl GridArrayCollection {
    /// TODO
    pub fn compress<P: AsRef<Path>>(
        grids: &[GridArray],
        metadata: &MetaData,
        path: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create(path)?;
        let buf_writer = BufWriter::new(file);
        let mut encoder = FrameEncoder::new(buf_writer);

        let metadata_serialized = bincode::serialize(metadata)?;
        let metadata_size = metadata_serialized.len() as u64;

        let metadata_size_bytes = bincode::serialize(&metadata_size)?;
        encoder.write_all(&metadata_size_bytes)?;
        encoder.write_all(&metadata_serialized)?;

        let count = grids.len() as u64;
        let count_bytes = bincode::serialize(&count)?;
        encoder.write_all(&count_bytes)?;

        let mut serialized_grids = Vec::new();
        for grid in grids {
            let serialized = bincode::serialize(grid)?;
            serialized_grids.push(serialized);
        }

        let mut offsets = Vec::new();
        let mut current_offset = 0u64;

        let offset_table_size = (serialized_grids.len() * 8) as u64;
        current_offset += 8 + offset_table_size;

        for serialized in &serialized_grids {
            offsets.push(current_offset);
            current_offset += 8;
            current_offset += serialized.len() as u64;
        }

        let offset_table_size_bytes = bincode::serialize(&offset_table_size)?;
        encoder.write_all(&offset_table_size_bytes)?;
        for offset in &offsets {
            let offset_bytes = bincode::serialize(offset)?;
            encoder.write_all(&offset_bytes)?;
        }

        for serialized in &serialized_grids {
            let size = serialized.len() as u64;
            let size_bytes = bincode::serialize(&size)?;
            encoder.write_all(&size_bytes)?;
            encoder.write_all(serialized)?;
        }

        encoder.finish()?;
        Ok(())
    }

    /// TODO
    pub fn decompress<P: AsRef<Path>>(
        path: P,
    ) -> Result<Vec<GridArrayWithMetadata>, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let buf_reader = BufReader::new(file);
        let mut decoder = FrameDecoder::new(buf_reader);

        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)?;

        let mut cursor = std::io::Cursor::new(decompressed);

        let metadata_size: u64 = bincode::deserialize_from(&mut cursor)?;
        let mut metadata_bytes = vec![0u8; metadata_size as usize];
        cursor.read_exact(&mut metadata_bytes)?;
        let metadata: MetaData = bincode::deserialize(&metadata_bytes)?;
        let shared_metadata = Arc::new(metadata);

        let count: u64 = bincode::deserialize_from(&mut cursor)?;

        let offset_table_size: u64 = bincode::deserialize_from(&mut cursor)?;
        cursor.set_position(cursor.position() + offset_table_size);
        let mut grids = Vec::with_capacity(count as usize);

        for _ in 0..count {
            let size: u64 = bincode::deserialize_from(&mut cursor)?;
            let mut grid_bytes = vec![0u8; size as usize];
            cursor.read_exact(&mut grid_bytes)?;

            let grid: GridArray = bincode::deserialize(&grid_bytes)?;
            grids.push(GridArrayWithMetadata {
                grid,
                metadata: Arc::clone(&shared_metadata),
            });
        }

        Ok(grids)
    }

    /// Extract just the metadata from a compressed file without loading grids.
    pub fn extract_metadata<P: AsRef<Path>>(
        path: P,
    ) -> Result<MetaData, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let buf_reader = BufReader::new(file);
        let mut decoder = FrameDecoder::new(buf_reader);

        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)?;

        let mut cursor = std::io::Cursor::new(decompressed);

        let metadata_size: u64 = bincode::deserialize_from(&mut cursor)?;
        let mut metadata_bytes = vec![0u8; metadata_size as usize];
        cursor.read_exact(&mut metadata_bytes)?;
        let metadata: MetaData = bincode::deserialize(&metadata_bytes)?;

        Ok(metadata)
    }
}

/// Access a given member of the set without loading the entire objects.
pub struct GridArrayReader {
    data: Vec<u8>,
    metadata: Arc<MetaData>,
    offsets: Vec<u64>,
    count: u64,
    data_start: u64,
}

impl GridArrayReader {
    /// Create a new reader from a file.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let buf_reader = BufReader::new(file);
        let mut decoder = FrameDecoder::new(buf_reader);

        let mut data = Vec::new();
        decoder.read_to_end(&mut data)?;

        let mut cursor = std::io::Cursor::new(&data);

        let metadata_size: u64 = bincode::deserialize_from(&mut cursor)?;
        let mut metadata_bytes = vec![0u8; metadata_size as usize];
        cursor.read_exact(&mut metadata_bytes)?;
        let metadata: MetaData = bincode::deserialize(&metadata_bytes)?;
        let shared_metadata = Arc::new(metadata);
        let count: u64 = bincode::deserialize_from(&mut cursor)?;

        let mut offsets = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let offset: u64 = bincode::deserialize_from(&mut cursor)?;
            offsets.push(offset);
        }

        let data_start = cursor.position();

        Ok(Self {
            data,
            metadata: shared_metadata,
            offsets,
            count,
            data_start,
        })
    }

    /// Get the number of GridArrays in the collection.
    pub fn len(&self) -> usize {
        self.count as usize
    }

    /// Check if the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Get a reference to the shared metadata.
    pub fn metadata(&self) -> &Arc<MetaData> {
        &self.metadata
    }

    /// Load a specific GridArray by index.
    pub fn load_grid(
        &self,
        index: usize,
    ) -> Result<GridArrayWithMetadata, Box<dyn std::error::Error>> {
        if index >= self.count as usize {
            return Err(format!(
                "Index {} out of bounds for collection of size {}",
                index, self.count
            )
            .into());
        }

        let offset = self.data_start + self.offsets[index];
        let mut cursor = std::io::Cursor::new(&self.data);
        cursor.set_position(offset);
        let size: u64 = bincode::deserialize_from(&mut cursor)?;

        let mut grid_bytes = vec![0u8; size as usize];
        cursor.read_exact(&mut grid_bytes)?;

        let grid: GridArray = bincode::deserialize(&grid_bytes)?;

        Ok(GridArrayWithMetadata {
            grid,
            metadata: Arc::clone(&self.metadata),
        })
    }
}

/// Lazily iterate over the PDF members.
pub struct LazyGridArrayIterator {
    cursor: std::io::Cursor<Vec<u8>>,
    remaining: u64,
    metadata: Arc<MetaData>,
    buffer: Vec<u8>,
}

impl LazyGridArrayIterator {
    /// Create a new lazy iterator from a reader.
    pub fn new<R: Read>(reader: R) -> Result<Self, Box<dyn std::error::Error>> {
        let mut decoder = FrameDecoder::new(reader);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)?;

        let mut cursor = std::io::Cursor::new(decompressed);

        let metadata_size: u64 = bincode::deserialize_from(&mut cursor)?;
        let mut metadata_bytes = vec![0u8; metadata_size as usize];
        cursor.read_exact(&mut metadata_bytes)?;
        let metadata: MetaData = bincode::deserialize(&metadata_bytes)?;
        let shared_metadata = Arc::new(metadata);

        let count: u64 = bincode::deserialize_from(&mut cursor)?;

        Ok(Self {
            cursor,
            remaining: count,
            metadata: shared_metadata,
            buffer: Vec::new(),
        })
    }

    /// Create from file path.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let buf_reader = BufReader::new(file);
        Self::new(buf_reader)
    }

    /// Get a reference to the shared metadata.
    pub fn metadata(&self) -> &Arc<MetaData> {
        &self.metadata
    }
}

impl Iterator for LazyGridArrayIterator {
    type Item = Result<GridArrayWithMetadata, Box<dyn std::error::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        let result = (|| -> Result<GridArrayWithMetadata, Box<dyn std::error::Error>> {
            // Read size
            let size: u64 = bincode::deserialize_from(&mut self.cursor)?;

            // Read grid data
            self.buffer.resize(size as usize, 0);
            self.cursor.read_exact(&mut self.buffer)?;

            let grid: GridArray = bincode::deserialize(&self.buffer)?;

            Ok(GridArrayWithMetadata {
                grid,
                metadata: Arc::clone(&self.metadata),
            })
        })();

        self.remaining -= 1;
        Some(result)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.remaining as usize;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for LazyGridArrayIterator {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metadata::{InterpolatorType, SetType};
    use ndarray::Array1;
    use tempfile::NamedTempFile;

    #[test]
    fn test_collection_with_metadata() {
        let metadata = MetaData {
            set_desc: "Test PDF".into(),
            set_index: 1,
            num_members: 2,
            x_min: 1e-5,
            x_max: 1.0,
            q_min: 1.0,
            q_max: 1000.0,
            flavors: vec![1, 2, 3],
            format: "NeoPDF".into(),
            alphas_q_values: vec![],
            alphas_vals: vec![],
            polarised: false,
            set_type: SetType::Pdf,
            interpolator_type: InterpolatorType::LogBicubic,
        };

        let grids = vec![test_grid(), test_grid()];
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        GridArrayCollection::compress(&grids, &metadata, path).unwrap();
        let extracted = GridArrayCollection::extract_metadata(path).unwrap();
        assert_eq!(metadata.set_desc, extracted.set_desc);
        assert_eq!(metadata.set_index, extracted.set_index);

        let decompressed = GridArrayCollection::decompress(path).unwrap();
        assert_eq!(decompressed.len(), 2);
        for g in &decompressed {
            assert_eq!(g.metadata.set_desc, "Test PDF");
            assert_eq!(g.grid.pids, Array1::from(vec![1, 2, 3]));
        }

        let g_iter = LazyGridArrayIterator::from_file(path).unwrap();
        assert_eq!(g_iter.metadata().set_index, 1);
        assert_eq!(g_iter.count(), 2);
    }

    fn test_grid() -> GridArray {
        GridArray {
            pids: Array1::from(vec![1, 2, 3]),
            subgrids: vec![],
        }
    }
}
