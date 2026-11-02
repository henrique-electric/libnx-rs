//! `switch/services/csrng.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

unsafe extern "C" {
    /// Initialize csrng.
    pub fn csrngInitialize() -> Result;

    /// Exit csrng.
    pub fn csrngExit();

    /// Gets the Service object for the actual csrng service session.
    pub fn csrngGetServiceSession() -> *mut Service;

    pub fn csrngGetRandomBytes(out: *mut c_void, out_size: usize) -> Result;
}
