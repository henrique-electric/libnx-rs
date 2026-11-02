//! `switch/services/smm.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

unsafe extern "C" {
    /// Initialize sm:m.
    pub fn smManagerInitialize() -> Result;

    /// Exit sm:m.
    pub fn smManagerExit();

    pub fn smManagerRegisterProcess(pid: u64, acid_sac: *const c_void, acid_sac_size: usize, aci0_sac: *const c_void, aci0_sac_size: usize) -> Result;

    pub fn smManagerUnregisterProcess(pid: u64) -> Result;

    /// Initialize sm:m exclusively for tipc (requires &lt;12.0.0 and non-Atmosphere).
    pub fn smManagerCmifInitialize() -> Result;

    /// Exit sm:m exclusively for tipc (requires &lt;12.0.0 and non-Atmosphere).
    pub fn smManagerCmifExit();

    /// Gets the Service object for the actual sm:m service session (requires &lt;12.0.0 and non-Atmosphere).
    pub fn smManagerCmifGetServiceSession() -> *mut Service;

    pub fn smManagerCmifRegisterProcess(pid: u64, acid_sac: *const c_void, acid_sac_size: usize, aci0_sac: *const c_void, aci0_sac_size: usize) -> Result;

    pub fn smManagerCmifUnregisterProcess(pid: u64) -> Result;

    /// Initialize sm:m exclusively for tipc (requires 12.0.0+ or Atmosphere).
    pub fn smManagerTipcInitialize() -> Result;

    /// Exit sm:m exclusively for tipc (requires 12.0.0+ or Atmosphere).
    pub fn smManagerTipcExit();

    /// Gets the TipcService object for the actual sm:m service session (requires 12.0.0+ or Atmosphere).
    pub fn smManagerTipcGetServiceSession() -> *mut TipcService;

    pub fn smManagerTipcRegisterProcess(pid: u64, acid_sac: *const c_void, acid_sac_size: usize, aci0_sac: *const c_void, aci0_sac_size: usize) -> Result;

    pub fn smManagerTipcUnregisterProcess(pid: u64) -> Result;
}
