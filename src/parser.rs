use crate::Info;
use std::fs;
use std::path::Path;

pub fn read_info(path: &Path) -> Result<Info, serde_yaml::Error> {
    let content = fs::read_to_string(path).unwrap();
    serde_yaml::from_str(&content)
}

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
    let q2s: Vec<f64> = q2_knots_line
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .map(|q: f64| q) // Values are already Q^2
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
