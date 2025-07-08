//! The interpolation strategies.
//! TODO: Move the taking of the logs of the input data outside.

use ndarray::{Data, RawDataClone};
use ninterp::data::{InterpData1D, InterpData2D, InterpData3D};
use ninterp::error::{InterpolateError, ValidateError};
use ninterp::strategy::traits::{Strategy1D, Strategy2D, Strategy3D};
use serde::{Deserialize, Serialize};

use super::utils;

/// Implements bilinear interpolation for 2D data.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BilinearInterpolation;

impl BilinearInterpolation {
    /// Performs linear interpolation between two points.
    ///
    /// Given two points `(x1, y1)` and `(x2, y2)`, this function calculates the
    /// y-value corresponding to a given `x` using linear interpolation.
    ///
    /// # Arguments
    ///
    /// * `x1` - The x-coordinate of the first point.
    /// * `x2` - The x-coordinate of the second point.
    /// * `y1` - The y-coordinate of the first point.
    /// * `y2` - The y-coordinate of the second point.
    /// * `x` - The x-coordinate at which to interpolate.
    ///
    /// # Returns
    ///
    /// The interpolated y-value.
    fn linear_interpolate(x1: f64, x2: f64, y1: f64, y2: f64, x: f64) -> f64 {
        if x1 == x2 {
            return y1; // Avoid division by zero
        }
        y1 + (y2 - y1) * (x - x1) / (x2 - x1)
    }
}

impl<D> Strategy2D<D> for BilinearInterpolation
where
    D: Data<Elem = f64> + RawDataClone + Clone,
{
    /// Performs bilinear interpolation at a given point.
    ///
    /// # Arguments
    ///
    /// * `data` - The interpolation data containing grid coordinates and values.
    /// * `point` - A 2-element array `[x, y]` representing the coordinates to interpolate at.
    fn interpolate(
        &self,
        data: &InterpData2D<D>,
        point: &[f64; 2],
    ) -> Result<f64, InterpolateError> {
        let [x, y] = *point;

        // Get the coordinate arrays and data values
        let x_coords = data.grid[0].as_slice().unwrap();
        let y_coords = data.grid[1].as_slice().unwrap();
        let values = &data.values;

        // Find the indices for interpolation
        let x_idx = utils::find_interval_index(x_coords, x)?;
        let y_idx = utils::find_interval_index(y_coords, y)?;

        // Get the four corner points
        let x1 = x_coords[x_idx];
        let x2 = x_coords[x_idx + 1];
        let y1 = y_coords[y_idx];
        let y2 = y_coords[y_idx + 1];

        // Get the four corner values
        let q11 = values[[x_idx, y_idx]]; // f(x1, y1)
        let q12 = values[[x_idx, y_idx + 1]]; // f(x1, y2)
        let q21 = values[[x_idx + 1, y_idx]]; // f(x2, y1)
        let q22 = values[[x_idx + 1, y_idx + 1]]; // f(x2, y2)

        // Perform bilinear interpolation
        let r1 = Self::linear_interpolate(x1, x2, q11, q21, x);
        let r2 = Self::linear_interpolate(x1, x2, q12, q22, x);

        // Then interpolate in y-direction
        let result = Self::linear_interpolate(y1, y2, r1, r2, y);

        Ok(result)
    }

    /// Indicates that this strategy does not allow extrapolation.
    fn allow_extrapolate(&self) -> bool {
        false
    }
}

/// Performs bilinear interpolation in log space.
///
/// This strategy transforms the input coordinates to their natural logarithms
/// before performing bilinear interpolation, which is suitable for data
/// that is linear in log-log space.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LogBilinearInterpolation;

impl<D> Strategy2D<D> for LogBilinearInterpolation
where
    D: Data<Elem = f64> + RawDataClone + Clone,
{
    /// Initializes the strategy, performing validation checks.
    ///
    /// Ensures that all x and y coordinates are positive, as logarithmic scaling
    /// is applied.
    ///
    /// # Arguments
    ///
    /// * `data` - The interpolation data to validate.
    fn init(&mut self, data: &InterpData2D<D>) -> Result<(), ValidateError> {
        // Get the coordinate arrays and data values
        let x_coords = data.grid[0].as_slice().unwrap();
        let y_coords = data.grid[1].as_slice().unwrap();

        if x_coords.iter().any(|&x| x <= 0.0) || y_coords.iter().any(|&y| y <= 0.0) {
            return Err(ValidateError::Other(
                "The input values must be positive for logarithmic scaling".to_string(),
            ));
        }

        Ok(())
    }

    /// Performs log-bilinear interpolation at a given point.
    ///
    /// The input `point` coordinates are first transformed to log space,
    /// then bilinear interpolation is applied.
    ///
    /// # Arguments
    ///
    /// * `data` - The interpolation data containing grid coordinates and values.
    /// * `point` - A 2-element array `[x, y]` representing the coordinates to interpolate at.
    fn interpolate(
        &self,
        data: &InterpData2D<D>,
        point: &[f64; 2],
    ) -> Result<f64, InterpolateError> {
        let [x, y] = *point;

        // Get the coordinate arrays and data values
        let x_coords = data.grid[0].as_slice().unwrap();
        let y_coords = data.grid[1].as_slice().unwrap();
        let values = &data.values;

        // Transform coordinates to log space
        let x_interp = x.ln();
        let y_interp = y.ln();

        // Transform grid coordinates to log space
        let x_grid: Vec<f64> = x_coords.iter().map(|&xi| xi.ln()).collect();
        let y_grid: Vec<f64> = y_coords.iter().map(|&yi| yi.ln()).collect();

        // Find the grid cell containing the point
        let i = utils::find_interval_index(&x_grid, x_interp)?;
        let j = utils::find_interval_index(&y_grid, y_interp)?;

        // Get the four corner points of the grid cell
        let x1 = x_grid[i];
        let x2 = x_grid[i + 1];
        let y1 = y_grid[j];
        let y2 = y_grid[j + 1];

        // Get the four corner values
        let z11 = values[[i, j]];
        let z12 = values[[i, j + 1]];
        let z21 = values[[i + 1, j]];
        let z22 = values[[i + 1, j + 1]];

        // Perform bilinear interpolation
        let dx = x2 - x1;
        let dy = y2 - y1;

        if dx == 0.0 || dy == 0.0 {
            unreachable!();
        }

        let wx = (x_interp - x1) / dx;
        let wy = (y_interp - y1) / dy;

        // Bilinear interpolation formula
        let z_interp = z11 * (1.0 - wx) * (1.0 - wy)
            + z21 * wx * (1.0 - wy)
            + z12 * (1.0 - wx) * wy
            + z22 * wx * wy;

        Ok(z_interp)
    }

    /// Indicates that this strategy does not allow extrapolation.
    fn allow_extrapolate(&self) -> bool {
        false
    }
}

