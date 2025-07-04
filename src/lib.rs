use ndarray::{s, Array1, Array3};
use ninterp::interpolator::{Extrapolate, Interp2D};
use ninterp::prelude::*;
use rayon::prelude::*;
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

/// Stores the PDF grid data for a single subgrid.
#[derive(Debug)]
pub struct Subgrid {
    /// Array of x-values (momentum fraction).
    pub xs: Array1<f64>,
    /// Array of Q2-values (energy scale squared).
    pub q2s: Array1<f64>,
    /// 3D grid of PDF values, indexed as `[flavor_index, x_index, q2_index]`.
    pub grid: Array3<f64>,
    x_min: f64,
    x_max: f64,
    q2_min: f64,
    q2_max: f64,
}

impl Subgrid {
    /// Creates a new `Subgrid` from raw data.
    pub fn new(xs: Vec<f64>, q2s: Vec<f64>, nflav: usize, grid_data: Vec<f64>) -> Self {
        let nx = xs.len();
        let nq2 = q2s.len();

        let x_min = *xs.first().unwrap();
        let x_max = *xs.last().unwrap();
        let q2_min = *q2s.first().unwrap();
        let q2_max = *q2s.last().unwrap();

        let xs = Array1::from_vec(xs);
        let q2s = Array1::from_vec(q2s);
        let grid = Array3::from_shape_vec((nx, nq2, nflav), grid_data)
            .expect("Failed to create grid from data")
            .permuted_axes([2, 0, 1]) // Permute (x, q2, flav) -> (flav, x, q2)
            .as_standard_layout()
            .to_owned();

        Self {
            xs,
            q2s,
            grid,
            x_min,
            x_max,
            q2_min,
            q2_max,
        }
    }

    /// Checks if a given (x, q2) point is within the boundaries of this subgrid.
    pub fn in_bounds(&self, x: f64, q2: f64) -> bool {
        x >= self.x_min && x <= self.x_max && q2 >= self.q2_min && q2 <= self.q2_max
    }
}

/// Stores the PDF grid data, including x-values, Q2-values, flavors, and the 3D grid itself.
#[derive(Debug)]
pub struct KnotArray {
    /// Array of flavor IDs.
    pub flavors: Array1<i32>,
    /// Vector of subgrids.
    pub subgrids: Vec<Subgrid>,
}

impl KnotArray {
    /// Creates a new `KnotArray` from raw data.
    ///
    /// # Arguments
    ///
    /// * `subgrid_data` - A vector of tuples, where each tuple contains the data for a subgrid.
    /// * `flavors` - A vector of flavor IDs.
    pub fn new(subgrid_data: Vec<(Vec<f64>, Vec<f64>, Vec<f64>)>, flavors: Vec<i32>) -> Self {
        let nflav = flavors.len();
        let flavors = Array1::from_vec(flavors);

        let subgrids = subgrid_data
            .into_iter()
            .map(|(xs, q2s, grid_data)| Subgrid::new(xs, q2s, nflav, grid_data))
            .collect();

        Self { flavors, subgrids }
    }

