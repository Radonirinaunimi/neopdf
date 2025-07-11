//! This module defines the structures and methods for handling PDF grid data and interpolation.
//!
//! It provides a clean, modular interface for accessing and interpolating PDF data through
//! the `GridPDF` struct, with support for multiple interpolation strategies and dimensions.

use ndarray::{s, Array1, Array3, Array5, ArrayView2, OwnedRepr};
use ninterp::error::InterpolateError;
use ninterp::interpolator::{Extrapolate, Interp2D, InterpND};
use ninterp::prelude::*;
use ninterp::strategy::traits::{Strategy2D, Strategy3D, StrategyND};
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

/// Errors that can occur during PDF grid operations.
#[derive(Debug, Error)]
pub enum Error {
    /// Error indicating that no suitable subgrid was found for the given `x` and `q2` values.
    #[error("No subgrid found for x={x}, q2={q2}")]
    SubgridNotFound {
        /// The momentum fraction `x` value.
        x: f64,
        /// The energy scale squared `q2` value.
        q2: f64,
    },
    /// Error indicating invalid interpolation parameters, with a descriptive message.
    #[error("Invalid interpolation parameters: {0}")]
    InterpolationError(String),
}

/// Represents the dimensionality and structure of interpolation needed.
///
/// This enum is used to select the appropriate interpolation strategy based on the
/// dimensions of the PDF grid data.
#[derive(Debug, Clone, Copy)]
pub enum InterpolationConfig {
    /// 2D interpolation, typically in `x` (momentum fraction) and `Q²` (energy scale).
    TwoD,
    /// 3D interpolation, including a dimension for varying nucleon numbers,
    /// in addition to `x` and `Q²`.
    ThreeDNucleons,
    /// 3D interpolation, including a dimension for varying alpha_s values,
    /// in addition to `x` and `Q²`.
    ThreeDAlphas,
    /// 4D interpolation, covering nucleons, alpha_s, `x`, and `Q²`.
    FourD,
}

impl InterpolationConfig {
    /// Determines the interpolation configuration from the number of nucleons and alpha_s values.
    ///
    /// # Panics
    ///
    /// Panics if the combination of `n_nucleons` and `n_alphas` is not supported.
    fn from_dimensions(n_nucleons: usize, n_alphas: usize) -> Self {
        match (n_nucleons, n_alphas) {
            (1, 1) => Self::TwoD,
            (n, 1) if n > 1 => Self::ThreeDNucleons,
            (1, n) if n > 1 => Self::ThreeDAlphas,
            (n, a) if n > 1 && a > 1 => Self::FourD,
            _ => panic!(
                "Invalid dimensions: nucleons={}, alphas={}",
                n_nucleons, n_alphas
            ),
        }
    }
}

/// Represents the valid range of a parameter, with a minimum and maximum value.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct ParamRange {
    /// The minimum value of the parameter.
    pub min: f64,
    /// The maximum value of the parameter.
    pub max: f64,
}

impl ParamRange {
    /// Creates a new `ParamRange`.
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum value.
    /// * `max` - The maximum value.
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    /// Checks if a given value is within the parameter range (inclusive).
    ///
    /// # Arguments
    ///
    /// * `value` - The value to check.
    ///
    /// # Returns
    ///
    /// `true` if the value is within the range, `false` otherwise.
    pub fn contains(&self, value: f64) -> bool {
        value >= self.min && value <= self.max
    }
}

/// Represents the parameter ranges for `x` and `q2`.
pub struct RangeParameters {
    /// The range for the momentum fraction `x`.
    pub x: ParamRange,
    /// The range for the energy scale squared `q2`.
    pub q2: ParamRange,
}

impl RangeParameters {
    /// Creates a new `RangeParameters`.
    ///
    /// # Arguments
    ///
    /// * `x` - The `ParamRange` for `x`.
    /// * `q2` - The `ParamRange` for `q2`.
    pub fn new(x: ParamRange, q2: ParamRange) -> Self {
        Self { x, q2 }
    }
}

/// A trait for dynamic interpolation across different dimensions.
///
/// This trait provides a common interface for different interpolator types,
/// allowing them to be used interchangeably.
pub trait DynInterpolator: Send + Sync {
    /// Interpolates a value at a given multi-dimensional point.
    ///
    /// # Arguments
    ///
    /// * `point` - A slice of `f64` representing the coordinates of the point.
    ///
    /// # Returns
    ///
    /// A `Result` containing the interpolated value or an `InterpolateError`.
    fn interpolate_point(&self, point: &[f64]) -> Result<f64, InterpolateError>;
}

