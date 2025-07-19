use core::panic;

use neopdf::pdf::PDF;
use numpy::{IntoPyArray, PyArray2};
use pyo3::prelude::*;

use super::gridpdf::PySubGrid;
use super::metadata::PyMetaData;

/// Python wrapper for the `neopdf::pdf::PDF` struct.
///
/// This class provides a Python-friendly interface to the core PDF
/// interpolation functionalities of the `neopdf` Rust library.
#[pyclass(name = "PDF")]
#[repr(transparent)]
pub struct PyPDF {
    pub(crate) pdf: PDF,
}

#[pymethods]
impl PyPDF {
    /// Creates a new `PDF` instance for a given PDF set and member.
    ///
    /// This is the primary constructor for the `PDF` class.
    ///
    /// Parameters
    /// ----------
    /// pdf_name : str
    ///     The name of the PDF set.
    /// member : int
    ///     The ID of the PDF member to load. Defaults to 0.
    ///
    /// Returns
    /// -------
    /// PDF
    ///     A new `PDF` instance.
    #[new]
    #[must_use]
    #[pyo3(signature = (pdf_name, member = 0))]
    pub fn new(pdf_name: &str, member: usize) -> Self {
        Self {
            pdf: PDF::load(pdf_name, member),
        }
    }

    /// Loads a given member of the PDF set.
    ///
    /// This is an alternative constructor for convenience, equivalent
    /// to `PDF(pdf_name, member)`.
    ///
    /// Parameters
    /// ----------
    /// pdf_name : str
    ///     The name of the PDF set.
    /// member : int
    ///     The ID of the PDF member. Defaults to 0.
    ///
    /// Returns
    /// -------
    /// PDF
    ///     A new `PDF` instance.
    #[must_use]
    #[staticmethod]
    #[pyo3(name = "mkPDF")]
    #[pyo3(signature = (pdf_name, member = 0))]
    pub fn mkpdf(pdf_name: &str, member: usize) -> Self {
        Self::new(pdf_name, member)
    }

    /// Loads all members of the PDF set.
    ///
    /// This function loads all available members for a given PDF set,
    /// returning a list of `PDF` instances.
    ///
    /// Parameters
    /// ----------
    /// pdf_name : str
    ///     The name of the PDF set.
    ///
    /// Returns
    /// -------
    /// list[PDF]
    ///     A list of `PDF` instances, one for each member.
    #[must_use]
    #[staticmethod]
    #[pyo3(name = "mkPDFs")]
    pub fn mkpdfs(pdf_name: &str) -> Vec<Self> {
        PDF::load_pdfs(pdf_name)
            .into_iter()
            .map(move |pdfobj| Self { pdf: pdfobj })
            .collect()
    }

    /// Returns the list of `PID` values.
    ///
    /// Returns
    /// -------
    /// list[int]
    ///     The PID values.
    #[must_use]
    pub fn pids(&self) -> Vec<i32> {
        self.pdf.pids().to_vec()
    }

    /// Returns the list of `Subgrid` objects.
    ///
    /// Returns
    /// -------
    /// list[PySubgrid]
    ///     The subgrids.
    #[must_use]
    pub fn subgrids(&self) -> Vec<PySubGrid> {
        self.pdf
            .subgrids()
            .iter()
            .map(|subgrid| PySubGrid {
                subgrid: subgrid.clone(),
            })
            .collect()
    }

    /// Returns the subgrid knots of a parameter for a given subgrid index.
    ///
    /// The parameter could be the nucleon numbers `A`, the strong coupling
    /// `alphas`, the momentum fraction `x`, or the momentum scale `Q2`.
    ///
    /// # Panics
    ///
    /// This panics if the parameter is not valid.
    ///
    /// Returns
    /// -------
    /// list[float]
    ///     The subgrid knots for a given parameter.
    #[must_use]
    pub fn subgrid_knots(&self, param: &str, subgrid_index: usize) -> Vec<f64> {
        match param.to_lowercase().as_str() {
            "alphas" => self.pdf.subgrid(subgrid_index).alphas.to_vec(),
            "x" => self.pdf.subgrid(subgrid_index).xs.to_vec(),
            "q2" => self.pdf.subgrid(subgrid_index).q2s.to_vec(),
            "nucleons" => self.pdf.subgrid(subgrid_index).nucleons.to_vec(),
            _ => panic!("The argument {param} is not a valid parameter."),
        }
    }

    /// Retrieves the minimum x-value for this PDF set.
    ///
    /// Returns
    /// -------
    /// float
    ///     The minimum x-value.
    #[must_use]
    pub fn x_min(&self) -> f64 {
        self.pdf.param_ranges().x.min
    }

    /// Retrieves the maximum x-value for this PDF set.
    ///
    /// Returns
    /// -------
    /// float
    ///     The maximum x-value.
    #[must_use]
    pub fn x_max(&self) -> f64 {
        self.pdf.param_ranges().x.max
    }

    /// Retrieves the minimum Q2-value for this PDF set.
    ///
    /// Returns
    /// -------
    /// float
    ///     The minimum Q2-value.
    #[must_use]
    pub fn q2_min(&self) -> f64 {
        self.pdf.param_ranges().q2.min
    }