/// LogBicubic interpolation strategy for PDF-like data
///
/// This strategy implements bicubic interpolation with logarithmic coordinate scaling:
/// - x-coordinates are logarithmically spaced (e.g., 1e-9 to 1)
/// - y-coordinates are logarithmically spaced (e.g., Q² values)
/// - z-values (PDF values) are interpolated using bicubic splines
///
/// Bicubic interpolation uses a 4x4 grid of points around the interpolation point
/// and provides C1 continuity (continuous first derivatives).
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogBicubicInterpolation {
    coeffs: Vec<f64>,
}

impl LogBicubicInterpolation {
    /// Find the interval for bicubic interpolation
    /// Returns the index i such that we can use points [i-1, i, i+1, i+2] for interpolation
    fn find_bicubic_interval(coords: &[f64], x: f64) -> Result<usize, InterpolateError> {
        // Find the interval [i, i+1] such that coords[i] <= x < coords[i+1]
        let i = utils::find_interval_index(coords, x)?;
        Ok(i)
    }

    /// Cubic interpolation using a passed array of coefficients (a*x^3 + b*x^2 + c*x + d)
    pub fn hermite_cubic_interpolate_from_coeffs(t: f64, coeffs: &[f64; 4]) -> f64 {
        let x = t;
        let x2 = x * x;
        let x3 = x2 * x;
        coeffs[0] * x3 + coeffs[1] * x2 + coeffs[2] * x + coeffs[3]
    }

    /// Calculates the derivative with respect to log(x) at a given knot.
    /// This mirrors the _ddx function in LHAPDF's C++ implementation.
    pub fn calculate_ddx<D>(data: &InterpData2D<D>, ix: usize, iq2: usize) -> f64
    where
        D: Data<Elem = f64> + RawDataClone + Clone,
    {
        let nxknots = data.grid[0].len();
        let x_coords = data.grid[0].as_slice().unwrap();
        let log_x_coords: Vec<f64> = x_coords.iter().map(|&xi| xi.ln()).collect();
        let values = &data.values;

        let del1 = match ix {
            0 => 0.0,
            i => log_x_coords[i] - log_x_coords[i - 1],
        };

        let del2 = match log_x_coords.get(ix + 1) {
            Some(&next) => next - log_x_coords[ix],
            None => 0.0,
        };

        if ix != 0 && ix != nxknots - 1 {
            // Central difference
            let lddx = (values[[ix, iq2]] - values[[ix - 1, iq2]]) / del1;
            let rddx = (values[[ix + 1, iq2]] - values[[ix, iq2]]) / del2;
            (lddx + rddx) / 2.0
        } else if ix == 0 {
            // Forward difference
            (values[[ix + 1, iq2]] - values[[ix, iq2]]) / del2
        } else if ix == nxknots - 1 {
            // Backward difference
            (values[[ix, iq2]] - values[[ix - 1, iq2]]) / del1
        } else {
            // This case should ideally not be reached given the checks above
            panic!("Should not reach here: Invalid index for derivative calculation.");
        }
    }

    /// Computes the polynomial coefficients for bicubic interpolation, mirroring LHAPDF's C++ implementation.
    fn compute_polynomial_coefficients<D>(data: &InterpData2D<D>) -> Vec<f64>
    where
        D: Data<Elem = f64> + RawDataClone + Clone,
    {
        let nxknots = data.grid[0].len();
        let nq2knots = data.grid[1].len();
        let values = &data.values;

        // The shape of the coefficients array: (nxknots-1) * nq2knots * 4 (for a,b,c,d)
        let mut coeffs: Vec<f64> = vec![0.0; (nxknots - 1) * nq2knots * 4];

        for ix in 0..nxknots - 1 {
            for iq2 in 0..nq2knots {
                let dlogx = data.grid[0].as_slice().unwrap()[ix + 1].ln()
                    - data.grid[0].as_slice().unwrap()[ix].ln();

                let vl = values[[ix, iq2]];
                let vh = values[[ix + 1, iq2]];
                let vdl = Self::calculate_ddx(data, ix, iq2) * dlogx;
                let vdh = Self::calculate_ddx(data, ix + 1, iq2) * dlogx;

                // polynomial coefficients
                let a = vdh + vdl - 2.0 * vh + 2.0 * vl;
                let b = 3.0 * vh - 3.0 * vl - 2.0 * vdl - vdh;
                let c = vdl;
                let d = vl;

                let base_idx = (ix * nq2knots + iq2) * 4;
                coeffs[base_idx] = a;
                coeffs[base_idx + 1] = b;
                coeffs[base_idx + 2] = c;
                coeffs[base_idx + 3] = d;
            }
        }
        coeffs
    }

