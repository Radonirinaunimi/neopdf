use super::metadata::MetaData;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubgridData {
    pub xs: Vec<f64>,
    pub q2s: Vec<f64>,
    pub grid_data: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PdfData {
    pub subgrid_data: Vec<SubgridData>,
    pub flavors: Vec<i32>,
}

/// Reads the `.info` file for a PDF set and deserializes it into an `Info` struct.
///
/// # Arguments
///
/// * `path` - The path to the `.info` file.
///
/// # Returns
///
/// A `Result` containing the `Info` struct if successful, or a `serde_yaml::Error` otherwise.
pub fn read_metadata(path: &Path) -> Result<MetaData, serde_yaml::Error> {
    let content = fs::read_to_string(path).unwrap();
    serde_yaml::from_str(&content)
}

/// Reads a `.dat` file for a PDF set and parses its content.
///
/// This function extracts x-knots, Q2-knots, flavor IDs, and the grid data
/// from the specified data file. It can handle files with multiple subgrids
/// separated by "---".
///
/// # Arguments
///
/// * `path` - The path to the `.dat` file.
///
/// # Returns
///
/// A tuple containing:
/// * `Vec<(Vec<f64>, Vec<f64>, Vec<f64>)>`: A vector of subgrid data, where each
///   tuple contains x-knots, Q2-knots, and the flat grid data for a subgrid.
/// * `Vec<i32>`: Flavor IDs, which are assumed to be the same for all subgrids.
pub fn read_data(path: &Path) -> PdfData {
    let content = fs::read_to_string(path).unwrap();
    let mut subgrid_data = Vec::new();
    let mut flavors = Vec::new();

    // Split the content by "---" to separate subgrids
    let blocks: Vec<&str> = content.split("---").map(|s| s.trim()).collect();

    for block in blocks.iter().skip(1) {
        // Skip empty blocks
        if block.is_empty() {
            continue;
        }

        let mut lines = block.lines();

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
            .map(|q: f64| q * q)
            .collect();

        // Read the flavors (only once from the first subgrid)
        if flavors.is_empty() {
            let flavors_line = lines.next().unwrap();
            flavors = flavors_line
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
        } else {
            // Skip the flavors line in subsequent subgrids
            lines.next();
        }

        // Read the grid values
        let mut grid_data = Vec::new();
        for line in lines {
            let values: Vec<f64> = line
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            grid_data.extend(values);
        }

        subgrid_data.push(SubgridData { xs, q2s, grid_data });
    }

    PdfData {
        subgrid_data,
        flavors,
    }
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
        let info = read_metadata(temp_file.path()).unwrap();

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
        ---
        1.0e-7 1.0e-6 1.0e-5
        100.0 1000.0 10000.0
        21 1 2
        10.0 11.0 12.0
        13.0 14.0 15.0
        16.0 17.0 18.0
        "#;
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "{}", data_content).unwrap();
        let pdf_data = read_data(temp_file.path());

        assert_eq!(pdf_data.flavors, vec![21, 1, 2]);
        assert_eq!(pdf_data.subgrid_data.len(), 2);

        // Check the first subgrid
        assert_eq!(pdf_data.subgrid_data[0].xs, vec![1.0e-9, 1.0e-8, 1.0e-7]);
        assert_eq!(pdf_data.subgrid_data[0].q2s, vec![1.0, 100.0, 10000.0]); // Q values are squared
        assert_eq!(
            pdf_data.subgrid_data[0].grid_data,
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]
        );

        // Check the second subgrid
        assert_eq!(pdf_data.subgrid_data[1].xs, vec![1.0e-7, 1.0e-6, 1.0e-5]);
        assert_eq!(
            pdf_data.subgrid_data[1].q2s,
            vec![10000.0, 1000000.0, 100000000.0]
        ); // Q values are squared
        assert_eq!(
            pdf_data.subgrid_data[1].grid_data,
            vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0]
        );
    }
}
