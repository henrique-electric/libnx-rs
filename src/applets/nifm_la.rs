//! `switch/applets/nifm_la.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

unsafe extern "C" {
    /// Uses `nifmGetResult`, then on failure launches the applet.
    ///
    /// # Arguments
    /// - `r`: `NifmRequest`
    pub fn nifmLaHandleNetworkRequestResult(r: *mut NifmRequest) -> Result;
}
