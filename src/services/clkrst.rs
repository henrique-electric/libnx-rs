//! `switch/services/clkrst.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ClkrstSession {
    pub s: Service,
}

impl_zeroed_default!(ClkrstSession);

unsafe extern "C" {
    /// Initialize clkrst. Only available on [8.0.0+].
    pub fn clkrstInitialize() -> Result;

    /// Exit clkrst.
    pub fn clkrstExit();

    /// Gets the Service object for the actual clkrst service session.
    pub fn clkrstGetServiceSession() -> *mut Service;

    /// Opens a ClkrstSession for the requested PcvModuleId, unk is set to 3 in official sysmodules.
    pub fn clkrstOpenSession(session_out: *mut ClkrstSession, module_id: PcvModuleId, unk: u32) -> Result;

    pub fn clkrstCloseSession(session: *mut ClkrstSession);

    pub fn clkrstSetClockRate(session: *mut ClkrstSession, hz: u32) -> Result;

    pub fn clkrstGetClockRate(session: *mut ClkrstSession, out_hz: *mut u32) -> Result;

    pub fn clkrstGetPossibleClockRates(session: *mut ClkrstSession, rates: *mut u32, max_count: s32, out_type: *mut PcvClockRatesListType, out_count: *mut s32) -> Result;
}
