use lhapdf_rust::*;
use std::path::Path;

fn main() {
    // TODO: Replace with the actual path to the PDF sets
    let pdf_set_path = Path::new("./_lhapdf/NNPDF40_nnlo_as_01180");
    let pdf = load(pdf_set_path);

    let x = 1e-3;
    let q2 = 4.0;
    let gluon_pdf = pdf.xfxq2(21, x, q2);

    println!("g(x={}, Q2={}) = {}", x, q2, gluon_pdf);
}
