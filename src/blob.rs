use glib::translate::*;
use ffi;
use super::{closure_into_raw, closure_trampoline};

use std::marker::PhantomData;

pub struct Blob<'a>(*mut ffi::hb_blob_t, PhantomData<&'a [u8]>);

impl<'a> ToGlibPtr<'a, *mut ffi::hb_blob_t> for &'a Blob<'a> {
    type Storage = &'a Blob<'a>;

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, *mut ffi::hb_blob_t, &'a Blob<'a>> {
        Stash(self.0, *self)
    }
}

impl<'a> FromGlibPtrNone<*mut ffi::hb_blob_t> for Blob<'a> {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::hb_blob_t) -> Blob<'a> {
        assert!(!ptr.is_null());
        ffi::hb_blob_reference(ptr);
        Blob(ptr, PhantomData)
    }
}

impl<'a> FromGlibPtrFull<*mut ffi::hb_blob_t> for Blob<'a> {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::hb_blob_t) -> Blob<'a> {
        assert!(!ptr.is_null());
        Blob(ptr, PhantomData)
    }
}

impl<'a> AsRef<Blob<'a>> for Blob<'a> {
    fn as_ref(&self) -> &Blob<'a> {
        self
    }
}

impl<'a> Clone for Blob<'a> {
    fn clone(&self) -> Blob<'a> {
        unsafe { from_glib_none(self.to_glib_none().0) }
    }
}

impl<'a> Drop for Blob<'a> {
    fn drop(&mut self) {
        unsafe { ffi::hb_blob_destroy(self.0); }
    }
}

impl<'a> Blob<'a> {
    pub fn create<F>(data: &'a [u8], destroy: F) -> Blob<'a>
        where F: FnOnce() + 'static,
    {
        let destroy = closure_into_raw(destroy);
        unsafe {
            from_glib_full(ffi::hb_blob_create(data.as_ptr() as *const i8,
                                               data.len() as u32,
                                               ffi::hb_memory_mode_t::Readonly,
                                               destroy, Some(closure_trampoline)))
        }
    }
}
