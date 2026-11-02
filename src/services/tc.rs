//! `switch/services/tc.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

unsafe extern "C" {
    /// Initialize tc.
    pub fn tcInitialize() -> Result;

    /// Exit tc.
    pub fn tcExit();

    /// Gets the Service for tc.
    pub fn tcGetServiceSession() -> *mut Service;

    pub fn tcEnableFanControl() -> Result;

    /// **Warning:** Disabling your fan can damage your system.
    pub fn tcDisableFanControl() -> Result;

    pub fn tcIsFanControlEnabled(status: *mut bool) -> Result;

    /// Only available on [5.0.0+].
    pub fn tcGetSkinTemperatureMilliC(skinTemp: *mut s32) -> Result;
}
