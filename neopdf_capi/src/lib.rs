//! The C-language interface for `NeoPDF`

use std::ffi::CStr;
use std::os::raw::{c_char, c_double, c_int};
use std::slice;

use neopdf::gridpdf::GridArray;
use neopdf::metadata::{InterpolatorType, MetaData, SetType};
use neopdf::parser::SubgridData;
use neopdf::pdf::PDF;
use neopdf::writer::GridArrayCollection;

/// Result codes for `NeoPDF` operations
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NeopdfResult {
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

impl From<NeopdfResult> for c_int {
    fn from(result: NeopdfResult) -> Self {
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

/// Returns the number of PIDs.
///
/// # Panics
///
/// TODO
///
/// # Safety
///
/// TODO
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_num_pids(pdf: *mut NeoPDFWrapper) -> usize {
    assert!(!pdf.is_null());
    let pdf_obj = unsafe { &(*pdf).0 };
    pdf_obj.pids().len()
}

/// Returns the PID representation of the PDF Grid.
///
/// # Panics
///
/// TODO
///
/// # Safety
///
/// TODO
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_pids(pdf: *mut NeoPDFWrapper, pids: *mut i32, num_pids: usize) {
    assert!(!pdf.is_null());
    let pdf_obj = unsafe { &(*pdf).0 };

    let pids = unsafe { slice::from_raw_parts_mut(pids, num_pids) };
    let pid_values = pdf_obj.pids();

    pids.copy_from_slice(pid_values.as_slice().unwrap());
}

/// TODO
#[repr(C)]
pub enum NeopdfSubgridParams {
    /// TODO
    Nucleons,
    /// TODO
    Alphas,
    /// TODO
    Momentum,
    /// TODO
    Scale,
}

/// Returns the number of subgrids in the PDF Grid.
///
/// # Panics
///
/// TODO
///
/// # Safety
///
/// TODO
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_num_subgrids(pdf: *mut NeoPDFWrapper) -> usize {
    assert!(!pdf.is_null());
    let pdf_obj = unsafe { &(*pdf).0 };
    pdf_obj.num_subgrids()
}

/// Returns the minimum and maximum value for a given parameter.
///
/// # Panics
///
/// TODO
///
/// # Safety
///
/// TODO
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_param_range(
    pdf: *mut NeoPDFWrapper,
    param: NeopdfSubgridParams,
    param_range: *mut f64,
) {
    assert!(!pdf.is_null());
    let pdf_obj = unsafe { &(*pdf).0 };

    let param_range = unsafe { slice::from_raw_parts_mut(param_range, 2) };
    let range_params = match param {
        NeopdfSubgridParams::Nucleons => &[
            pdf_obj.param_ranges().nucleons.min,
            pdf_obj.param_ranges().nucleons.max,
        ],
        NeopdfSubgridParams::Alphas => &[
            pdf_obj.param_ranges().alphas.min,
            pdf_obj.param_ranges().alphas.max,
        ],
        NeopdfSubgridParams::Momentum => {
            &[pdf_obj.param_ranges().x.min, pdf_obj.param_ranges().x.max]
        }
        NeopdfSubgridParams::Scale => {
            &[pdf_obj.param_ranges().q2.min, pdf_obj.param_ranges().q2.max]
        }
    };

    param_range.copy_from_slice(range_params);
}

/// Returns the shape of the subgrids in the order of their index for a given parameter.
///
/// # Panics
///
/// TODO
///
/// # Safety
///
/// TODO
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_subgrids_shape_for_param(
    pdf: *mut NeoPDFWrapper,
    subgrid_shape: *mut usize,
    num_subgrid: usize,
    subgrid_param: NeopdfSubgridParams,
) {
    assert!(!pdf.is_null());
    let pdf_obj = unsafe { &(*pdf).0 };

    let subgrid_shape = unsafe { slice::from_raw_parts_mut(subgrid_shape, num_subgrid) };
    let shape_subgrids: Vec<usize> = pdf_obj
        .subgrids()
        .iter()
        .map(|sub| match subgrid_param {
            NeopdfSubgridParams::Nucleons => sub.nucleons.len(),
            NeopdfSubgridParams::Alphas => sub.alphas.len(),
            NeopdfSubgridParams::Momentum => sub.xs.len(),
            NeopdfSubgridParams::Scale => sub.q2s.len(),
        })
        .collect();

    subgrid_shape.copy_from_slice(&shape_subgrids);
}

/// Returns the grid values of a parameter for a given subgrid.
///
/// # Panics
///
/// TODO
///
/// # Safety
///
/// TODO
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_subgrids_for_param(
    pdf: *mut NeoPDFWrapper,
    subgrid: *mut f64,
    subgrid_param: NeopdfSubgridParams,
    num_subgrid: usize,
    subgrid_shape: *mut usize,
    subgrid_index: usize,
) {
    assert!(!pdf.is_null());
    let pdf_obj = unsafe { &(*pdf).0 };

    let subgrid_shape = unsafe { slice::from_raw_parts(subgrid_shape, num_subgrid) };
    let subgrid = unsafe { slice::from_raw_parts_mut(subgrid, subgrid_shape[subgrid_index]) };
    let subgrid_knots = match subgrid_param {
        NeopdfSubgridParams::Nucleons => &pdf_obj.subgrids()[subgrid_index].nucleons,
        NeopdfSubgridParams::Alphas => &pdf_obj.subgrids()[subgrid_index].alphas,
        NeopdfSubgridParams::Momentum => &pdf_obj.subgrids()[subgrid_index].xs,
        NeopdfSubgridParams::Scale => &pdf_obj.subgrids()[subgrid_index].q2s,
    };

    subgrid.copy_from_slice(subgrid_knots.as_slice().unwrap());
}

/// An opaque struct holding the data for a single grid, including its subgrids and flavors.
/// C code should not access its fields directly.
pub struct NeoPDFGrid {
    subgrids: Vec<SubgridData>,
    flavors: Vec<i32>,
}

impl NeoPDFGrid {
    /// Creates a new, empty grid
    const fn new() -> Self {
        Self {
            subgrids: Vec::new(),
            flavors: Vec::new(),
        }
    }

    /// Adds a subgrid to the grid
    #[allow(clippy::too_many_arguments)]
    unsafe fn add_subgrid(
        &mut self,
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
    ) -> NeopdfResult {
        // Check for null pointers
        if nucleons.is_null()
            || alphas.is_null()
            || xs.is_null()
            || q2s.is_null()
            || grid_data.is_null()
        {
            return NeopdfResult::ErrorNullPointer;
        }

        let subgrid = unsafe {
            SubgridData {
                nucleons: slice::from_raw_parts(nucleons, num_nucleons).to_vec(),
                alphas: slice::from_raw_parts(alphas, num_alphas).to_vec(),
                xs: slice::from_raw_parts(xs, num_xs).to_vec(),
                q2s: slice::from_raw_parts(q2s, num_q2s).to_vec(),
                grid_data: slice::from_raw_parts(grid_data, grid_data_len).to_vec(),
            }
        };
        self.subgrids.push(subgrid);

        NeopdfResult::Success
    }

    /// Sets the flavor IDs for the grid
    unsafe fn set_flavors(&mut self, flavors: *const c_int, num_flavors: usize) -> NeopdfResult {
        if flavors.is_null() {
            return NeopdfResult::ErrorNullPointer;
        }
        self.flavors = unsafe { slice::from_raw_parts(flavors, num_flavors).to_vec() };

        NeopdfResult::Success
    }
}

/// Creates a new, empty `NeoPDFGrid`.
/// The caller is responsible for freeing the returned grid using `neopdf_grid_free`.
#[no_mangle]
pub extern "C" fn neopdf_grid_new() -> *mut NeoPDFGrid {
    Box::into_raw(Box::new(NeoPDFGrid::new()))
}

/// Adds a subgrid to an existing `NeoPDFGrid`.
/// This function takes ownership of the provided data arrays and resizes them as needed.
///
/// # Safety
/// - `grid` must be a valid pointer to a `NeoPDFGrid` created by `neopdf_grid_new`.
/// - The data pointers must be valid for the specified lengths.
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
) -> NeopdfResult {
    unsafe {
        grid.as_mut()
            .map_or(NeopdfResult::ErrorNullPointer, |grid| {
                grid.add_subgrid(
                    nucleons,
                    num_nucleons,
                    alphas,
                    num_alphas,
                    xs,
                    num_xs,
                    q2s,
                    num_q2s,
                    grid_data,
                    grid_data_len,
                )
            })
    }
}

/// Sets the flavor IDs for a `NeoPDFGrid`.
///
/// # Safety
/// - `grid` must be a valid pointer to a `NeoPDFGrid`.
/// - `flavors` must be a valid pointer to an array of integers of size `num_flavors`.
#[no_mangle]
pub unsafe extern "C" fn neopdf_grid_set_flavors(
    grid: *mut NeoPDFGrid,
    flavors: *const c_int,
    num_flavors: usize,
) -> NeopdfResult {
    unsafe {
        grid.as_mut()
            .map_or(NeopdfResult::ErrorNullPointer, |grid| {
                grid.set_flavors(flavors, num_flavors)
            })
    }
}

/// Frees the memory allocated for a `NeoPDFGrid`.
///
/// # Safety
/// `grid` must be a valid pointer to a `NeoPDFGrid` created by `neopdf_grid_new`.
#[no_mangle]
pub unsafe extern "C" fn neopdf_grid_free(grid: *mut NeoPDFGrid) {
    if !grid.is_null() {
        unsafe { drop(Box::from_raw(grid)) };
    }
}

/// Metadata for PDF grids
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

/// Safely converts C string to Rust string
unsafe fn cstr_to_string(ptr: *const c_char) -> Option<String> {
    if ptr.is_null() {
        None
    } else {
        unsafe { Some(CStr::from_ptr(ptr).to_string_lossy().into_owned()) }
    }
}

/// Safely converts C array to Rust Vec
unsafe fn carray_to_vec<T: Copy>(ptr: *const T, len: usize) -> Option<Vec<T>> {
    if ptr.is_null() {
        None
    } else {
        unsafe { Some(slice::from_raw_parts(ptr, len).to_vec()) }
    }
}

/// Processes metadata from C struct to Rust struct
fn process_metadata(meta: *const NeoPDFMetaData) -> Option<MetaData> {
    if meta.is_null() {
        return None;
    }

    let meta = unsafe { &*meta };

    // Use helper functions for safer conversions
    let set_desc = unsafe { cstr_to_string(meta.set_desc) }?;
    let format = unsafe { cstr_to_string(meta.format) }?;
    let flavors = unsafe { carray_to_vec(meta.flavors, meta.num_flavors) }?;
    let alphas_q_values = unsafe { carray_to_vec(meta.alphas_q_values, meta.num_alphas_q) }?;
    let alphas_vals = unsafe { carray_to_vec(meta.alphas_vals, meta.num_alphas_vals) }?;

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

/// Represents a dynamically-sized collection of `NeoPDFGrid` pointers.
/// This struct is exposed to C and manages memory for the array of pointers.
#[repr(C)]
pub struct NeoPDFGridArrayCollection {
    /// A raw pointer to a C-style array of `NeoPDFGrid` pointers.
    /// This array holds the pointers to the individual grids added to the collection.
    grids: *mut *mut NeoPDFGrid,
    /// The current number of `NeoPDFGrid` pointers stored in the `grids` array.
    num_grids: usize,
    /// The total allocated capacity of the `grids` array.
    /// When `num_grids` reaches `capacity`, the array is reallocated to a larger size.
    capacity: usize,
}

impl NeoPDFGridArrayCollection {
    /// Creates a new, empty `NeoPDFGridArrayCollection`.
    /// Initializes the collection with no grids and zero capacity.
    const fn new() -> Self {
        Self {
            grids: std::ptr::null_mut(),
            num_grids: 0,
            capacity: 0,
        }
    }

    /// Adds a `NeoPDFGrid` pointer to the collection.
    /// This function handles dynamic resizing of the underlying array if needed.
    ///
    /// # Arguments
    /// * `grid` - A raw pointer to the `NeoPDFGrid` to be added.
    ///
    /// # Returns
    /// `NeoPDFResult::Success` if the grid was added successfully, or an error code
    /// (`ErrorNullPointer`, `ErrorMemoryError`) if an issue occurred.
    fn add_grid(&mut self, grid: *mut NeoPDFGrid) -> NeopdfResult {
        // Ensure the provided grid pointer is not null.
        if grid.is_null() {
            return NeopdfResult::ErrorNullPointer;
        }

        // Check if the current number of grids has reached the allocated capacity.
        // If so, resize the underlying array.
        if self.num_grids == self.capacity {
            // Determine the new capacity: double it if not zero, otherwise start with 4.
            let new_capacity = if self.capacity == 0 {
                4
            } else {
                self.capacity * 2
            };

            // Allocate new memory for the array of grid pointers.
            // If `grids` is null (first allocation), use `alloc`; otherwise, use `realloc`.
            let new_ptr = if self.grids.is_null() {
                unsafe {
                    // Allocate memory for `new_capacity` number of `*mut NeoPDFGrid`.
                    std::alloc::alloc(
                        std::alloc::Layout::array::<*mut NeoPDFGrid>(new_capacity).unwrap(),
                    )
                    // TODO: Would using `libc` better? See commit `54b3044`.
                    .cast::<()>()
                    .cast::<*mut NeoPDFGrid>()
                }
            } else {
                unsafe {
                    // Reallocate existing memory to `new_capacity`.
                    // The old pointer is cast to `u8` for `realloc`.
                    std::alloc::realloc(
                        self.grids.cast::<u8>(),
                        std::alloc::Layout::array::<*mut NeoPDFGrid>(self.capacity).unwrap(),
                        new_capacity * std::mem::size_of::<*mut NeoPDFGrid>(),
                    )
                    // TODO: Would using `libc` better? See commit `54b3044`.
                    .cast::<()>()
                    .cast::<*mut NeoPDFGrid>()
                }
            };

            // Check if reallocation failed (returned null).
            if new_ptr.is_null() {
                return NeopdfResult::ErrorMemoryError;
            }

            // Update the collection's pointer and capacity to the new allocation.
            self.grids = new_ptr;
            self.capacity = new_capacity;
        }

        // Add the new grid pointer to the end of the array.
        unsafe {
            *self.grids.add(self.num_grids) = grid;
        }
        // Increment the count of grids in the collection.
        self.num_grids += 1;

        NeopdfResult::Success
    }

    /// Returns the number of grids currently in the collection.
    const fn len(&self) -> usize {
        self.num_grids
    }

    /// Retrieves a reference to the `NeoPDFGrid` at the specified index.
    ///
    /// # Arguments
    /// * `index` - The zero-based index of the grid to retrieve.
    ///
    /// # Returns
    /// An `Option<&NeoPDFGrid>`: `Some` if the index is valid, `None` otherwise.
    fn get(&self, index: usize) -> Option<&NeoPDFGrid> {
        // Check for out-of-bounds access.
        if index >= self.num_grids {
            return None;
        }
        // Dereference the raw pointer at the given index to get a reference to NeoPDFGrid.
        unsafe { (*self.grids.add(index)).as_ref() }
    }
}

impl Drop for NeoPDFGridArrayCollection {
    fn drop(&mut self) {
        if self.grids.is_null() {
            return;
        }
        // Free each individual grid in the collection
        let grids_slice = unsafe { slice::from_raw_parts(self.grids, self.num_grids) };
        for &grid_ptr in grids_slice {
            if !grid_ptr.is_null() {
                // Re-Box the raw pointer and let it drop, freeing the memory
                unsafe { drop(Box::from_raw(grid_ptr)) };
            }
        }

        // Deallocate the memory for the pointer array itself
        unsafe {
            std::alloc::dealloc(
                self.grids.cast::<u8>(),
                std::alloc::Layout::array::<*mut NeoPDFGrid>(self.capacity).unwrap(),
            );
        }
    }
}

/// Creates a new, empty `NeoPDFGridArrayCollection`.
///
/// # Safety
/// The caller is responsible for freeing the returned collection using
/// `neopdf_gridarray_collection_free` to prevent memory leaks.
#[no_mangle]
pub extern "C" fn neopdf_gridarray_collection_new() -> *mut NeoPDFGridArrayCollection {
    // Allocate a new `NeoPDFGridArrayCollection` on the heap and return a raw pointer to it.
    Box::into_raw(Box::new(NeoPDFGridArrayCollection::new()))
}

/// Adds a `NeoPDFGrid` to a `NeoPDFGridArrayCollection`.
///
/// # Safety
/// - `collection` must be a valid, non-null pointer to a `NeoPDFGridArrayCollection`.
/// - `grid` must be a valid, non-null pointer to a `NeoPDFGrid`.
/// - The `grid` pointer is taken ownership of by the collection; it should not be freed separately
///   until the collection itself is freed or the grid is removed from the collection.
#[no_mangle]
pub unsafe extern "C" fn neopdf_gridarray_collection_add_grid(
    collection: *mut NeoPDFGridArrayCollection,
    grid: *mut NeoPDFGrid,
) -> NeopdfResult {
    // Convert the raw `collection` pointer to a mutable reference.
    // This is unsafe because the pointer could be null or invalid.
    unsafe {
        collection
            .as_mut()
            .map_or(NeopdfResult::ErrorNullPointer, |collection| {
                // Call the safe `add_grid` method on the Rust struct.
                collection.add_grid(grid)
            })
    }
}

/// Frees the memory of a `NeoPDFGridArrayCollection` and all the grids it contains.
///
/// # Safety
/// - `collection` must be a valid, non-null pointer to a `NeoPDFGridArrayCollection`
///   that was previously created by `neopdf_gridarray_collection_new`.
/// - After this call, the `collection` pointer and all grids it contained become invalid
///   and should not be used.
#[no_mangle]
pub unsafe extern "C" fn neopdf_gridarray_collection_free(
    collection: *mut NeoPDFGridArrayCollection,
) {
    if !collection.is_null() {
        // This will call the `Drop` implementation for `NeoPDFGridArrayCollection`
        unsafe { drop(Box::from_raw(collection)) };
    }
}

/// Compresses a collection of `NeoPDFGrid` objects and writes them to a file.
///
/// This function iterates through the grids in the collection, converts them to `GridArray`s,
/// and then uses the `neopdf::writer::GridArrayCollection::compress` function to write them.
///
/// # Safety
/// - `collection` must be a valid, non-null pointer to a `NeoPDFGridArrayCollection`.
/// - `metadata` must be a valid, non-null pointer to a `NeoPDFMetaData` struct.
/// - `output_path` must be a valid, null-terminated C string representing the output file path.
#[no_mangle]
pub unsafe extern "C" fn neopdf_grid_compress(
    collection: *const NeoPDFGridArrayCollection,
    metadata: *const NeoPDFMetaData,
    output_path: *const c_char,
) -> NeopdfResult {
    // Perform null pointer checks for all input arguments.
    if collection.is_null() || metadata.is_null() || output_path.is_null() {
        return NeopdfResult::ErrorNullPointer;
    }

    // Convert the raw `collection` pointer to an immutable reference.
    // This is unsafe as the pointer could be invalid.
    let collection = unsafe { &*collection };

    // Process the C-style `metadata` struct into a Rust `MetaData` struct.
    // If processing fails (e.g., due to invalid C strings), return an error.
    let Some(meta) = process_metadata(metadata) else {
        return NeopdfResult::ErrorInvalidData;
    };

    // Convert the C-style `output_path` string to a Rust string slice.
    // Check for valid UTF-8 conversion.
    let out_path = unsafe { CStr::from_ptr(output_path).to_str() };
    let Ok(out_path) = out_path else {
        return NeopdfResult::ErrorInvalidData;
    };

    // Create a `Vec` to hold `GridArray` objects, pre-allocating capacity for efficiency.
    let mut grid_arrays = Vec::with_capacity(collection.len());

    // Iterate through each `NeoPDFGrid` pointer in the collection.
    for i in 0..collection.len() {
        // Retrieve a reference to the `NeoPDFGrid` at the current index.
        // If retrieval fails (e.g., invalid index), return an error.
        let Some(grid) = collection.get(i) else {
            return NeopdfResult::ErrorInvalidData;
        };

        // Create a `GridArray` from the `NeoPDFGrid`'s internal `subgrids` and `flavors`.
        // `clone()` is used here to create owned copies of the `Vec` data.
        let grid_array = GridArray::new(grid.subgrids.clone(), grid.flavors.clone());
        // Add the newly created `GridArray` to the `grid_arrays` vector.
        grid_arrays.push(grid_array);
    }

    // Create a vector of references to the `GridArray` objects.
    // This is required by the `compress` function's signature.
    let grid_refs: Vec<&GridArray> = grid_arrays.iter().collect();

    // Call the `compress` function from the `neopdf::writer` module.
    // Map the `Result` to `NeoPDFResult`.
    match GridArrayCollection::compress(&grid_refs, &meta, out_path) {
        Ok(()) => NeopdfResult::Success,
        Err(_) => NeopdfResult::ErrorMemoryError,
    }
}
