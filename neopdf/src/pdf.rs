use super::gridpdf::{GridArray, GridPDF, RangeParameters};
use super::metadata::MetaData;
use super::parser::LhapdfSet;
use ndarray::Array2;
use rayon::prelude::*;

/// Represents a Parton Distribution Function (PDF) set.
///
/// This struct provides a high-level interface for accessing PDF data,
/// including interpolation and metadata retrieval. It encapsulates the
/// `GridPDF` struct, which handles the low-level grid operations.
pub struct PDF {
    grid_pdf: GridPDF,
}

impl PDF {
    /// Loads a given member of the PDF set.
    ///
    /// This function reads the `.info` file and the corresponding `.dat` member file
    /// to construct a `GridPDF` object, which is then wrapped in a `PDF` instance.
    ///
    /// # Arguments
    ///
    /// * `pdf_name` - The name of the PDF set (e.g., "NNPDF40_nnlo_as_01180").
    /// * `member` - The ID of the PDF member to load (0-indexed).
    ///
    /// # Returns
    ///
    /// A `PDF` instance representing the loaded PDF member.
    pub fn load(pdf_name: &str, member: usize) -> Self {
        let lhapdf_set = LhapdfSet::new(pdf_name);
        let (info, pdf_data) = lhapdf_set.member(member);
        let knot_array = GridArray::new(pdf_data.subgrid_data, pdf_data.pids);

        Self {
            grid_pdf: GridPDF::new(info, knot_array),
        }
    }

    /// Loads all members of a PDF set.
    ///
    /// This function reads the `.info` file and all `.dat` member files
    /// to construct a `Vec<PDF>`, with each `PDF` instance representing a member
    /// of the set. The loading is performed in parallel.
    ///
    /// # Arguments
    ///
    /// * `pdf_name` - The name of the PDF set.
    ///
    /// # Returns
    ///
    /// A `Vec<PDF>` where each element is a `PDF` instance for a member of the set.
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

    /// Interpolates the PDF value (xf) for a given nucleon, alphas, flavor, x, and Q2.
    ///
    /// Abstraction to the `GridPDF::xfxq2` method.
    ///
    /// # Arguments
    ///
    /// * `id` - The flavor ID (PDG ID).
    /// * `points` - A slice containing the collection of points to interpolate on.
    ///
    /// # Returns
    ///
    /// The interpolated PDF value `xf(nuclone, alphas, flavor, x, Q^2)`.
    pub fn xfxq2(&self, pid: i32, points: &[f64]) -> f64 {
        self.grid_pdf.xfxq2(pid, points).unwrap()
    }

    /// Interpolates the PDF value (xf) for multiple nucleons, alphas, flavors, xs, and Q2s.
    ///
    /// Abstraction to the `GridPDF::xfxq2s` method.
    ///
    /// # Arguments
    ///
    /// * `ids` - A vector of flavor IDs.
    /// * `slice_points` - A slice containing the collection of knots to interpolate on.
    ///   A knot is a collection of points containing `(nucleon, alphas, x, Q2)`.
    ///
    /// # Returns
    ///
    /// A 2D array of interpolated PDF values with shape `[flavors, N_knots]`.
    pub fn xfxq2s(&self, pids: Vec<i32>, slice_points: &[&[f64]]) -> Array2<f64> {
        self.grid_pdf.xfxq2s(pids, slice_points)
    }

    /// Interpolates the strong coupling constant `alpha_s` for a given Q2.
    ///
    /// Abstraction to the `GridPDF::alphas_q2` method.
    ///
    /// # Arguments
    ///
    /// * `q2` - The squared energy scale.
    ///
    /// # Returns
    ///
    /// The interpolated `alpha_s` value.
    pub fn alphas_q2(&self, q2: f64) -> f64 {
        self.grid_pdf.alphas_q2(q2)
    }

    /// Returns the metadata for the PDF set.
    ///
    /// Abstraction to the `GridPDF::info` method.
    ///
    /// # Returns
    ///
    /// A `MetaData` struct containing information about the PDF set.
    pub fn info(&self) -> &MetaData {
        self.grid_pdf.metadata()
    }

    /// Retrieves the PDF value (xf) at a specific knot point in the grid.
    ///
    /// Abstraction to the `GridArray::xf_from_index` method. This method does not
    /// perform any interpolation.
    ///
    /// # Arguments
    ///
    /// * `i_nucleons` - The index of the nucleon.
    /// * `i_alphas` - The index of the alpha_s value.
    /// * `ix` - The index of the x-value.
    /// * `iq2` - The index of the Q2-value.
    /// * `id` - The flavor ID.
    /// * `subgrid_id` - The ID of the subgrid.
    ///
    /// # Returns
    ///
    /// The PDF value at the specified knot.
    pub fn xf_from_index(
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

    /// Retrieves the ranges for the parameters.
    ///
    /// Abstraction to the `GridPDF::param_ranges` method.
    ///
    /// # Returns
    ///
    /// The minimum `x` value.
    pub fn param_ranges(&self) -> RangeParameters {
        self.grid_pdf.param_ranges()
    }
}