// Implement `DynInterpolator` for 2D interpolators.
impl<S> DynInterpolator for Interp2DOwned<f64, S>
where
    S: Strategy2D<OwnedRepr<f64>> + 'static + Clone + Send + Sync,
{
    fn interpolate_point(&self, point: &[f64]) -> Result<f64, InterpolateError> {
        let [x, y] = point
            .try_into()
            .map_err(|_| InterpolateError::Other("Expected 2D point".to_string()))?;
        self.interpolate(&[x, y])
    }
}

// Implement `DynInterpolator` for 3D interpolators.
impl<S> DynInterpolator for Interp3DOwned<f64, S>
where
    S: Strategy3D<OwnedRepr<f64>> + 'static + Clone + Send + Sync,
{
    fn interpolate_point(&self, point: &[f64]) -> Result<f64, InterpolateError> {
        let [x, y, z] = point
            .try_into()
            .map_err(|_| InterpolateError::Other("Expected 3D point".to_string()))?;
        self.interpolate(&[x, y, z])
    }
}

// Implement `DynInterpolator` for N-dimensional interpolators.
impl<S> DynInterpolator for InterpNDOwned<f64, S>
where
    S: StrategyND<OwnedRepr<f64>> + 'static + Clone + Send + Sync,
{
    fn interpolate_point(&self, point: &[f64]) -> Result<f64, InterpolateError> {
        self.interpolate(point)
    }
}

/// Stores the PDF grid data for a single subgrid.
///
/// A subgrid represents a region of the phase space with a consistent
/// grid of `x` and `Q²` values.
#[derive(Debug, Serialize, Deserialize)]
pub struct SubGrid {
    /// Array of `x` values (momentum fraction).
    pub xs: Array1<f64>,
    /// Array of `Q²` values (energy scale squared).
    pub q2s: Array1<f64>,
    /// 5-dimensional grid data: [nucleons, alphas, pids, x, Q²].
    pub grid: Array5<f64>,
    /// A vector of nucleon numbers.
    pub nucleons: Vec<u32>,
    /// A vector of alpha_s values.
    pub alphas: Vec<f64>,
    /// The valid range for the `x` parameter in this subgrid.
    x_range: ParamRange,
    /// The valid range for the `q2` parameter in this subgrid.
    q2_range: ParamRange,
}

impl SubGrid {
    /// Creates a new `SubGrid` from raw data.
    ///
    /// # Arguments
    ///
    /// * `nucleon_numbers` - A vector of nucleon numbers.
    /// * `alphas_values` - A vector of alpha_s values.
    /// * `xs` - A vector of `x` values.
    /// * `q2s` - A vector of `q2` values.
    /// * `nflav` - The number of quark flavors.
    /// * `grid_data` - A flat vector of grid data points.
    ///
    /// # Panics
    ///
    /// Panics if the grid data cannot be reshaped to the expected dimensions.
    pub fn new(
        nucleon_numbers: Vec<u32>,
        alphas_values: Vec<f64>,
        xs: Vec<f64>,
        q2s: Vec<f64>,
        nflav: usize,
        grid_data: Vec<f64>,
    ) -> Self {
        let x_range = ParamRange::new(*xs.first().unwrap(), *xs.last().unwrap());
        let q2_range = ParamRange::new(*q2s.first().unwrap(), *q2s.last().unwrap());

        let grid = Array5::from_shape_vec(
            (
                nucleon_numbers.len(),
                alphas_values.len(),
                xs.len(),
                q2s.len(),
                nflav,
            ),
            grid_data,
        )
        .expect("Failed to create grid")
        .permuted_axes([0, 1, 4, 2, 3])
        .as_standard_layout()
        .to_owned();

        Self {
            xs: Array1::from_vec(xs),
            q2s: Array1::from_vec(q2s),
            grid,
            nucleons: nucleon_numbers,
            alphas: alphas_values,
            x_range,
            q2_range,
        }
    }

    /// Checks if a point (`x`, `q2`) is within the boundaries of this subgrid.
    ///
    /// # Arguments
    ///
    /// * `x` - The momentum fraction `x`.
    /// * `q2` - The energy scale squared `q2`.
    ///
    /// # Returns
    ///
    /// `true` if the point is within the subgrid, `false` otherwise.
    pub fn contains_point(&self, x: f64, q2: f64) -> bool {
        self.x_range.contains(x) && self.q2_range.contains(q2)
    }