    /// Performs bicubic interpolation using pre-computed coefficients.
    fn interpolate_with_coeffs<D>(
        &self,
        data: &InterpData2D<D>,
        ix: usize,
        iq2: usize,
        u: f64,
        v: f64,
    ) -> f64
    where
        D: Data<Elem = f64> + RawDataClone + Clone,
    {
        let nq2knots = data.grid[1].len();

        // Get the coefficients for the current cell (x-interpolation)
        let base_idx_vl = (ix * nq2knots + iq2) * 4;
        let coeffs_vl: [f64; 4] = self.coeffs[base_idx_vl..base_idx_vl + 4]
            .try_into()
            .unwrap();
        let vl = Self::hermite_cubic_interpolate_from_coeffs(u, &coeffs_vl);

        let base_idx_vh = (ix * nq2knots + iq2 + 1) * 4;
        let coeffs_vh: [f64; 4] = self.coeffs[base_idx_vh..base_idx_vh + 4]
            .try_into()
            .unwrap();
        let vh = Self::hermite_cubic_interpolate_from_coeffs(u, &coeffs_vh);

        // Derivatives in Q2 (y-interpolation)
        let log_q2_grid: Vec<f64> = data.grid[1]
            .as_slice()
            .unwrap()
            .iter()
            .map(|&qi| qi.ln())
            .collect();

        let dlogq_1 = log_q2_grid[iq2 + 1] - log_q2_grid[iq2];

        let vdl: f64;
        let vdh: f64;

        if iq2 == 0 {
            // Forward difference for lower q
            vdl = vh - vl;
            // Central difference for higher q
            let vhh_base_idx = (ix * nq2knots + iq2 + 2) * 4;
            let coeffs_vhh: [f64; 4] = self.coeffs[vhh_base_idx..vhh_base_idx + 4]
                .try_into()
                .unwrap();
            let vhh = Self::hermite_cubic_interpolate_from_coeffs(u, &coeffs_vhh);
            let dlogq_2 = 1.0 / (log_q2_grid[iq2 + 2] - log_q2_grid[iq2 + 1]);
            vdh = (vdl + (vhh - vh) * dlogq_1 * dlogq_2) * 0.5;
        } else if iq2 == nq2knots - 2 {
            // Backward difference for higher q
            vdh = vh - vl;
            // Central difference for lower q
            let vll_base_idx = (ix * nq2knots + iq2 - 1) * 4;
            let coeffs_vll: [f64; 4] = self.coeffs[vll_base_idx..vll_base_idx + 4]
                .try_into()
                .unwrap();
            let vll = Self::hermite_cubic_interpolate_from_coeffs(u, &coeffs_vll);
            let dlogq_0 = 1.0 / (log_q2_grid[iq2] - log_q2_grid[iq2 - 1]);
            vdl = (vdh + (vl - vll) * dlogq_1 * dlogq_0) * 0.5;
        } else {
            // Central difference for both q
            let vll_base_idx = (ix * nq2knots + iq2 - 1) * 4;
            let coeffs_vll: [f64; 4] = self.coeffs[vll_base_idx..vll_base_idx + 4]
                .try_into()
                .unwrap();
            let vll = Self::hermite_cubic_interpolate_from_coeffs(u, &coeffs_vll);
            let dlogq_0 = 1.0 / (log_q2_grid[iq2] - log_q2_grid[iq2 - 1]);

            let vhh_base_idx = (ix * nq2knots + iq2 + 2) * 4;
            let coeffs_vhh: [f64; 4] = self.coeffs[vhh_base_idx..vhh_base_idx + 4]
                .try_into()
                .unwrap();
            let vhh = Self::hermite_cubic_interpolate_from_coeffs(u, &coeffs_vhh);
            let dlogq_2 = 1.0 / (log_q2_grid[iq2 + 2] - log_q2_grid[iq2 + 1]);

            vdl = ((vh - vl) + (vl - vll) * dlogq_1 * dlogq_0) * 0.5;
            vdh = ((vh - vl) + (vhh - vh) * dlogq_1 * dlogq_2) * 0.5;
        }

        utils::hermite_cubic_interpolate(v, vl, vdl, vh, vdh)
    }
}

impl<D> Strategy2D<D> for LogBicubicInterpolation
where
    D: Data<Elem = f64> + RawDataClone + Clone,
{
    fn init(&mut self, data: &InterpData2D<D>) -> Result<(), ValidateError> {
        // Get the coordinate arrays and data values
        let x_coords = data.grid[0].as_slice().unwrap();
        let y_coords = data.grid[1].as_slice().unwrap();

        if x_coords.iter().any(|&x| x <= 0.0) || y_coords.iter().any(|&y| y <= 0.0) {
            return Err(ValidateError::Other(
                "The input values must be positive for logarithmic scaling".to_string(),
            ));
        }

        // Check that we have at least 4x4 grid for bicubic interpolation
        if x_coords.len() < 4 || y_coords.len() < 4 {
            return Err(ValidateError::Other(
                "Need at least 4x4 grid for bicubic interpolation".to_string(),
            ));
        }

        self.coeffs = Self::compute_polynomial_coefficients(data);
        Ok(())
    }

    fn interpolate(
        &self,
        data: &InterpData2D<D>,
        point: &[f64; 2],
    ) -> Result<f64, InterpolateError> {
        let [x, y] = *point;

        // Get the coordinate arrays and data values
        let x_coords = data.grid[0].as_slice().unwrap();
        let y_coords = data.grid[1].as_slice().unwrap();

        // Transform coordinates to log space
        let log_x = x.ln();
        let log_y = y.ln();

        // Transform grid coordinates to log space
        let log_x_grid: Vec<f64> = x_coords.iter().map(|&xi| xi.ln()).collect();
        let log_y_grid: Vec<f64> = y_coords.iter().map(|&yi| yi.ln()).collect();

        // Find the grid cell containing the point
        let i = Self::find_bicubic_interval(&log_x_grid, log_x)?;
        let j = Self::find_bicubic_interval(&log_y_grid, log_y)?;

        // Normalize coordinates to [0,1] within the central cell
        let dx = log_x_grid[i + 1] - log_x_grid[i];
        let dy = log_y_grid[j + 1] - log_y_grid[j];

        if dx == 0.0 || dy == 0.0 {
            return Err(InterpolateError::Other("Grid spacing is zero".to_string()));
        }

        let u = (log_x - log_x_grid[i]) / dx;
        let v = (log_y - log_y_grid[j]) / dy;

        // Perform bicubic interpolation using pre-computed coefficients
        let result = self.interpolate_with_coeffs(data, i, j, u, v);

        Ok(result)
    }

    fn allow_extrapolate(&self) -> bool {
        false
    }
}

/// LogTricubic interpolation strategy for PDF-like data
///
/// This strategy implements tricubic interpolation with logarithmic coordinate scaling:
/// - x-coordinates are logarithmically spaced (e.g., 1e-9 to 1)
/// - y-coordinates are logarithmically spaced (e.g., Q² values)
/// - z-coordinates are logarithmically spaced (e.g., Mass Atomic A, AlphaS)
/// - w-values (PDF values) are interpolated using tricubic splines
///
/// Tricubic interpolation uses a 4x4x4 grid of points around the interpolation point
/// and provides C1 continuity (continuous first derivatives).
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogTricubicInterpolation {
    coeffs: Vec<f64>,
}

