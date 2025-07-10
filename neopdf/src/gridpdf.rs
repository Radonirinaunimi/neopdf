use ndarray::{s, Array1, Array3, Array5};
use ninterp::interpolator::{Extrapolate, Interp2D, InterpND};
use ninterp::prelude::*;
use ninterp::strategy::Linear;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::interpolation::{
    AlphaSCubicInterpolation, BilinearInterpolation, LogBicubicInterpolation,
    LogBilinearInterpolation, LogTricubicInterpolation,
};
use super::metadata::{InterpolatorType, MetaData};
use super::parser::SubgridData;

#[derive(Debug, Error)]
pub enum Error {
    #[error("No subgrid found for x={x}, q2={q2}")]
    SubgridNotFound { x: f64, q2: f64 },
}

/// Stores the PDF grid data for a single subgrid.
#[derive(Debug, Serialize, Deserialize)]
pub struct SubGrid {
    /// Array of x-values (momentum fraction).
    pub xs: Array1<f64>,
    /// Array of Q2-values (energy scale squared).
    pub q2s: Array1<f64>,
    /// 5D grid of PDF values, indexed as `[nucleons, alphas, pids, x, Q2]`.
    pub grid: Array5<f64>,
    /// Numbers representing the nucleons contained in the PDF.
    pub nucleons: Vec<u32>,
    /// Values of alphas contained in the PDF.
    pub alphas: Vec<f64>,
    /// Minimum value of the `x` subgrid
    x_min: f64,
    /// Maximum value of the `x` subgrid
    x_max: f64,
    /// Minimum value of the `Q2` subgrid
    q2_min: f64,
    /// Maximum value of the `Q2` subgrid
    q2_max: f64,
}

impl SubGrid {
    /// Creates a new `Subgrid` from raw data.
    pub fn new(
        nucleon_numbers: Vec<u32>,
        alphas_values: Vec<f64>,
        xs: Vec<f64>,
        q2s: Vec<f64>,
        nflav: usize,
        grid_data: Vec<f64>,
    ) -> Self {
        let n_nucleons = nucleon_numbers.len();
        let n_alphas = alphas_values.len();
        let nx = xs.len();
        let nq2 = q2s.len();

        let x_subgrid_min = *xs.first().unwrap();
        let x_subgrid_max = *xs.last().unwrap();
        let q2_subgrid_min = *q2s.first().unwrap();
        let q2_subgrid_max = *q2s.last().unwrap();

        let x_subgrid = Array1::from_vec(xs);
        let q2_subgrid = Array1::from_vec(q2s);
        let subgrid_array =
            Array5::from_shape_vec((n_nucleons, n_alphas, nx, nq2, nflav), grid_data)
                .expect("Failed to create grid from data")
                // Permute  (nucleons, alphas, x, Q2, pids) -> (nucleons, alphas, pids, x, Q2)
                .permuted_axes([0, 1, 4, 2, 3])
                .as_standard_layout()
                .to_owned();

        Self {
            xs: x_subgrid,
            q2s: q2_subgrid,
            grid: subgrid_array,
            nucleons: nucleon_numbers,
            alphas: alphas_values,
            x_min: x_subgrid_min,
            x_max: x_subgrid_max,
            q2_min: q2_subgrid_min,
            q2_max: q2_subgrid_max,
        }
    }

    /// Checks if a given (x, q2) point is within the boundaries of this subgrid.
    pub fn is_in_subgrid(&self, x: f64, q2: f64) -> bool {
        x >= self.x_min && x <= self.x_max && q2 >= self.q2_min && q2 <= self.q2_max
    }
}

/// Stores the PDF grid data, including x-values, Q2-values, flavors, and the 3D grid itself.
#[derive(Debug, Serialize, Deserialize)]
pub struct GridArray {
    /// Array of flavor IDs.
    pub pids: Array1<i32>,
    /// Vector of subgrids.
    pub subgrids: Vec<SubGrid>,
}

impl GridArray {
    /// Creates a new `KnotArray` from raw data.
    ///
    /// # Arguments
    ///
    /// * `subgrid_data` - A vector of tuples, where each tuple contains the data for a subgrid.
    /// * `pids` - A vector of flavor IDs.
    pub fn new(subgrid_data: Vec<SubgridData>, pids: Vec<i32>) -> Self {
        let nflav = pids.len();
        let pids = Array1::from_vec(pids);

        let subgrids = subgrid_data
            .into_iter()
            .map(|subgrid| {
                SubGrid::new(
                    subgrid.nucleons,
                    subgrid.alphas,
                    subgrid.xs,
                    subgrid.q2s,
                    nflav,
                    subgrid.grid_data,
                )
            })
            .collect();

        Self { pids, subgrids }
    }

    /// Retrieves the PDF value (xf) at a specific knot point.
    ///
    /// # Arguments
    ///
    /// * `ix` - The index of the x-value.
    /// * `iq2` - The index of the Q2-value.
    /// * `id` - The flavor ID.
    /// * `subgrid_id` - The subgrid to be used.
    pub fn xf_from_index(
        &self,
        i_nucleons: usize,
        i_alphas: usize,
        ix: usize,
        iq2: usize,
        id: i32,
        subgrid_id: usize,
    ) -> f64 {
        let pid_index = self.pids.iter().position(|&p| p == id).unwrap();
        self.subgrids[subgrid_id].grid[[i_nucleons, i_alphas, pid_index, ix, iq2]]
    }
}

