use ndarray::Array3;
use neopdf::pdf::PDF;

const PRECISION: f64 = 1e-12;

#[test]
fn test_xf_at_knots() {
    let pdf = PDF::load("NNPDF40_nnlo_as_01180");

    let cases = vec![
        (0, 0, 1, 1.4254154), // at the (x, Q) boundaries
        (0, 0, 2, 1.4257712), // at the (x, Q) boundaries
        (1, 0, 1, 1.3883271), // at the Q boundary
        (1, 0, 2, 1.3887002), // at the Q boundary
        (1, 2, 1, 1.9235433),
        (1, 2, 2, 1.9239212),
        (1, 2, 21, -3.164867),
        (0, 0, 21, 0.14844111), // at the (x, Q) boundaries
        (1, 0, 21, 0.15395356), // at the Q boundary
    ];

    for (x_id, q_id, pid, expected) in cases {
        assert!(
            (pdf.xf(x_id, q_id, pid, 0) - expected).abs() < PRECISION,
            "Failed on knot (x, Q, pid)=({x_id}, {q_id}, {pid})"
        );
    }
}

#[test]
fn test_xfxq2_at_knots() {
    let pdf = PDF::load("NNPDF40_nnlo_as_01180");

    let cases = vec![
        (21, 1e-9, 1.65 * 1.65, 0.14844111), // at the (x, Q2) boundaries
        (1, 1e-9, 1.65 * 1.65, 1.4254154),   // at the (x, Q2) boundaries
        (2, 1e-9, 1.65 * 1.65, 1.4257712),   // at the (x, Q2) boundaries
        (1, 1.2970848e-9, 1.65 * 1.65, 1.3883271), // at the Q2 boundary
        (2, 1.2970848e-9, 1.65 * 1.65, 1.3887002), // at the Q2 boundary
        (21, 1.2970848e-9, 1.65 * 1.65, 0.15395356), // at the Q2 boundary
        (21, 1.2970848e-9, 1.9429053 * 1.9429053, -3.164867),
        (1, 1.2970848e-9, 1.9429053 * 1.9429053, 1.9235433),
        (2, 1.2970848e-9, 1.9429053 * 1.9429053, 1.9239212),
    ];

    for (pid, x, q2, expected) in cases {
        let res = pdf.xfxq2(pid, x, q2);
        assert!(
            (pdf.xfxq2(pid, x, q2) - expected).abs() < PRECISION,
            "Failed on knot (pid, x, Q2)=({pid}, {x}, {q2})={res}"
        );
    }
}

#[test]
fn test_xfxq2_interpolations() {
    let pdf = PDF::load("NNPDF40_nnlo_as_01180");

    let cases = vec![
        (21, 1e-3, 4.0, 3.316316680794655),
        (4, 1.0, 4.0 * 4.0, -9.886904707204448e-24),
        (4, 1e-9, 4.92000000 * 4.92000000, 6.1829621), // at the threshold
        (5, 1e-9, 4.93 * 4.93, -0.009691974661863908), // slightly above the threshold
        (21, 1e-9, 5.5493622 * 5.5493622, 24.419091),  // 2nd subgrid
        (1, 1e-9, 5.5493622 * 5.5493622, 8.5646215),   // 2nd subgrid
        (2, 1.0, 1e4 * 1e4, 5.538128473634297e-26),    // 2nd subgrid
        (2, 1.0, 1e5 * 1e5, 2.481541837659083e-24),    // at the upper Q2 boundary
    ];

    for (pid, x, q2, expected) in cases {
        assert!(
            (pdf.xfxq2(pid, x, q2) - expected).abs() < PRECISION,
            "Failed on knot (pid, x, Q2)=({pid}, {x}, {q2})"
        );
    }
}

#[test]
#[should_panic(
    expected = "called `Result::unwrap()` on an `Err` value: SubgridNotFound { x: 1.0, q2: 1e40 }"
)]
fn test_xfxq2_extrapolations() {
    let pdf = PDF::load("NNPDF40_nnlo_as_01180");

    assert!((pdf.xfxq2(2, 1.0, 1e20 * 1e20) - 1e10).abs() < PRECISION);
}

