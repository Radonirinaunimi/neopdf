//! The C-language interface for `NeoPDF`

use neopdf::pdf::PDF;
use std::ffi::CStr;
use std::os::raw::c_char;

/// Opaque pointer to a PDF object.
pub struct NeoPDF(PDF);

/// Loads a given member of the PDF set.
///
/// # Panics
///
/// TODO
///
/// # Safety
///
/// The `pdf_name` C string must be null-terminated and valid UTF-8.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_load(pdf_name: *const c_char, member: usize) -> *mut NeoPDF {
    let c_str = unsafe { CStr::from_ptr(pdf_name) };
    let pdf_name = c_str.to_str().expect("Invalid UTF-8 string");
    let pdf = PDF::load(pdf_name, member);
    Box::into_raw(Box::new(NeoPDF(pdf)))
}

/// Frees a PDF object.
///
/// # Panics
///
/// TODO
///
/// # Safety
///
/// The `pdf` pointer must be a valid pointer to a `NeoPDF` object previously
/// allocated by `neopdf_pdf_load`.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_free(pdf: *mut NeoPDF) {
    if pdf.is_null() {
        return;
    }
    unsafe { drop(Box::from_raw(pdf)) };
}

/// Retrieves the `x_min` for this PDF set.
///
/// # Panics
///
/// TODO
///
/// # Safety
///
/// The `pdf` pointer must be a valid pointer to a `CPDF` object.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_x_min(pdf: *mut NeoPDF) -> f64 {
    assert!(!pdf.is_null());
    let pdf_obj = unsafe { &(*pdf).0 };
    pdf_obj.x_min()
}

/// Retrieves the `x_max` for this PDF set.
///
/// # Panics
///
/// TODO
///
/// # Safety
///
/// The `pdf` pointer must be a valid pointer to a `CPDF` object.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_x_max(pdf: *mut NeoPDF) -> f64 {
    assert!(!pdf.is_null());
    let pdf_obj = unsafe { &(*pdf).0 };
    pdf_obj.x_max()
}

/// Retrieves the `q2_min` for this PDF set.
///
/// # Panics
///
/// TODO
///
/// # Safety
///
/// The `pdf` pointer must be a valid pointer to a `CPDF` object.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_q2_min(pdf: *mut NeoPDF) -> f64 {
    assert!(!pdf.is_null());
    let pdf_obj = unsafe { &(*pdf).0 };
    pdf_obj.q2_min()
}

/// Retrieves the `q2_max` for this PDF set.
///
/// # Panics
///
/// TODO
///
/// # Safety
///
/// The `pdf` pointer must be a valid pointer to a `CPDF` object.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_q2_max(pdf: *mut NeoPDF) -> f64 {
    assert!(!pdf.is_null());
    let pdf_obj = unsafe { &(*pdf).0 };
    pdf_obj.q2_max()
}

/// Interpolates the PDF value (xf) for a given flavor, x, and Q2.
///
/// # Panics
///
/// TODO
///
/// # Safety
///
/// The `pdf` pointer must be a valid pointer to a `CPDF` object.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_xfxq2(pdf: *mut NeoPDF, id: i32, x: f64, q2: f64) -> f64 {
    assert!(!pdf.is_null());
    let pdf_obj = unsafe { &(*pdf).0 };
    pdf_obj.xfxq2(id, x, q2)
}

/// Computes the `alpha_s` value at a given Q2.
///
/// # Panics
///
/// TODO
///
/// # Safety
///
/// The `pdf` pointer must be a valid pointer to a `CPDF` object.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_alphas_q2(pdf: *mut NeoPDF, q2: f64) -> f64 {
    assert!(!pdf.is_null());
    let pdf_obj = unsafe { &(*pdf).0 };
    pdf_obj.alphas_q2(q2)
}
