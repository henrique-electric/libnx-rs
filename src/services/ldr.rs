//! `switch/services/ldr.h`

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
pub struct LoaderProgramInfoV1 {
    pub main_thread_priority: u8,
    pub default_cpu_id: u8,
    pub application_type: u16,
    pub main_thread_stack_size: u32,
    pub program_id: u64,
    pub acid_sac_size: u32,
    pub aci0_sac_size: u32,
    pub acid_fac_size: u32,
    pub aci0_fah_size: u32,
    pub ac_buffer: [u8; 992],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LoaderProgramInfo {
    pub main_thread_priority: u8,
    pub default_cpu_id: u8,
    pub application_type: u16,
    pub main_thread_stack_size: u32,
    pub program_id: u64,
    pub acid_sac_size: u32,
    pub aci0_sac_size: u32,
    pub acid_fac_size: u32,
    pub aci0_fah_size: u32,
    pub unused_20: [u8; 16],
    pub ac_buffer: [u8; 992],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LoaderModuleInfo {
    pub build_id: [u8; 32],
    pub base_address: u64,
    pub size: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LoaderProgramAttributes {
    /// NcmContentMetaPlatform
    pub platform: u8,
    /// FsContentAttributes
    pub content_attributes: u8,
}

impl_zeroed_default!(LoaderProgramInfoV1, LoaderProgramInfo, LoaderModuleInfo, LoaderProgramAttributes);

unsafe extern "C" {
    /// Initialize ldr:shel.
    pub fn ldrShellInitialize() -> Result;

    /// Exit ldr:shel.
    pub fn ldrShellExit();

    /// Gets the Service object for the actual ldr:shel service session.
    pub fn ldrShellGetServiceSession() -> *mut Service;

    /// Initialize ldr:dmnt.
    pub fn ldrDmntInitialize() -> Result;

    /// Exit ldr:dmnt.
    pub fn ldrDmntExit();

    /// Gets the Service object for the actual ldr:dmnt service session.
    pub fn ldrDmntGetServiceSession() -> *mut Service;

    /// Initialize ldr:pm.
    pub fn ldrPmInitialize() -> Result;

    /// Exit ldr:pm.
    pub fn ldrPmExit();

    /// Gets the Service object for the actual ldr:pm service session.
    pub fn ldrPmGetServiceSession() -> *mut Service;

    pub fn ldrShellSetProgramArguments(program_id: u64, args: *const c_void, args_size: usize) -> Result;

    pub fn ldrShellFlushArguments() -> Result;

    pub fn ldrDmntSetProgramArguments(program_id: u64, args: *const c_void, args_size: usize) -> Result;

    pub fn ldrDmntFlushArguments() -> Result;

    pub fn ldrDmntGetProcessModuleInfo(pid: u64, out_module_infos: *mut LoaderModuleInfo, max_out_modules: usize, num_out: *mut s32) -> Result;

    pub fn ldrPmCreateProcess(pin_id: u64, flags: u32, reslimit_h: Handle, attrs: *const LoaderProgramAttributes, out_process_h: *mut Handle) -> Result;

    /// [19.0.0+/Atmosphere]
    pub fn ldrPmGetProgramInfo(loc: *const NcmProgramLocation, attrs: *const LoaderProgramAttributes, out_program_info: *mut LoaderProgramInfo) -> Result;

    /// [1.0.0-18.1.0/Non-Atmosphere]
    pub fn ldrPmGetProgramInfoV1(loc: *const NcmProgramLocation, out_program_info: *mut LoaderProgramInfoV1) -> Result;

    pub fn ldrPmPinProgram(loc: *const NcmProgramLocation, out_pin_id: *mut u64) -> Result;

    pub fn ldrPmUnpinProgram(pin_id: u64) -> Result;

    /// [10.0.0+]
    pub fn ldrPmSetEnabledProgramVerification(enabled: bool) -> Result;
}
