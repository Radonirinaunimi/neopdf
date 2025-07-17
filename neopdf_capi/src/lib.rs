//! The C-language interface for `NeoPDF`

use neopdf::gridpdf::GridArray;
use neopdf::metadata::{InterpolatorType, MetaData, SetType};
use neopdf::pdf::PDF;
use std::ffi::CStr;
use std::os::raw::{c_char, c_double, c_int};
use std::slice;

/// TODO
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NeoPDFResult {
    /// TODO
    Success = 0,
    /// TODO
    ErrorNullPointer = -1,
    /// TODO
    ErrorInvalidData = -2,
    /// TODO
    ErrorMemoryError = -3,
    /// TODO
    ErrorInvalidLength = -4,
}

impl From<NeoPDFResult> for c_int {
    fn from(result: NeoPDFResult) -> Self {
        result as Self
    }
}

/// Opaque pointer to a PDF object.
pub struct NeoPDFWrapper(PDF);

/// Structure to hold an array of PDF pointers and its length.
#[repr(C)]
pub struct NeoPDFMembers {
    /// Pointers to the `NeoPDF` objects.
    pub pdfs: *mut *mut NeoPDFWrapper,
    /// The number of PDF members.
    pub size: usize,
}

/// Contains all the data that defines a grid.
#[repr(C)]
pub struct NeoPDFGrid {
    /// A list of subgrids, each with its own kinematic coverage.
    pub subgrids: *mut neopdf::parser::SubgridData,
    /// The number of subgrids in the `subgrids` list.
    pub num_subgrids: usize,
    /// A list of flavor IDs.
    pub flavors: *mut i32,
    /// The number of flavors in the `flavors` list.
    pub num_flavors: usize,
}

/// Creates a new, empty `NeoPDFGrid`.
///
/// The caller is responsible for freeing the returned grid using `neopdf_grid_free`.
#[no_mangle]
pub extern "C" fn neopdf_grid_new() -> *mut NeoPDFGrid {
    let grid = Box::new(NeoPDFGrid {
        subgrids: std::ptr::null_mut(),
        num_subgrids: 0,
        flavors: std::ptr::null_mut(),
        num_flavors: 0,
    });
    Box::into_raw(grid)
}

/// Adds a subgrid to an existing `NeoPDFGrid`.
///
/// This function takes ownership of the provided data arrays and resizes them as needed.
///
/// # Safety
///
/// - `grid` must be a valid pointer to a `NeoPDFGrid` created by `neopdf_grid_new`.
/// - The data pointers (`nucleons`, `alphas`, etc.) must be valid for the specified lengths.
#[no_mangle]
pub unsafe extern "C" fn neopdf_grid_add_subgrid(
    grid: *mut NeoPDFGrid,
    nucleons: *const c_double,
    num_nucleons: usize,
    alphas: *const c_double,
    num_alphas: usize,
    xs: *const c_double,
    num_xs: usize,
    q2s: *const c_double,
    num_q2s: usize,
    grid_data: *const c_double,
    grid_data_len: usize,
) -> NeoPDFResult {
    if grid.is_null() {
        return NeoPDFResult::ErrorNullPointer;
    }

    unsafe {
        let grid = &mut *grid;

        // Convert raw parts back to a Vec to append the new subgrid
        let mut subgrids = Vec::from_raw_parts(grid.subgrids, grid.num_subgrids, grid.num_subgrids);

        let subgrid = neopdf::parser::SubgridData {
            nucleons: slice::from_raw_parts(nucleons, num_nucleons).to_vec(),
            alphas: slice::from_raw_parts(alphas, num_alphas).to_vec(),
            xs: slice::from_raw_parts(xs, num_xs).to_vec(),
            q2s: slice::from_raw_parts(q2s, num_q2s).to_vec(),
            grid_data: slice::from_raw_parts(grid_data, grid_data_len).to_vec(),
        };
        subgrids.push(subgrid);

        // Update the grid with the new subgrid list
        grid.subgrids = subgrids.as_mut_ptr();
        grid.num_subgrids = subgrids.len();
        std::mem::forget(subgrids);
    }

    NeoPDFResult::Success
}

/// Sets the flavor IDs for a `NeoPDFGrid`.
///
/// # Safety
///
/// - `grid` must be a valid pointer to a `NeoPDFGrid`.
/// - `flavors` must be a valid pointer to an array of integers of size `num_flavors`.
#[no_mangle]
pub unsafe extern "C" fn neopdf_grid_set_flavors(
    grid: *mut NeoPDFGrid,
    flavors: *const c_int,
    num_flavors: usize,
) -> NeoPDFResult {
    if grid.is_null() {
        return NeoPDFResult::ErrorNullPointer;
    }
    unsafe {
        let grid = &mut *grid;
        let slice = slice::from_raw_parts(flavors, num_flavors);
        let mut flavors_vec = slice.to_vec();

        // Update the grid with the new flavors list
        grid.flavors = flavors_vec.as_mut_ptr();
        grid.num_flavors = flavors_vec.len();
        std::mem::forget(flavors_vec);
    }

    NeoPDFResult::Success
}

