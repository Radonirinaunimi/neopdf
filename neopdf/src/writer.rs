//! This module provides utilities for serializing, compressing, and efficiently accessing PDF grid data.
//!
//! It defines types and methods for writing and reading collections of [`GridArray`]s to and from
//! compressed files, supporting both eager and lazy access patterns. The module is designed for
//! efficient storage and retrieval of large PDF sets, with shared metadata and support for random
//! access to individual members.
//!
//! # Main Features
//!
//! - Compression and decompression of multiple [`GridArray`]s with shared metadata using LZ4 and bincode serialization.
//! - Random access to individual grid members without loading the entire collection into memory.
//! - Lazy iteration over grid members for memory-efficient processing of large sets.
//! - Extraction of metadata without full decompression.
//!
//! # Key Types
//!
//! - [`GridArrayWithMetadata`]: Container for a grid and its associated metadata.
//! - [`GridArrayCollection`]: Static interface for compressing and decompressing collections of grids.
//! - [`GridArrayReader`]: Provides random access to individual grids in a compressed file.
//! - [`LazyGridArrayIterator`]: Enables lazy, sequential iteration over grid members.
//!
//! See the documentation for each type for more details on available methods and usage patterns.
use lz4_flex::frame::{FrameDecoder, FrameEncoder};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

use super::gridpdf::GridArray;
use super::metadata::MetaData;
use std::sync::Arc;

/// Container for a [`GridArray`] with a shared reference to its associated metadata.
///
/// Used to bundle grid data and metadata together for convenient access after decompression
/// or random access.
#[derive(Debug)]
pub struct GridArrayWithMetadata {
    pub grid: GridArray,
    pub metadata: Arc<MetaData>,
}

/// Static interface for compressing and decompressing of [`GridArray`]s with shared metadata.
///
/// Provides methods for writing, reading, and extracting metadata from compressed files.
pub struct GridArrayCollection;

impl GridArrayCollection {
    /// Compresses and writes a collection of [`GridArray`]s and shared metadata to a file.
    ///
    /// # Arguments
    ///
    /// * `grids` - Slice of grid arrays to compress.
    /// * `metadata` - Shared metadata for all grids.
    /// * `path` - Output file path.
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or an error if writing fails.
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

    /// Decompresses and loads all [`GridArray`]s and shared metadata from a file.
    ///
    /// # Arguments
    ///
    /// * `path` - Input file path.
    ///
    /// # Returns
    ///
    /// A vector of [`GridArrayWithMetadata`] on success, or an error if reading fails.
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

    /// Extracts just the metadata from a compressed file without loading the grids.
    ///
    /// # Arguments
    ///
    /// * `path` - Input file path.
    ///
    /// # Returns
    ///
    /// The [`MetaData`] struct on success, or an error if reading fails.
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

/// Provides random access to individual [`GridArray`]s in a compressed file without loading the entire collection.
///
/// Useful for efficient access to large PDF sets where only a subset of members is needed.
pub struct GridArrayReader {
    data: Vec<u8>,
    metadata: Arc<MetaData>,
    offsets: Vec<u64>,
    count: u64,
    data_start: u64,
}

impl GridArrayReader {
    /// Creates a new reader from a file, enabling random access to grid members.
    ///
    /// # Arguments
    ///
    /// * `path` - Input file path.
    ///
    /// # Returns
    ///
    /// A [`GridArrayReader`] instance on success, or an error if reading fails.
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

    /// Returns the number of grid arrays in the collection.
    pub fn len(&self) -> usize {
        self.count as usize
    }

    /// Returns true if the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Returns a reference to the shared metadata.
    pub fn metadata(&self) -> &Arc<MetaData> {
        &self.metadata
    }

    /// Loads a specific [`GridArrayWithMetadata`] by index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the grid to load.
    ///
    /// # Returns
    ///
    /// The requested [`GridArrayWithMetadata`] on success, or an error if the index is out
    /// of bounds or reading fails.
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

/// Iterator for lazily reading [`GridArrayWithMetadata`] members from a compressed file.
///
/// Useful for memory-efficient sequential processing of large PDF sets.
pub struct LazyGridArrayIterator {
    cursor: std::io::Cursor<Vec<u8>>,
    remaining: u64,
    metadata: Arc<MetaData>,
    buffer: Vec<u8>,
}

impl LazyGridArrayIterator {
    /// Creates a new lazy iterator from a reader.
    ///
    /// # Arguments
    ///
    /// * `reader` - Any type implementing [`Read`].
    ///
    /// # Returns
    ///
    /// A [`LazyGridArrayIterator`] instance on success, or an error if reading fails.
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

    /// Creates a new lazy iterator from a file path.
    ///
    /// # Arguments
    ///
    /// * `path` - Input file path.
    ///
    /// # Returns
    ///
    /// A [`LazyGridArrayIterator`] instance on success, or an error if reading fails.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let buf_reader = BufReader::new(file);
        Self::new(buf_reader)
    }

    /// Returns a reference to the shared metadata.
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
