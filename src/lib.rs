use ndarray::{s, Array1, Array3};
use ninterp::prelude::*;
use ninterp::strategy::Linear;
use serde::Deserialize;
use std::path::Path;

pub mod interpolation;
pub mod parser;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Info {
    #[serde(rename = "SetDesc")]
    pub set_desc: String,
    #[serde(rename = "SetIndex")]
    pub set_index: u32,
    #[serde(rename = "NumMembers")]
    pub num_members: u32,
    #[serde(rename = "XMin")]
    pub x_min: f64,
    #[serde(rename = "XMax")]
    pub x_max: f64,
    #[serde(rename = "QMin")]
    pub q_min: f64,
    #[serde(rename = "QMax")]
    pub q_max: f64,
    #[serde(rename = "Flavors")]
    pub flavors: Vec<i32>,
    #[serde(rename = "Format")]
    pub format: String,
}

#[derive(Debug)]
pub struct KnotArray {
    pub xs: Array1<f64>,
    pub q2s: Array1<f64>,
    pub flavors: Array1<i32>,
    pub grid: Array3<f64>,
}

impl KnotArray {
    pub fn new(xs: Vec<f64>, q2s: Vec<f64>, flavors: Vec<i32>, grid_data: Vec<f64>) -> Self {
        let nx = xs.len();
        let nq2 = q2s.len();
        let nflav = flavors.len();

        let xs = Array1::from_vec(xs);
        let q2s = Array1::from_vec(q2s);
        let flavors = Array1::from_vec(flavors);
        let grid = Array3::from_shape_vec((nx, nq2, nflav), grid_data)
            .expect("Failed to create grid from data")
            .permuted_axes([2, 0, 1]) // Permute (x, q2, flav) -> (flav, x, q2)
            .as_standard_layout()
            .to_owned();

        Self {
            xs,
            q2s,
            flavors,
            grid,
        }
    }

    pub fn xf(&self, ix: usize, iq2: usize, id: i32) -> f64 {
        let pid_index = self.flavors.iter().position(|&p| p == id).unwrap();
        self.grid[[pid_index, ix, iq2]]
    }
}

pub struct GridPDF {
    info: Info,
    pub knot_array: KnotArray,
    interpolators: Vec<Interp2DOwned<f64, Linear>>,
}

impl GridPDF {
    pub fn new(info: Info, knot_array: KnotArray) -> Self {
        let mut interpolators = Vec::new();
        for i in 0..knot_array.flavors.len() {
            let grid_slice = knot_array.grid.slice(s![i, .., ..]);

            let interp = interpolation::interpolate(
                knot_array.xs.to_owned(),
                knot_array.q2s.to_owned(),
                grid_slice.to_owned(),
            );
            interpolators.push(interp);
        }
        Self {
            info,
            knot_array,
            interpolators,
        }
    }

    pub fn xfxq2(&self, id: i32, x: f64, q2: f64) -> f64 {
        let pid_index = self
            .knot_array
            .flavors
            .iter()
            .position(|&p| p == id)
            .unwrap();
        self.interpolators[pid_index]
            .interpolate(&[x, q2])
            .unwrap_or(0.0)
    }
}

pub fn load(path: &Path) -> GridPDF {
    let info_path = path.join(format!(
        "{}.info",
        path.file_name().unwrap().to_str().unwrap()
    ));
    let info: Info = parser::read_info(&info_path).unwrap();

    // For now, only load the first member
    let data_path = path.join(format!(
        "{}_{:04}.dat",
        path.file_name().unwrap().to_str().unwrap(),
        0
    ));
    let (xs, q2s, flavors, grid_data) = parser::read_data(&data_path);
    let knot_array = KnotArray::new(xs, q2s, flavors, grid_data);

    GridPDF::new(info, knot_array)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knot_array_new() {
        let xs = vec![1.0, 2.0, 3.0];
        let q2s = vec![4.0, 5.0];
        let flavors = vec![21, 22];
        let grid_data = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0,
        ];
        let knot_array = KnotArray::new(xs, q2s, flavors, grid_data);
        assert_eq!(knot_array.grid.shape(), &[2, 3, 2]);
    }
}
