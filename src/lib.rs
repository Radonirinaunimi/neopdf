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
    "LogBicubic".to_string()
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
        let idx = self
            .xs
            .binary_search_by(|val| val.partial_cmp(&x).unwrap())
            .unwrap_or_else(|e| e);
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
        let idx = self
            .q2s
            .binary_search_by(|val| val.partial_cmp(&q2).unwrap())
            .unwrap_or_else(|e| e);
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
        let idx = self
            .logxs
            .binary_search_by(|val| val.partial_cmp(&logx).unwrap())
            .unwrap_or_else(|e| e);
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
        let idx = self
            .logq2s
            .binary_search_by(|val| val.partial_cmp(&logq2).unwrap())
            .unwrap_or_else(|e| e);
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
    let info: Info = parser::read_info(&info_path).unwrap();
    dbg!(&info);

    // For now, only load the first member
    let data_path = path.join(format!(
        "{}_{:04}.dat",
        path.file_name().unwrap().to_str().unwrap(),
        0
    ));
    let mut knot_array = KnotArray::new();
    parser::read_data(&data_path, &mut knot_array);

    let interpolator: Box<dyn Interpolator> = match info.interpolator_type.as_str() {
        "Bilinear" => Box::new(BilinearInterpolator {}),
        "LogBilinear" => Box::new(LogBilinearInterpolator {}),
        "Bicubic" => Box::new(BicubicInterpolator {}),
        "LogBicubic" => Box::new(LogBicubicInterpolator {}),
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

        let result_log = (1.0 - t) * (1.0 - u) * (f11 + 1e-10).ln()
            + t * (1.0 - u) * (f21 + 1e-10).ln()
            + (1.0 - t) * u * (f12 + 1e-10).ln()
            + t * u * (f22 + 1e-10).ln();
        result_log.exp()
    }
}

// One-dimensional cubic interpolation
fn _interpolate_cubic(t: f64, vl: f64, vdl: f64, vh: f64, vdh: f64) -> f64 {
    let t2 = t * t;
    let t3 = t2 * t;

    let p0 = (2.0 * t3 - 3.0 * t2 + 1.0) * vl;
    let m0 = (t3 - 2.0 * t2 + t) * vdl;
    let p1 = (-2.0 * t3 + 3.0 * t2) * vh;
    let m1 = (t3 - t2) * vdh;

    p0 + m0 + p1 + m1
}

// Helper function to calculate derivative with respect to x
fn _ddx(grid: &KnotArray, ix: usize, iq2: usize, id: i32, logspace: bool) -> f64 {
    let nxknots = grid.xs.len();
    let del1: f64;
    let del2: f64;

    if logspace {
        del1 = if ix == 0 {
            0.0
        } else {
            grid.logxs[ix] - grid.logxs[ix - 1]
        };
        del2 = if ix == nxknots - 1 {
            0.0
        } else {
            grid.logxs[ix + 1] - grid.logxs[ix]
        };
    } else {
        del1 = if ix == 0 {
            0.0
        } else {
            grid.xs[ix] - grid.xs[ix - 1]
        };
        del2 = if ix == nxknots - 1 {
            0.0
        } else {
            grid.xs[ix + 1] - grid.xs[ix]
        };
    }

    let xf_val = |x_idx, q2_idx, flavor_id| {
        if logspace {
            (grid.xf(x_idx, q2_idx, flavor_id) + 1e-10).ln()
        } else {
            grid.xf(x_idx, q2_idx, flavor_id)
        }
    };

    if ix != 0 && ix != nxknots - 1 {
        // Central difference
        let lddx = (xf_val(ix, iq2, id) - xf_val(ix - 1, iq2, id)) / del1;
        let rddx = (xf_val(ix + 1, iq2, id) - xf_val(ix, iq2, id)) / del2;
        (lddx + rddx) / 2.0
    } else if ix == 0 {
        // Forward difference
        (xf_val(ix + 1, iq2, id) - xf_val(ix, iq2, id)) / del2
    } else if ix == nxknots - 1 {
        // Backward difference
        (xf_val(ix, iq2, id) - xf_val(ix - 1, iq2, id)) / del1
    } else {
        // Should not happen
        0.0
    }
}

