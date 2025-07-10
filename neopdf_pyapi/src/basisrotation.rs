//! TODO

use neopdf::basisrotation::PidBasis;
use pyo3::prelude::*;

/// PyO3 wrapper to :rustdoc:`neopdf::basisrotation::PidBasis <pids/enum.PidBasis.html>`.
#[pyclass(eq, eq_int, name = "PidBasis")]
#[derive(Clone, PartialEq)]
pub enum PyPidBasis {
    /// PDG Monte Carlo IDs.
    Pdg,
    /// NNPDF's evolution basis IDs.
    Evol,
}

impl From<PyPidBasis> for PidBasis {
    fn from(basis: PyPidBasis) -> Self {
        match basis {
            PyPidBasis::Pdg => Self::Pdg,
            PyPidBasis::Evol => Self::Evol,
        }
    }
}

/// Registers the `basisrotation` submodule with the parent Python module.
///
/// This function is typically called during the initialization of the
/// `neopdf` Python package to expose the `PidBasis` class.
///
/// Parameters
/// ----------
/// `parent_module` : pyo3.Bound[pyo3.types.PyModule]
///     The parent Python module to which the `basisrotation` submodule will be added.
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
    let m = PyModule::new(parent_module.py(), "basisrotation")?;
    m.setattr(pyo3::intern!(m.py(), "__doc__"), "PIDs interface.")?;
    pyo3::py_run!(
        parent_module.py(),
        m,
        "import sys; sys.modules['neopdf.basisrotation'] = m"
    );
    m.add_class::<PyPidBasis>()?;
    parent_module.add_submodule(&m)
}
