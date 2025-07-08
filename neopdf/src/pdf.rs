use super::gridpdf::{GridArray, GridPDF};
use super::metadata::MetaData;
use super::parser::LhapdfSet;
use ndarray::Array3;
use rayon::prelude::*;

pub struct PDF {
    grid_pdf: GridPDF,
}

impl PDF {
    /// Loads a given member of the PDF set.
    ///
    /// This function reads the `.info` file and the first `.dat` member file
    /// to construct a `GridPDF` object.
    ///
    /// # Arguments
    ///
    /// * `pdf_name` - The name of the PDF set.
    /// * `member` - ID of the PDF member.
    ///
    /// # Returns
    ///
    /// A `PDF` instance representing the loaded PDF set.
    pub fn load(pdf_name: &str, member: usize) -> Self {
        let lhapdf_set = LhapdfSet::new(pdf_name);
        let (info, pdf_data) = lhapdf_set.member(member);
        let knot_array = GridArray::new(pdf_data.subgrid_data, pdf_data.pids);

        Self {
            grid_pdf: GridPDF::new(info, knot_array),
        }
    }

    /// Loads all the members of the PDF set.
    ///
    /// This function reads the `.info` file and all `.dat` member files
    /// to construct a `Vec<PDF>` object.
    ///
    /// # Arguments
    ///
    /// * `pdf_name` - The name of the PDF set.
    ///
    /// # Returns
    ///
    /// A `Vec<PDF>` instance representing all loaded PDF sets.
    pub fn load_pdfs(pdf_name: &str) -> Vec<PDF> {
        let lhapdf_set = LhapdfSet::new(pdf_name);
        lhapdf_set
            .members()
            .into_par_iter()
            .map(|(info, pdf_data)| {
                let knot_array = GridArray::new(pdf_data.subgrid_data, pdf_data.pids);
                PDF {
                    grid_pdf: GridPDF::new(info, knot_array),
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
    pub fn info(&self) -> MetaData {
        self.grid_pdf.info()
    }

    /// Retrieves the PDF value (xf) at a specific knot point.
    pub fn xf(
        &self,
        i_nucleons: usize,
        i_alphas: usize,
        ix: usize,
        iq2: usize,
        id: i32,
        subgrid_id: usize,
    ) -> f64 {
        self.grid_pdf
            .knot_array
            .xf_from_index(i_nucleons, i_alphas, ix, iq2, id, subgrid_id)
    }

    /// Retrieves the `x_min` for this PDF set.
    pub fn x_min(&self) -> f64 {
        self.grid_pdf.x_min()
    }

    /// Retrieves the `x_max` for this PDF set.
    pub fn x_max(&self) -> f64 {
        self.grid_pdf.x_max()
    }

    /// Retrieves the `q2_min` for this PDF set.
    pub fn q2_min(&self) -> f64 {
        self.grid_pdf.q2_min()
    }

    /// Retrieves the `q2_max` for this PDF set.
    pub fn q2_max(&self) -> f64 {
        self.grid_pdf.q2_max()
    }
}
