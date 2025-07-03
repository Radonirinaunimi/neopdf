use ndarray::{Array1, Array3};
use serde::Deserialize;
use std::path::Path;

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

        dbg!(nx, nq2, nflav, grid_data.len());

        let xs = Array1::from_vec(xs);
        let q2s = Array1::from_vec(q2s);
        let flavors = Array1::from_vec(flavors);
        let grid = Array3::from_shape_vec((nx, nflav, nq2), grid_data)
            .expect("Failed to create grid from data")
            .permuted_axes([1, 0, 2]) // Permute (x, flav, q2) -> (flav, x, q2)
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

    pub fn ixbelow(&self, x: f64) -> usize {
        let idx = self
            .xs
            .iter()
            .position(|&v| v >= x)
            .unwrap_or(self.xs.len() - 1);
        if idx == 0 {
            0
        } else if self.xs[idx] == x {
            if idx == self.xs.len() - 1 {
                idx - 1
            } else {
                idx
            }
        } else {
            idx - 1
        }
    }

    pub fn iq2below(&self, q2: f64) -> usize {
        let idx = self
            .q2s
            .iter()
            .position(|&v| v >= q2)
            .unwrap_or(self.q2s.len() - 1);
        if idx == 0 {
            0
        } else if self.q2s[idx] == q2 {
            if idx == self.q2s.len() - 1 {
                idx - 1
            } else {
                idx
            }
        } else {
            idx - 1
        }
    }
}

pub trait Interpolator {
    fn interpolate_xq2<T>(
        &self,
        grid: &KnotArray,
        x: f64,
        ix: usize,
        q2: f64,
        iq2: usize,
        id: i32,
    ) -> T;
}

pub struct GridPDF {
    info: Info,
    pub knot_array: KnotArray,
}

impl GridPDF {
    pub fn new(info: Info, knot_array: KnotArray) -> Self {
        Self { info, knot_array }
    }

    pub fn xfxq2(&self, id: i32, x: f64, q2: f64) -> f64 {
        let ix = self.knot_array.ixbelow(x);
        let iq2 = self.knot_array.iq2below(q2);
        // This is a placeholder for the actual interpolation
        self.knot_array.xf(ix, iq2, id)
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
    use ndarray::array;

    #[test]
    fn test_ixbelow() {
        let knot_array = KnotArray::new(vec![1.0, 2.0, 3.0, 4.0, 5.0], vec![], vec![], vec![]);

        // Test values within the range
        assert_eq!(knot_array.ixbelow(1.0), 0);
        assert_eq!(knot_array.ixbelow(1.5), 0);
        assert_eq!(knot_array.ixbelow(2.0), 1);
        assert_eq!(knot_array.ixbelow(2.5), 1);
        assert_eq!(knot_array.ixbelow(3.0), 2);
        assert_eq!(knot_array.ixbelow(3.5), 2);

        // Test values at the boundaries
        assert_eq!(knot_array.ixbelow(0.5), 0); // Below min
        assert_eq!(knot_array.ixbelow(4.0), 3); // At max
        assert_eq!(knot_array.ixbelow(4.5), 3); // Above max
        assert_eq!(knot_array.ixbelow(5.0), 3); // At last knot
        assert_eq!(knot_array.ixbelow(5.5), 3); // Above last knot
    }

    #[test]
    fn test_iq2below() {
        let knot_array = KnotArray::new(vec![], vec![10.0, 20.0, 30.0, 40.0, 50.0], vec![], vec![]);

        // Test values within the range
        assert_eq!(knot_array.iq2below(10.0), 0);
        assert_eq!(knot_array.iq2below(15.0), 0);
        assert_eq!(knot_array.iq2below(20.0), 1);
        assert_eq!(knot_array.iq2below(25.0), 1);
        assert_eq!(knot_array.iq2below(30.0), 2);
        assert_eq!(knot_array.iq2below(35.0), 2);

        // Test values at the boundaries
        assert_eq!(knot_array.iq2below(5.0), 0); // Below min
        assert_eq!(knot_array.iq2below(40.0), 3); // At max
        assert_eq!(knot_array.iq2below(45.0), 3); // Above max
        assert_eq!(knot_array.iq2below(50.0), 3); // At last knot
        assert_eq!(knot_array.iq2below(55.0), 3); // Above last knot
    }
}
