use neopdf::manage::{ManageData, PdfSetFormat};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

/// Python wrapper for the `PdfSetFormat` enum.
#[pyclass(name = "PdfSetFormat")]
#[derive(Clone)]
pub enum PyPdfSetFormat {
    /// TODO
    Lhapdf,
    /// TODO
    Neopdf,
}

impl From<PyPdfSetFormat> for PdfSetFormat {
    fn from(fmt: PyPdfSetFormat) -> Self {
        match fmt {
            PyPdfSetFormat::Lhapdf => Self::Lhapdf,
            PyPdfSetFormat::Neopdf => Self::Neopdf,
        }
    }
}

/// Python wrapper for the `ManageData` struct.
#[pyclass(name = "ManageData")]
pub struct PyManageData {
    pub(crate) inner: ManageData,
}

#[pymethods]
impl PyManageData {
    /// Create a new ManageData instance.
    #[new]
    #[must_use]
    pub fn new(set_name: &str, format: PyPdfSetFormat) -> Self {
        Self {
            inner: ManageData::new(set_name, format.into()),
        }
    }

    /// Download the PDF set and extract it into the designated path.
    ///
    /// # Errors
    ///
    /// TODO
    pub fn download_pdf(&self) -> PyResult<()> {
        self.inner
            .download_pdf()
            .map_err(|e| PyRuntimeError::new_err(format!("{e}")))
    }

    /// Check that the PDF set is installed in the correct path.
    ///
    /// # Errors
    ///
    /// TODO
    #[must_use]
    pub fn is_pdf_installed(&self) -> bool {
        self.inner.is_pdf_installed()
    }

    /// Ensure that the PDF set is installed, otherwise download it.
    ///
    /// # Errors
    ///
    /// TODO
    pub fn ensure_pdf_installed(&self) -> PyResult<()> {
        self.inner
            .ensure_pdf_installed()
            .map_err(|e| PyRuntimeError::new_err(format!("{e}")))
    }

    /// Get the name of the PDF set.
    #[must_use]
    pub fn set_name(&self) -> &str {
        self.inner.set_name()
    }

    /// Get the path where PDF sets are stored.
    #[must_use]
    pub fn data_path(&self) -> String {
        self.inner.data_path().to_string_lossy().to_string()
    }

    /// Get the full path to this specific PDF set.
    #[must_use]
    pub fn set_path(&self) -> String {
        self.inner.set_path().to_string_lossy().to_string()
    }
}

/// Registers the manage module with the parent Python module.
///
/// # Errors
///
/// TODO
pub fn register(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(parent_module.py(), "manage")?;
    m.setattr(
        pyo3::intern!(m.py(), "__doc__"),
        "PDF set management utilities.",
    )?;
    m.add_class::<PyPdfSetFormat>()?;
    m.add_class::<PyManageData>()?;
    parent_module.add_submodule(&m)
}