    /// Gets the interpolation configuration for this subgrid.
    pub fn interpolation_config(&self) -> InterpolationConfig {
        InterpolationConfig::from_dimensions(self.nucleons.len(), self.alphas.len())
    }

    /// Gets the parameter ranges for this subgrid.
    pub fn ranges(&self) -> RangeParameters {
        RangeParameters::new(self.x_range, self.q2_range)
    }

    /// Gets a 2D slice of the grid for interpolation.
    ///
    /// This method is only valid for 2D interpolation configurations.
    ///
    /// # Arguments
    ///
    /// * `pid_index` - The index of the particle ID (flavor).
    ///
    /// # Panics
    ///
    /// Panics if called on a subgrid that is not 2D.
    pub fn grid_slice(&self, pid_index: usize) -> ArrayView2<f64> {
        match self.interpolation_config() {
            InterpolationConfig::TwoD => self.grid.slice(s![0, 0, pid_index, .., ..]),
            _ => panic!("grid_slice only valid for 2D interpolation"),
        }
    }
}

/// A factory for creating interpolators based on the interpolation type and grid dimensions.
pub struct InterpolatorFactory;

impl InterpolatorFactory {
    /// Creates a dynamic interpolator for a given subgrid and flavor.
    ///
    /// # Arguments
    ///
    /// * `interp_type` - The type of interpolation to use.
    /// * `subgrid` - A reference to the `SubGrid`.
    /// * `pid_index` - The index of the particle ID (flavor).
    ///
    /// # Returns
    ///
    /// A `Box<dyn DynInterpolator>` that can be used for interpolation.
    pub fn create(
        interp_type: InterpolatorType,
        subgrid: &SubGrid,
        pid_index: usize,
    ) -> Box<dyn DynInterpolator> {
        match subgrid.interpolation_config() {
            InterpolationConfig::TwoD => Self::interpolator_xfxq2(interp_type, subgrid, pid_index),
            InterpolationConfig::ThreeDNucleons => {
                Self::interpolator_xfxq2_nucleons(interp_type, subgrid, pid_index)
            }
            InterpolationConfig::ThreeDAlphas => {
                Self::interpolator_xfxq2_alphas(interp_type, subgrid, pid_index)
            }
            InterpolationConfig::FourD => Self::interpolator_ndim(interp_type, subgrid, pid_index),
        }
    }

    /// Creates a 2D interpolator for `x` and `q2`.
    fn interpolator_xfxq2(
        interp_type: InterpolatorType,
        subgrid: &SubGrid,
        pid_index: usize,
    ) -> Box<dyn DynInterpolator> {
        let grid_slice = subgrid.grid_slice(pid_index).to_owned();

        match interp_type {
            InterpolatorType::Bilinear => Box::new(
                Interp2D::new(
                    subgrid.xs.clone(),
                    subgrid.q2s.clone(),
                    grid_slice,
                    BilinearInterpolation,
                    Extrapolate::Error,
                )
                .expect("Failed to create 2D interpolator"),
            ),
            InterpolatorType::LogBilinear => Box::new(
                Interp2D::new(
                    subgrid.xs.clone(),
                    subgrid.q2s.clone(),
                    grid_slice,
                    LogBilinearInterpolation,
                    Extrapolate::Error,
                )
                .expect("Failed to create 2D interpolator"),
            ),
            InterpolatorType::LogBicubic => Box::new(
                Interp2D::new(
                    subgrid.xs.clone(),
                    subgrid.q2s.clone(),
                    grid_slice,
                    LogBicubicInterpolation::default(),
                    Extrapolate::Error,
                )
                .expect("Failed to create 2D interpolator"),
            ),
            _ => panic!("Unsupported 2D interpolator: {:?}", interp_type),
        }
    }

    /// Creates a 3D interpolator for nucleons, `x`, and `q2`.
    fn interpolator_xfxq2_nucleons(
        interp_type: InterpolatorType,
        subgrid: &SubGrid,
        pid_index: usize,
    ) -> Box<dyn DynInterpolator> {
        let grid_data = subgrid.grid.slice(s![.., .., pid_index, .., ..]).to_owned();
        let nucleon_coords = subgrid.nucleons.iter().map(|&n| n as f64).collect();
        let reshaped_data = grid_data
            .into_shape_with_order((subgrid.nucleons.len(), subgrid.xs.len(), subgrid.q2s.len()))
            .expect("Failed to reshape 3D data");

        match interp_type {
            InterpolatorType::LogTricubic => Box::new(
                Interp3D::new(
                    nucleon_coords,
                    subgrid.xs.clone(),
                    subgrid.q2s.clone(),
                    reshaped_data,
                    LogTricubicInterpolation,
                    Extrapolate::Error,
                )
                .expect("Failed to create 3D interpolator"),
            ),
            _ => panic!("Unsupported 3D interpolator: {:?}", interp_type),
        }
    }

