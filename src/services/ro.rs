//! `switch/services/ro.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

unsafe extern "C" {
    /// Initialize ldr:ro.
    pub fn ldrRoInitialize() -> Result;

    /// Exit ldr:ro.
    pub fn ldrRoExit();

    /// Gets the Service object for the actual ldr:ro service session.
    pub fn ldrRoGetServiceSession() -> *mut Service;

    /// Initialize ro:1. Only available with [7.0.0+].
    pub fn ro1Initialize() -> Result;

    /// Exit ro:1.
    pub fn ro1Exit();

    /// Gets the Service object for the actual ro:1 service session.
    pub fn ro1GetServiceSession() -> *mut Service;

    /// Initialize ro:dmnt. Only available with [3.0.0+].
    pub fn roDmntInitialize() -> Result;

    /// Exit ro:dmnt.
    pub fn roDmntExit();

    /// Gets the Service object for the actual ro:dmnt service session.
    pub fn roDmntGetServiceSession() -> *mut Service;

    pub fn ldrRoLoadNro(out_address: *mut u64, nro_address: u64, nro_size: u64, bss_address: u64, bss_size: u64) -> Result;

    pub fn ldrRoUnloadNro(nro_address: u64) -> Result;

    pub fn ldrRoLoadNrr(nrr_address: u64, nrr_size: u64) -> Result;

    pub fn ldrRoUnloadNrr(nrr_address: u64) -> Result;

    pub fn ldrRoLoadNrrEx(nrr_address: u64, nrr_size: u64) -> Result;

    pub fn ro1LoadNro(out_address: *mut u64, nro_address: u64, nro_size: u64, bss_address: u64, bss_size: u64) -> Result;

    pub fn ro1UnloadNro(nro_address: u64) -> Result;

    pub fn ro1LoadNrr(nrr_address: u64, nrr_size: u64) -> Result;

    pub fn ro1UnloadNrr(nrr_address: u64) -> Result;

    pub fn ro1LoadNrrEx(nrr_address: u64, nrr_size: u64) -> Result;

    pub fn roDmntGetProcessModuleInfo(pid: u64, out_module_infos: *mut LoaderModuleInfo, max_out_modules: usize, num_out: *mut s32) -> Result;
}
