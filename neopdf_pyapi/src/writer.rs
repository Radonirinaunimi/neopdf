use neopdf::gridpdf::GridArray;
use neopdf::writer::GridArrayCollection;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

use super::gridpdf::PyGridArray;
use super::metadata::PyMetaData;

/// Python interface for GridArrayCollection utilities.
///
/// # Errors
///
/// TODO
#[pymodule]
pub fn writer(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_compress, m)?)?;
    m.add_function(wrap_pyfunction!(py_decompress, m)?)?;
    m.add_function(wrap_pyfunction!(py_extract_metadata, m)?)?;
    Ok(())
}

/// Compresses and writes a collection of GridArrays and shared metadata to a file.
///
/// # Errors
///
/// TODO
#[pyfunction(name = "compress")]
#[allow(clippy::needless_pass_by_value)]
pub fn py_compress(
    grids: Vec<PyRef<PyGridArray>>,
    metadata: &PyMetaData,
    path: &str,
) -> PyResult<()> {
    let grids: Vec<&GridArray> = grids.iter().map(|g| &g.gridarray).collect();
    GridArrayCollection::compress(&grids, &metadata.meta, path)
        .map_err(|e| PyRuntimeError::new_err(format!("Compress failed: {e}")))
}

/// Decompresses and loads all GridArrays and shared metadata from a file.
///
/// # Panics
///
/// TODO
#[must_use]
#[pyfunction(name = "decompress")]
pub fn py_decompress(path: &str) -> Vec<(PyMetaData, PyGridArray)> {
    let grid_meta = GridArrayCollection::decompress(path).unwrap();
    grid_meta
        .into_iter()
        .map(|gm| {
            let meta = PyMetaData {
                meta: gm.metadata.as_ref().clone(),
            };
            let gridarray = PyGridArray { gridarray: gm.grid };
            (meta, gridarray)
        })
        .collect()
}

/// Extracts just the metadata from a compressed file without loading the grids.
///
/// # Panics
///
/// TODO
#[must_use]
#[pyfunction(name = "extract_metadata")]
pub fn py_extract_metadata(path: &str) -> PyMetaData {
    let meta = GridArrayCollection::extract_metadata(path).unwrap();
    PyMetaData { meta }
}

/// Registers the writer module with the parent Python module.
///
/// # Panics
///
/// TODO
///
/// # Errors
///
/// TODO
pub fn register(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(parent_module.py(), "writer")?;
    m.setattr(
        pyo3::intern!(m.py(), "__doc__"),
        "PDF grid writer utilities.",
    )?;
    m.add_function(wrap_pyfunction!(py_compress, &m)?)?;
    m.add_function(wrap_pyfunction!(py_decompress, &m)?)?;
    m.add_function(wrap_pyfunction!(py_extract_metadata, &m)?)?;
    parent_module.add_submodule(&m)
}
