#![feature(fnbox)]

extern crate harfbuzz_sys as ffi;
extern crate libc;
//#[macro_use]
extern crate glib;

pub mod blob;

use libc::c_void;
use std::boxed::FnBox;

unsafe extern "C" fn closure_trampoline(ptr: *mut c_void) {
    let func: Box<Box<FnBox() + 'static>> = Box::from_raw(ptr as *mut _);
    (*func)();
}

fn closure_into_raw<F: FnOnce() + 'static>(func: F) -> *mut c_void {
    let func: Box<Box<FnBox() + 'static>> = Box::new(Box::new(func));
    Box::into_raw(func) as *mut c_void
}
