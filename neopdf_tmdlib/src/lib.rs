#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::too_many_arguments)]
#![allow(missing_docs)]

use cxx::UniquePtr;

#[cxx::bridge]
pub mod ffi {
    #[namespace = "TMDlib"]
    unsafe extern "C++" {
        include!("tmdlib/TMDlib.h");
        type TMD;
    }

    unsafe extern "C++" {
        include!("neopdf_tmdlib/src/tmdlib.hpp");

        fn make_tmd() -> UniquePtr<TMD>;

        fn tmd_init(tmd: Pin<&mut TMD>, setname: &str, member: i32);
        fn tmd_init_set(tmd: Pin<&mut TMD>, setname: &str);
        fn tmd_get_num_members(tmd: Pin<&mut TMD>) -> usize;
        fn tmd_get_xmin(tmd: Pin<&mut TMD>) -> f64;
        fn tmd_get_xmax(tmd: Pin<&mut TMD>) -> f64;
        fn tmd_get_q2min(tmd: Pin<&mut TMD>) -> f64;
        fn tmd_get_q2max(tmd: Pin<&mut TMD>) -> f64;
        fn tmd_get_ktmin(tmd: Pin<&mut TMD>) -> f64;
        fn tmd_get_ktmax(tmd: Pin<&mut TMD>) -> f64;
        fn tmd_pdf(tmd: Pin<&mut TMD>, x: f64, kt: f64, q: f64) -> Vec<f64>;
        fn tmd_set_verbosity(tmd: Pin<&mut TMD>, verbosity: i32);
    }
}

pub struct Tmd {
    ptr: UniquePtr<ffi::TMD>,
}

impl Tmd {
    pub fn new() -> Self {
        Self {
            ptr: ffi::make_tmd(),
        }
    }

    pub fn init(&mut self, setname: &str, member: i32) {
        ffi::tmd_init(self.ptr.pin_mut(), setname, member);
    }

    pub fn init_set(&mut self, setname: &str) {
        ffi::tmd_init_set(self.ptr.pin_mut(), setname);
    }

    pub fn num_members(&mut self) -> usize {
        ffi::tmd_get_num_members(self.ptr.pin_mut())
    }

    pub fn x_min(&mut self) -> f64 {
        ffi::tmd_get_xmin(self.ptr.pin_mut())
    }

    pub fn x_max(&mut self) -> f64 {
        ffi::tmd_get_xmax(self.ptr.pin_mut())
    }

    pub fn q2_min(&mut self) -> f64 {
        ffi::tmd_get_q2min(self.ptr.pin_mut())
    }

    pub fn q2_max(&mut self) -> f64 {
        ffi::tmd_get_q2max(self.ptr.pin_mut())
    }

    pub fn kt_min(&mut self) -> f64 {
        ffi::tmd_get_ktmin(self.ptr.pin_mut())
    }

    pub fn kt_max(&mut self) -> f64 {
        ffi::tmd_get_ktmax(self.ptr.pin_mut())
    }

    pub fn xfxq2kt(&mut self, x: f64, kt: f64, q: f64) -> Vec<f64> {
        ffi::tmd_pdf(self.ptr.pin_mut(), x, kt, q)
    }

    pub fn set_verbosity(&mut self, verbosity: i32) {
        ffi::tmd_set_verbosity(self.ptr.pin_mut(), verbosity);
    }
}

impl Default for Tmd {
    fn default() -> Self {
        Self::new()
    }
}
