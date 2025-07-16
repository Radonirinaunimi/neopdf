use neopdf::converter::{combine_lhapdf_npdfs, convert_lhapdf};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

/// Python interface for PDF set conversion utilities.
///
/// # Errors
///
/// TODO
#[pymodule]
pub fn converter(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_convert_lhapdf, m)?)?;
    m.add_function(wrap_pyfunction!(py_combine_lhapdf_npdfs, m)?)?;
    Ok(())
}

/// Converts an LHAPDF set to the NeoPDF format and writes it to disk.
///
/// # Errors
///
/// TODO
///
/// Parameters
/// ----------
/// pdf_name : str
///     The name of the LHAPDF set (e.g., "NNPDF40_nnlo_as_01180").
/// output_path : str
///     The path to the output NeoPDF file.
///
/// Returns
/// -------
/// None
#[pyfunction]
pub fn py_convert_lhapdf(pdf_name: &str, output_path: &str) -> PyResult<()> {
    convert_lhapdf(pdf_name, output_path)
        .map_err(|e| PyRuntimeError::new_err(format!("Conversion failed: {e}")))
}

/// Combines a list of nuclear PDF sets into a single NeoPDF file with explicit A dependence.
///
/// # Errors
///
/// TODO
///
/// Parameters
/// ----------
/// pdf_names : list[str]
///     List of PDF set names (each with a different A).
/// output_path : str
///     Output NeoPDF file path.
///
/// Returns
/// -------
/// None
#[pyfunction]
#[allow(clippy::needless_pass_by_value)]
pub fn py_combine_lhapdf_npdfs(pdf_names: Vec<String>, output_path: &str) -> PyResult<()> {
    let pdf_names: Vec<&str> = pdf_names.iter().map(std::string::String::as_str).collect();
    combine_lhapdf_npdfs(&pdf_names, output_path)
        .map_err(|e| PyRuntimeError::new_err(format!("Combine failed: {e}")))
}

/// Registers the converter module with the parent Python module.
///
/// # Errors
///
/// TODO
pub fn register(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(parent_module.py(), "converter")?;
    m.setattr(
        pyo3::intern!(m.py(), "__doc__"),
        "PDF set conversion utilities.",
    )?;
    m.add_function(wrap_pyfunction!(py_convert_lhapdf, &m)?)?;
    m.add_function(wrap_pyfunction!(py_combine_lhapdf_npdfs, &m)?)?;
    parent_module.add_submodule(&m)
}
