//! `switch/services/wlaninf.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// WLAN State.
pub type WlanInfState = u32;
/// WLAN is disabled or enabled and not connected.
pub const WlanInfState_NotConnected: WlanInfState = 1;
/// WLAN is connecting.
pub const WlanInfState_Connecting: WlanInfState = 2;
/// WLAN is connected.
pub const WlanInfState_Connected: WlanInfState = 3;

unsafe extern "C" {
    /// [1.0.0-14.1.2] Initialize wlan:inf.
    pub fn wlaninfInitialize() -> Result;

    /// Exit wlan:inf.
    pub fn wlaninfExit();

    /// Gets the Service object for the actual wlan:inf service session.
    pub fn wlaninfGetServiceSession() -> *mut Service;

    /// Gets `WlanInfState`.
    pub fn wlaninfGetState(out: *mut WlanInfState) -> Result;

    /// Value goes from -30 (really good signal) to -90 (barely enough to stay connected)
    /// on a logarithmic scale
    pub fn wlaninfGetRSSI(out: *mut s32) -> Result;
}
