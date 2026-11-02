//! `switch/services/spsm.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

unsafe extern "C" {
    /// Initialize spsm.
    pub fn spsmInitialize() -> Result;

    /// Exit spsm.
    pub fn spsmExit();

    /// Gets the Service object for the actual spsm service session.
    pub fn spsmGetServiceSession() -> *mut Service;

    pub fn spsmShutdown(reboot: bool) -> Result;

    pub fn spsmPutErrorState() -> Result;
}
