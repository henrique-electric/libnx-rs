//! `switch/services/bpc.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub type BpcSleepButtonState = u32;
pub const BpcSleepButtonState_Held: BpcSleepButtonState = 0;
pub const BpcSleepButtonState_Released: BpcSleepButtonState = 1;

unsafe extern "C" {
    /// Initialize bpc.
    pub fn bpcInitialize() -> Result;

    /// Exit bpc.
    pub fn bpcExit();

    /// Gets the Service object for the actual bpc service session.
    pub fn bpcGetServiceSession() -> *mut Service;

    pub fn bpcShutdownSystem() -> Result;

    pub fn bpcRebootSystem() -> Result;

    /// [2.0.0-13.2.1]
    pub fn bpcGetSleepButtonState(out: *mut BpcSleepButtonState) -> Result;

    /// [6.0.0+]
    pub fn bpcGetPowerButton(out_is_pushed: *mut bool) -> Result;
}
