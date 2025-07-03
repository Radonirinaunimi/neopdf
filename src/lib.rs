use ndarray::{s, Array1, Array3};
use ninterp::interpolator::{Extrapolate, Interp2D};
use ninterp::prelude::*;
use serde::Deserialize;
use std::path::Path;

pub mod interpolation;
pub mod parser;
pub mod utils;

/// Represents the information block of a PDF set, typically found in an `.info` file.
/// This struct is deserialized from a YAML-like format.
#[derive(Clone, Debug, Deserialize)]
pub struct Info {
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
    /// Type of interpolator used for the PDF (e.g., "LogBilinear").
    #[serde(rename = "InterpolatorType", default = "default_interpolator_type")]
    pub interpolator_type: String,
    /// AlphaS Q values (non-squared) for interpolation.
    #[serde(rename = "AlphaS_Qs", default)]
    pub alphas_q_values: Vec<f64>,
    /// AlphaS values for interpolation.
    #[serde(rename = "AlphaS_Vals", default)]
    pub alphas_vals: Vec<f64>,
}

/// Provides the default interpolator type, "LogBilinear", for `Info`.
fn default_interpolator_type() -> String {
    "LogBicubic".to_string()
}

/// Stores the PDF grid data, including x-values, Q2-values, flavors, and the 3D grid itself.
#[derive(Debug)]
pub struct KnotArray {
    /// Array of x-values (momentum fraction).
    pub xs: Array1<f64>,
    /// Array of Q2-values (energy scale squared).
    pub q2s: Array1<f64>,
    /// Array of flavor IDs.
    pub flavors: Array1<i32>,
    /// 3D grid of PDF values, indexed as `[flavor_index, x_index, q2_index]`.
    pub grid: Array3<f64>,
}

impl KnotArray {
    /// Creates a new `KnotArray` from raw data.
    ///
    /// # Arguments
    ///
    /// * `xs` - A vector of x-values.
    /// * `q2s` - A vector of Q2-values.
    /// * `flavors` - A vector of flavor IDs.
    /// * `grid_data` - A flat vector of PDF values.
    pub fn new(xs: Vec<f64>, q2s: Vec<f64>, flavors: Vec<i32>, grid_data: Vec<f64>) -> Self {
        let nx = xs.len();
        let nq2 = q2s.len();
        let nflav = flavors.len();

        let xs = Array1::from_vec(xs);
        let q2s = Array1::from_vec(q2s);
        let flavors = Array1::from_vec(flavors);
        let grid = Array3::from_shape_vec((nx, nq2, nflav), grid_data)
            .expect("Failed to create grid from data")
            .permuted_axes([2, 0, 1]) // Permute (x, q2, flav) -> (flav, x, q2)
            .as_standard_layout()
            .to_owned();

        Self {
            xs,
            q2s,
            flavors,
            grid,
        }
    }

    /// Retrieves the PDF value (xf) at a specific knot point.
    ///
    /// # Arguments
    ///
    /// * `ix` - The index of the x-value.
    /// * `iq2` - The index of the Q2-value.
    /// * `id` - The flavor ID.
    pub fn xf(&self, ix: usize, iq2: usize, id: i32) -> f64 {
        let pid_index = self.flavors.iter().position(|&p| p == id).unwrap();
        self.grid[[pid_index, ix, iq2]]
    }
}

/// A trait for dynamic interpolation, allowing different interpolation strategies to be used interchangeably.
pub trait DynInterpolator: Send + Sync {
    /// Interpolates a point given its coordinates.
    ///
    /// # Arguments
    ///
    /// * `point` - A 2-element array `[x, y]` representing the coordinates to interpolate at.
    fn interpolate_point(&self, point: &[f64; 2]) -> Result<f64, ninterp::error::InterpolateError>;
}

impl<S> DynInterpolator for Interp2DOwned<f64, S>
where
    S: ninterp::strategy::traits::Strategy2D<ndarray::OwnedRepr<f64>>
        + 'static
        + Clone
        + Send
        + Sync,
{
    fn interpolate_point(&self, point: &[f64; 2]) -> Result<f64, ninterp::error::InterpolateError> {
        self.interpolate(point)
    }
}

/// Represents a Parton Distribution Function (PDF) grid, containing the PDF info, knot array, and interpolators.
pub struct GridPDF {
    info: Info,
    /// The underlying knot array containing the PDF grid data.
    pub knot_array: KnotArray,
    interpolators: Vec<Box<dyn DynInterpolator>>,
    alphas_interpolator: Interp1DOwned<f64, interpolation::AlphaSCubicStrategy>,
}

