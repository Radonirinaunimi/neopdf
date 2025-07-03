use crate::Info;
use std::fs;
use std::path::Path;

/// Reads the `.info` file for a PDF set and deserializes it into an `Info` struct.
///
/// # Arguments
///
/// * `path` - The path to the `.info` file.
///
/// # Returns
///
/// A `Result` containing the `Info` struct if successful, or a `serde_yaml::Error` otherwise.
pub fn read_info(path: &Path) -> Result<Info, serde_yaml::Error> {
    let content = fs::read_to_string(path).unwrap();
    serde_yaml::from_str(&content)
}

/// Reads a `.dat` file for a PDF set and parses its content.
///
/// This function extracts x-knots, Q2-knots, flavor IDs, and the grid data
/// from the specified data file.
///
/// # Arguments
///
/// * `path` - The path to the `.dat` file.
///
/// # Returns
///
/// A tuple containing:
/// * `Vec<f64>`: x-knots
/// * `Vec<f64>`: Q2-knots (squared Q values)
/// * `Vec<i32>`: Flavor IDs
/// * `Vec<f64>`: Flat vector of grid data
pub fn read_data(path: &Path) -> (Vec<f64>, Vec<f64>, Vec<i32>, Vec<f64>) {
    let content = fs::read_to_string(path).unwrap();
    let mut lines = content.lines();

    // Skip lines until "---" is encountered
    for line in lines.by_ref() {
        if line.trim() == "---" {
            break;
        }
    }

    // Read the x knots
    let x_knots_line = lines.next().unwrap();
    let xs: Vec<f64> = x_knots_line
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Read the Q2 knots
    let q2_knots_line = lines.next().unwrap();
    // NOTE: Values might be in `Q` or `Q2`. To check.
    let q2s: Vec<f64> = q2_knots_line
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .map(|q: f64| q * q)
        .collect();

    // Read the flavors
    let flavors_line = lines.next().unwrap();
    let flavors: Vec<i32> = flavors_line
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Read the grid values
    let mut grid_data = Vec::new();
    for line in lines {
        if line.trim() == "---" {
            break;
        }
        let values: Vec<f64> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        grid_data.extend(values);
    }

    (xs, q2s, flavors, grid_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_read_info() {
        let yaml_content = r#"
SetDesc: "NNPDF40_nnlo_as_01180"
SetIndex: 4000
NumMembers: 101
XMin: 1.0e-9
XMax: 1.0
QMin: 1.0
QMax: 10000.0
Flavors: [21, 1, 2, 3, 4, 5, -1, -2, -3, -4, -5]
Format: "LHAPDF"
"#;
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "{}", yaml_content).unwrap();
        let info = read_info(temp_file.path()).unwrap();

        assert_eq!(info.set_desc, "NNPDF40_nnlo_as_01180");
        assert_eq!(info.set_index, 4000);
        assert_eq!(info.num_members, 101);
        assert_eq!(info.x_min, 1.0e-9);
        assert_eq!(info.x_max, 1.0);
        assert_eq!(info.q_min, 1.0);
        assert_eq!(info.q_max, 10000.0);
        assert_eq!(info.flavors, vec![21, 1, 2, 3, 4, 5, -1, -2, -3, -4, -5]);
        assert_eq!(info.format, "LHAPDF");
    }

    #[test]
    fn test_read_data() {
        let data_content = r#"
# Some header
---
1.0e-9 1.0e-8 1.0e-7
1.0 10.0 100.0
21 1 2
1.0 2.0 3.0
4.0 5.0 6.0
7.0 8.0 9.0
"#;
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "{}", data_content).unwrap();
        let (xs, q2s, flavors, grid_data) = read_data(temp_file.path());

        assert_eq!(xs, vec![1.0e-9, 1.0e-8, 1.0e-7]);
        assert_eq!(q2s, vec![1.0, 100.0, 10000.0]); // Q values are squared
        assert_eq!(flavors, vec![21, 1, 2]);
        assert_eq!(grid_data, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
    }
}