impl LogTricubicInterpolation {
    /// Find the interval for tricubic interpolation
    /// Returns the index i such that we can use points [i-1, i, i+1, i+2] for interpolation
    fn find_tricubic_interval(coords: &[f64], x: f64) -> Result<usize, InterpolateError> {
        // Find the interval [i, i+1] such that coords[i] <= x < coords[i+1]
        let i = utils::find_interval_index(coords, x)?;
        Ok(i)
    }

    /// Cubic interpolation using a passed array of coefficients (a*x^3 + b*x^2 + c*x + d)
    pub fn hermite_cubic_interpolate_from_coeffs(t: f64, coeffs: &[f64; 4]) -> f64 {
        let x = t;
        let x2 = x * x;
        let x3 = x2 * x;
        coeffs[0] * x3 + coeffs[1] * x2 + coeffs[2] * x + coeffs[3]
    }

    /// Calculates the derivative with respect to x (or log(x)) at a given knot.
    pub fn calculate_ddx<D>(data: &InterpData3D<D>, ix: usize, iq2: usize, iz: usize) -> f64
    where
        D: Data<Elem = f64> + RawDataClone + Clone,
    {
        let nxknots = data.grid[0].len();
        let x_coords = data.grid[0].as_slice().unwrap();
        let log_x_coords: Vec<f64> = x_coords.iter().map(|&xi| xi.ln()).collect();
        let values = &data.values;

        let del1 = match ix {
            0 => 0.0,
            i => log_x_coords[i] - log_x_coords[i - 1],
        };

        let del2 = match log_x_coords.get(ix + 1) {
            Some(&next) => next - log_x_coords[ix],
            None => 0.0,
        };

        if ix != 0 && ix != nxknots - 1 {
            // Central difference
            let lddx = (values[[ix, iq2, iz]] - values[[ix - 1, iq2, iz]]) / del1;
            let rddx = (values[[ix + 1, iq2, iz]] - values[[ix, iq2, iz]]) / del2;
            (lddx + rddx) / 2.0
        } else if ix == 0 {
            // Forward difference
            (values[[ix + 1, iq2, iz]] - values[[ix, iq2, iz]]) / del2
        } else if ix == nxknots - 1 {
            // Backward difference
            (values[[ix, iq2, iz]] - values[[ix - 1, iq2, iz]]) / del1
        } else {
            // This case should ideally not be reached given the checks above
            panic!("Should not reach here: Invalid index for derivative calculation.");
        }
    }

    /// Calculates the derivative with respect to y (or log(y)) at a given knot.
    pub fn calculate_ddy<D>(data: &InterpData3D<D>, ix: usize, iq2: usize, iz: usize) -> f64
    where
        D: Data<Elem = f64> + RawDataClone + Clone,
    {
        let nq2knots = data.grid[1].len();
        let q2_coords = data.grid[1].as_slice().unwrap();
        let log_q2_coords: Vec<f64> = q2_coords.iter().map(|&qi| qi.ln()).collect();
        let values = &data.values;

        let del1 = match iq2 {
            0 => 0.0,
            i => log_q2_coords[i] - log_q2_coords[i - 1],
        };

        let del2 = match log_q2_coords.get(iq2 + 1) {
            Some(&next) => next - log_q2_coords[iq2],
            None => 0.0,
        };

        if iq2 != 0 && iq2 != nq2knots - 1 {
            // Central difference
            let lddq = (values[[ix, iq2, iz]] - values[[ix, iq2 - 1, iz]]) / del1;
            let rddq = (values[[ix, iq2 + 1, iz]] - values[[ix, iq2, iz]]) / del2;
            (lddq + rddq) / 2.0
        } else if iq2 == 0 {
            // Forward difference
            (values[[ix, iq2 + 1, iz]] - values[[ix, iq2, iz]]) / del2
        } else if iq2 == nq2knots - 1 {
            // Backward difference
            (values[[ix, iq2, iz]] - values[[ix, iq2 - 1, iz]]) / del1
        } else {
            panic!("Should not reach here: Invalid index for derivative calculation.");
        }
    }

    /// Calculates the derivative with respect to z (or log(z)) at a given knot.
    pub fn calculate_ddz<D>(data: &InterpData3D<D>, ix: usize, iq2: usize, iz: usize) -> f64
    where
        D: Data<Elem = f64> + RawDataClone + Clone,
    {
        let nmu2knots = data.grid[2].len();
        let mu2_coords = data.grid[2].as_slice().unwrap();
        let log_mu2_coords: Vec<f64> = mu2_coords.iter().map(|&mui| mui.ln()).collect();
        let values = &data.values;

        let del1 = match iz {
            0 => 0.0,
            i => log_mu2_coords[i] - log_mu2_coords[i - 1],
        };

        let del2 = match log_mu2_coords.get(iz + 1) {
            Some(&next) => next - log_mu2_coords[iz],
            None => 0.0,
        };

        if iz != 0 && iz != nmu2knots - 1 {
            // Central difference
            let lddmu = (values[[ix, iq2, iz]] - values[[ix, iq2, iz - 1]]) / del1;
            let rddmu = (values[[ix, iq2, iz + 1]] - values[[ix, iq2, iz]]) / del2;
            (lddmu + rddmu) / 2.0
        } else if iz == 0 {
            // Forward difference
            (values[[ix, iq2, iz + 1]] - values[[ix, iq2, iz]]) / del2
        } else if iz == nmu2knots - 1 {
            // Backward difference
            (values[[ix, iq2, iz]] - values[[ix, iq2, iz - 1]]) / del1
        } else {
            panic!("Should not reach here: Invalid index for derivative calculation.");
        }
    }

