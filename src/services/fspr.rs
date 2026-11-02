//! `switch/services/fspr.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

unsafe extern "C" {
    /// Initialize fsp-pr.
    pub fn fsprInitialize() -> Result;

    /// Exit fsp-pr.
    pub fn fsprExit();

    /// Gets the Service object for the actual fsp-pr service session.
    pub fn fsprGetServiceSession() -> *mut Service;

    pub fn fsprRegisterProgram(pid: u64, tid: u64, sid: NcmStorageId, fs_access_header: *const c_void, fah_size: usize, fs_access_control: *const c_void, fac_size: usize, fs_access_control_restriction_mode: u8) -> Result;

    pub fn fsprUnregisterProgram(pid: u64) -> Result;

    pub fn fsprSetCurrentProcess() -> Result;

    pub fn fsprSetEnabledProgramVerification(enabled: bool) -> Result;
}
