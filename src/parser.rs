use crate::{Info, KnotArray};
use std::fs;
use std::path::Path;

pub fn read_info(path: &Path) -> Result<Info, serde_yaml::Error> {
    let content = fs::read_to_string(path).unwrap();
    serde_yaml::from_str(&content)
}

pub fn read_data(path: &Path, knot_array: &mut KnotArray) {
    let content = fs::read_to_string(path).unwrap();
    let mut lines = content.lines();

    // Skip lines until "---" is encountered
    while let Some(line) = lines.next() {
        if line.trim() == "---" {
            break;
        }
    }

    // Read the x knots
    let x_knots_line = lines.next().unwrap();
    knot_array.xs = x_knots_line
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    knot_array.logxs = knot_array.xs.iter().map(|&x| x.ln()).collect();

    // Read the Q2 knots
    let q2_knots_line = lines.next().unwrap();
    knot_array.q2s = q2_knots_line
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    knot_array.logq2s = knot_array.q2s.iter().map(|&q2| q2.ln()).collect();

    // Read the flavors
    let flavors_line = lines.next().unwrap();
    knot_array.flavors = flavors_line
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Read the grid values
    for line in lines {
        let values: Vec<f64> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        knot_array.grid.extend(values);
    }

    knot_array.shape = vec![
        knot_array.xs.len(),
        knot_array.q2s.len(),
        knot_array.flavors.len(),
    ];
}