    /// Corrected Hermite tricubic interpolation that properly handles the 3D nature
    fn hermite_tricubic_interpolate<D>(
        &self,
        data: &InterpData3D<D>,
        indices: (usize, usize, usize),
        u: f64,
        v: f64,
        w: f64,
    ) -> f64
    where
        D: Data<Elem = f64> + RawDataClone + Clone,
    {
        let (ix, iq2, iz) = indices;
        let values = &data.values;

        let get = |dx, dy, dz| values[[ix + dx, iq2 + dy, iz + dz]];
        let ddx = |dx, dy, dz| Self::calculate_ddx(data, ix + dx, iq2 + dy, iz + dz);
        let ddy = |dx, dy, dz| Self::calculate_ddy(data, ix + dx, iq2 + dy, iz + dz);
        let ddz = |dx, dy, dz| Self::calculate_ddz(data, ix + dx, iq2 + dy, iz + dz);

        let cx = [(0, 0), (0, 1), (1, 0), (1, 1)].map(|(dy, dz)| {
            let f0 = get(0, dy, dz);
            let f1 = get(1, dy, dz);
            let d0 = ddx(0, dy, dz);
            let d1 = ddx(1, dy, dz);
            Self::cubic_interpolate(u, f0, d0, f1, d1)
        });

        let cy = [(0, 0), (0, 1), (1, 0), (1, 1)].map(|(dy, dz)| ddy(0, dy, dz));

        let c0 = Self::cubic_interpolate(v, cx[0], cy[0], cx[2], cy[2]);
        let c1 = Self::cubic_interpolate(v, cx[1], cy[1], cx[3], cy[3]);

        let cz0 = ddz(0, 0, 0);
        let cz1 = ddz(0, 0, 1);

        Self::cubic_interpolate(w, c0, cz0, c1, cz1)
    }

    /// Hermite cubic interpolation with derivatives
    fn cubic_interpolate(t: f64, f0: f64, f0_prime: f64, f1: f64, f1_prime: f64) -> f64 {
        let t2 = t * t;
        let t3 = t2 * t;

        // Hermite basis functions
        let h00 = 2.0 * t3 - 3.0 * t2 + 1.0;
        let h10 = t3 - 2.0 * t2 + t;
        let h01 = -2.0 * t3 + 3.0 * t2;
        let h11 = t3 - t2;

        h00 * f0 + h10 * f0_prime + h01 * f1 + h11 * f1_prime
    }
}

impl<D> Strategy3D<D> for LogTricubicInterpolation
where
    D: Data<Elem = f64> + RawDataClone + Clone,
{
    fn init(&mut self, data: &InterpData3D<D>) -> Result<(), ValidateError> {
        // Get the coordinate arrays
        let x_coords = data.grid[0].as_slice().unwrap();
        let y_coords = data.grid[1].as_slice().unwrap();
        let z_coords = data.grid[2].as_slice().unwrap();

        // Check that all coordinates are positive for logarithmic scaling
        if x_coords.iter().any(|&x| x <= 0.0)
            || y_coords.iter().any(|&y| y <= 0.0)
            || z_coords.iter().any(|&z| z <= 0.0)
        {
            return Err(ValidateError::Other(
                "All input values must be positive for logarithmic scaling".to_string(),
            ));
        }

        // Check that we have at least 4x4x4 grid for tricubic interpolation
        if x_coords.len() < 4 || y_coords.len() < 4 || z_coords.len() < 4 {
            return Err(ValidateError::Other(
                "Need at least 4x4x4 grid for tricubic interpolation".to_string(),
            ));
        }

        // Use the Hermite approach instead of coefficient precomputation
        // This is more straightforward and avoids the complex 64x64 matrix
        self.coeffs = Vec::new(); // Not needed for Hermite approach
        Ok(())
    }

    fn interpolate(
        &self,
        data: &InterpData3D<D>,
        point: &[f64; 3],
    ) -> Result<f64, InterpolateError> {
        let [x, y, z] = *point;

        // Get the coordinate arrays
        let x_coords = data.grid[0].as_slice().unwrap();
        let y_coords = data.grid[1].as_slice().unwrap();
        let z_coords = data.grid[2].as_slice().unwrap();

        // Transform coordinates to log space
        let log_x = x.ln();
        let log_y = y.ln();
        let log_z = z.ln();

        // Transform grid coordinates to log space
        let log_x_grid: Vec<f64> = x_coords.iter().map(|&xi| xi.ln()).collect();
        let log_y_grid: Vec<f64> = y_coords.iter().map(|&yi| yi.ln()).collect();
        let log_z_grid: Vec<f64> = z_coords.iter().map(|&zi| zi.ln()).collect();

        // Find the grid cell containing the point
        let i = Self::find_tricubic_interval(&log_x_grid, log_x)?;
        let j = Self::find_tricubic_interval(&log_y_grid, log_y)?;
        let k = Self::find_tricubic_interval(&log_z_grid, log_z)?;

        // Normalize coordinates to [0,1] within the cell
        let dx = log_x_grid[i + 1] - log_x_grid[i];
        let dy = log_y_grid[j + 1] - log_y_grid[j];
        let dz = log_z_grid[k + 1] - log_z_grid[k];

        if dx == 0.0 || dy == 0.0 || dz == 0.0 {
            return Err(InterpolateError::Other("Grid spacing is zero".to_string()));
        }

        let u = (log_x - log_x_grid[i]) / dx;
        let v = (log_y - log_y_grid[j]) / dy;
        let w = (log_z - log_z_grid[k]) / dz;

        // Use the corrected Hermite tricubic interpolation
        let result = self.hermite_tricubic_interpolate(data, (i, j, k), u, v, w);

        Ok(result)
    }

    fn allow_extrapolate(&self) -> bool {
        false
    }
}

/// Implements cubic interpolation for alpha_s values in log-Q2 space.
///
/// This strategy handles the specific extrapolation and interpolation rules
/// for alpha_s as defined in LHAPDF.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AlphaSCubicInterpolation;