    /// Creates a 3D interpolator for alpha_s, `x`, and `q2`.
    fn interpolator_xfxq2_alphas(
        interp_type: InterpolatorType,
        subgrid: &SubGrid,
        pid_index: usize,
    ) -> Box<dyn DynInterpolator> {
        let grid_data = subgrid.grid.slice(s![.., .., pid_index, .., ..]).to_owned();
        let alpha_coords = Array1::from(subgrid.alphas.clone());
        let reshaped_data = grid_data
            .into_shape_with_order((subgrid.alphas.len(), subgrid.xs.len(), subgrid.q2s.len()))
            .expect("Failed to reshape 3D data");

        match interp_type {
            InterpolatorType::LogTricubic => Box::new(
                Interp3D::new(
                    alpha_coords,
                    subgrid.xs.clone(),
                    subgrid.q2s.clone(),
                    reshaped_data,
                    LogTricubicInterpolation,
                    Extrapolate::Error,
                )
                .expect("Failed to create 3D interpolator"),
            ),
            _ => panic!("Unsupported 3D interpolator: {:?}", interp_type),
        }
    }

    /// Creates an N-dimensional interpolator.
    fn interpolator_ndim(
        interp_type: InterpolatorType,
        subgrid: &SubGrid,
        pid_index: usize,
    ) -> Box<dyn DynInterpolator> {
        let grid_data = subgrid.grid.slice(s![.., .., pid_index, .., ..]).to_owned();
        let coords = vec![
            subgrid.nucleons.iter().map(|&n| n as f64).collect(),
            Array1::from(subgrid.alphas.clone()),
            subgrid.xs.clone(),
            subgrid.q2s.clone(),
        ];
        let reshaped_data = grid_data
            .into_shape_with_order((
                subgrid.nucleons.len(),
                subgrid.alphas.len(),
                subgrid.xs.len(),
                subgrid.q2s.len(),
            ))
            .expect("Failed to reshape 4D data");

        match interp_type {
            InterpolatorType::InterpNDLinear => Box::new(
                InterpND::new(coords, reshaped_data.into_dyn(), Linear, Extrapolate::Error)
                    .expect("Failed to create 4D interpolator"),
            ),
            _ => panic!("Unsupported 4D interpolator: {:?}", interp_type),
        }
    }
}

/// Stores the complete PDF grid data, including all subgrids and flavor information.
#[derive(Debug, Serialize, Deserialize)]
pub struct GridArray {
    /// An array of particle flavor IDs (PIDs).
    pub pids: Array1<i32>,
    /// A collection of `SubGrid` instances that make up the full grid.
    pub subgrids: Vec<SubGrid>,
}

impl GridArray {
    /// Creates a new `GridArray` from a vector of `SubgridData`.
    ///
    /// # Arguments
    ///
    /// * `subgrid_data` - A vector of `SubgridData` parsed from the PDF data file.
    /// * `pids` - A vector of particle flavor IDs.
    pub fn new(subgrid_data: Vec<SubgridData>, pids: Vec<i32>) -> Self {
        let nflav = pids.len();
        let subgrids = subgrid_data
            .into_iter()
            .map(|data| {
                SubGrid::new(
                    data.nucleons,
                    data.alphas,
                    data.xs,
                    data.q2s,
                    nflav,
                    data.grid_data,
                )
            })
            .collect();

        Self {
            pids: Array1::from_vec(pids),
            subgrids,
        }
    }

