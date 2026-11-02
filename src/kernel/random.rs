//! `switch/kernel/random.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

unsafe extern "C" {
    /// Fills a buffer with random data.
    ///
    /// # Arguments
    /// - `buf`: Pointer to the buffer.
    /// - `len`: Size of the buffer in bytes.
    pub fn randomGet(buf: *mut c_void, len: usize);

    /// Returns a random 64-bit value.
    ///
    /// **Returns:** Random value.
    pub fn randomGet64() -> u64;
}
