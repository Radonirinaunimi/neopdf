use lhapdf_rust::*;
use std::path::Path;

#[test]
fn test_xf_at_knots() {
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

#[test]
fn test_xfxq2_at_knots() {
    let pdf_set_path = Path::new("./_lhapdf/NNPDF40_nnlo_as_01180");
    let pdf = load(pdf_set_path);

    // test for x=1e-9 and Q=1.65
    let q2 = 1.65 * 1.65;
    assert!((dbg!(pdf.xfxq2(21, 1e-9, q2)) - 0.14844111).abs() < 1e-8);
    assert!((dbg!(pdf.xfxq2(1, 1e-9, q2)) - 1.4254154).abs() < 1e-8);
    assert!((dbg!(pdf.xfxq2(2, 1e-9, q2)) - 1.4257712).abs() < 1e-8);

    // test for x=1.2970848e-9 and Q=1.65
    let q2 = 1.65 * 1.65;
    assert!((pdf.xfxq2(21, 1.2970848e-9, q2) - 0.15395356).abs() < 1e-8);
    assert!((pdf.xfxq2(1, 1.2970848e-9, q2) - 1.3883271).abs() < 1e-8);
    assert!((pdf.xfxq2(2, 1.2970848e-9, q2) - 1.3887002).abs() < 1e-8);

    // test for x=1.2970848e-9 and Q=1.9429053
    let q2 = 1.9429053 * 1.9429053;
    assert!((pdf.xfxq2(21, 1.2970848e-9, q2) + 3.164867).abs() < 1e-8);
    assert!((pdf.xfxq2(1, 1.2970848e-9, q2) - 1.9235433).abs() < 1e-8);
    assert!((pdf.xfxq2(2, 1.2970848e-9, q2) - 1.9239212).abs() < 1e-8);
}

#[test]
fn test_xfxq2() {
    let pdf_set_path = Path::new("./_lhapdf/NNPDF40_nnlo_as_01180");
    let pdf = load(pdf_set_path);

    assert!((pdf.xfxq2(21, 1e-3, 4.0) - 3.316316680794655).abs() < 1e-8);
}

#[test]
fn test_alphas_q2() {
    let pdf_set_path = Path::new("./_lhapdf/NNPDF40_nnlo_as_01180");
    let pdf = load(pdf_set_path);

    assert!((pdf.alphas_q2(1.65) - 0.3754297714824437).abs() < 1e-8);
    assert!((pdf.alphas_q2(4.0) - 0.30095312523656437).abs() < 1e-8);
    assert!((pdf.alphas_q2(100.0) - 0.17812270669689784).abs() < 1e-8);
}
