//! TODO: Add proper documentation.

use neopdf::pdf::PDF;
use numpy::{IntoPyArray, PyArray3};
use pyo3::prelude::*;

/// PyO3 wrapper to :rustdoc:`neopdf::pdf::PDF <pdf/struct.PDF.html>`.
#[pyclass(name = "PDF")]
#[repr(transparent)]
pub struct PyPDF {
    pub(crate) pdf: PDF,
}

#[pymethods]
impl PyPDF {
    /// Constructor.
    ///
    /// Parameters
    /// ----------
    /// pdf_name: str
    ///     name of the PDF set
    #[new]
    #[must_use]
    pub fn new(pdf_name: &str) -> Self {
        Self {
            pdf: PDF::load(pdf_name),
        }
    }

    /// Loads a given member of the PDF set.
    ///
    /// Parameters
    /// ----------
    /// pdf_name: str
    ///     name of the PDF set
    #[must_use]
    #[staticmethod]
    #[pyo3(name = "mkPDF")]
    pub fn mkpdf(pdf_name: &str) -> Self {
        Self::new(pdf_name)
    }

    /// Loads all of the members of the PDF set.
    ///
    /// Parameters
    /// ----------
    /// pdf_name: str
    ///     name of the PDF set
    #[must_use]
    #[staticmethod]
    #[pyo3(name = "mkPDFs")]
    pub fn mkpdfs(pdf_name: &str) -> Vec<Self> {
        PDF::load_pdfs(pdf_name)
            .into_iter()
            .map(move |pdfobj| Self { pdf: pdfobj })
            .collect()
    }

    /// Retrieves the `x_min` for this PDF set.
    #[must_use]
    pub fn x_min(&self) -> f64 {
        self.pdf.x_min()
    }

    /// Retrieves the `x_max` for this PDF set.
    #[must_use]
    pub fn x_max(&self) -> f64 {
        self.pdf.x_max()
    }

    /// Retrieves the `q2_min` for this PDF set.
    #[must_use]
    pub fn q2_min(&self) -> f64 {
        self.pdf.q2_min()
    }

    /// Retrieves the `q2_max` for this PDF set.
    #[must_use]
    pub fn q2_max(&self) -> f64 {
        self.pdf.q2_max()
    }

    /// Interpolates the PDF value (xf) for a given flavor, x, and Q2.
    #[must_use]
    #[pyo3(name = "xfxQ2")]
    pub fn xfxq2(&self, id: i32, x: f64, q2: f64) -> f64 {
        self.pdf.xfxq2(id, x, q2)
    }

    /// Interpolates the PDF value (xf) for some lists of flavors, xs, and Q2s.
    #[must_use]
    #[pyo3(name = "xfxQ2s")]
    pub fn xfxq2s<'py>(
        &self,
        id: Vec<i32>,
        x: Vec<f64>,
        q2: Vec<f64>,
        py: Python<'py>,
    ) -> Bound<'py, PyArray3<f64>> {
        self.pdf.xfxq2s(id, x, q2).into_pyarray(py)
    }

    /// Compute the `alphas` interpolation at `Q2` value.
    #[must_use]
    #[pyo3(name = "alphasQ2")]
    pub fn alphas_q2(&self, q2: f64) -> f64 {
        self.pdf.alphas_q2(q2)
    }
}

/// Register submodule in parent.
///
/// # Errors
///
/// Raises an error if (sub)module is not found.
pub fn register(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(parent_module.py(), "pdf")?;
    m.setattr(pyo3::intern!(m.py(), "__doc__"), "Interface for PDF.")?;
    pyo3::py_run!(
        parent_module.py(),
        m,
        "import sys; sys.modules['neopdf.pdf'] = m"
    );
    m.add_class::<PyPDF>()?;
    parent_module.add_submodule(&m)
}
