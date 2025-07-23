//! This module defines the [`SubGrid`] struct and its implementation for PDF grid handling.
//!
//! # Contents
//!
//! - [`ParamRange`], [`RangeParameters`]: Parameter range types for grid axes.
//! - [`SubGrid`]: Represents a region of phase space with a consistent grid and provides methods for subgrid logic.

use ndarray::{s, Array1, Array6, ArrayView2};
use serde::{Deserialize, Serialize};

use super::interpolator::InterpolationConfig;

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
    /// The range for the nucleon numbers `A`.
    pub nucleons: ParamRange,
    /// The range for the AlphaS values `as`.
    pub alphas: ParamRange,
    /// The range for the transverse momentum `kT`.
    pub kt: ParamRange,
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
    /// * `nucleons` - The `ParamRange` for the nuleon numbers `A`.
    /// * `alphas` - The `ParamRange` for the strong coupling `as`.
    /// * `kt` - The `ParamRange` for the transverse momentum `kT`.
    /// * `x` - The `ParamRange` for the momentum fraction `x`.
    /// * `q2` - The `ParamRange` for the energy scale `q2`.
    pub fn new(
        nucleons: ParamRange,
        alphas: ParamRange,
        kt: ParamRange,
        x: ParamRange,
        q2: ParamRange,
    ) -> Self {
        Self {
            nucleons,
            alphas,
            kt,
            x,
            q2,
        }
    }
}

/// Stores the PDF grid data for a single subgrid.
///
/// A subgrid represents a region of the phase space with a consistent
/// grid of `x` and `Q²` values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubGrid {
    /// Array of `x` values (momentum fraction).
    pub xs: Array1<f64>,
    /// Array of `Q²` values (energy scale squared).
    pub q2s: Array1<f64>,
    /// Array of `kT` values (transverse momentum).
    pub kts: Array1<f64>,
    /// 6-dimensional grid data: [nucleons, alphas, pids, kT, x, Q²].
    pub grid: Array6<f64>,
    /// Array of nucleon number values.
    pub nucleons: Array1<f64>,
    /// Array of alpha_s values.
    pub alphas: Array1<f64>,
    /// The valid range for the `nucleons` parameter in this subgrid.
    pub nucleons_range: ParamRange,
    /// The valid range for the `AlphaS` parameter in this subgrid.
    pub alphas_range: ParamRange,
    /// The valid range for the `kT` parameter in this subgrid.
    pub kt_range: ParamRange,
    /// The valid range for the `x` parameter in this subgrid.
    pub x_range: ParamRange,
    /// The valid range for the `q2` parameter in this subgrid.
    pub q2_range: ParamRange,
}

impl SubGrid {
    /// Creates a new `SubGrid` from raw data.
    ///
    /// # Arguments
    ///
    /// * `nucleon_numbers` - A vector of nucleon numbers.
    /// * `alphas_values` - A vector of alpha_s values.
    /// * `kt_subgrid` - A vector of `kT` values.
    /// * `xs` - A vector of `x` values.
    /// * `q2s` - A vector of `q2` values.
    /// * `nflav` - The number of quark flavors.
    /// * `grid_data` - A flat vector of grid data points.
    ///
    /// # Panics
    ///
    /// Panics if the grid data cannot be reshaped to the expected dimensions.
    pub fn new(
        nucleon_numbers: Vec<f64>,
        alphas_values: Vec<f64>,
        kt_subgrid: Vec<f64>,
        x_subgrid: Vec<f64>,
        q2_subgrid: Vec<f64>,
        nflav: usize,
        grid_data: Vec<f64>,
    ) -> Self {
        let xs_range = ParamRange::new(*x_subgrid.first().unwrap(), *x_subgrid.last().unwrap());
        let q2s_range = ParamRange::new(*q2_subgrid.first().unwrap(), *q2_subgrid.last().unwrap());
        let kts_range = ParamRange::new(*kt_subgrid.first().unwrap(), *kt_subgrid.last().unwrap());
        let ncs_range = ParamRange::new(
            *nucleon_numbers.first().unwrap(),
            *nucleon_numbers.last().unwrap(),
        );
        let as_range = ParamRange::new(
            *alphas_values.first().unwrap(),
            *alphas_values.last().unwrap(),
        );

        let subgrid = Array6::from_shape_vec(
            (
                nucleon_numbers.len(),
                alphas_values.len(),
                kt_subgrid.len(),
                x_subgrid.len(),
                q2_subgrid.len(),
                nflav,
            ),
            grid_data,
        )
        .expect("Failed to create grid")
        .permuted_axes([0, 1, 5, 2, 3, 4])
        .as_standard_layout()
        .to_owned();

        Self {
            xs: Array1::from_vec(x_subgrid),
            q2s: Array1::from_vec(q2_subgrid),
            kts: Array1::from_vec(kt_subgrid),
            grid: subgrid,
            nucleons: Array1::from_vec(nucleon_numbers),
            alphas: Array1::from_vec(alphas_values),
            nucleons_range: ncs_range,
            alphas_range: as_range,
            kt_range: kts_range,
            x_range: xs_range,
            q2_range: q2s_range,
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
        InterpolationConfig::from_dimensions(self.nucleons.len(), self.alphas.len(), self.kts.len())
    }

    /// Gets the parameter ranges for this subgrid.
    pub fn ranges(&self) -> RangeParameters {
        RangeParameters::new(
            self.nucleons_range,
            self.alphas_range,
            self.kt_range,
            self.x_range,
            self.q2_range,
        )
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
            InterpolationConfig::TwoD => self.grid.slice(s![0, 0, pid_index, 0, .., ..]),
            _ => panic!("grid_slice only valid for 2D interpolation"),
        }
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
}
