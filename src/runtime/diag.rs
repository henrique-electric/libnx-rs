//! `switch/runtime/diag.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

unsafe extern "C" {
    /// Aborts program execution with a result code.
    ///
    /// # Arguments
    /// - `res` (in): Result code.
    pub fn diagAbortWithResult(res: Result) -> !;
}
