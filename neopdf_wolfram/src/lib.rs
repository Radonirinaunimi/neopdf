use lazy_static::lazy_static;
use parking_lot::Mutex;
use wll::{wolfram_library_function, FromExpr, ToExpr};
use wolfram_library_link as wll;
use wolfram_library_link::expr::{Expr, Symbol};

use neopdf::pdf::PDF;

lazy_static! {
    static ref LOADED_PDFS: Mutex<Vec<PDF>> = Mutex::new(Vec::new());
}

#[wolfram_library_function(name = "NeoPDF_Load")]
pub fn load_pdf(args: &[Expr]) -> Expr {
    if args.len() != 2 {
        return Expr::string(format!("Error: expected 2 arguments, got {}", args.len()));
    }

    let pdf_name = match String::from_expr(&args[0]) {
        Ok(s) => s,
        Err(_) => return Expr::string("Error: could not parse PDF name"),
    };

    let member = match i64::from_expr(&args[1]) {
        Ok(i) => i as usize,
        Err(_) => return Expr::string("Error: could not parse member index"),
    };

    let pdf = PDF::load(&pdf_name, member);
    let mut pdfs = LOADED_PDFS.lock();
    pdfs.push(pdf);
    let index = pdfs.len() - 1;

    Expr::from(index as i64)
}

#[wolfram_library_function(name = "NeoPDF_XFXQ2")]
pub fn xfxq2(args: &[Expr]) -> Expr {
    if args.len() != 3 {
        return Expr::string(format!("Error: expected 3 arguments, got {}", args.len()));
    }

    let index = match i64::from_expr(&args[0]) {
        Ok(i) => i as usize,
        Err(_) => return Expr::string("Error: could not parse PDF index"),
    };

    let pid = match i64::from_expr(&args[1]) {
        Ok(i) => i as i32,
        Err(_) => return Expr::string("Error: could not parse PID"),
    };

    let points = match <Vec<f64>>::from_expr(&args[2]) {
        Ok(p) => p,
        Err(_) => return Expr::string("Error: could not parse points"),
    };

    let pdfs = LOADED_PDFS.lock();
    if index >= pdfs.len() {
        return Expr::string(format!("Error: invalid PDF index {}", index));
    }

    let result = pdfs[index].xfxq2(pid, &points);
    Expr::from(result)
}

#[wolfram_library_function(name = "NeoPDF_AlphasQ2")]
pub fn alphas_q2(args: &[Expr]) -> Expr {
    if args.len() != 2 {
        return Expr::string(format!("Error: expected 2 arguments, got {}", args.len()));
    }

    let index = match i64::from_expr(&args[0]) {
        Ok(i) => i as usize,
        Err(_) => return Expr::string("Error: could not parse PDF index"),
    };

    let q2 = match f64::from_expr(&args[1]) {
        Ok(q) => q,
        Err(_) => return Expr::string("Error: could not parse Q2"),
    };

    let pdfs = LOADED_PDFS.lock();
    if index >= pdfs.len() {
        return Expr::string(format!("Error: invalid PDF index {}", index));
    }

    let result = pdfs[index].alphas_q2(q2);
    Expr::from(result)
}

#[wolfram_library_function(name = "NeoPDF_Clear")]
pub fn clear_pdfs(_args: &[Expr]) -> Expr {
    let mut pdfs = LOADED_PDFS.lock();
    pdfs.clear();
    Symbol::new("System`Null").into()
}
