//! Generate `PyO3` interface for `neopdf`

#![allow(unsafe_op_in_unsafe_fn)]

use pyo3::prelude::*;

/// Python bindings for the PDF module.
pub mod pdf;

/// PyO3 Python module that contains all exposed classes from Rust.
#[pymodule]
fn neopdf(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("version", env!("CARGO_PKG_VERSION"))?;
    pdf::register(m)?;

    Ok(())
}