/// A trait for dynamic interpolation, allowing different interpolation strategies to be
/// used interchangeably.
pub trait DynInterpolator: Send + Sync {
    /// Interpolates a point given its coordinates.
    ///
    /// # Arguments
    ///
    /// * `point` - A 2-element array `[x, y]` representing the coordinates to interpolate at.
    fn interpolate_point(&self, point: &[f64]) -> Result<f64, ninterp::error::InterpolateError>;
}

impl<S> DynInterpolator for Interp2DOwned<f64, S>
where
    S: ninterp::strategy::traits::Strategy2D<ndarray::OwnedRepr<f64>>
        + 'static
        + Clone
        + Send
        + Sync,
{
    fn interpolate_point(&self, point: &[f64]) -> Result<f64, ninterp::error::InterpolateError> {
        // Interp2D expects a [f64; 2] array, so we need to convert the slice.
        // This assumes that for Interp2D, the point will always have 2 elements.
        if point.len() != 2 {
            return Err(ninterp::error::InterpolateError::Other(
                "Expected a 2-element array for 2D interpolation".to_string(),
            ));
        }
        let point_array: [f64; 2] = [point[0], point[1]];
        self.interpolate(&point_array)
    }
}

impl<S> DynInterpolator for Interp3DOwned<f64, S>
where
    S: ninterp::strategy::traits::Strategy3D<ndarray::OwnedRepr<f64>>
        + 'static
        + Clone
        + Send
        + Sync,
{
    fn interpolate_point(&self, point: &[f64]) -> Result<f64, ninterp::error::InterpolateError> {
        // Interp2D expects a [f64; 2] array, so we need to convert the slice.
        // This assumes that for Interp2D, the point will always have 2 elements.
        if point.len() != 2 {
            return Err(ninterp::error::InterpolateError::Other(
                "Expected a 2-element array for 2D interpolation".to_string(),
            ));
        }
        let point_array: [f64; 3] = [point[0], point[1], point[3]];
        self.interpolate(&point_array)
    }
}

impl<S> DynInterpolator for InterpNDOwned<f64, S>
where
    S: ninterp::strategy::traits::StrategyND<ndarray::OwnedRepr<f64>>
        + 'static
        + Clone
        + Send
        + Sync,
{
    fn interpolate_point(&self, point: &[f64]) -> Result<f64, ninterp::error::InterpolateError> {
        self.interpolate(point)
    }
}

