//! The C-language interface for `NeoPDF`

use std::ffi::CStr;
use std::os::raw::{c_char, c_double, c_int};
use std::slice;

use neopdf::gridpdf::GridArray;
use neopdf::metadata::{InterpolatorType, MetaData, SetType};
use neopdf::parser::SubgridData;
use neopdf::pdf::PDF;
use neopdf::writer::GridArrayCollection;

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

/// An opaque struct holding the data for a single grid, including its subgrids and flavors.
/// C code should not access its fields directly.
pub struct NeoPDFGrid {
    subgrids: Vec<SubgridData>,
    flavors: Vec<i32>,
}

/// Creates a new, empty `NeoPDFGrid`.
///
/// The caller is responsible for freeing the returned grid using `neopdf_grid_free`.
#[no_mangle]
pub extern "C" fn neopdf_grid_new() -> *mut NeoPDFGrid {
    let grid = Box::new(NeoPDFGrid {
        subgrids: Vec::new(),
        flavors: Vec::new(),
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

        let subgrid = SubgridData {
            nucleons: slice::from_raw_parts(nucleons, num_nucleons).to_vec(),
            alphas: slice::from_raw_parts(alphas, num_alphas).to_vec(),
            xs: slice::from_raw_parts(xs, num_xs).to_vec(),
            q2s: slice::from_raw_parts(q2s, num_q2s).to_vec(),
            grid_data: slice::from_raw_parts(grid_data, grid_data_len).to_vec(),
        };
        grid.subgrids.push(subgrid);
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
        grid.flavors = slice.to_vec();
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
#[repr(C)]
pub struct NeoPDFGridArrayCollection {
    grids: *mut *mut NeoPDFGrid,
    num_grids: usize,
    capacity: usize,
}

/// Creates a new, empty `NeoPDFGridArrayCollection`.
///
/// The caller is responsible for freeing the returned collection using
/// `neopdf_gridarray_collection_free`.
#[no_mangle]
pub extern "C" fn neopdf_gridarray_collection_new() -> *mut NeoPDFGridArrayCollection {
    let collection = Box::new(NeoPDFGridArrayCollection {
        grids: std::ptr::null_mut(),
        num_grids: 0,
        capacity: 0,
    });
    Box::into_raw(collection)
}

/// Adds a `NeoPDFGrid` to a `NeoPDFGridArrayCollection`.
///
/// This function resizes the collection as needed.
///
/// # Safety
///
/// - `collection` must be a valid pointer to a `NeoPDFGridArrayCollection`.
/// - `grid` must be a valid pointer to a `NeoPDFGrid`.
#[no_mangle]
pub unsafe extern "C" fn neopdf_gridarray_collection_add_grid(
    collection: *mut NeoPDFGridArrayCollection,
    grid: *mut NeoPDFGrid,
) -> NeoPDFResult {
    if collection.is_null() || grid.is_null() {
        return NeoPDFResult::ErrorNullPointer;
    }

    unsafe {
        let collection = &mut *collection;
        if collection.num_grids == collection.capacity {
            let new_capacity = if collection.capacity == 0 {
                4
            } else {
                collection.capacity * 2
            };
            let new_ptr = if collection.grids.is_null() {
                libc::calloc(new_capacity, std::mem::size_of::<*mut NeoPDFGrid>())
                    .cast::<*mut NeoPDFGrid>()
            } else {
                libc::realloc(
                    collection.grids.cast::<libc::c_void>(),
                    new_capacity * std::mem::size_of::<*mut NeoPDFGrid>(),
                )
                .cast::<*mut NeoPDFGrid>()
            };
            if new_ptr.is_null() {
                return NeoPDFResult::ErrorMemoryError;
            }
            collection.grids = new_ptr;
            collection.capacity = new_capacity;
        }
        *collection.grids.add(collection.num_grids) = grid;
        collection.num_grids += 1;
    }

    NeoPDFResult::Success
}

/// Frees the memory of a `NeoPDFGridArrayCollection` and its owned `NeoPDFGrid` pointers.
///
/// # Safety
///
/// - `collection` must be a valid pointer to a `NeoPDFGridArrayCollection` created by
///   `neopdf_gridarray_collection_new`.
#[no_mangle]
pub unsafe extern "C" fn neopdf_gridarray_collection_free(
    collection: *mut NeoPDFGridArrayCollection,
) {
    if !collection.is_null() {
        unsafe {
            let collection = Box::from_raw(collection);
            if !collection.grids.is_null() {
                libc::free(collection.grids.cast::<libc::c_void>());
            }
        }
        // Do not free the NeoPDFGrid pointers themselves here (let user manage them)
    }
}

/// Compresses a collection of grids and writes them to a file.
///
/// # Safety
///
/// - `collection` must be a valid pointer to a `NeoPDFGridArrayCollection`.
/// - `metadata` must be a valid pointer to a `NeoPDFMetaData` struct.
/// - `out_path` must be a valid, null-terminated C string.
#[no_mangle]
pub unsafe extern "C" fn neopdf_grid_compress(
    collection: *const NeoPDFGridArrayCollection,
    metadata: *const NeoPDFMetaData,
    out_path: *const c_char,
) -> NeoPDFResult {
    unsafe {
        if collection.is_null() || metadata.is_null() || out_path.is_null() {
            return NeoPDFResult::ErrorNullPointer;
        }

        let collection = &*collection;
        let Some(meta) = process_metadata(metadata) else {
            return NeoPDFResult::ErrorInvalidData;
        };

        let out_path = CStr::from_ptr(out_path).to_string_lossy();
        let mut grid_arrays = Vec::with_capacity(collection.num_grids);

        for i in 0..collection.num_grids {
            let grid = &*(*collection.grids.add(i));
            let grid_array = GridArray::new(grid.subgrids.clone(), grid.flavors.clone());
            grid_arrays.push(grid_array);
        }

        let grid_refs: Vec<&GridArray> = grid_arrays.iter().collect();

        match GridArrayCollection::compress(&grid_refs, &meta, out_path.as_ref()) {
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
