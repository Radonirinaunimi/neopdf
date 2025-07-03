use lhapdf_rust::*;
use std::path::Path;

#[test]
fn test_xf_at_knots() {
    let pdf_set_path = Path::new("./_lhapdf/NNPDF40_nnlo_as_01180");
    let pdf = load(pdf_set_path);

    // test for `xf` at x_0 and q_0
    assert!((dbg!(pdf.knot_array.xf(0, 0, 21)) - 0.14844111).abs() < 1e-8);
    assert!((dbg!(pdf.knot_array.xf(0, 0, 1)) - 1.4254154).abs() < 1e-8);
    assert!((dbg!(pdf.knot_array.xf(0, 0, 2)) - 1.4257712).abs() < 1e-8);
}

#[test]
fn test_xfxq2() {
    let pdf_set_path = Path::new("./_lhapdf/NNPDF40_nnlo_as_01180");
    let pdf = load(pdf_set_path);

    // test for `xf` at x_0 and q_0
    assert!((dbg!(pdf.xfxq2(21, 1e-9, 1.65 * 1.65)) - 0.14844111).abs() < 1e-8);
    assert!((dbg!(pdf.xfxq2(1, 1e-9, 1.65 * 1.65)) - 1.4254154).abs() < 1e-8);
    assert!((dbg!(pdf.xfxq2(2, 1e-9, 1.65 * 1.65)) - 1.4257712).abs() < 1e-8);
}
