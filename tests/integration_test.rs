use lhapdf_rust::*;
use std::path::Path;

#[test]
fn test_xf_at_knot() {
    let pdf_set_path = Path::new("../NNPDF40_nnlo_as_01180");
    let pdf = load(pdf_set_path);

    let expected_value = 0.14844111; // Value for gluon (id 21) at x_0, q2_0
    let actual_value = pdf.knot_array.xf(0, 0, 21);

    assert!((actual_value - expected_value).abs() < 1e-8);
}

#[test]
fn test_xfxq2() {
    // TODO: Replace with the actual path to the PDF sets
    let pdf_set_path = Path::new("../NNPDF40_nnlo_as_01180");
    let pdf = load(pdf_set_path);

    let x = 1e-3;
    let q2 = 4.0;
    let gluon_pdf = pdf.xfxq2(21, x, q2);

    assert!((gluon_pdf - 3.316316680794655).abs() < 1e-8);
}