    /// Retrieves the PDF value (xf) at a specific knot point.
    ///
    /// # Arguments
    ///
    /// * `ix` - The index of the x-value.
    /// * `iq2` - The index of the Q2-value.
    /// * `id` - The flavor ID.
    /// * `subgrid_id` - The subgrid to be used.
    pub fn xf(&self, ix: usize, iq2: usize, id: i32, subgrid_id: usize) -> f64 {
        let pid_index = self.flavors.iter().position(|&p| p == id).unwrap();
        self.subgrids[subgrid_id].grid[[pid_index, ix, iq2]]
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
    interpolators: Vec<Vec<Box<dyn DynInterpolator>>>,
    alphas_interpolator: Interp1DOwned<f64, interpolation::AlphaSCubicInterpolation>,
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
        let mut interpolators: Vec<Vec<Box<dyn DynInterpolator>>> = Vec::new();
        for subgrid in &knot_array.subgrids {
            let mut subgrid_interpolators: Vec<Box<dyn DynInterpolator>> = Vec::new();
            for i in 0..knot_array.flavors.len() {
                let grid_slice = subgrid.grid.slice(s![i, .., ..]);

                let interp: Box<dyn DynInterpolator> = match info.interpolator_type.as_str() {
                    "LogBilinear" => Box::new(
                        Interp2D::new(
                            subgrid.xs.to_owned(),
                            subgrid.q2s.to_owned(),
                            grid_slice.to_owned(),
                            interpolation::LogBilinearInterpolation,
                            Extrapolate::Error,
                        )
                        .unwrap(),
                    ),
                    "Bilinear" => Box::new(
                        Interp2D::new(
                            subgrid.xs.to_owned(),
                            subgrid.q2s.to_owned(),
                            grid_slice.to_owned(),
                            interpolation::BilinearInterpolation,
                            // TODO: Implement extrapolation
                            Extrapolate::Error,
                        )
                        .unwrap(),
                    ),
                    "LogBicubic" => Box::new(
                        Interp2D::new(
                            subgrid.xs.to_owned(),
                            subgrid.q2s.to_owned(),
                            grid_slice.to_owned(),
                            interpolation::LogBicubicInterpolation::default(),
                            // TODO: Implement extrapolation
                            Extrapolate::Error,
                        )
                        .unwrap(),
                    ),
                    _ => panic!("Unknown interpolator type: {}", info.interpolator_type),
                };
                subgrid_interpolators.push(interp);
            }
            interpolators.push(subgrid_interpolators);
        }

        let alphas_q2s: Vec<f64> = info.alphas_q_values.iter().map(|&q| q * q).collect();
        let alphas_interpolator = Interp1D::new(
            alphas_q2s.into(),
            info.alphas_vals.clone().into(),
            interpolation::AlphaSCubicInterpolation,
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

    /// Finds the index of the subgrid that contains the given (x, q2) point.
    fn find_subgrid_index(&self, x: f64, q2: f64) -> Option<usize> {
        self.knot_array
            .subgrids
            .iter()
            .position(|subgrid| subgrid.in_bounds(x, q2))
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
        if let Some(subgrid_index) = self.find_subgrid_index(x, q2) {
            let pid_index = self
                .knot_array
                .flavors
                .iter()
                .position(|&p| p == id)
                .unwrap();
            self.interpolators[subgrid_index][pid_index]
                .interpolate_point(&[x, q2])
                .unwrap_or(0.0)
        } else {
            // Handle the case where the point is out of all subgrid bounds
            // For now, we return 0.0, but a more sophisticated error handling might be needed.
            0.0
        }
    }

    /// Interpolates the PDF value (xf) for some lists of flavors, xs, and Q2s.
    pub fn xfxq2s(&self, ids: Vec<i32>, xs: Vec<f64>, q2s: Vec<f64>) -> Array3<f64> {
        let shape = [ids.len(), xs.len(), q2s.len()];
        let flatten_len = shape.iter().product();

        // Generate all indices and compute in parallel
        let data: Vec<f64> = (0..flatten_len)
            .into_par_iter()
            .map(|linear_idx| {
                // Convert linear index to 3D indices
                let k = linear_idx % shape[2];
                let j = (linear_idx / shape[2]) % shape[1];
                let i = linear_idx / (shape[1] * shape[2]);

                self.xfxq2(ids[i], xs[j], q2s[k])
            })
            .collect();

        Array3::from_shape_vec(shape, data).unwrap()
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

pub struct PDF {
    grid_pdf: GridPDF,
}

impl PDF {
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
    /// A `PDF` instance representing the loaded PDF set.
    pub fn load(path: &Path) -> PDF {
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
        let (subgrid_data, flavors) = parser::read_data(&data_path);
        let knot_array = KnotArray::new(subgrid_data, flavors);

        PDF {
            grid_pdf: GridPDF::new(info, knot_array),
        }
    }

    /// Loads all PDF sets from the specified path in parallel.
    ///
    /// This function reads the `.info` file and all `.dat` member files
    /// to construct a `Vec<PDF>` object.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the directory containing the PDF set files.
    ///
    /// # Returns
    ///
    /// A `Vec<PDF>` instance representing all loaded PDF sets.
    pub fn load_pdfs(path: &Path) -> Vec<PDF> {
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
                let (subgrid_data, flavors) = parser::read_data(&data_path);
                let knot_array = KnotArray::new(subgrid_data, flavors);
                PDF {
                    grid_pdf: GridPDF::new(info.clone(), knot_array),
                }
            })
            .collect()
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
        self.grid_pdf.xfxq2(id, x, q2)
    }

    /// Interpolates the PDF value (xf) for some lists of flavors, xs, and Q2s.
    pub fn xfxq2s(&self, ids: Vec<i32>, xs: Vec<f64>, q2s: Vec<f64>) -> Array3<f64> {
        self.grid_pdf.xfxq2s(ids, xs, q2s)
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
        self.grid_pdf.alphas_q2(q2)
    }

    /// Returns the metadata info of the PDF.
    pub fn info(&self) -> Info {
        self.grid_pdf.info()
    }

    /// Retrieves the PDF value (xf) at a specific knot point.
    pub fn xf(&self, ix: usize, iq2: usize, id: i32, subgrid_id: usize) -> f64 {
        self.grid_pdf.knot_array.xf(ix, iq2, id, subgrid_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knot_array_new() {
        let subgrid_data = vec![(
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0],
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0,
            ],
        )];
        let flavors = vec![21, 22];
        let knot_array = KnotArray::new(subgrid_data, flavors);
        assert_eq!(knot_array.subgrids[0].grid.shape(), &[2, 3, 2]);
    }
}
