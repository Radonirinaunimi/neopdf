/// Finds the index of the interval in a sorted coordinate array that contains the given value.
///
/// This function performs a binary search to efficiently locate the correct interval.
///
/// # Arguments
///
/// * `coords` - A sorted slice of f64 values representing the coordinates.
/// * `value` - The f64 value for which to find the interval.
///
/// # Returns
///
/// A `Result` containing the 0-based index of the left bound of the interval if successful.
/// Returns an `InterpolateError::ExtrapolateError` if the value is outside the bounds
/// of the `coords` array.
pub fn find_interval_index(
    coords: &[f64],
    value: f64,
) -> Result<usize, ninterp::error::InterpolateError> {
    // Check bounds
    if value < coords[0] || value > coords[coords.len() - 1] {
        return Err(ninterp::error::InterpolateError::ExtrapolateError(
            "Out of Bounds!".to_string(),
        ));
    }

    // Handle exact match with last coordinate
    if value == coords[coords.len() - 1] {
        return Ok(coords.len() - 2);
    }

    // Binary search for the interval
    let mut left = 0;
    let mut right = coords.len() - 1;

    while left < right {
        let mid = (left + right) / 2;
        if coords[mid] <= value {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    Ok(left - 1)
}

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
pub fn linear_interpolate(x1: f64, x2: f64, y1: f64, y2: f64, x: f64) -> f64 {
    if x1 == x2 {
        return y1; // Avoid division by zero
    }
    y1 + (y2 - y1) * (x - x1) / (x2 - x1)
}

use ninterp::data::InterpData2D;
use ndarray::{Data, RawDataClone};



/// Cubic interpolation using Catmull-Rom spline
/// t should be in [1, 2] for interpolation between points[1] and points[2]
pub fn catmull_rom_cubic_interpolate(points: &[f64; 4], t: f64) -> f64 {
    let t2 = t * t;
    let t3 = t2 * t;

    // Catmull-Rom spline coefficients
    let c0 = -0.5 * t3 + t2 - 0.5 * t;
    let c1 = 1.5 * t3 - 2.5 * t2 + 1.0;
    let c2 = -1.5 * t3 + 2.0 * t2 + 0.5 * t;
    let c3 = 0.5 * t3 - 0.5 * t2;

    c0 * points[0] + c1 * points[1] + c2 * points[2] + c3 * points[3]
}

/// One-dimensional cubic interpolation using Hermite basis functions.
///
/// @arg t is the fractional distance of the evaluation x into the dx
/// interval.  @arg vl and @arg vh are the function values at the low and
/// high edges of the interval. @arg vdl and @arg vdh are linearly
/// extrapolated value changes from the product of dx and the discrete low-
/// and high-edge derivative estimates.
pub fn hermite_cubic_interpolate(t: f64, vl: f64, vdl: f64, vh: f64, vdh: f64) -> f64 {
    let t2 = t * t;
    let t3 = t2 * t;

    let p0 = (2.0 * t3 - 3.0 * t2 + 1.0) * vl;
    let m0 = (t3 - 2.0 * t2 + t) * vdl;
    let p1 = (-2.0 * t3 + 3.0 * t2) * vh;
    let m1 = (t3 - t2) * vdh;

    p0 + m0 + p1 + m1
}

/// Cubic interpolation using a passed array of coefficients (a*x^3 + b*x^2 + c*x + d)
pub fn hermite_cubic_interpolate_from_coeffs(t: f64, coeffs: &[f64; 4]) -> f64 {
    let x = t;
    let x2 = x * x;
    let x3 = x2 * x;
    coeffs[0] * x3 + coeffs[1] * x2 + coeffs[2] * x + coeffs[3]
}

/// Calculates the derivative with respect to x (or log(x)) at a given knot.
/// This mirrors the _ddx function in LHAPDF's C++ implementation.
pub fn calculate_ddx<D>(
    data: &InterpData2D<D>,
    ix: usize,
    iq2: usize,
    logspace: bool,
) -> f64
where
    D: Data<Elem = f64> + RawDataClone + Clone,
{
    let nxknots = data.grid[0].len();
    let x_coords = data.grid[0].as_slice().unwrap();
    let log_x_coords: Vec<f64> = x_coords.iter().map(|&xi| xi.ln()).collect();
    let values = &data.values;

    let (del1, del2) = if logspace {
        let d1 = if ix == 0 { 0.0 } else { log_x_coords[ix] - log_x_coords[ix - 1] };
        let d2 = if ix == nxknots - 1 { 0.0 } else { log_x_coords[ix + 1] - log_x_coords[ix] };
        (d1, d2)
    } else {
        let d1 = if ix == 0 { 0.0 } else { x_coords[ix] - x_coords[ix - 1] };
        let d2 = if ix == nxknots - 1 { 0.0 } else { x_coords[ix + 1] - x_coords[ix] };
        (d1, d2)
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


/// Find the interval for bicubic interpolation
/// Returns the index i such that we can use points [i-1, i, i+1, i+2] for interpolation
pub fn find_bicubic_interval(
    coords: &[f64],
    x: f64,
) -> Result<usize, ninterp::error::InterpolateError> {
    // For bicubic, we need to ensure we can access [i-1, i, i+1, i+2]
    // So i must be in range [1, len-2]
    if x < coords[1] || x > coords[coords.len() - 2] {
        return Err(ninterp::error::InterpolateError::Other(format!(
            "Point {} is outside the bicubic interpolation bounds [{}, {}]",
            x,
            coords[1],
            coords[coords.len() - 2]
        )));
    }

    // Binary search for the interval, but constrained to [1, len-2]
    let mut left = 1;
    let mut right = coords.len() - 2;

    while right - left > 1 {
        let mid = (left + right) / 2;
        if coords[mid] <= x {
            left = mid;
        } else {
            right = mid;
        }
    }

    Ok(left)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_interval_index() {
        let coords = vec![0.0, 1.0, 2.0, 3.0, 4.0];

        // Test within bounds
        assert_eq!(find_interval_index(&coords, 0.5).unwrap(), 0);
        assert_eq!(find_interval_index(&coords, 1.0).unwrap(), 1);
        assert_eq!(find_interval_index(&coords, 1.5).unwrap(), 1);
        assert_eq!(find_interval_index(&coords, 3.9).unwrap(), 3);

        // Test at boundaries
        assert_eq!(find_interval_index(&coords, 0.0).unwrap(), 0);
        assert_eq!(find_interval_index(&coords, 4.0).unwrap(), 3);

        // Test out of bounds
        assert!(find_interval_index(&coords, -0.1).is_err());
        assert!(find_interval_index(&coords, 4.1).is_err());
    }

    #[test]
    fn test_linear_interpolate() {
        // Basic interpolation
        assert_eq!(linear_interpolate(0.0, 1.0, 0.0, 10.0, 0.5), 5.0);
        assert_eq!(linear_interpolate(0.0, 10.0, 0.0, 100.0, 2.5), 25.0);

        // At endpoints
        assert_eq!(linear_interpolate(0.0, 1.0, 0.0, 10.0, 0.0), 0.0);
        assert_eq!(linear_interpolate(0.0, 1.0, 0.0, 10.0, 1.0), 10.0);

        // x1 == x2 case
        assert_eq!(linear_interpolate(5.0, 5.0, 10.0, 20.0, 5.0), 10.0);
    }
}
