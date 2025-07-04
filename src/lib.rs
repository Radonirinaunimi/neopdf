use gridpdf::{GridPDF, Info, KnotArray};
use ndarray::Array3;
use rayon::prelude::*;
use std::path::Path;

pub mod gridpdf;
pub mod interpolation;
pub mod parser;
pub mod utils;

pub struct PDF {
    grid_pdf: GridPDF,
}

impl PDF {
    /// Loads a PDF set from the specified path.
    ///
    /// This function reads the `.info` file and the first `.dat` member file
    /// to construct a `GridPDF` object.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the directory containing the PDF set files.
    ///
    /// # Returns
    ///
    /// A `PDF` instance representing the loaded PDF set.
    pub fn load(path: &Path) -> PDF {
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
        let pdf_data = parser::read_data(&data_path);
        let knot_array = KnotArray::new(pdf_data.subgrid_data, pdf_data.flavors);

        PDF {
            grid_pdf: GridPDF::new(info, knot_array),
        }
    }

    /// Loads all PDF sets from the specified path in parallel.
    ///
    /// This function reads the `.info` file and all `.dat` member files
    /// to construct a `Vec<PDF>` object.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the directory containing the PDF set files.
    ///
    /// # Returns
    ///
    /// A `Vec<PDF>` instance representing all loaded PDF sets.
    pub fn load_pdfs(path: &Path) -> Vec<PDF> {
        let info_path = path.join(format!(
            "{}.info",
            path.file_name().unwrap().to_str().unwrap()
        ));
        let info: Info = parser::read_info(&info_path).unwrap();

        (0..info.num_members)
            .into_par_iter()
            .map(|i| {
                let data_path = path.join(format!(
                    "{}_{:04}.dat",
                    path.file_name().unwrap().to_str().unwrap(),
                    i
                ));
                let pdf_data = parser::read_data(&data_path);
                let knot_array = KnotArray::new(pdf_data.subgrid_data, pdf_data.flavors);
                PDF {
                    grid_pdf: GridPDF::new(info.clone(), knot_array),
                }
            })
            .collect()
    }

    /// Interpolates the PDF value (xf) for a given flavor, x, and Q2.
    ///
    /// # Arguments
    ///
    /// * `id` - The flavor ID.
    /// * `x` - The x-value (momentum fraction).
    /// * `q2` - The Q2-value (energy scale squared).
    ///
    /// # Returns
    ///
    /// The interpolated PDF value. Returns 0.0 if extrapolation is attempted and not allowed.
    pub fn xfxq2(&self, id: i32, x: f64, q2: f64) -> f64 {
        self.grid_pdf.xfxq2(id, x, q2)
    }

    /// Interpolates the PDF value (xf) for some lists of flavors, xs, and Q2s.
    pub fn xfxq2s(&self, ids: Vec<i32>, xs: Vec<f64>, q2s: Vec<f64>) -> Array3<f64> {
        self.grid_pdf.xfxq2s(ids, xs, q2s)
    }

    /// Interpolates the alpha_s value for a given Q2.
    ///
    /// # Arguments
    ///
    /// * `q2` - The Q2-value (energy scale squared).
    ///
    /// # Returns
    ///
    /// The interpolated alpha_s value.
    pub fn alphas_q2(&self, q2: f64) -> f64 {
        self.grid_pdf.alphas_q2(q2)
    }

    /// Returns the metadata info of the PDF.
    pub fn info(&self) -> Info {
        self.grid_pdf.info()
    }

    /// Retrieves the PDF value (xf) at a specific knot point.
    pub fn xf(&self, ix: usize, iq2: usize, id: i32, subgrid_id: usize) -> f64 {
        self.grid_pdf.knot_array.xf(ix, iq2, id, subgrid_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knot_array_new() {
        let subgrid_data = vec![parser::SubgridData {
            xs: vec![1.0, 2.0, 3.0],
            q2s: vec![4.0, 5.0],
            grid_data: vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0,
            ],
        }];
        let flavors = vec![21, 22];
        let knot_array = KnotArray::new(subgrid_data, flavors);
        assert_eq!(knot_array.subgrids[0].grid.shape(), &[2, 3, 2]);
    }
}