// Helper function to calculate derivative with respect to Q2
fn _ddq2(grid: &KnotArray, ix: usize, iq2: usize, id: i32, logspace: bool) -> f64 {
    let nq2knots = grid.q2s.len();
    let del1: f64;
    let del2: f64;

    if logspace {
        del1 = if iq2 == 0 {
            0.0
        } else {
            grid.logq2s[iq2] - grid.logq2s[iq2 - 1]
        };
        del2 = if iq2 == nq2knots - 1 {
            0.0
        } else {
            grid.logq2s[iq2 + 1] - grid.logq2s[iq2]
        };
    } else {
        del1 = if iq2 == 0 {
            0.0
        } else {
            grid.q2s[iq2] - grid.q2s[iq2 - 1]
        };
        del2 = if iq2 == nq2knots - 1 {
            0.0
        } else {
            grid.q2s[iq2 + 1] - grid.q2s[iq2]
        };
    }

    let xf_val = |x_idx, q2_idx, flavor_id| {
        if logspace {
            (grid.xf(x_idx, q2_idx, flavor_id) + 1e-10).ln()
        } else {
            grid.xf(x_idx, q2_idx, flavor_id)
        }
    };

    if iq2 != 0 && iq2 != nq2knots - 1 {
        // Central difference
        let lddq2 = (xf_val(ix, iq2, id) - xf_val(ix, iq2 - 1, id)) / del1;
        let rddq2 = (xf_val(ix, iq2 + 1, id) - xf_val(ix, iq2, id)) / del2;
        (lddq2 + rddq2) / 2.0
    } else if iq2 == 0 {
        // Forward difference
        (xf_val(ix, iq2 + 1, id) - xf_val(ix, iq2, id)) / del2
    } else if iq2 == nq2knots - 1 {
        // Backward difference
        (xf_val(ix, iq2, id) - xf_val(ix, iq2 - 1, id)) / del1
    } else {
        // Should not happen
        0.0
    }
}

// Helper function to check grid size for bicubic interpolation
fn _check_grid_size(grid: &KnotArray) {
    if grid.xs.len() < 4 {
        panic!(
            "PDF subgrids are required to have at least 4 x-knots for use with BicubicInterpolator"
        );
    }
    if grid.q2s.len() < 4 {
        panic!("PDF subgrids are required to have at least 4 Q2-knots for use with BicubicInterpolator");
    }
}

pub struct BicubicInterpolator;

impl Interpolator for BicubicInterpolator {
    fn interpolate_xq2(
        &self,
        grid: &KnotArray,
        x: f64,
        ix: usize,
        q2: f64,
        iq2: usize,
        id: i32,
    ) -> f64 {
        _check_grid_size(grid);

        let x1 = grid.xs[ix];
        let x2 = grid.xs[ix + 1];
        let q21 = grid.q2s[iq2];
        let q22 = grid.q2s[iq2 + 1];

        let t = (x - x1) / (x2 - x1);
        let u = (q2 - q21) / (q22 - q21);

        // Function values at the corners
        let f11 = grid.xf(ix, iq2, id);
        let f12 = grid.xf(ix, iq2 + 1, id);
        let f21 = grid.xf(ix + 1, iq2, id);
        let f22 = grid.xf(ix + 1, iq2 + 1, id);

        // Derivatives with respect to x
        let df11_dx = _ddx(grid, ix, iq2, id, false);
        let df12_dx = _ddx(grid, ix, iq2 + 1, id, false);
        let df21_dx = _ddx(grid, ix + 1, iq2, id, false);
        let df22_dx = _ddx(grid, ix + 1, iq2 + 1, id, false);

        // Derivatives with respect to Q2
        let df11_dq2 = _ddq2(grid, ix, iq2, id, false);
        let df12_dq2 = _ddq2(grid, ix, iq2 + 1, id, false);
        let df21_dq2 = _ddq2(grid, ix + 1, iq2, id, false);
        let df22_dq2 = _ddq2(grid, ix + 1, iq2 + 1, id, false);

        // Interpolate along x-direction to get values and derivatives at (x, q21) and (x, q22)
        let val_at_q21 = _interpolate_cubic(t, f11, df11_dx * (x2 - x1), f21, df21_dx * (x2 - x1));
        let val_at_q22 = _interpolate_cubic(t, f12, df12_dx * (x2 - x1), f22, df22_dx * (x2 - x1));

        let dval_at_q21_dq2 = _interpolate_cubic(t, df11_dq2, 0.0, df21_dq2, 0.0); // Simplified cross-derivative
        let dval_at_q22_dq2 = _interpolate_cubic(t, df12_dq2, 0.0, df22_dq2, 0.0); // Simplified cross-derivative

        // Interpolate along Q2-direction
        _interpolate_cubic(
            u,
            val_at_q21,
            dval_at_q21_dq2 * (q22 - q21),
            val_at_q22,
            dval_at_q22_dq2 * (q22 - q21),
        )
    }
}

pub struct LogBicubicInterpolator;