/// Represents a Parton Distribution Function (PDF) grid, containing the PDF info, knot array,
/// and interpolators.
pub struct GridPDF {
    info: MetaData,
    /// The underlying knot array containing the PDF grid data.
    pub knot_array: GridArray,
    interpolators: Vec<Vec<Box<dyn DynInterpolator>>>,
    alphas_interpolator: Interp1DOwned<f64, AlphaSCubicInterpolation>,
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
    pub fn new(info: MetaData, knot_array: GridArray) -> Self {
        let mut interpolators: Vec<Vec<Box<dyn DynInterpolator>>> = Vec::new();
        for subgrid in &knot_array.subgrids {
            let mut subgrid_interpolators: Vec<Box<dyn DynInterpolator>> = Vec::new();
            for i in 0..knot_array.pids.len() {
                // TODO: Currently, always ignoring the extra-alphas/nucleon information
                let n_nucleons = subgrid.nucleons.len();
                let n_alphas = subgrid.alphas.len();
                let nx = subgrid.xs.len();
                let nq2 = subgrid.q2s.len();

                let interp: Box<dyn DynInterpolator> = if n_nucleons == 1 && n_alphas == 1 {
                    // 2D interpolation (x, Q2)
                    let grid_slice = subgrid.grid.slice(s![0, 0, i, .., ..]);
                    match info.interpolator_type {
                        InterpolatorType::LogBilinear => Box::new(
                            Interp2D::new(
                                subgrid.xs.to_owned(),
                                subgrid.q2s.to_owned(),
                                grid_slice.to_owned(),
                                LogBilinearInterpolation,
                                Extrapolate::Error,
                            )
                            .unwrap(),
                        ),
                        InterpolatorType::Bilinear => Box::new(
                            Interp2D::new(
                                subgrid.xs.to_owned(),
                                subgrid.q2s.to_owned(),
                                grid_slice.to_owned(),
                                BilinearInterpolation,
                                Extrapolate::Error,
                            )
                            .unwrap(),
                        ),
                        InterpolatorType::LogBicubic => Box::new(
                            Interp2D::new(
                                subgrid.xs.to_owned(),
                                subgrid.q2s.to_owned(),
                                grid_slice.to_owned(),
                                LogBicubicInterpolation::default(),
                                Extrapolate::Error,
                            )
                            .unwrap(),
                        ),
                        _ => panic!("Unknown 2D interpolator type."),
                    }
                } else if (n_nucleons > 1 && n_alphas == 1) || (n_nucleons == 1 && n_alphas > 1) {
                    // 3D interpolation (nucleons/alphas, x, Q2)
                    let grid_data = subgrid.grid.slice(s![.., .., i, .., ..]).to_owned();
                    let (coords, reshaped_grid_data) = if n_nucleons > 1 {
                        // Varying nucleons, fixed alpha
                        (
                            subgrid
                                .nucleons
                                .iter()
                                .map(|&n| n as f64)
                                .collect::<Array1<f64>>(),
                            grid_data
                                .into_shape_with_order((n_nucleons, nx, nq2))
                                .unwrap(),
                        )
                    } else {
                        // Fixed nucleon, varying alphas
                        (
                            Array1::from(subgrid.alphas.to_owned()),
                            grid_data
                                .into_shape_with_order((n_alphas, nx, nq2))
                                .unwrap(),
                        )
                    };
                    match info.interpolator_type {
                        InterpolatorType::LogTricubic => Box::new(
                            Interp3D::new(
                                coords.to_owned(),
                                subgrid.xs.to_owned(),
                                subgrid.q2s.to_owned(),
                                reshaped_grid_data.to_owned(),
                                LogTricubicInterpolation,
                                Extrapolate::Error,
                            )
                            .unwrap(),
                        ),
                        _ => panic!("Unknown 3D interpolator type."),
                    }
                } else if n_nucleons > 1 && n_alphas > 1 {
                    // 4D interpolation (nucleons, alphas, x, Q2)
                    let grid_data = subgrid.grid.slice(s![.., .., i, .., ..]).to_owned();
                    let coords = vec![
                        subgrid
                            .nucleons
                            .iter()
                            .map(|&n| n as f64)
                            .collect::<Array1<f64>>(),
                        Array1::from(subgrid.alphas.to_owned()),
                        subgrid.xs.to_owned(),
                        subgrid.q2s.to_owned(),
                    ];
                    let reshaped_grid_data = grid_data
                        .into_shape_with_order((n_nucleons, n_alphas, nx, nq2))
                        .unwrap();
                    match info.interpolator_type {
                        InterpolatorType::InterpNDLinear => Box::new(
                            InterpND::new(
                                coords,
                                reshaped_grid_data.into_dyn(),
                                Linear,
                                Extrapolate::Error,
                            )
                            .unwrap(),
                        ),
                        _ => panic!("Unknown 4D interpolator type."),
                    }
                } else {
                    panic!("Unsupported subgrid dimensions.");
                };
                subgrid_interpolators.push(interp);
            }
            interpolators.push(subgrid_interpolators);
        }

        let alphas_q2s: Vec<f64> = info.alphas_q_values.iter().map(|&q| q * q).collect();
        let alphas_interpolator = Interp1D::new(
            alphas_q2s.into(),
            info.alphas_vals.clone().into(),
            AlphaSCubicInterpolation,
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
    fn find_subgrid_index(&self, x: f64, q2: f64) -> Result<usize, Error> {
        // TODO: This does not allow for any extrapolation
        self.knot_array
            .subgrids
            .iter()
            .position(|subgrid| subgrid.is_in_subgrid(x, q2))
            .ok_or(Error::SubgridNotFound { x, q2 })
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
    /// The interpolated PDF value.
    pub fn xfxq2(&self, id: i32, x: f64, q2: f64) -> f64 {
        let subgrid_index = self.find_subgrid_index(x, q2).unwrap();
        let pid_index = self.knot_array.pids.iter().position(|&p| p == id).unwrap();
        self.interpolators[subgrid_index][pid_index]
            .interpolate_point(&[x, q2])
            .unwrap()
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
    pub fn info(&self) -> MetaData {
        self.info.clone()
    }

    /// Get `x_min` from the complete PDF grid.
    pub fn x_min(&self) -> f64 {
        self.knot_array
            .subgrids
            .iter()
            .map(|subgrid| subgrid.x_min)
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
    }

    /// Get `x_max` from the complete PDF grid.
    pub fn x_max(&self) -> f64 {
        self.knot_array
            .subgrids
            .iter()
            .map(|subgrid| subgrid.x_max)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
    }

    /// Get `Q2_min` from the complete PDF grid.
    pub fn q2_min(&self) -> f64 {
        self.knot_array
            .subgrids
            .iter()
            .map(|subgrid| subgrid.q2_min)
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
    }

    /// Get `Q2_max` from the complete PDF grid.
    pub fn q2_max(&self) -> f64 {
        self.knot_array
            .subgrids
            .iter()
            .map(|subgrid| subgrid.q2_max)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knot_array_new() {
        let subgrid_data = vec![SubgridData {
            nucleons: vec![1],
            alphas: vec![0.118],
            xs: vec![1.0, 2.0, 3.0],
            q2s: vec![4.0, 5.0],
            grid_data: vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0,
            ],
        }];
        let flavors = vec![21, 22];
        let knot_array = GridArray::new(subgrid_data, flavors);
        assert_eq!(knot_array.subgrids[0].grid.shape(), &[1, 1, 2, 3, 2]);
    }
}
