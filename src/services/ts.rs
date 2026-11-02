//! `switch/services/ts.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Location
pub type TsLocation = u32;
/// TMP451 Internal: PCB
pub const TsLocation_Internal: TsLocation = 0;
/// TMP451 External: SoC
pub const TsLocation_External: TsLocation = 1;

pub type TsDeviceCode = u32;
pub const TsDeviceCode_LocationInternal: TsDeviceCode = 0x41000001;
pub const TsDeviceCode_LocationExternal: TsDeviceCode = 0x41000002;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TsSession {
    pub s: Service,
}

impl_zeroed_default!(TsSession);

unsafe extern "C" {
    /// Initialize ts.
    pub fn tsInitialize() -> Result;

    /// Exit ts.
    pub fn tsExit();

    /// Gets the Service for ts.
    pub fn tsGetServiceSession() -> *mut Service;

    /// Gets the min/max temperature for the specified `TsLocation`.
    ///
    /// # Arguments
    /// - `location` (in): `TsLocation`
    /// - `min_temperature` (out): Output minimum temperature in Celsius.
    /// - `max_temperature` (out): Output maximum temperature in Celsius.
    pub fn tsGetTemperatureRange(location: TsLocation, min_temperature: *mut s32, max_temperature: *mut s32) -> Result;

    /// Gets the temperature for the specified `TsLocation`.
    ///
    /// # Arguments
    /// - `location` (in): `TsLocation`
    /// - `temperature` (out): Output temperature in Celsius.
    pub fn tsGetTemperature(location: TsLocation, temperature: *mut s32) -> Result;

    /// Gets the temperature for the specified `TsLocation`, in MilliC. [1.0.0-13.2.1]
    ///
    /// # Arguments
    /// - `location` (in): `TsLocation`
    /// - `temperature` (out): Output temperature in MilliC.
    pub fn tsGetTemperatureMilliC(location: TsLocation, temperature: *mut s32) -> Result;

    /// [8.0.0+]
    pub fn tsOpenSession(s: *mut TsSession, device_code: u32) -> Result;

    /// [10.0.0+]
    pub fn tsSessionGetTemperature(s: *mut TsSession, temperature: *mut f32) -> Result;

    pub fn tsSessionClose(s: *mut TsSession);
}
