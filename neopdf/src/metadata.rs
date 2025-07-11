use serde::{Deserialize, Serialize};

/// Represents the type of PDF set.
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum SetType {
    #[default]
    Pdf,
    Fragfn,
}

/// Represents the type of interpolator used for the PDF.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum InterpolatorType {
    Bilinear,
    LogBilinear,
    #[default]
    LogBicubic,
    LogTricubic,
    InterpNDLinear,
}

/// Represents the information block of a PDF set, typically found in an `.info` file.
/// This struct is deserialized from a YAML-like format.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MetaData {
    /// Description of the PDF set.
    #[serde(rename = "SetDesc")]
    pub set_desc: String,
    /// Index of the PDF set.
    #[serde(rename = "SetIndex")]
    pub set_index: u32,
    /// Number of members in the PDF set (e.g., for error analysis).
    #[serde(rename = "NumMembers")]
    pub num_members: u32,
    /// Minimum x-value for which the PDF is valid.
    #[serde(rename = "XMin")]
    pub x_min: f64,
    /// Maximum x-value for which the PDF is valid.
    #[serde(rename = "XMax")]
    pub x_max: f64,
    /// Minimum Q-value (energy scale) for which the PDF is valid.
    #[serde(rename = "QMin")]
    pub q_min: f64,
    /// Maximum Q-value (energy scale) for which the PDF is valid.
    #[serde(rename = "QMax")]
    pub q_max: f64,
    /// List of particle data group (PDG) IDs for the flavors included in the PDF.
    #[serde(rename = "Flavors")]
    pub flavors: Vec<i32>,
    /// Format of the PDF data.
    #[serde(rename = "Format")]
    pub format: String,
    /// AlphaS Q values (non-squared) for interpolation.
    #[serde(rename = "AlphaS_Qs", default)]
    pub alphas_q_values: Vec<f64>,
    /// AlphaS values for interpolation.
    #[serde(rename = "AlphaS_Vals", default)]
    pub alphas_vals: Vec<f64>,
    /// Polarisation of the hadrons.
    #[serde(rename = "Polarized", default)]
    pub polarised: bool,
    /// Type of the hadrons.
    #[serde(rename = "SetType", default)]
    pub set_type: SetType,
    /// Type of interpolator used for the PDF (e.g., "LogBicubic").
    #[serde(rename = "InterpolatorType", default)]
    pub interpolator_type: InterpolatorType,
}
