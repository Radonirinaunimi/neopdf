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
    #[serde(rename = "Interpolator", default = "default_interpolator_type")]
    pub interpolator_type: String,
}

fn default_interpolator_type() -> String {
    "Bilinear".to_string()
}

#[derive(Debug)]
pub struct KnotArray {
    pub xs: Vec<f64>,
    pub q2s: Vec<f64>,
    pub logxs: Vec<f64>,
    pub logq2s: Vec<f64>,
    pub grid: Vec<f64>,
    pub shape: Vec<usize>,
    pub flavors: Vec<i32>,
}

impl KnotArray {
    pub fn new() -> Self {
        Self {
            xs: Vec::new(),
            q2s: Vec::new(),
            logxs: Vec::new(),
            logq2s: Vec::new(),
            grid: Vec::new(),
            shape: Vec::new(),
            flavors: Vec::new(),
        }
    }

    pub fn xf(&self, ix: usize, iq2: usize, id: i32) -> f64 {
        let pid_index = self.flavors.iter().position(|&p| p == id).unwrap();
        let x_stride = self.shape[1] * self.shape[2];
        let q2_stride = self.shape[2];
        self.grid[ix * x_stride + iq2 * q2_stride + pid_index]
    }

    pub fn ixbelow(&self, x: f64) -> usize {
        let idx = self.xs.binary_search_by(|val| val.partial_cmp(&x).unwrap()).unwrap_or_else(|e| e);
        if idx == 0 {
            0
        } else if idx == self.xs.len() {
            self.xs.len() - 2
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
        let idx = self.q2s.binary_search_by(|val| val.partial_cmp(&q2).unwrap()).unwrap_or_else(|e| e);
        if idx == 0 {
            0
        } else if idx == self.q2s.len() {
            self.q2s.len() - 2
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

    pub fn ilogxbelow(&self, logx: f64) -> usize {
        let idx = self.logxs.binary_search_by(|val| val.partial_cmp(&logx).unwrap()).unwrap_or_else(|e| e);
        if idx == 0 {
            0
        } else if idx == self.logxs.len() {
            self.logxs.len() - 2
        } else if self.logxs[idx] == logx {
            if idx == self.logxs.len() - 1 {
                idx - 1
            } else {
                idx
            }
        } else {
            idx - 1
        }
    }

    pub fn ilogq2below(&self, logq2: f64) -> usize {
        let idx = self.logq2s.binary_search_by(|val| val.partial_cmp(&logq2).unwrap()).unwrap_or_else(|e| e);
        if idx == 0 {
            0
        } else if idx == self.logq2s.len() {
            self.logq2s.len() - 2
        } else if self.logq2s[idx] == logq2 {
            if idx == self.logq2s.len() - 1 {
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
    fn interpolate_xq2(
        &self,
        grid: &KnotArray,
        x: f64,
        ix: usize,
        q2: f64,
        iq2: usize,
        id: i32,
    ) -> f64;
}

pub struct GridPDF {
    info: Info,
    pub knot_array: KnotArray,
    interpolator: Box<dyn Interpolator>,
}

impl GridPDF {
    pub fn new(info: Info, knot_array: KnotArray, interpolator: Box<dyn Interpolator>) -> Self {
        Self {
            info,
            knot_array,
            interpolator,
        }
    }

    pub fn xfxq2(&self, id: i32, x: f64, q2: f64) -> f64 {
        let ix = self.knot_array.ixbelow(x);
        let iq2 = self.knot_array.iq2below(q2);
        self.interpolator
            .interpolate_xq2(&self.knot_array, x, ix, q2, iq2, id)
    }
}

pub fn load(path: &Path) -> GridPDF {
    let info_path = path.join(format!(
        "{}.info",
        path.file_name().unwrap().to_str().unwrap()
    ));
    let info = parser::read_info(&info_path).unwrap();

    // For now, only load the first member
    let data_path = path.join(format!(
        "{}_{:04}.dat",
        path.file_name().unwrap().to_str().unwrap(),
        0
    ));
    let mut knot_array = KnotArray::new();
    parser::read_data(&data_path, &mut knot_array);

    println!("Rust flavors: {:?}", knot_array.flavors);
    println!("Rust grid (first 20 elements): {:?}", &knot_array.grid[0..20]);

    let interpolator: Box<dyn Interpolator> = match info.interpolator_type.as_str() {
        "Bilinear" => Box::new(BilinearInterpolator {}),
        "LogBilinear" => Box::new(LogBilinearInterpolator {}),
        _ => panic!("Unsupported interpolator type: {}", info.interpolator_type),
    };
    GridPDF::new(info, knot_array, interpolator)
}

pub struct BilinearInterpolator;

impl Interpolator for BilinearInterpolator {
    fn interpolate_xq2(
        &self,
        grid: &KnotArray,
        x: f64,
        ix: usize,
        q2: f64,
        iq2: usize,
        id: i32,
    ) -> f64 {
        let x1 = grid.xs[ix];
        let x2 = grid.xs[ix + 1];
        let q21 = grid.q2s[iq2];
        let q22 = grid.q2s[iq2 + 1];

        let f11 = grid.xf(ix, iq2, id);
        let f12 = grid.xf(ix, iq2 + 1, id);
        let f21 = grid.xf(ix + 1, iq2, id);
        let f22 = grid.xf(ix + 1, iq2 + 1, id);

        let t = (x - x1) / (x2 - x1);
        let u = (q2 - q21) / (q22 - q21);

        (1.0 - t) * (1.0 - u) * f11 + t * (1.0 - u) * f21 + (1.0 - t) * u * f12 + t * u * f22
    }
}

pub struct LogBilinearInterpolator;

impl Interpolator for LogBilinearInterpolator {
    fn interpolate_xq2(
        &self,
        grid: &KnotArray,
        x: f64,
        _ix: usize,
        q2: f64,
        _iq2: usize,
        id: i32,
    ) -> f64 {
        let logx = x.ln();
        let logq2 = q2.ln();

        let ix = grid.ilogxbelow(logx);
        let iq2 = grid.ilogq2below(logq2);

        let logx1 = grid.logxs[ix];
        let logx2 = grid.logxs[ix + 1];
        let logq21 = grid.logq2s[iq2];
        let logq22 = grid.logq2s[iq2 + 1];

        let f11 = grid.xf(ix, iq2, id);
        let f12 = grid.xf(ix, iq2 + 1, id);
        let f21 = grid.xf(ix + 1, iq2, id);
        let f22 = grid.xf(ix + 1, iq2 + 1, id);

        let t = (logx - logx1) / (logx2 - logx1);
        let u = (logq2 - logq21) / (logq22 - logq21);

        let result_log = (1.0 - t) * (1.0 - u) * (f11 + 1e-10).ln() + t * (1.0 - u) * (f21 + 1e-10).ln() + (1.0 - t) * u * (f12 + 1e-10).ln() + t * u * (f22 + 1e-10).ln();
        result_log.exp()
    }
}

#[cfg(test)]
mod tests {
    use super::KnotArray;

    #[test]
    fn test_ixbelow() {
        let mut knot_array = KnotArray::new();
        knot_array.xs = vec![1.0, 2.0, 3.0, 4.0, 5.0];

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
        let mut knot_array = KnotArray::new();
        knot_array.q2s = vec![10.0, 20.0, 30.0, 40.0, 50.0];

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