impl GridPDF {
    /// Creates a new `GridPDF` instance.
    ///
    /// Initializes interpolators for each flavor based on the `info.interpolator_type`.
    ///
    /// # Arguments
    ///
    /// * `info` - The `Info` struct containing metadata about the PDF set.
    /// * `knot_array` - The `KnotArray` containing the PDF grid data.
    pub fn new(info: Info, knot_array: KnotArray) -> Self {
        let mut interpolators: Vec<Box<dyn DynInterpolator>> = Vec::new();
        for i in 0..knot_array.flavors.len() {
            let grid_slice = knot_array.grid.slice(s![i, .., ..]);

            let interp: Box<dyn DynInterpolator> = match info.interpolator_type.as_str() {
                "LogBilinear" => Box::new(
                    Interp2D::new(
                        knot_array.xs.to_owned(),
                        knot_array.q2s.to_owned(),
                        grid_slice.to_owned(),
                        interpolation::LogBilinearStrategy,
                        Extrapolate::Error,
                    )
                    .unwrap(),
                ),
                "Bilinear" => Box::new(
                    Interp2D::new(
                        knot_array.xs.to_owned(),
                        knot_array.q2s.to_owned(),
                        grid_slice.to_owned(),
                        interpolation::Bilinear,
                        // TODO: Implement extrapolation
                        Extrapolate::Error,
                    )
                    .unwrap(),
                ),
                "LogBicubic" => Box::new(
                    Interp2D::new(
                        knot_array.xs.to_owned(),
                        knot_array.q2s.to_owned(),
                        grid_slice.to_owned(),
                        interpolation::LogBicubic::default(),
                        // TODO: Implement extrapolation
                        Extrapolate::Error,
                    )
                    .unwrap(),
                ),
                _ => panic!("Unknown interpolator type: {}", info.interpolator_type),
            };
            interpolators.push(interp);
        }

        let alphas_q2s: Vec<f64> = info.alphas_q_values.iter().map(|&q| q * q).collect();
        let alphas_interpolator = Interp1D::new(
            alphas_q2s.into(),
            info.alphas_vals.clone().into(),
            interpolation::AlphaSCubicStrategy,
            Extrapolate::Error,
        )
        .unwrap();

        Self {
            info,
            knot_array,
            interpolators,
            alphas_interpolator,
        }
    }

    /// Interpolates the PDF value (xf) for a given flavor, x, and Q2.
    ///
    /// # Arguments
    ///
    /// * `id` - The flavor ID.
    /// * `x` - The x-value (momentum fraction).
    /// * `q2` - The Q2-value (energy scale squared).
    ///
    /// # Returns
    ///
    /// The interpolated PDF value. Returns 0.0 if extrapolation is attempted and not allowed.
    pub fn xfxq2(&self, id: i32, x: f64, q2: f64) -> f64 {
        let pid_index = self
            .knot_array
            .flavors
            .iter()
            .position(|&p| p == id)
            .unwrap();
        self.interpolators[pid_index]
            .interpolate_point(&[x, q2])
            .unwrap_or(0.0)
    }

    /// Interpolates the alpha_s value for a given Q2.
    ///
    /// # Arguments
    ///
    /// * `q2` - The Q2-value (energy scale squared).
    ///
    /// # Returns
    ///
    /// The interpolated alpha_s value.
    pub fn alphas_q2(&self, q2: f64) -> f64 {
        self.alphas_interpolator.interpolate(&[q2]).unwrap_or(0.0)
    }

    /// Returns the metadata info of the PDF.
    pub fn info(&self) -> Info {
        self.info.clone()
    }
}

/// Loads a PDF set from the specified path.
///
/// This function reads the `.info` file and the first `.dat` member file
/// to construct a `GridPDF` object.
///
/// # Arguments
///
/// * `path` - The path to the directory containing the PDF set files.
///
/// # Returns
///
/// A `GridPDF` instance representing the loaded PDF set.
pub fn load(path: &Path) -> GridPDF {
    let info_path = path.join(format!(
        "{}.info",
        path.file_name().unwrap().to_str().unwrap()
    ));
    let info: Info = parser::read_info(&info_path).unwrap();

    // For now, only load the first member
    let data_path = path.join(format!(
        "{}_{:04}.dat",
        path.file_name().unwrap().to_str().unwrap(),
        0
    ));
    let (xs, q2s, flavors, grid_data) = parser::read_data(&data_path);
    let knot_array = KnotArray::new(xs, q2s, flavors, grid_data);

    GridPDF::new(info, knot_array)
}

/// Loads all PDF sets from the specified path in parallel.
///
/// This function reads the `.info` file and all `.dat` member files
/// to construct a `Vec<GridPDF>` object.
///
/// # Arguments
///
/// * `path` - The path to the directory containing the PDF set files.
///
/// # Returns
///
/// A `Vec<GridPDF>` instance representing all loaded PDF sets.
pub fn load_pdfs(path: &Path) -> Vec<GridPDF> {
    use rayon::prelude::*;

    let info_path = path.join(format!(
        "{}.info",
        path.file_name().unwrap().to_str().unwrap()
    ));
    let info: Info = parser::read_info(&info_path).unwrap();

    (0..info.num_members)
        .into_par_iter()
        .map(|i| {
            let data_path = path.join(format!(
                "{}_{:04}.dat",
                path.file_name().unwrap().to_str().unwrap(),
                i
            ));
            let (xs, q2s, flavors, grid_data) = parser::read_data(&data_path);
            let knot_array = KnotArray::new(xs, q2s, flavors, grid_data);
            GridPDF::new(info.clone(), knot_array)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knot_array_new() {
        let xs = vec![1.0, 2.0, 3.0];
        let q2s = vec![4.0, 5.0];
        let flavors = vec![21, 22];
        let grid_data = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0,
        ];
        let knot_array = KnotArray::new(xs, q2s, flavors, grid_data);
        assert_eq!(knot_array.grid.shape(), &[2, 3, 2]);
    }
}
