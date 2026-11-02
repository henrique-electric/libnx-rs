//! `switch/services/ectx.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

unsafe extern "C" {
    /// Initialize ectx:r.
    pub fn ectxrInitialize() -> Result;

    /// Exit ectx:r.
    pub fn ectxrExit();

    /// Gets the Service object for the actual ectx:r service session.
    pub fn ectxrGetServiceSession() -> *mut Service;

    /// Retrieves the error context associated with an error descriptor and result.
    ///
    /// # Arguments
    /// - `out0` (out): Output value.
    /// - `out_total_size` (out): Total error context size.
    /// - `out_size` (out): Error context size.
    /// - `dst` (out): Buffer for output error context.
    /// - `dst_size` (in): Buffer size for output error context.
    /// - `descriptor` (in): Error descriptor.
    /// - `result` (in): Error result.
    ///
    /// **Returns:** Result code.
    pub fn ectxrPullContext(out0: *mut s32, out_total_size: *mut u32, out_size: *mut u32, dst: *mut c_void, dst_size: usize, descriptor: u32, result: Result) -> Result;
}