    /// Gets the PDF value at a specific knot point in the grid.
    ///
    /// # Arguments
    ///
    /// * `nucleon_idx` - The index of the nucleon.
    /// * `alpha_idx` - The index of the alpha_s value.
    /// * `x_idx` - The index of the `x` value.
    /// * `q2_idx` - The index of the `q2` value.
    /// * `flavor_id` - The particle flavor ID.
    /// * `subgrid_idx` - The index of the subgrid.
    ///
    /// # Returns
    ///
    /// The PDF value `f64` at the specified grid point.
    ///
    /// # Panics
    ///
    /// Panics if the `flavor_id` is invalid.
    pub fn xf_from_index(
        &self,
        nucleon_idx: usize,
        alpha_idx: usize,
        x_idx: usize,
        q2_idx: usize,
        flavor_id: i32,
        subgrid_idx: usize,
    ) -> f64 {
        let pid_idx = self.pid_index(flavor_id).expect("Invalid flavor ID");
        self.subgrids[subgrid_idx].grid[[nucleon_idx, alpha_idx, pid_idx, x_idx, q2_idx]]
    }

    /// Finds the index of the subgrid that contains the given `(x, q2)` point.
    ///
    /// # Arguments
    ///
    /// * `x` - The momentum fraction `x`.
    /// * `q2` - The energy scale squared `q2`.
    ///
    /// # Returns
    ///
    /// An `Option<usize>` containing the index of the subgrid if found, otherwise `None`.
    pub fn find_subgrid(&self, x: f64, q2: f64) -> Option<usize> {
        self.subgrids.iter().position(|sg| sg.contains_point(x, q2))
    }

    /// Gets the index corresponding to a given flavor ID.
    fn pid_index(&self, flavor_id: i32) -> Option<usize> {
        self.pids.iter().position(|&pid| pid == flavor_id)
    }

    /// Gets the overall parameter ranges across all subgrids.
    ///
    /// This method calculates the minimum and maximum values for `x` and `q2`
    /// across all subgrids to determine the global parameter space.
    ///
    /// # Returns
    ///
    /// A `RangeParameters` struct containing the global `x` and `q2` ranges.
    pub fn global_ranges(&self) -> RangeParameters {
        let x_min = self
            .subgrids
            .iter()
            .map(|sg| sg.x_range.min)
            .fold(f64::INFINITY, f64::min);
        let x_max = self
            .subgrids
            .iter()
            .map(|sg| sg.x_range.max)
            .fold(f64::NEG_INFINITY, f64::max);
        let q2_min = self
            .subgrids
            .iter()
            .map(|sg| sg.q2_range.min)
            .fold(f64::INFINITY, f64::min);
        let q2_max = self
            .subgrids
            .iter()
            .map(|sg| sg.q2_range.max)
            .fold(f64::NEG_INFINITY, f64::max);

        RangeParameters::new(
            ParamRange::new(x_min, x_max),
            ParamRange::new(q2_min, q2_max),
        )
    }
}

/// The main PDF grid interface, providing high-level methods for interpolation.
pub struct GridPDF {
    /// The metadata associated with the PDF set.
    info: MetaData,
    /// The underlying grid data stored in a `GridArray`.
    pub knot_array: GridArray,
    /// A nested vector of interpolators for each subgrid and flavor.
    interpolators: Vec<Vec<Box<dyn DynInterpolator>>>,
    /// An interpolator for the running of alpha_s.
    alphas_interpolator: Interp1DOwned<f64, AlphaSCubicInterpolation>,
}

impl GridPDF {
    /// Creates a new `GridPDF` instance.
    ///
    /// # Arguments
    ///
    /// * `info` - The `MetaData` for the PDF set.
    /// * `knot_array` - The `GridArray` containing the grid data.
    pub fn new(info: MetaData, knot_array: GridArray) -> Self {
        let interpolators = Self::build_interpolators(&info, &knot_array);
        let alphas_interpolator = Self::build_alphas_interpolator(&info);

        Self {
            info,
            knot_array,
            interpolators,
            alphas_interpolator,
        }
    }

    /// Builds the interpolators for all subgrids and flavors.
    fn build_interpolators(
        info: &MetaData,
        knot_array: &GridArray,
    ) -> Vec<Vec<Box<dyn DynInterpolator>>> {
        knot_array
            .subgrids
            .iter()
            .map(|subgrid| {
                (0..knot_array.pids.len())
                    .map(|pid_idx| {
                        InterpolatorFactory::create(
                            info.interpolator_type.clone(),
                            subgrid,
                            pid_idx,
                        )
                    })
                    .collect()
            })
            .collect()
    }