impl AlphaSCubicInterpolation {
    /// Get the index of the closest Q2 knot row <= q2
    ///
    /// If the value is >= q2_max, return i_max-1 (for polynomial spine construction)
    fn iq2below<D>(data: &InterpData1D<D>, q2: f64) -> usize
    where
        D: Data<Elem = f64> + RawDataClone + Clone,
    {
        let q2s = data.grid[0].as_slice().unwrap();
        // Test that Q2 is in the grid range
        if q2 < *q2s.first().unwrap() {
            panic!(
                "Q2 value {} is lower than lowest-Q2 grid point at {}",
                q2,
                q2s.first().unwrap()
            );
        }
        if q2 > *q2s.last().unwrap() {
            panic!(
                "Q2 value {} is higher than highest-Q2 grid point at {}",
                q2,
                q2s.last().unwrap()
            );
        }

        // Find the closest knot below the requested value
        let idx = q2s.partition_point(|&x| x < q2);

        if idx == q2s.len() {
            // q2 is greater than or equal to the last element.
            // Since we already checked q2 > last element, it must be equal.
            // For interpolation, we need the interval [idx-1, idx].
            idx - 1
        } else if (q2s[idx] - q2).abs() < 1e-9 {
            // q2 is exactly a knot.
            // If it's the last knot, we need the interval [idx-1, idx].
            // Otherwise, we use the knot itself as the lower bound of the interval.
            if idx == q2s.len() - 1 && q2s.len() >= 2 {
                idx - 1
            } else {
                idx
            }
        } else {
            // q2 is between two knots.
            // idx is the first element greater than q2, so idx-1 is the lower bound.
            idx - 1
        }
    }

    /// Forward derivative w.r.t. logQ2
    fn ddlogq_forward<D>(data: &InterpData1D<D>, i: usize) -> f64
    where
        D: Data<Elem = f64> + RawDataClone + Clone,
    {
        let logq2s: Vec<f64> = data.grid[0]
            .as_slice()
            .unwrap()
            .iter()
            .map(|&q2| q2.ln())
            .collect();
        let alphas = data.values.as_slice().unwrap();
        (alphas[i + 1] - alphas[i]) / (logq2s[i + 1] - logq2s[i])
    }

    /// Backward derivative w.r.t. logQ2
    fn ddlogq_backward<D>(data: &InterpData1D<D>, i: usize) -> f64
    where
        D: Data<Elem = f64> + RawDataClone + Clone,
    {
        let logq2s: Vec<f64> = data.grid[0]
            .as_slice()
            .unwrap()
            .iter()
            .map(|&q2| q2.ln())
            .collect();
        let alphas = data.values.as_slice().unwrap();
        (alphas[i] - alphas[i - 1]) / (logq2s[i] - logq2s[i - 1])
    }

    /// Central (avg of forward and backward) derivative w.r.t. logQ2
    fn ddlogq_central<D>(data: &InterpData1D<D>, i: usize) -> f64
    where
        D: Data<Elem = f64> + RawDataClone + Clone,
    {
        0.5 * (Self::ddlogq_forward(data, i) + Self::ddlogq_backward(data, i))
    }
}