/// Frees the memory allocated for a `NeoPDFGrid`.
///
/// # Safety
///
/// `grid` must be a valid pointer to a `NeoPDFGrid` created by `neopdf_grid_new`.
#[no_mangle]
pub unsafe extern "C" fn neopdf_grid_free(grid: *mut NeoPDFGrid) {
    if !grid.is_null() {
        unsafe {
            // Reconstruct the Vecs from the raw parts to ensure proper deallocation
            let subgrids =
                Vec::from_raw_parts((*grid).subgrids, (*grid).num_subgrids, (*grid).num_subgrids);
            let flavors =
                Vec::from_raw_parts((*grid).flavors, (*grid).num_flavors, (*grid).num_flavors);
            drop(subgrids);
            drop(flavors);

            // Finally, free the grid itself
            drop(Box::from_raw(grid));
        }
    }
}

/// TODO
#[repr(C)]
pub struct NeoPDFMetaData {
    set_desc: *const c_char,
    set_index: u32,
    num_members: u32,
    x_min: c_double,
    x_max: c_double,
    q_min: c_double,
    q_max: c_double,
    flavors: *const c_int,
    num_flavors: usize,
    format: *const c_char,
    alphas_q_values: *const c_double,
    num_alphas_q: usize,
    alphas_vals: *const c_double,
    num_alphas_vals: usize,
    polarised: bool,
    set_type: c_int,
    interpolator_type: c_int,
}

/// TODO
fn process_metadata(meta: *const NeoPDFMetaData) -> Option<MetaData> {
    if meta.is_null() {
        return None;
    }
    let meta = unsafe { &*meta };
    let set_desc = unsafe { CStr::from_ptr(meta.set_desc) }
        .to_string_lossy()
        .into_owned();
    let format = unsafe { CStr::from_ptr(meta.format) }
        .to_string_lossy()
        .into_owned();
    let flavors = unsafe { std::slice::from_raw_parts(meta.flavors, meta.num_flavors).to_vec() };
    let alphas_q_values =
        unsafe { std::slice::from_raw_parts(meta.alphas_q_values, meta.num_alphas_q).to_vec() };
    let alphas_vals =
        unsafe { std::slice::from_raw_parts(meta.alphas_vals, meta.num_alphas_vals).to_vec() };

    Some(MetaData {
        set_desc,
        set_index: meta.set_index,
        num_members: meta.num_members,
        x_min: meta.x_min,
        x_max: meta.x_max,
        q_min: meta.q_min,
        q_max: meta.q_max,
        flavors,
        format,
        alphas_q_values,
        alphas_vals,
        polarised: meta.polarised,
        set_type: match meta.set_type {
            1 => SetType::Fragfn,
            _ => SetType::Pdf,
        },
        interpolator_type: match meta.interpolator_type {
            0 => InterpolatorType::Bilinear,
            1 => InterpolatorType::LogBilinear,
            3 => InterpolatorType::LogTricubic,
            4 => InterpolatorType::InterpNDLinear,
            _ => InterpolatorType::LogBicubic,
        },
    })
}

