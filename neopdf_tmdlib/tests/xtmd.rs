#![allow(missing_docs)]

use neopdf_tmdlib::Tmd;

#[test]
#[ignore]
fn test_tmd_init_and_pdf() {
    let mut tmd = Tmd::new();
    let setname = "MAP22_grids_FF_Km_N3LL";

    tmd.init(setname, 0);
    let n_members = tmd.num_members();
    assert!(n_members > 0, "TMD set should have members.");

    let x = 1e-1;
    let kt = 1.0;
    let q = 10.0;
    let pdfs = tmd.xfxq2kt(x, kt, q);
    assert!(!pdfs.is_empty(), "PDF array should not be empty.");

    let is_any_nonzero = pdfs.iter().any(|&val| val != 0.0);
    assert!(is_any_nonzero, "All PDF values are zero!");
}