    /// Builds the interpolator for alpha_s.
    fn build_alphas_interpolator(info: &MetaData) -> Interp1DOwned<f64, AlphaSCubicInterpolation> {
        let q2_values: Vec<f64> = info.alphas_q_values.iter().map(|&q| q * q).collect();
        Interp1D::new(
            q2_values.into(),
            info.alphas_vals.clone().into(),
            AlphaSCubicInterpolation,
            Extrapolate::Error,
        )
        .expect("Failed to create alpha_s interpolator")
    }

    /// Interpolates the PDF value for a single point `(x, q2)` and a given flavor.
    ///
    /// # Arguments
    ///
    /// * `flavor_id` - The particle flavor ID.
    /// * `x` - The momentum fraction `x`.
    /// * `q2` - The energy scale squared `q2`.
    ///
    /// # Returns
    ///
    /// A `Result` containing the interpolated PDF value or an `Error`.
    pub fn xfxq2(&self, flavor_id: i32, x: f64, q2: f64) -> Result<f64, Error> {
        let subgrid_idx = self
            .knot_array
            .find_subgrid(x, q2)
            .ok_or(Error::SubgridNotFound { x, q2 })?;

        let pid_idx = self.knot_array.pid_index(flavor_id).ok_or_else(|| {
            Error::InterpolationError(format!("Invalid flavor ID: {}", flavor_id))
        })?;

        self.interpolators[subgrid_idx][pid_idx]
            .interpolate_point(&[x, q2])
            .map_err(|e| Error::InterpolationError(e.to_string()))
    }

    /// Interpolates PDF values for multiple points in parallel.
    ///
    /// # Arguments
    ///
    /// * `flavors` - A vector of flavor IDs.
    /// * `xs` - A vector of `x` values.
    /// * `q2s` - A vector of `q2` values.
    ///
    /// # Returns
    ///
    /// A 3D array of interpolated PDF values with shape `[flavors, xs, q2s]`.
    pub fn xfxq2s(&self, flavors: Vec<i32>, xs: Vec<f64>, q2s: Vec<f64>) -> Array3<f64> {
        let shape = [flavors.len(), xs.len(), q2s.len()];
        let total_len = shape.iter().product();

        let data: Vec<f64> = (0..total_len)
            .into_par_iter()
            .map(|idx| {
                let (i, j, k) = self.unravel_index(idx, &shape);
                self.xfxq2(flavors[i], xs[j], q2s[k]).unwrap_or(0.0)
            })
            .collect();

        Array3::from_shape_vec(shape, data).expect("Failed to create result array")
    }

    /// Converts a linear index into 3D coordinates `(i, j, k)`.
    fn unravel_index(&self, idx: usize, shape: &[usize; 3]) -> (usize, usize, usize) {
        let k = idx % shape[2];
        let j = (idx / shape[2]) % shape[1];
        let i = idx / (shape[1] * shape[2]);
        (i, j, k)
    }

    /// Gets the alpha_s value at a given `Q²`.
    ///
    /// # Arguments
    ///
    /// * `q2` - The energy scale squared `q2`.
    ///
    /// # Returns
    ///
    /// The interpolated alpha_s value.
    pub fn alphas_q2(&self, q2: f64) -> f64 {
        self.alphas_interpolator.interpolate(&[q2]).unwrap_or(0.0)
    }

    /// Returns a reference to the PDF metadata.
    pub fn metadata(&self) -> &MetaData {
        &self.info
    }

    /// Gets the global parameter ranges for the entire PDF set.
    pub fn param_ranges(&self) -> RangeParameters {
        self.knot_array.global_ranges()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_param_range() {
        let range = ParamRange::new(1.0, 10.0);
        assert!(range.contains(5.0));
        assert!(!range.contains(15.0));
    }

    #[test]
    fn test_interpolation_config() {
        assert!(matches!(
            InterpolationConfig::from_dimensions(1, 1),
            InterpolationConfig::TwoD
        ));
        assert!(matches!(
            InterpolationConfig::from_dimensions(2, 1),
            InterpolationConfig::ThreeDNucleons
        ));
        assert!(matches!(
            InterpolationConfig::from_dimensions(1, 2),
            InterpolationConfig::ThreeDAlphas
        ));
        assert!(matches!(
            InterpolationConfig::from_dimensions(2, 2),
            InterpolationConfig::FourD
        ));
    }

    #[test]
    fn test_grid_array_creation() {
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
        let grid_array = GridArray::new(subgrid_data, flavors);

        assert_eq!(grid_array.subgrids[0].grid.shape(), &[1, 1, 2, 3, 2]);
        assert!(grid_array.find_subgrid(1.5, 4.5).is_some());
    }
}
