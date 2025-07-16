use ndarray::Array1;
use neopdf::gridpdf::{GridArray, ParamRange, SubGrid};
use numpy::{PyArrayMethods, PyReadonlyArray5};
use pyo3::prelude::*;

/// Python wrapper for the `SubGrid` struct.
#[pyclass(name = "SubGrid")]
pub struct PySubGrid {
    pub(crate) subgrid: SubGrid,
}

#[pymethods]
impl PySubGrid {
    /// TODO
    ///
    /// # Panics
    ///
    /// TODO
    ///
    /// # Errors
    ///
    /// TODO
    #[new]
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(
        xs: Vec<f64>,
        q2s: Vec<f64>,
        nucleons: Vec<f64>,
        alphas: Vec<f64>,
        grid: PyReadonlyArray5<f64>,
    ) -> PyResult<Self> {
        let alphas_range = ParamRange::new(*alphas.first().unwrap(), *alphas.last().unwrap());
        let x_range = ParamRange::new(*xs.first().unwrap(), *xs.last().unwrap());
        let q2_range = ParamRange::new(*q2s.first().unwrap(), *q2s.last().unwrap());
        let nucleons_range = ParamRange::new(*nucleons.first().unwrap(), *nucleons.last().unwrap());

        let subgrid = SubGrid {
            xs: Array1::from(xs),
            q2s: Array1::from(q2s),
            grid: grid.to_owned_array(),
            nucleons: Array1::from(nucleons),
            alphas: Array1::from(alphas),
            nucleons_range,
            alphas_range,
            x_range,
            q2_range,
        };

        Ok(Self { subgrid })
    }

    /// TODO
    #[must_use]
    pub const fn alphas_range(&self) -> (f64, f64) {
        (self.subgrid.alphas_range.min, self.subgrid.alphas_range.max)
    }

    /// TODO
    #[must_use]
    pub const fn x_range(&self) -> (f64, f64) {
        (self.subgrid.x_range.min, self.subgrid.x_range.max)
    }

    /// TODO
    #[must_use]
    pub const fn q2_range(&self) -> (f64, f64) {
        (self.subgrid.q2_range.min, self.subgrid.q2_range.max)
    }
}

/// Python wrapper for the `GridArray` struct.
#[pyclass(name = "GridArray")]
#[repr(transparent)]
pub struct PyGridArray {
    pub(crate) gridarray: GridArray,
}

#[pymethods]
impl PyGridArray {
    /// TODO
    #[new]
    #[must_use]
    pub fn new(pids: Vec<i32>, subgrids: Vec<PyRef<PySubGrid>>) -> Self {
        let subgrids = subgrids
            .into_iter()
            .map(|py_ref| py_ref.subgrid.clone())
            .collect();

        let gridarray = GridArray {
            pids: Array1::from(pids),
            subgrids,
        };
        Self { gridarray }
    }

    /// TODO
    #[must_use]
    pub fn pids(&self) -> Vec<i32> {
        self.gridarray.pids.to_vec()
    }

    /// TODO
    #[must_use]
    pub fn subgrids(&self) -> Vec<PySubGrid> {
        self.gridarray
            .subgrids
            .iter()
            .cloned()
            .map(|sg| PySubGrid { subgrid: sg })
            .collect()
    }
}

/// Registers the gridpdf module with the parent Python module.
///
/// # Errors
///
/// TODO
pub fn register(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(parent_module.py(), "gridpdf")?;
    m.setattr(
        pyo3::intern!(m.py(), "__doc__"),
        "GridPDF interpolation interface.",
    )?;
    m.add_class::<PySubGrid>()?;
    m.add_class::<PyGridArray>()?;
    parent_module.add_submodule(&m)
}
