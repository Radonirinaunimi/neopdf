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
            SetType::SpaceLike => Self::Pdf,
            SetType::TimeLike => Self::Fragfn,
        }
    }
}

impl From<&PySetType> for SetType {
    fn from(set_type: &PySetType) -> Self {
        match set_type {
            PySetType::Pdf => Self::SpaceLike,
            PySetType::Fragfn => Self::TimeLike,
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

impl From<&PyInterpolatorType> for InterpolatorType {
    fn from(basis: &PyInterpolatorType) -> Self {
        match basis {
            PyInterpolatorType::Bilinear => Self::Bilinear,
            PyInterpolatorType::LogBilinear => Self::LogBilinear,
            PyInterpolatorType::LogBicubic => Self::LogBicubic,
            PyInterpolatorType::LogTricubic => Self::LogTricubic,
            PyInterpolatorType::NDLinear => Self::InterpNDLinear,
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
    /// Constructor for PyMetaData.
    #[new]
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (
        set_desc,
        set_index,
        num_members,
        x_min,
        x_max,
        q_min,
        q_max,
        flavors,
        format,
        alphas_q_values = vec![],
        alphas_vals = vec![],
        polarised = false,
        set_type = PySetType::Pdf,
        interpolator_type = PyInterpolatorType::LogBicubic,
        error_type = "replicas".to_string(),
        hadron_pid = 2212
    ))]
    pub fn new(
        set_desc: String,
        set_index: u32,
        num_members: u32,
        x_min: f64,
        x_max: f64,
        q_min: f64,
        q_max: f64,
        flavors: Vec<i32>,
        format: String,
        alphas_q_values: Vec<f64>,
        alphas_vals: Vec<f64>,
        polarised: bool,
        set_type: PySetType,
        interpolator_type: PyInterpolatorType,
        error_type: String,
        hadron_pid: i32,
    ) -> Self {
        let meta = MetaData {
            set_desc,
            set_index,
            num_members,
            x_min,
            x_max,
            q_min,
            q_max,
            flavors,
            format,
            alphas_q_values,
            alphas_vals,
            polarised,
            set_type: SetType::from(&set_type),
            interpolator_type: InterpolatorType::from(&interpolator_type),
            error_type,
            hadron_pid,
        };

        Self { meta }
    }

    /// Convert to Python dictionary
    ///
    /// # Errors
    ///
    /// Raises an erro if the values are not Python compatible.
    pub fn to_dict(&self, py: Python) -> PyResult<PyObject> {
        let dict = pyo3::types::PyDict::new(py);

        let set_type = match &self.meta.set_type {
            SetType::SpaceLike => "PDF",
            SetType::TimeLike => "FragFn",
        };

        let interpolator_type = match &self.meta.interpolator_type {
            InterpolatorType::Bilinear => "Bilinear",
            InterpolatorType::LogBilinear => "LogBilinear",
            InterpolatorType::LogBicubic => "LogBicubic",
            InterpolatorType::LogTricubic => "LogTricubic",
            InterpolatorType::InterpNDLinear => "NDLinear",
        };

        dict.set_item("set_desc", &self.meta.set_desc)?;
        dict.set_item("set_index", self.meta.set_index)?;
        dict.set_item("num_members", self.meta.num_members)?;
        dict.set_item("x_min", self.meta.x_min)?;
        dict.set_item("x_max", self.meta.x_max)?;
        dict.set_item("q_min", self.meta.q_min)?;
        dict.set_item("q_max", self.meta.q_max)?;
        dict.set_item("flavors", &self.meta.flavors)?;
        dict.set_item("format", &self.meta.format)?;
        dict.set_item("alphas_q_values", &self.meta.alphas_q_values)?;
        dict.set_item("alphas_vals", &self.meta.alphas_vals)?;
        dict.set_item("polarised", self.meta.polarised)?;
        dict.set_item("set_type", set_type)?;
        dict.set_item("interpolator_type", interpolator_type)?;
        dict.set_item("error_type", &self.meta.error_type)?;
        dict.set_item("hadron_pid", self.meta.hadron_pid)?;

        Ok(dict.into())
    }

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

    /// The type of error.
    #[must_use]
    pub const fn error_type(&self) -> &String {
        &self.meta.error_type
    }

    /// The hadron PID.
    #[must_use]
    pub const fn hadron_pid(&self) -> i32 {
        self.meta.hadron_pid
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
