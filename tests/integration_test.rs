use lhapdf_rust::*;
use std::path::Path;

#[test]
fn test_xf_at_knots() {
    let pdf_set_path = Path::new("./_lhapdf/NNPDF40_nnlo_as_01180");
    let pdf = load(pdf_set_path);

    let cases = vec![
        (0, 0, 21, 0.14844111), // at the (x, Q) boundaries
        (0, 0, 1, 1.4254154),   // at the (x, Q) boundaries
        (0, 0, 2, 1.4257712),   // at the (x, Q) boundaries
        (1, 0, 21, 0.15395356), // at the Q boundary
        (1, 0, 1, 1.3883271),   // at the Q boundary
        (1, 0, 2, 1.3887002),   // at the Q boundary
        (1, 2, 21, -3.164867),
        (1, 2, 1, 1.9235433),
        (1, 2, 2, 1.9239212),
    ];

    for (x_id, q_id, pid, expected) in cases {
        assert!(
            (pdf.knot_array.xf(x_id, q_id, pid) - expected).abs() < 1e-8,
            "Failed on knot (x, Q, pid)=({x_id}, {q_id}, {pid})"
        );
    }
}

#[test]
fn test_xfxq2_at_knots() {
    let pdf_set_path = Path::new("./_lhapdf/NNPDF40_nnlo_as_01180");
    let pdf = load(pdf_set_path);

    let cases = vec![
        (21, 1e-9, 1.65 * 1.65, 0.14844111), // at the (x, Q2) boundaries
        (1, 1e-9, 1.65 * 1.65, 1.4254154),   // at the (x, Q2) boundaries
        (2, 1e-9, 1.65 * 1.65, 1.4257712),   // at the (x, Q2) boundaries
        (21, 1.2970848e-9, 1.65 * 1.65, 0.15395356), // at the Q2 boundary
        (1, 1.2970848e-9, 1.65 * 1.65, 1.3883271), // at the Q2 boundary
        (2, 1.2970848e-9, 1.65 * 1.65, 1.3887002), // at the Q2 boundary
        (21, 1.2970848e-9, 1.9429053 * 1.9429053, -3.164867),
        (1, 1.2970848e-9, 1.9429053 * 1.9429053, 1.9235433),
        (2, 1.2970848e-9, 1.9429053 * 1.9429053, 1.9239212),
    ];

    for (pid, x, q2, expected) in cases {
        let res = pdf.xfxq2(pid, x, q2);
        assert!(
            (pdf.xfxq2(pid, x, q2) - expected).abs() < 1e-8,
            "Failed on knot (pid, x, Q2)=({pid}, {x}, {q2})={res}"
        );
    }
}

#[test]
fn test_xfxq2_interpolations() {
    let pdf_set_path = Path::new("./_lhapdf/NNPDF40_nnlo_as_01180");
    let pdf = load(pdf_set_path);

    let cases = vec![
        (21, 1e-3, 4.0, 3.316316680794655),
        (4, 1.0, 4.0 * 4.0, -9.886904707204448e-24),
        (4, 1e-9, 4.92000000 * 4.92000000, 6.1829621), // at the threshold
        (5, 1e-9, 4.93 * 4.93, -0.009691974661863908), // slightly above the threshold
        (21, 1e-9, 5.5493622 * 5.5493622, 24.419091),  // 2nd subgrid
        (1, 1e-9, 5.5493622 * 5.5493622, 8.5646215),   // 2nd subgrid
        (2, 1.0, 1e4 * 1e4, 5.538128473634297e-26),    // 2nd subgrid
        (2, 1.0, 1e5 * 1e5, 2.481541837659083e-24),    // at upper Q2 boundary
    ];

    for (pid, x, q2, expected) in cases {
        assert!(
            (pdf.xfxq2(pid, x, q2) - expected).abs() < 1e-8,
            "Failed on knot (pid, x, Q2)=({pid}, {x}, {q2})"
        );
    }
}

#[test]
fn test_alphas_q2_interpolations() {
    let pdf_set_path = Path::new("./_lhapdf/NNPDF40_nnlo_as_01180");
    let pdf = load(pdf_set_path);

    let cases = vec![
        (2.75, 0.32992260049326716),
        (4.0, 0.30095312523656437),
        (100.0, 0.17812270669689784),
    ];

    for (q2, expected) in cases {
        assert!(
            (pdf.alphas_q2(q2) - expected).abs() < 1e-8,
            "Failed AlphaSQ2(Q2={q2})"
        );
    }
}
