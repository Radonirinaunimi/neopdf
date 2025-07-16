use neopdf::metadata::{InterpolatorType, MetaData, SetType};
use pyo3::prelude::*;

/// The type of the set.
#[pyclass(eq, eq_int, name = "SetType")]
#[derive(Clone, PartialEq, Eq)]
pub enum PySetType {
    /// Parton Distribution Function.
    Pdf,
    /// Fragmentation Function.
    Fragfn,
}

impl From<&SetType> for PySetType {
    fn from(set_type: &SetType) -> Self {
        match set_type {
            SetType::Pdf => Self::Pdf,
            SetType::Fragfn => Self::Fragfn,
        }
    }
}

/// The interpolation method used for the grid.
#[pyclass(eq, eq_int, name = "InterpolatorType")]
#[derive(Clone, PartialEq, Eq)]
pub enum PyInterpolatorType {
    /// Bilinear interpolation strategy.
    Bilinear,
    /// Bilinear logarithmic interpolation strategy.
    LogBilinear,
    /// Bicubic logarithmic interpolation strategy.
    LogBicubic,
    /// Tricubic logarithmic interpolation strategy.
    LogTricubic,
    /// Linear interpolation for N-dimensional data.
    NDLinear,
}

impl From<&InterpolatorType> for PyInterpolatorType {
    fn from(basis: &InterpolatorType) -> Self {
        match basis {
            InterpolatorType::Bilinear => Self::Bilinear,
            InterpolatorType::LogBilinear => Self::LogBilinear,
            InterpolatorType::LogBicubic => Self::LogBicubic,
            InterpolatorType::LogTricubic => Self::LogTricubic,
            InterpolatorType::InterpNDLinear => Self::NDLinear,
        }
    }
}

/// Grid metadata.
#[pyclass(name = "MetaData")]
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct PyMetaData {
    pub(crate) meta: MetaData,
}

#[pymethods]
impl PyMetaData {
    /// The description of the grid.
    #[must_use]
    pub const fn description(&self) -> &String {
        &self.meta.set_desc
    }

    /// The index of the grid.
    #[must_use]
    pub const fn set_index(&self) -> u32 {
        self.meta.set_index
    }

    /// The number of sets in the grid.
    #[must_use]
    pub const fn number_sets(&self) -> u32 {
        self.meta.num_members
    }

    /// The minimum value of `x` in the grid.
    #[must_use]
    pub const fn x_min(&self) -> f64 {
        self.meta.x_min
    }

    /// The maximum value of `x` in the grid.
    #[must_use]
    pub const fn x_max(&self) -> f64 {
        self.meta.x_max
    }

    /// The minimum value of `q` in the grid.
    #[must_use]
    pub const fn q_min(&self) -> f64 {
        self.meta.q_min
    }

    /// The maximum value of `q` in the grid.
    #[must_use]
    pub const fn q_max(&self) -> f64 {
        self.meta.q_max
    }

    /// The particle IDs of the grid.
    #[must_use]
    pub const fn pids(&self) -> &Vec<i32> {
        &self.meta.flavors
    }

    /// The format of the grid.
    #[must_use]
    pub const fn format(&self) -> &String {
        &self.meta.format
    }

    /// The values of `q` for the running of the strong coupling constant.
    #[must_use]
    pub const fn alphas_q(&self) -> &Vec<f64> {
        &self.meta.alphas_q_values
    }

    /// The values of the running of the strong coupling constant.
    #[must_use]
    pub const fn alphas_values(&self) -> &Vec<f64> {
        &self.meta.alphas_vals
    }

    /// Whether the grid is polarised.
    #[must_use]
    pub const fn is_polarised(&self) -> bool {
        self.meta.polarised
    }

    /// The type of the set.
    #[must_use]
    pub fn set_type(&self) -> PySetType {
        PySetType::from(&self.meta.set_type)
    }

    /// The interpolation method used for the grid.
    #[must_use]
    pub fn interpolator_type(&self) -> PyInterpolatorType {
        PyInterpolatorType::from(&self.meta.interpolator_type)
    }
}

/// Registers the `metadata` submodule with the parent Python module.
///
/// Parameters
/// ----------
/// `parent_module` : pyo3.Bound[pyo3.types.PyModule]
///     The parent Python module to which the `metadata` submodule will be added.
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
    let m = PyModule::new(parent_module.py(), "metadata")?;
    m.setattr(pyo3::intern!(m.py(), "__doc__"), "Interface for PDF.")?;
    pyo3::py_run!(
        parent_module.py(),
        m,
        "import sys; sys.modules['neopdf.metadata'] = m"
    );
    m.add_class::<PySetType>()?;
    m.add_class::<PyInterpolatorType>()?;
    m.add_class::<PyMetaData>()?;
    parent_module.add_submodule(&m)
}
