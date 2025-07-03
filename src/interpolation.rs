use ndarray::{Data, RawDataClone};
use ninterp::data::InterpData2D;
use ninterp::error::InterpolateError;
use ninterp::strategy::traits::Strategy2D;

use crate::utils;

/// Implements bilinear interpolation for 2D data.
#[derive(Debug, Clone)]
pub struct Bilinear;

impl<D> Strategy2D<D> for Bilinear
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
        // First, interpolate in x-direction
        let r1 = utils::linear_interpolate(x1, x2, q11, q21, x);
        let r2 = utils::linear_interpolate(x1, x2, q12, q22, x);

        // Then interpolate in y-direction
        let result = utils::linear_interpolate(y1, y2, r1, r2, y);

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
#[derive(Debug, Clone)]
pub struct LogBilinearStrategy;

impl<D> Strategy2D<D> for LogBilinearStrategy
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
    fn init(&mut self, data: &InterpData2D<D>) -> Result<(), ninterp::error::ValidateError> {
        // Get the coordinate arrays and data values
        let x_coords = data.grid[0].as_slice().unwrap();
        let y_coords = data.grid[1].as_slice().unwrap();

        if x_coords.iter().any(|&x| x <= 0.0) || y_coords.iter().any(|&y| y <= 0.0) {
            return Err(ninterp::error::ValidateError::Other(
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
    ) -> Result<f64, ninterp::error::InterpolateError> {
        let [x, y] = *point;

        // Get the coordinate arrays and data values
        let x_coords = data.grid[0].as_slice().unwrap();
        let y_coords = data.grid[1].as_slice().unwrap();
        let values = &data.values;

        // Transform coordinates to log space if needed
        let x_interp = x.ln();
        let y_interp = y.ln();

        // Transform grid coordinates to log space if needed
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
#[derive(Debug, Clone)]
pub struct LogBicubic;

impl<D> Strategy2D<D> for LogBicubic
where
    D: Data<Elem = f64> + RawDataClone + Clone,
{
    fn init(&mut self, data: &InterpData2D<D>) -> Result<(), ninterp::error::ValidateError> {
        // Get the coordinate arrays and data values
        let x_coords = data.grid[0].as_slice().unwrap();
        let y_coords = data.grid[1].as_slice().unwrap();

        if x_coords.iter().any(|&x| x <= 0.0) || y_coords.iter().any(|&y| y <= 0.0) {
            return Err(ninterp::error::ValidateError::Other(
                "The input values must be positive for logarithmic scaling".to_string(),
            ));
        }

        // Check that we have at least 4x4 grid for bicubic interpolation
        if x_coords.len() < 4 || y_coords.len() < 4 {
            return Err(ninterp::error::ValidateError::Other(
                "Need at least 4x4 grid for bicubic interpolation".to_string(),
            ));
        }

        Ok(())
    }

    fn interpolate(
        &self,
        data: &InterpData2D<D>,
        point: &[f64; 2],
    ) -> Result<f64, ninterp::error::InterpolateError> {
        let [x, y] = *point;

        // Get the coordinate arrays and data values
        let x_coords = data.grid[0].as_slice().unwrap();
        let y_coords = data.grid[1].as_slice().unwrap();
        let values = &data.values;

        // Transform coordinates to log space
        let log_x = x.ln();
        let log_y = y.ln();

        // Transform grid coordinates to log space
        let log_x_grid: Vec<f64> = x_coords.iter().map(|&xi| xi.ln()).collect();
        let log_y_grid: Vec<f64> = y_coords.iter().map(|&yi| yi.ln()).collect();

        // Find the grid cell containing the point
        let i = utils::find_bicubic_interval(&log_x_grid, log_x)?;
        let j = utils::find_bicubic_interval(&log_y_grid, log_y)?;

        // Extract 4x4 grid of values around the interpolation point
        let mut grid_values = [[0.0f64; 4]; 4];
        for di in 0..4 {
            for dj in 0..4 {
                grid_values[di][dj] = values[[i - 1 + di, j - 1 + dj]];
            }
        }

        // Get the grid coordinates for the 4x4 region
        let x_grid = [
            log_x_grid[i - 1],
            log_x_grid[i],
            log_x_grid[i + 1],
            log_x_grid[i + 2],
        ];
        let y_grid = [
            log_y_grid[j - 1],
            log_y_grid[j],
            log_y_grid[j + 1],
            log_y_grid[j + 2],
        ];

        // Normalize coordinates to [0,1] within the central cell
        let dx = x_grid[2] - x_grid[1];
        let dy = y_grid[2] - y_grid[1];

        if dx == 0.0 || dy == 0.0 {
            return Err(ninterp::error::InterpolateError::Other(
                "Grid spacing is zero".to_string(),
            ));
        }

        let u = (log_x - x_grid[1]) / dx;
        let v = (log_y - y_grid[1]) / dy;

        // Perform bicubic interpolation
        let result = utils::bicubic_interpolate(&grid_values, u, v);

        Ok(result)
    }

    fn allow_extrapolate(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::Array2;
    use ninterp::data::InterpData2D;

    #[test]
    fn test_bilinear_interpolation() {
        let x_coords = vec![0.0, 1.0, 2.0];
        let y_coords = vec![0.0, 1.0, 2.0];
        let values =
            Array2::from_shape_vec((3, 3), vec![0.0, 1.0, 2.0, 1.0, 2.0, 3.0, 2.0, 3.0, 4.0])
                .unwrap();

        let data = InterpData2D::new(x_coords.into(), y_coords.into(), values).unwrap();
        let bilinear = Bilinear;

        // Test at a known point within a cell
        let point = [0.5, 0.5];
        let result = bilinear.interpolate(&data, &point).unwrap();
        assert!((result - 1.0).abs() < 1e-9);

        // Test at a grid point
        let point = [1.0, 1.0];
        let result = bilinear.interpolate(&data, &point).unwrap();
        assert!((result - 2.0).abs() < 1e-9);

        // Test another point
        let point = [0.25, 0.75];
        let result = bilinear.interpolate(&data, &point).unwrap();
        assert!((result - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_log_bilinear_interpolation() {
        let x_coords = vec![1.0, 10.0, 100.0];
        let y_coords = vec![1.0, 10.0, 100.0];
        let values =
            Array2::from_shape_vec((3, 3), vec![0.0, 1.0, 2.0, 1.0, 2.0, 3.0, 2.0, 3.0, 4.0])
                .unwrap();

        let data = InterpData2D::new(x_coords.into(), y_coords.into(), values).unwrap();
        let mut log_bilinear = LogBilinearStrategy;
        log_bilinear.init(&data).unwrap();

        // Test at a known point within a cell (log(x)=0.5, log(y)=0.5)
        let point = [3.16227766, 3.16227766]; // sqrt(10)
        let result = log_bilinear.interpolate(&data, &point).unwrap();
        assert!((result - 1.0).abs() < 1e-9);

        // Test at a grid point
        let point = [10.0, 10.0];
        let result = log_bilinear.interpolate(&data, &point).unwrap();
        assert!((result - 2.0).abs() < 1e-9);

        // Test another point
        let point = [1.77827941, 5.62341325]; // 10^0.25, 10^0.75
        let result = log_bilinear.interpolate(&data, &point).unwrap();
        assert!((result - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_log_bilinear_init_validation() {
        let x_coords = vec![0.0, 1.0, 2.0]; // Contains non-positive value
        let y_coords = vec![1.0, 2.0, 3.0];
        let values = Array2::from_elem((3, 3), 0.0);

        let data = InterpData2D::new(x_coords.into(), y_coords.into(), values).unwrap();
        let mut log_bilinear = LogBilinearStrategy;
        let result = log_bilinear.init(&data);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "The input values must be positive for logarithmic scaling"
        );
    }
}
