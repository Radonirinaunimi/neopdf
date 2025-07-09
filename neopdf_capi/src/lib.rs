//! The C-language interface for `NeoPDF`

use neopdf::pdf::PDF;
use std::ffi::CStr;
use std::os::raw::c_char;

/// Opaque pointer to a PDF object.
pub struct NeoPDF(PDF);

/// Structure to hold an array of PDF pointers and its length.
#[repr(C)]
pub struct NeoPDFArray {
    /// Pointers to the `NeoPDF` objects.
    pub pdfs: *mut *mut NeoPDF,
    /// The number of PDF members.
    pub size: usize,
}

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

/// Loads all members of the PDF set.
///
/// Returns a `NeoPDFArray` containing pointers to all PDF objects in the set.
/// The caller is responsible for freeing the memory using `neopdf_pdf_array_free`.
///
/// # Panics
///
/// TODO
///
/// # Safety
///
/// The `pdf_name` C string must be null-terminated and valid UTF-8.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_load_all(pdf_name: *const c_char) -> NeoPDFArray {
    let c_str = unsafe { CStr::from_ptr(pdf_name) };
    let pdf_name = c_str.to_str().expect("Invalid UTF-8 string");

    let pdfs = PDF::load_pdfs(pdf_name);
    let length = pdfs.len();

    let mut pdf_pointers: Vec<*mut NeoPDF> = pdfs
        .into_iter()
        .map(|pdf| Box::into_raw(Box::new(NeoPDF(pdf))))
        .collect();

    let pdfs_ptr = pdf_pointers.as_mut_ptr();
    std::mem::forget(pdf_pointers); // Prevent Vec from being dropped

    NeoPDFArray {
        pdfs: pdfs_ptr,
        size: length,
    }
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

/// Frees the memory allocated for a `NeoPDFArray`.
///
/// # Safety
///
/// The `array` must be a valid `NeoPDFArray` returned by `neopdf_pdf_load_all`.
/// After calling this function, the array and all PDF objects it contains
/// become invalid and must not be used.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_array_free(array: NeoPDFArray) {
    if array.pdfs.is_null() {
        return;
    }

    let pdf_pointers = unsafe { Vec::from_raw_parts(array.pdfs, array.size, array.size) };

    for pdf_ptr in pdf_pointers {
        if !pdf_ptr.is_null() {
            unsafe { drop(Box::from_raw(pdf_ptr)) };
        }
    }
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
