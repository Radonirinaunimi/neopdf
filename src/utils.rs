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
