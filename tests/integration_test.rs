use lhapdf_rust::*;
use std::path::Path;

#[test]
fn test_xf_at_knot() {
    // TODO: Replace with the actual path to the PDF sets
    let pdf_set_path = Path::new("./_lhapdf/NNPDF40_nnlo_as_01180");
    let pdf = load(pdf_set_path);

    // test for `xf` at x_0 and q_0
    assert!((pdf.knot_array.xf(0, 0, 21) - 0.14844111).abs() < 1e-8);
    assert!((pdf.knot_array.xf(0, 0, 1) - 1.4254154).abs() < 1e-8);
    assert!((pdf.knot_array.xf(0, 0, 2) - 1.4257712).abs() < 1e-8);

    // test for `xf` at x_1 and q_0
    assert!((pdf.knot_array.xf(1, 0, 21) - 0.15395356).abs() < 1e-8);
    assert!((pdf.knot_array.xf(1, 0, 1) - 1.3883271).abs() < 1e-8);
    assert!((pdf.knot_array.xf(1, 0, 2) - 1.3887002).abs() < 1e-8);

    // test for `xf` at x_1 and q_2
    assert!((pdf.knot_array.xf(1, 2, 21) + 3.164867).abs() < 1e-8);
    assert!((pdf.knot_array.xf(1, 2, 1) - 1.9235433).abs() < 1e-8);
    assert!((pdf.knot_array.xf(1, 2, 2) - 1.9239212).abs() < 1e-8);
}

#[ignore = "Issues with interpolation?"]
#[test]
fn test_xfxq2() {
    // TODO: Replace with the actual path to the PDF sets
    let pdf_set_path = Path::new("./_lhapdf/NNPDF40_nnlo_as_01180");
    let pdf = load(pdf_set_path);

    let x = 1e-3;
    let q2 = 4.0;
    let gluon_pdf = pdf.xfxq2(21, x, q2);

    dbg!(gluon_pdf);
    assert!((gluon_pdf - 3.316316680794655).abs() < 1e-8);
}