/// TODO
///
/// # Safety
///
/// TODO
#[no_mangle]
pub unsafe extern "C" fn neopdf_grid_compress(
    grid: *const NeoPDFGrid,
    metadata: *const NeoPDFMetaData,
    out_path: *const c_char,
) -> NeoPDFResult {
    if grid.is_null() || metadata.is_null() || out_path.is_null() {
        return NeoPDFResult::ErrorNullPointer;
    }
    unsafe {
        let grid = &*grid;
        let Some(meta) = process_metadata(metadata) else {
            return NeoPDFResult::ErrorInvalidData;
        };
        let out_path = CStr::from_ptr(out_path).to_string_lossy();
        let subgrids = std::slice::from_raw_parts(grid.subgrids, grid.num_subgrids).to_vec();
        let flavors = std::slice::from_raw_parts(grid.flavors, grid.num_flavors).to_vec();
        let grid_array = GridArray::new(subgrids, flavors);
        match neopdf::writer::GridArrayCollection::compress(
            &[&grid_array],
            &meta,
            out_path.as_ref(),
        ) {
            Ok(()) => NeoPDFResult::Success,
            Err(_) => NeoPDFResult::ErrorMemoryError,
        }
    }
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
pub unsafe extern "C" fn neopdf_pdf_load(
    pdf_name: *const c_char,
    member: usize,
) -> *mut NeoPDFWrapper {
    let c_str = unsafe { CStr::from_ptr(pdf_name) };
    let pdf_name = c_str.to_str().expect("Invalid UTF-8 string");
    let pdf = PDF::load(pdf_name, member);
    Box::into_raw(Box::new(NeoPDFWrapper(pdf)))
}

/// Loads all members of the PDF set.
///
/// Returns a `NeoPDFMembers` containing pointers to all PDF objects in the set.
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
pub unsafe extern "C" fn neopdf_pdf_load_all(pdf_name: *const c_char) -> NeoPDFMembers {
    let c_str = unsafe { CStr::from_ptr(pdf_name) };
    let pdf_name = c_str.to_str().expect("Invalid UTF-8 string");

    let pdfs = PDF::load_pdfs(pdf_name);
    let length = pdfs.len();

    let mut pdf_pointers: Vec<*mut NeoPDFWrapper> = pdfs
        .into_iter()
        .map(|pdf| Box::into_raw(Box::new(NeoPDFWrapper(pdf))))
        .collect();

    let pdfs_ptr = pdf_pointers.as_mut_ptr();
    std::mem::forget(pdf_pointers); // Prevent Vec from being dropped

    NeoPDFMembers {
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
pub unsafe extern "C" fn neopdf_pdf_free(pdf: *mut NeoPDFWrapper) {
    if pdf.is_null() {
        return;
    }
    unsafe { drop(Box::from_raw(pdf)) };
}

/// Frees the memory allocated for a `NeoPDFMembers`.
///
/// # Safety
///
/// The `array` must be a valid `NeoPDFMembers` returned by `neopdf_pdf_load_all`.
/// After calling this function, the array and all PDF objects it contains
/// become invalid and must not be used.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_array_free(array: NeoPDFMembers) {
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
/// The `pdf` pointer must be a valid pointer to a `NeoPDF` object.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_x_min(pdf: *mut NeoPDFWrapper) -> f64 {
    assert!(!pdf.is_null());
    let pdf_obj = unsafe { &(*pdf).0 };
    pdf_obj.param_ranges().x.min
}

/// Retrieves the `x_max` for this PDF set.
///
/// # Panics
///
/// TODO
///
/// # Safety
///
/// The `pdf` pointer must be a valid pointer to a `NeoPDF` object.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_x_max(pdf: *mut NeoPDFWrapper) -> f64 {
    assert!(!pdf.is_null());
    let pdf_obj = unsafe { &(*pdf).0 };
    pdf_obj.param_ranges().x.max
}

/// Retrieves the `q2_min` for this PDF set.
///
/// # Panics
///
/// TODO
///
/// # Safety
///
/// The `pdf` pointer must be a valid pointer to a `NeoPDF` object.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_q2_min(pdf: *mut NeoPDFWrapper) -> f64 {
    assert!(!pdf.is_null());
    let pdf_obj = unsafe { &(*pdf).0 };
    pdf_obj.param_ranges().q2.min
}

/// Retrieves the `q2_max` for this PDF set.
///
/// # Panics
///
/// TODO
///
/// # Safety
///
/// The `pdf` pointer must be a valid pointer to a `NeoPDF` object.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_q2_max(pdf: *mut NeoPDFWrapper) -> f64 {
    assert!(!pdf.is_null());
    let pdf_obj = unsafe { &(*pdf).0 };
    pdf_obj.param_ranges().q2.max
}

/// Interpolates the PDF value (xf) for a given flavor, x, and Q2.
///
/// # Panics
///
/// TODO
///
/// # Safety
///
/// The `pdf` pointer must be a valid pointer to a `NeoPDF` object.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_xfxq2(
    pdf: *mut NeoPDFWrapper,
    id: i32,
    x: f64,
    q2: f64,
) -> f64 {
    assert!(!pdf.is_null());
    let pdf_obj = unsafe { &(*pdf).0 };
    pdf_obj.xfxq2(id, &[x, q2])
}

/// Computes the `alpha_s` value at a given Q2.
///
/// # Panics
///
/// TODO
///
/// # Safety
///
/// The `pdf` pointer must be a valid pointer to a `NeoPDF` object.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_alphas_q2(pdf: *mut NeoPDFWrapper, q2: f64) -> f64 {
    assert!(!pdf.is_null());
    let pdf_obj = unsafe { &(*pdf).0 };
    pdf_obj.alphas_q2(q2)
}