impl Interpolator for LogBicubicInterpolator {
    fn interpolate_xq2(
        &self,
        grid: &KnotArray,
        x: f64,
        _ix: usize,
        q2: f64,
        _iq2: usize,
        id: i32,
    ) -> f64 {
        _check_grid_size(grid);

        let logx = x.ln();
        let logq2 = q2.ln();

        let ix = grid.ilogxbelow(logx);
        let iq2 = grid.ilogq2below(logq2);

        let logx1 = grid.logxs[ix];
        let logx2 = grid.logxs[ix + 1];
        let logq21 = grid.logq2s[iq2];
        let logq22 = grid.logq2s[iq2 + 1];

        // Function values at the corners (log-interpolated)
        let f11 = (grid.xf(ix, iq2, id) + 1e-10).ln();
        let f12 = (grid.xf(ix, iq2 + 1, id) + 1e-10).ln();
        let f21 = (grid.xf(ix + 1, iq2, id) + 1e-10).ln();
        let f22 = (grid.xf(ix + 1, iq2 + 1, id) + 1e-10).ln();

        // Derivatives with respect to log(x)
        let df11_dx = _ddx(grid, ix, iq2, id, true);
        let df12_dx = _ddx(grid, ix, iq2 + 1, id, true);
        let df21_dx = _ddx(grid, ix + 1, iq2, id, true);
        let df22_dx = _ddx(grid, ix + 1, iq2 + 1, id, true);

        // Derivatives with respect to log(Q2)
        let df11_dq2 = _ddq2(grid, ix, iq2, id, true);
        let df12_dq2 = _ddq2(grid, ix, iq2 + 1, id, true);
        let df21_dq2 = _ddq2(grid, ix + 1, iq2, id, true);
        let df22_dq2 = _ddq2(grid, ix + 1, iq2 + 1, id, true);

        // Interpolate along log(x)-direction to get values and derivatives at (logx, logq21) and (logx, logq22)
        let val_at_logq21 = _interpolate_cubic(
            (logx - logx1) / (logx2 - logx1),
            f11,
            df11_dx * (logx2 - logx1),
            f21,
            df21_dx * (logx2 - logx1),
        );
        let val_at_logq22 = _interpolate_cubic(
            (logx - logx1) / (logx2 - logx1),
            f12,
            df12_dx * (logx2 - logx1),
            f22,
            df22_dx * (logx2 - logx1),
        );

        let dval_at_logq21_dq2 = _interpolate_cubic(
            (logx - logx1) / (logx2 - logx1),
            df11_dq2,
            0.0,
            df21_dq2,
            0.0,
        ); // Simplified cross-derivative
        let dval_at_logq22_dq2 = _interpolate_cubic(
            (logx - logx1) / (logx2 - logx1),
            df12_dq2,
            0.0,
            df22_dq2,
            0.0,
        ); // Simplified cross-derivative

        let t_interp = (logx - logx1) / (logx2 - logx1);
        let u_interp = (logq2 - logq21) / (logq22 - logq21);

        // Interpolate along log(Q2)-direction
        _interpolate_cubic(
            u_interp,
            val_at_logq21,
            dval_at_logq21_dq2 * (logq22 - logq21),
            val_at_logq22,
            dval_at_logq22_dq2 * (logq22 - logq21),
        )
        .exp()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_bilinear_interpolation() {
        let mut knot_array = KnotArray::new();
        knot_array.xs = vec![1.0, 2.0];
        knot_array.q2s = vec![10.0, 20.0];
        knot_array.flavors = vec![21];
        knot_array.grid = vec![100.0, 200.0, 300.0, 400.0]; // f(1,10)=100, f(1,20)=200, f(2,10)=300, f(2,20)=400
        knot_array.shape = vec![2, 2, 1];

        let interpolator = BilinearInterpolator {};

        // Test at a point within the grid
        let x = 1.5;
        let q2 = 15.0;
        let expected = 250.0; // (1-0.5)*(1-0.5)*100 + 0.5*(1-0.5)*300 + (1-0.5)*0.5*200 + 0.5*0.5*400 = 25+75+50+100 = 250
        let result = interpolator.interpolate_xq2(&knot_array, x, 0, q2, 0, 21);
        assert!((result - expected).abs() < 1e-9);
    }

    #[test]
    fn test_log_bilinear_interpolation() {
        let mut knot_array = KnotArray::new();
        knot_array.xs = vec![1.0, 10.0];
        knot_array.logxs = vec![1.0f64.ln(), 10.0f64.ln()];
        knot_array.q2s = vec![10.0, 100.0];
        knot_array.logq2s = vec![10.0f64.ln(), 100.0f64.ln()];
        knot_array.flavors = vec![21];
        knot_array.grid = vec![100.0, 200.0, 300.0, 400.0]; // f(1,10)=100, f(1,100)=200, f(10,10)=300, f(10,100)=400
        knot_array.shape = vec![2, 2, 1];

        let interpolator = LogBilinearInterpolator {};

        // Test at a point within the grid
        let x: f64 = 3.16227766; // sqrt(10)
        let q2: f64 = 31.6227766; // sqrt(1000)

        let logx = x.ln();
        let logq2 = q2.ln();

        let logx1 = knot_array.logxs[0];
        let logx2 = knot_array.logxs[1];
        let logq21 = knot_array.logq2s[0];
        let logq22 = knot_array.logq2s[1];

        let t = (logx - logx1) / (logx2 - logx1);
        let u = (logq2 - logq21) / (logq22 - logq21);

        let f11_log = (100.0f64 + 1e-10f64).ln();
        let f12_log = (200.0f64 + 1e-10f64).ln();
        let f21_log = (300.0f64 + 1e-10f64).ln();
        let f22_log = (400.0f64 + 1e-10f64).ln();

        let expected_log = (1.0f64 - t) * (1.0f64 - u) * f11_log
            + t * (1.0f64 - u) * f21_log
            + (1.0f64 - t) * u * f12_log
            + t * u * f22_log;
        let expected = expected_log.exp();

        let result = interpolator.interpolate_xq2(&knot_array, x, 0, q2, 0, 21);
        assert!((result - expected).abs() < 1e-9);
    }

    #[test]
    fn test_bicubic_interpolation() {
        let mut knot_array = KnotArray::new();
        knot_array.xs = vec![1.0, 2.0, 3.0, 4.0];
        knot_array.q2s = vec![10.0, 20.0, 30.0, 40.0];
        knot_array.flavors = vec![21];
        // Simplified grid for testing: f(x,q2) = x + q2
        knot_array.grid = vec![
            1.0 + 10.0,
            1.0 + 20.0,
            1.0 + 30.0,
            1.0 + 40.0,
            2.0 + 10.0,
            2.0 + 20.0,
            2.0 + 30.0,
            2.0 + 40.0,
            3.0 + 10.0,
            3.0 + 20.0,
            3.0 + 30.0,
            3.0 + 40.0,
            4.0 + 10.0,
            4.0 + 20.0,
            4.0 + 30.0,
            4.0 + 40.0,
        ];
        knot_array.shape = vec![4, 4, 1];

        let interpolator = BicubicInterpolator {};

        // Test at a point within the grid
        let x = 2.5;
        let q2 = 25.0;
        let expected = x + q2; // For f(x,q2) = x + q2, bicubic should be exact
        let result = interpolator.interpolate_xq2(&knot_array, x, 1, q2, 1, 21);
        assert!((result - expected).abs() < 1e-9);
    }

    #[test]
    fn test_log_bicubic_interpolation() {
        let mut knot_array = KnotArray::new();
        knot_array.xs = vec![1.0, 10.0, 100.0, 1000.0];
        knot_array.logxs = knot_array.xs.iter().map(|&x| x.ln()).collect();
        knot_array.q2s = vec![10.0, 100.0, 1000.0, 10000.0];
        knot_array.logq2s = knot_array.q2s.iter().map(|&q2| q2.ln()).collect();
        knot_array.flavors = vec![21];
        // Simplified grid for testing: f(x,q2) = x * q2
        knot_array.grid = vec![
            1.0 * 10.0,
            1.0 * 100.0,
            1.0 * 1000.0,
            1.0 * 10000.0,
            10.0 * 10.0,
            10.0 * 100.0,
            10.0 * 1000.0,
            10.0 * 10000.0,
            100.0 * 10.0,
            100.0 * 100.0,
            100.0 * 1000.0,
            100.0 * 10000.0,
            1000.0 * 10.0,
            1000.0 * 100.0,
            1000.0 * 1000.0,
            1000.0 * 10000.0,
        ];
        knot_array.shape = vec![4, 4, 1];

        let interpolator = LogBicubicInterpolator {};

        // Test at a point within the grid
        let x = 31.6227766; // sqrt(1000)
        let q2 = 316.227766; // sqrt(100000)
        let expected = x * q2; // For f(x,q2) = x * q2, log-bicubic should be exact
        let result = interpolator.interpolate_xq2(&knot_array, x, 1, q2, 1, 21);
        assert!((result - expected).abs() < 1e-9);
    }
}
