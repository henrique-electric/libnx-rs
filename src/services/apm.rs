//! `switch/services/apm.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// PerformanceMode
pub type ApmPerformanceMode = i32;
/// Invalid
pub const ApmPerformanceMode_Invalid: ApmPerformanceMode = -1;
/// Normal
pub const ApmPerformanceMode_Normal: ApmPerformanceMode = 0;
/// Boost
pub const ApmPerformanceMode_Boost: ApmPerformanceMode = 1;

/// CpuBoostMode. With `appletSetCpuBoostMode`, only values 0/1 are available. This allows using higher clock rates.
pub type ApmCpuBoostMode = u32;
/// Default, boost-mode disabled.
pub const ApmCpuBoostMode_Normal: ApmCpuBoostMode = 0;
/// Boost CPU. Additionally, throttle GPU to minimum. Use performance configurations 0x92220009 (Docked) and 0x9222000A (Handheld), or 0x9222000B and 0x9222000C.
pub const ApmCpuBoostMode_FastLoad: ApmCpuBoostMode = 1;
/// Conserve power. Only throttle GPU to minimum. Use performance configurations 0x9222000B and 0x9222000C.
pub const ApmCpuBoostMode_Type2: ApmCpuBoostMode = 2;

unsafe extern "C" {
    /// Initialize apm. Used automatically by `appletInitialize` with AppletType_Application.
    pub fn apmInitialize() -> Result;

    /// Exit apm. Used automatically by `appletExit` with AppletType_Application.
    pub fn apmExit();

    /// Gets the Service object for the actual apm service session.
    pub fn apmGetServiceSession() -> *mut Service;

    /// Gets the Service object for ISession.
    pub fn apmGetServiceSession_Session() -> *mut Service;

    /// Gets the current ApmPerformanceMode.
    ///
    /// # Arguments
    /// - `out_performanceMode` (out): ApmPerformanceMode
    pub fn apmGetPerformanceMode(out_performanceMode: *mut ApmPerformanceMode) -> Result;

    /// Sets the PerformanceConfiguration for the specified PerformanceMode.
    ///
    /// # Arguments
    /// - `PerformanceMode` (in): `ApmPerformanceMode`
    /// - `PerformanceConfiguration` (in): PerformanceConfiguration
    pub fn apmSetPerformanceConfiguration(PerformanceMode: ApmPerformanceMode, PerformanceConfiguration: u32) -> Result;

    /// Gets the PerformanceConfiguration for the specified PerformanceMode.
    ///
    /// # Arguments
    /// - `PerformanceMode` (in): `ApmPerformanceMode`
    /// - `PerformanceConfiguration` (out): PerformanceConfiguration
    pub fn apmGetPerformanceConfiguration(PerformanceMode: ApmPerformanceMode, PerformanceConfiguration: *mut u32) -> Result;
}