#[test]
fn test_alphas_q2_interpolations() {
    let pdf = PDF::load("NNPDF40_nnlo_as_01180");

    let cases = vec![
        (1.65 * 1.65, 0.33074891), // at the lower Q2 boundary
        (2.75, 0.32992260049326716),
        (4.0, 0.30095312523656437),
        (100.0, 0.17812270669689784),
        (1e5 * 1e5, 0.057798546), // at the upper Q2 boundary
    ];

    for (q2, expected) in cases {
        assert!(
            (pdf.alphas_q2(q2) - expected).abs() < PRECISION,
            "Failed AlphaSQ2(Q2={q2})"
        );
    }
}

#[test]
pub fn test_xfxq2s() {
    let expected = vec![
        0.27337409518414,
        0.63299029999538,
        2.16069749660397,
        0.10357790530199,
        0.21504114381371,
        0.57006831040759,
        0.10357790530199,
        0.21504114381371,
        0.57006831040759,
        -0.00000000000000,
        0.00000000000000,
        -0.00000000000000,
        0.86033466186096,
        1.21367385845476,
        2.72657545285167,
        0.45312519721315,
        0.55795149610763,
        0.89691032146793,
        0.45312519721315,
        0.55795149610763,
        0.89691032146793,
        0.00000000000000,
        0.00000000000000,
        -0.00000000000000,
        0.86855511153327,
        1.22213535382013,
        2.73568304608097,
        0.48361949215031,
        0.58942045717086,
        0.93078902550505,
        0.48361949215031,
        0.58942045717086,
        0.93078902550505,
        0.00000000000000,
        0.00000000000000,
        -0.00000000000000,
        0.86622333102636,
        1.21976794660474,
        2.73323797415359,
        0.48055031214630,
        0.58691729456046,
        0.92977087849558,
        0.48055031214630,
        0.58691729456046,
        0.92977087849558,
        -0.00000000000000,
        -0.00000000000000,
        0.00000000000000,
        0.87487444338356,
        1.22887665534675,
        2.74349810690426,
        0.51619078907508,
        0.62430481766834,
        0.97168760226333,
        0.51619078907508,
        0.62430481766834,
        0.97168760226333,
        -0.00000000000000,
        0.00000000000000,
        -0.00000000000000,
        0.87740420216858,
        1.23152920693911,
        2.74649287970872,
        0.52893044337440,
        0.63851269748660,
        0.98988377695890,
        0.52893044337440,
        0.63851269748660,
        0.98988377695890,
        0.00000000000000,
        0.00000000000000,
        0.00000000000000,
        0.85571557090467,
        1.20898893155978,
        2.72171633403578,
        0.44645311782484,
        0.55175442146873,
        0.89181674344183,
        0.44645311782484,
        0.55175442146873,
        0.89181674344183,
        0.00000000000000,
        -0.00000000000000,
        0.00000000000000,
        0.27342326301934,
        0.63307809523194,
        2.16085463431868,
        0.10365907477091,
        0.21517119223069,
        0.57023552763510,
        0.10365907477091,
        0.21517119223069,
        0.57023552763510,
        0.00000000000000,
        0.00000000000000,
        0.00000000000000,
    ];

    let pdf = PDF::load("NNPDF40_nnlo_as_01180");

    // Define the vectors of kinematics & flavours
    let ids = (-4..=4).filter(|&x| x != 0).collect();
    let xs = vec![1e-5, 1e-3, 1e-3, 1.0];
    let q2s = vec![5.0, 10.0, 100.0];

    let results = pdf.xfxq2s(ids, xs, q2s);
    let expected_res = Array3::from_shape_vec(results.raw_dim(), expected).unwrap();

    for ((i, j, k), elems) in results.indexed_iter() {
        assert!((*elems - expected_res[[i, j, k]]).abs() < PRECISION);
    }
}
