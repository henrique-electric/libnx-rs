//! `switch/runtime/hosversion.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Returns true if the current HOS version is equal to or above the specified major/minor/micro version.
#[inline(always)]
pub fn hosversionAtLeast(major: u8, minor: u8, micro: u8) -> bool {
    // SAFETY: reads a value cached by the libnx runtime.
    unsafe { hosversionGet() >= MAKEHOSVERSION(major as u32, minor as u32, micro as u32) }
}

/// Returns true if the current HOS version is earlier than the specified major/minor/micro version.
#[inline(always)]
pub fn hosversionBefore(major: u8, minor: u8, micro: u8) -> bool {
    !hosversionAtLeast(major, minor, micro)
}

/// Returns true if the current HOS version is between the two specified major versions, i.e. [major1, major2).
#[inline(always)]
pub fn hosversionBetween(major1: u8, major2: u8) -> bool {
    // SAFETY: reads a value cached by the libnx runtime.
    let ver = unsafe { hosversionGet() };
    ver >= MAKEHOSVERSION(major1 as u32, 0, 0) && ver < MAKEHOSVERSION(major2 as u32, 0, 0)
}

/// Builds a HOS version value.
#[inline(always)]
pub const fn MAKEHOSVERSION(major: u32, minor: u32, micro: u32) -> u32 {
    (major << 16) | (minor << 8) | micro
}

/// Extracts the major number from a HOS version value.
#[inline(always)]
pub const fn HOSVER_MAJOR(version: u32) -> u32 {
    (version >> 16) & 0xFF
}

/// Extracts the minor number from a HOS version value.
#[inline(always)]
pub const fn HOSVER_MINOR(version: u32) -> u32 {
    (version >> 8) & 0xFF
}

/// Extracts the micro number from a HOS version value.
#[inline(always)]
pub const fn HOSVER_MICRO(version: u32) -> u32 {
    version & 0xFF
}

unsafe extern "C" {
    /// Returns the current HOS version that was previously set with `hosversionSet`. If version initialization fails during startup (such as in the case set:sys is not available), this function returns zero.
    pub fn hosversionGet() -> u32;

    /// Sets or overrides the current HOS version. This function is normally called automatically by libnx on startup with the version info obtained with `setsysGetFirmwareVersion`.
    pub fn hosversionSet(version: u32);

    /// Returns whether the current HOS version is augmented by running the Atmosphère custom firmware.
    pub fn hosversionIsAtmosphere() -> bool;
}
