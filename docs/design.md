# Design

## PDF Object Construction: Multi-dimensional Grid Representation

The core of `NeoPDF`'s data model is the representation of Parton Distribution Functions (PDFs)
as multi-dimensional arrays. This is implemented via the `GridArray` and `SubGrid` structures
in the `neopdf::gridpdf` module.

- **GridArray**: Stores the full set of subgrids and flavor IDs. Each subgrid represents a region
  of phase space with a consistent grid of variables ($A$, $\alpha_s$, $x$, $Q^2$).
- **SubGrid**: Contains a 5-dimensional array: `[nucleons, alphas, pids, x, Q²]`. This allows for
  efficient storage and interpolation across all relevant physical parameters.
- **Interpolation**: The library supports 2D, 3D, and 4D interpolation strategies, automatically
  selecting the appropriate method based on the grid structure and metadata. Interpolators are
  built for each subgrid and flavor, supporting log-space and linear strategies for high accuracy.

This design enables:
- Efficient access to PDF values at arbitrary kinematic points.
- Support for advanced use cases, such as nuclear PDFs and variable alpha_s.
- Modular extension to new interpolation strategies or additional dimensions.

## NeoPDF File Format: Compression, Metadata, and Lazy Loading

The `NeoPDF` file format is designed for efficient storage and fast, random access to large collections
of PDF grids. Due to various technical reasons (see below), the format is **not human-readable**.
However, `NeoPDF` provides a [CLI tool](/docs/cli-tutorials.md) that allows the user to easily and
quickly inspect the contents of a given PDF set. The implementation of the logics is found in
`neopdf::writer`.

- **Serialization & Compression**: All grid data (`GridArray`), along with shared metadata, are
  serialized using `bincode` and compressed with LZ4. This results in compact files that are quick
  to read and write.
- **Metadata**: Metadata is stored at the beginning of the file, allowing extraction without
  decompressing the entire file.
- **Offset Table**: An offset table is written after the metadata, enabling random access to any
  grid member without reading the whole file.
- **Grid Data**: Each grid is stored with its size and data, allowing for efficient deserialization.

### Access Patterns
- **Eager Loading**: The entire collection of grids can be decompressed and loaded into memory for
  batch operations.
- **Random Access**: The `GridArrayReader` provides random access to individual grids using the offset
  table, without loading all data.
- **Lazy Iteration**: The `LazyGridArrayIterator` enables sequential, memory-efficient iteration over
  grid members, suitable for processing very large sets.

### Advantages
- **Performance**: LZ4 compression and binary serialization provide fast read/write speeds and small
  file sizes.
- **Scalability**: Lazy and random access patterns allow working with very large PDF sets without
  high memory usage.
- **Extensibility**: The format is designed to accommodate future extensions, such as new metadata
  fields or additional grid dimensions.
