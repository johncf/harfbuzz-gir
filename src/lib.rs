#![feature(fnbox)]

extern crate harfbuzz_sys as ffi;
extern crate libc;
//#[macro_use]
extern crate glib;

pub mod blob;

use libc::c_void;
use std::boxed::FnBox;

pub struct DestroyFunc(*mut c_void, unsafe extern "C" fn(*mut c_void));

impl DestroyFunc {
    pub fn from_closure<F: FnOnce() + 'static>(func: F) -> DestroyFunc {
        let func: Box<Box<FnBox() + 'static>> = Box::new(Box::new(func));
        DestroyFunc(Box::into_raw(func) as *mut c_void, destroy_trampoline)
    }

    /// This is not marked unsafe since the unsafety depends solely on `f`.
    pub fn from_raw(data: *mut c_void, f: unsafe extern "C" fn(*mut c_void)) -> DestroyFunc {
        DestroyFunc(data, f)
    }

    fn maybe_raw(maybe_this: Option<DestroyFunc>) -> (*mut c_void, ffi::hb_destroy_func_t) {
        maybe_this.map_or((std::ptr::null_mut(), None), |df| (df.0, Some(df.1)))
    }
}

unsafe extern "C" fn destroy_trampoline(ptr: *mut c_void) {
    let func: Box<Box<FnBox() + 'static>> = Box::from_raw(ptr as *mut _);
    (*func)();
}
