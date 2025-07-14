use crate::gridpdf::GridArray;
use crate::parser::LhapdfSet;
use crate::writer::GridArrayCollection;

/// Converts an LHAPDF set to the NeoPDF format and writes it to disk.
///
/// # Arguments
///
/// * `pdf_name` - The name of the LHAPDF set (e.g., "NNPDF40_nnlo_as_01180").
/// * `output_path` - The path to the output NeoPDF file.
///
/// # Errors
///
/// Returns an error if reading or writing fails.
pub fn convert_lhapdf_to_neopdf<P: AsRef<std::path::Path>>(
    pdf_name: &str,
    output_path: P,
) -> Result<(), Box<dyn std::error::Error>> {
    let lhapdf_set = LhapdfSet::new(pdf_name);
    let members = lhapdf_set.members();
    if members.is_empty() {
        return Err("No members found in the LHAPDF set".into());
    }

    // All members share the same metadata
    let metadata = &members[0].0.clone();
    let grids: Vec<GridArray> = members
        .into_iter()
        .map(|(_meta, pdf_data)| GridArray::new(pdf_data.subgrid_data, pdf_data.pids))
        .collect();

    GridArrayCollection::compress(&grids, metadata, output_path)?;
    Ok(())
}