    /// Retrieves the maximum Q2-value for this PDF set.
    ///
    /// Returns
    /// -------
    /// float
    ///     The maximum Q2-value.
    #[must_use]
    pub fn q2_max(&self) -> f64 {
        self.pdf.param_ranges().q2.max
    }

    /// Interpolates the PDF value (xf) for a given flavor, x, and Q2.
    ///
    /// Parameters
    /// ----------
    /// id : int
    ///     The flavor ID (e.g., 21 for gluon, 1 for d-quark).
    /// x : float
    ///     The momentum fraction.
    /// q2 : float
    ///     The energy scale squared.
    ///
    /// Returns
    /// -------
    /// float
    ///     The interpolated PDF value. Returns 0.0 if extrapolation is
    ///     attempted and not allowed.
    #[must_use]
    #[pyo3(name = "xfxQ2")]
    pub fn xfxq2(&self, id: i32, x: f64, q2: f64) -> f64 {
        self.pdf.xfxq2(id, &[x, q2])
    }

    /// Interpolates the PDF value (xf) for a given set of parameters.
    ///
    /// Parameters
    /// ----------
    /// id : int
    ///     The flavor ID (e.g., 21 for gluon, 1 for d-quark).
    /// params: list[float]
    ///     A list of parameters that the grids depends on. If the PDF
    ///     grid only contains `x` and `Q2` dependence then its value is
    ///     `[x, q2]`; if it contains either the `A` and `alpha_s`
    ///     dependence, then its value is `[A, x, q2]` or `[alpha_s, x, q2]`
    ///     respectively; if it contains both, then `[A, alpha_s, x, q2]`.
    ///
    /// Returns
    /// -------
    /// float
    ///     The interpolated PDF value. Returns 0.0 if extrapolation is
    ///     attempted and not allowed.
    #[must_use]
    #[pyo3(name = "xfxQ2_ND")]
    #[allow(clippy::needless_pass_by_value)]
    pub fn xfxq2_nd(&self, id: i32, params: Vec<f64>) -> f64 {
        self.pdf.xfxq2(id, &params)
    }

    /// Interpolates the PDF value (xf) for lists of flavors, x-values,
    /// and Q2-values.
    ///
    /// Parameters
    /// ----------
    /// id : list[int]
    ///     A list of flavor IDs.
    /// xs : list[float]
    ///     A list of momentum fractions.
    /// q2s : list[float]
    ///     A list of energy scales squared.
    ///
    /// Returns
    /// -------
    /// numpy.ndarray
    ///     A 2D NumPy array containing the interpolated PDF values.
    #[must_use]
    #[pyo3(name = "xfxQ2s")]
    #[allow(clippy::needless_pass_by_value)]
    pub fn xfxq2s<'py>(
        &self,
        pids: Vec<i32>,
        xs: Vec<f64>,
        q2s: Vec<f64>,
        py: Python<'py>,
    ) -> Bound<'py, PyArray2<f64>> {
        let flatten_points: Vec<Vec<f64>> = xs
            .iter()
            .flat_map(|&x| q2s.iter().map(move |&q2| vec![x, q2]))
            .collect();
        let points_interp: Vec<&[f64]> = flatten_points.iter().map(Vec::as_slice).collect();
        let slice_points: &[&[f64]] = &points_interp;

        self.pdf.xfxq2s(pids, slice_points).into_pyarray(py)
    }

    /// Computes the alpha_s value at a given Q2.
    ///
    /// Parameters
    /// ----------
    /// q2 : float
    ///     The energy scale squared.
    ///
    /// Returns
    /// -------
    /// float
    ///     The interpolated alpha_s value.
    #[must_use]
    #[pyo3(name = "alphasQ2")]
    pub fn alphas_q2(&self, q2: f64) -> f64 {
        self.pdf.alphas_q2(q2)
    }

    /// Returns the metadata associated with this PDF set.
    ///
    /// Provides access to the metadata describing the PDF set, including information
    /// such as the set description, number of members, parameter ranges, and other
    /// relevant details.
    ///
    /// Returns
    /// -------
    /// MetaData
    ///     The metadata for this PDF set as a `MetaData` Python object.
    #[must_use]
    #[pyo3(name = "metadata")]
    pub fn metadata(&self) -> PyMetaData {
        PyMetaData {
            meta: self.pdf.metadata().clone(),
        }
    }
}

/// Registers the `pdf` submodule with the parent Python module.
///
/// This function is typically called during the initialization of the
/// `neopdf` Python package to expose the `PDF` class.
///
/// Parameters
/// ----------
/// `parent_module` : pyo3.Bound[pyo3.types.PyModule]
///     The parent Python module to which the `pdf` submodule will be added.
///
/// Returns
/// -------
/// pyo3.PyResult<()>
///     `Ok(())` if the registration is successful, or an error if the submodule
///     cannot be created or added.
///
/// # Errors
///
/// Raises an error if the (sub)module is not found or cannot be registered.
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
