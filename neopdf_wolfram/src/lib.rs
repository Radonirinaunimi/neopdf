use lazy_static::lazy_static;
use parking_lot::Mutex;
use wolfram_library_link::{export, NumericArray};

use neopdf::pdf::PDF;

lazy_static! {
    static ref LOADED_PDFS: Mutex<Vec<PDF>> = Mutex::new(Vec::new());
}

#[export(name = "NeoPDF_Load")]
fn load_pdf(pdf_name: String, member: i64) -> i64 {
    let pdf = PDF::load(&pdf_name, member as usize);
    let mut pdfs = LOADED_PDFS.lock();
    pdfs.push(pdf);
    let index = pdfs.len() - 1;
    index as i64
}

#[export(name = "NeoPDF_XFXQ2")]
fn xfxq2(index: i64, pid: i64, points: &NumericArray<f64>) -> f64 {
    let pdfs = LOADED_PDFS.lock();
    if index as usize >= pdfs.len() {
        panic!("invalid PDF index");
    }
    pdfs[index as usize].xfxq2(pid as i32, points.as_slice())
}

#[export(name = "NeoPDF_AlphasQ2")]
fn alphas_q2(index: i64, q2: f64) -> f64 {
    let pdfs = LOADED_PDFS.lock();
    if index as usize >= pdfs.len() {
        panic!("invalid PDF index");
    }
    pdfs[index as usize].alphas_q2(q2)
}

#[export(name = "NeoPDF_Clear")]
fn clear_pdfs() {
    let mut pdfs = LOADED_PDFS.lock();
    pdfs.clear();
}