impl<D> Strategy1D<D> for AlphaSCubicInterpolation
where
    D: Data<Elem = f64> + RawDataClone + Clone,
{
    fn interpolate(
        &self,
        data: &InterpData1D<D>,
        point: &[f64; 1],
    ) -> Result<f64, InterpolateError> {
        let q2 = point[0];
        let q2s = data.grid[0].as_slice().unwrap();
        let alphas = data.values.as_slice().unwrap();
        let logq2s: Vec<f64> = q2s.iter().map(|&q2| q2.ln()).collect();

        assert!(q2 >= 0.0);

        // Using base 10 for logs to get constant gradient extrapolation in
        // a log 10 - log 10 plot
        if q2 < *q2s.first().unwrap() {
            // Remember to take situations where the first knot also is a
            // flavor threshold into account
            let mut next_point = 1;
            while q2s[0] == q2s[next_point] {
                next_point += 1;
            }
            let dlogq2 = (q2s[next_point] / q2s[0]).log10();
            let dlogas = (alphas[next_point] / alphas[0]).log10();
            let loggrad = dlogas / dlogq2;
            return Ok(alphas[0] * (q2 / q2s[0]).powf(loggrad));
        }

        if q2 > *q2s.last().unwrap() {
            return Ok(*alphas.last().unwrap());
        }

        // Get the Q/alpha_s index on this array which is *below* this Q point
        let i = Self::iq2below(data, q2);

        // Calculate derivatives
        let didlogq2: f64;
        let di1dlogq2: f64;
        if i == 0 {
            didlogq2 = Self::ddlogq_forward(data, i);
            di1dlogq2 = Self::ddlogq_central(data, i + 1);
        } else if i == logq2s.len() - 2 {
            didlogq2 = Self::ddlogq_central(data, i);
            di1dlogq2 = Self::ddlogq_backward(data, i + 1);
        } else {
            didlogq2 = Self::ddlogq_central(data, i);
            di1dlogq2 = Self::ddlogq_central(data, i + 1);
        }

        // Calculate alpha_s
        let dlogq2 = logq2s[i + 1] - logq2s[i];
        let tlogq2 = (q2.ln() - logq2s[i]) / dlogq2;
        Ok(utils::hermite_cubic_interpolate(
            tlogq2,
            alphas[i],
            didlogq2 * dlogq2,
            alphas[i + 1],
            di1dlogq2 * dlogq2,
        ))
    }

    fn allow_extrapolate(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::{Array1, Array2, Array3, OwnedRepr};
    use ninterp::data::{InterpData1D, InterpData2D};
    use ninterp::interpolator::{Extrapolate, InterpND};
    use ninterp::prelude::Interpolator;
    use ninterp::strategy::Linear;

    // Helper constants for commonly used values
    const EPSILON: f64 = 1e-9;

    fn create_test_data_1d(
        q2_values: Vec<f64>,
        alphas_vals: Vec<f64>,
    ) -> InterpData1D<OwnedRepr<f64>> {
        InterpData1D::new(Array1::from(q2_values), Array1::from(alphas_vals)).unwrap()
    }

    fn create_test_data_2d(
        x_coords: Vec<f64>,
        y_coords: Vec<f64>,
        values: Vec<f64>,
    ) -> InterpData2D<OwnedRepr<f64>> {
        let shape = (x_coords.len(), y_coords.len());
        let values_array = Array2::from_shape_vec(shape, values).unwrap();
        InterpData2D::new(x_coords.into(), y_coords.into(), values_array).unwrap()
    }

    fn create_test_data_3d(
        x_coords: Vec<f64>,
        y_coords: Vec<f64>,
        z_coords: Vec<f64>,
        values: Vec<f64>,
    ) -> InterpData3D<OwnedRepr<f64>> {
        let shape = (x_coords.len(), y_coords.len(), z_coords.len());
        let values_array = Array3::from_shape_vec(shape, values).unwrap();
        InterpData3D::new(
            x_coords.into(),
            y_coords.into(),
            z_coords.into(),
            values_array,
        )
        .unwrap()
    }

    fn create_target_data_2d(max_num: i32) -> Vec<f64> {
        (1..=max_num)
            .flat_map(|i| (1..=max_num).map(move |j| (i * j) as f64))
            .collect()
    }

    fn assert_close(actual: f64, expected: f64, tolerance: f64) {
        assert!(
            (actual - expected).abs() < tolerance,
            "Expected {}, got {} (diff: {})",
            expected,
            actual,
            (actual - expected).abs()
        );
    }

    #[test]
    fn test_linear_interpolate() {
        let test_cases = [
            // (x1, x2, y1, y2, x, expected)
            (0.0, 1.0, 0.0, 10.0, 0.5, 5.0),
            (0.0, 10.0, 0.0, 100.0, 2.5, 25.0),
            (0.0, 1.0, 0.0, 10.0, 0.0, 0.0),   // At start endpoint
            (0.0, 1.0, 0.0, 10.0, 1.0, 10.0),  // At end endpoint
            (5.0, 5.0, 10.0, 20.0, 5.0, 10.0), // x1 == x2 case
        ];

        for (x1, x2, y1, y2, x, expected) in test_cases {
            let result = BilinearInterpolation::linear_interpolate(x1, x2, y1, y2, x);
            assert_close(result, expected, EPSILON);
        }
    }

    #[test]
    fn test_bilinear_interpolation() {
        let data = create_test_data_2d(
            vec![0.0, 1.0, 2.0],
            vec![0.0, 1.0, 2.0],
            vec![0.0, 1.0, 2.0, 1.0, 2.0, 3.0, 2.0, 3.0, 4.0],
        );

        let test_cases = [
            ([0.5, 0.5], 1.0),
            ([1.0, 1.0], 2.0), // Grid point
            ([0.25, 0.75], 1.0),
        ];

        for (point, expected) in test_cases {
            let result = BilinearInterpolation.interpolate(&data, &point).unwrap();
            assert_close(result, expected, EPSILON);
        }
    }

    #[test]
    fn test_log_bilinear_interpolation() {
        let data = create_test_data_2d(
            vec![1.0, 10.0, 100.0],
            vec![1.0, 10.0, 100.0],
            vec![0.0, 1.0, 2.0, 1.0, 2.0, 3.0, 2.0, 3.0, 4.0],
        );
        LogBilinearInterpolation.init(&data).unwrap();

        let test_cases = [
            ([3.16227766, 3.16227766], 1.0), // sqrt(10)
            ([10.0, 10.0], 2.0),             // Grid point
            ([1.77827941, 5.62341325], 1.0), // 10^0.25, 10^0.75
        ];

        for (point, expected) in test_cases {
            let result = LogBilinearInterpolation.interpolate(&data, &point).unwrap();
            assert_close(result, expected, EPSILON);
        }
    }

    #[test]
    fn test_log_bilinear_init_validation() {
        let invalid_data = create_test_data_2d(
            vec![0.0, 1.0, 2.0],  // Contains zero-valued
            vec![-1.0, 2.0, 3.0], // contains negative alue
            vec![0.0; 9],
        );

        let result = LogBilinearInterpolation.init(&invalid_data);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "The input values must be positive for logarithmic scaling"
        );
    }

    #[test]
    fn test_log_tricubic_interpolation() {
        // Create a simple 5x5x5 grid
        let x_coords = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y_coords = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let z_coords = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let values: Vec<f64> = (1..6)
            .flat_map(|i| (1..6).flat_map(move |j| (1..6).map(move |k| (i + j + k) as f64)))
            .collect();
        let interp_data = create_test_data_3d(
            x_coords.clone(),
            y_coords.clone(),
            z_coords.clone(),
            values.clone(),
        );

        let mut interpolator = LogTricubicInterpolation::default();
        interpolator.init(&interp_data).unwrap();

        let point = [1.5, 1.5, 1.5];
        let result = interpolator.interpolate(&interp_data, &point).unwrap();
        assert_close(result, 4.5, 2e-2);

        // Compare to general ND interpolation
        let interp_data_arr = Array3::from_shape_vec((5, 5, 5), values).unwrap();
        let nd_interp = InterpND::new(
            vec![x_coords.into(), y_coords.into(), z_coords.into()],
            interp_data_arr.into_dyn(),
            Linear,
            Extrapolate::Error,
        )
        .unwrap();
        let nd_interp_res = nd_interp.interpolate(&point).unwrap();
        assert_close(nd_interp_res, 4.5, EPSILON);
    }

    #[test]
    fn test_alphas_cubic_interpolation() {
        let q_values = [1.0, 2.0, 3.0, 4.0, 5.0];
        let alphas_vals = vec![0.1, 0.11, 0.12, 0.13, 0.14];
        let q2_values: Vec<f64> = q_values.iter().map(|&q| q * q).collect();
        let data = create_test_data_1d(q2_values, alphas_vals);
        let alphas_cubic = AlphaSCubicInterpolation;

        // Test within interpolation range
        let result = alphas_cubic.interpolate(&data, &[2.25]).unwrap(); // Q=1.5
        assert!(result > 0.1 && result < 0.14);

        // Test at grid point
        let result = alphas_cubic.interpolate(&data, &[4.0]).unwrap(); // Q=2.0
        assert_close(result, 0.11, EPSILON);

        // Test extrapolation below range
        let result = alphas_cubic.interpolate(&data, &[0.5]).unwrap(); // Q=sqrt(0.5)
        assert!(result < 0.1);

        // Test extrapolation above range
        let result = alphas_cubic.interpolate(&data, &[30.0]).unwrap(); // Q=sqrt(30)
        assert_close(result, 0.14, EPSILON);
    }

    #[test]
    fn test_find_bicubic_interval() {
        let coords = vec![1.0, 2.0, 3.0, 4.0, 5.0];

        let test_cases = [
            (1.5, Ok(0)),
            (3.5, Ok(2)),
            (2.0, Ok(1)),   // At knot point
            (1.0, Ok(0)),   // At boundary
            (4.99, Ok(3)),  // Near boundary
            (0.5, Err(())), // Out of bounds
            (5.5, Err(())), // Out of bounds
        ];

        for (value, expected) in test_cases {
            let result = LogBicubicInterpolation::find_bicubic_interval(&coords, value);
            match expected {
                Ok(expected_idx) => assert_eq!(result.unwrap(), expected_idx),
                Err(_) => assert!(result.is_err()),
            }
        }
    }

    #[test]
    fn test_hermite_cubic_interpolate_from_coeffs() {
        let test_cases = [
            // Linear function x: coeffs = [0, 0, 1, 0]
            ([0.0, 0.0, 1.0, 0.0], 0.5, 0.5),
            ([0.0, 0.0, 1.0, 0.0], 1.0, 1.0),
            // Constant function 5: coeffs = [0, 0, 0, 5]
            ([0.0, 0.0, 0.0, 5.0], 0.5, 5.0),
            // Cubic function x^3: coeffs = [1, 0, 0, 0]
            ([1.0, 0.0, 0.0, 0.0], 2.0, 8.0),
            ([1.0, 0.0, 0.0, 0.0], 0.5, 0.125),
            // Complex polynomial 2x^3 - 3x^2 + x + 4
            ([2.0, -3.0, 1.0, 4.0], 1.0, 4.0),
            ([2.0, -3.0, 1.0, 4.0], 0.0, 4.0),
            ([2.0, -3.0, 1.0, 4.0], 2.0, 10.0),
        ];

        for (coeffs, x, expected) in test_cases {
            let result = LogBicubicInterpolation::hermite_cubic_interpolate_from_coeffs(x, &coeffs);
            assert_close(result, expected, EPSILON);
        }
    }

    #[test]
    fn test_log_bicubic_init_validation() {
        let test_cases = [
            // Non-positive x_coords
            (vec![-1.0, 1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0, 4.0]),
            // Non-positive y_coords
            (vec![1.0, 2.0, 3.0, 4.0], vec![-1.0, 2.0, 3.0, 4.0]),
        ];

        for (x_coords, y_coords) in test_cases {
            let data = create_test_data_2d(x_coords, y_coords, vec![0.0; 16]);
            let mut log_bicubic = LogBicubicInterpolation::default();
            let result = log_bicubic.init(&data);

            assert!(result.is_err());
            assert_eq!(
                result.unwrap_err().to_string(),
                "The input values must be positive for logarithmic scaling"
            );
        }

        // Test insufficient grid size
        let data_small =
            create_test_data_2d(vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0], vec![0.0; 9]);
        let mut log_bicubic = LogBicubicInterpolation::default();
        let result = log_bicubic.init(&data_small);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Need at least 4x4 grid for bicubic interpolation"
        );

        // Test valid data
        let data_valid = create_test_data_2d(
            vec![1.0, 2.0, 3.0, 4.0],
            vec![1.0, 2.0, 3.0, 4.0],
            vec![0.0; 16],
        );
        let mut log_bicubic = LogBicubicInterpolation::default();
        assert!(log_bicubic.init(&data_valid).is_ok());
    }

    #[test]
    fn test_log_bicubic_interpolation() {
        let target_data = create_target_data_2d(4);
        let data = create_test_data_2d(
            vec![1.0, 10.0, 100.0, 1000.0],
            vec![1.0, 10.0, 100.0, 1000.0],
            target_data,
        );

        let mut log_bicubic = LogBicubicInterpolation::default();
        log_bicubic.init(&data).unwrap();

        let test_cases = [
            ([10.0, 10.0], 4.0),              // Grid point
            ([3.16227766, 3.16227766], 2.25), // sqrt(10)
            ([31.6227766, 31.6227766], 6.25), // 10^1.5
        ];

        for (point, expected) in test_cases {
            let result = log_bicubic.interpolate(&data, &point).unwrap();
            assert_close(result, expected, EPSILON);
        }
    }

    #[test]
    fn test_ddlogq_derivatives() {
        let data = create_test_data_1d(vec![1.0, 2.0, 3.0, 4.0], vec![0.1, 0.2, 0.3, 0.4]);

        // Forward derivative
        let expected_forward = 0.1 / 2.0f64.ln();
        assert_close(
            AlphaSCubicInterpolation::ddlogq_forward(&data, 0),
            expected_forward,
            EPSILON,
        );

        // Backward derivative
        let expected_backward = 0.1 / 2.0f64.ln();
        assert_close(
            AlphaSCubicInterpolation::ddlogq_backward(&data, 1),
            expected_backward,
            EPSILON,
        );

        // Central derivative
        let expected_central =
            0.5 * (0.1 / (3.0f64.ln() - 2.0f64.ln()) + 0.1 / (2.0f64.ln() - 1.0f64.ln()));
        assert_close(
            AlphaSCubicInterpolation::ddlogq_central(&data, 1),
            expected_central,
            EPSILON,
        );
    }

    #[test]
    fn test_iq2below() {
        let data =
            create_test_data_1d(vec![1.0, 2.0, 3.0, 4.0, 5.0], vec![0.1, 0.2, 0.3, 0.4, 0.5]);

        let test_cases = [
            (1.5, 0),
            (2.0, 1),
            (3.9, 2), // Within range
            (1.0, 0),
            (5.0, 3), // At boundaries
        ];

        for (q2_val, expected_idx) in test_cases {
            assert_eq!(
                AlphaSCubicInterpolation::iq2below(&data, q2_val),
                expected_idx
            );
        }

        // Test edge cases with different data sizes
        let data_small = create_test_data_1d(vec![1.0, 2.0], vec![0.1, 0.2]);
        assert_eq!(AlphaSCubicInterpolation::iq2below(&data_small, 2.0), 0);

        let data_with_mid = create_test_data_1d(vec![1.0, 2.0, 3.0], vec![0.1, 0.2, 0.3]);
        assert_eq!(AlphaSCubicInterpolation::iq2below(&data_with_mid, 2.0), 1);

        // Test panic conditions
        let data_single = create_test_data_1d(vec![1.0], vec![0.1]);

        let result = std::panic::catch_unwind(|| {
            AlphaSCubicInterpolation::iq2below(&data_single, 0.5);
        });
        assert!(result.is_err());

        let result = std::panic::catch_unwind(|| {
            AlphaSCubicInterpolation::iq2below(&data_single, 1.5);
        });
        assert!(result.is_err());
    }
}
