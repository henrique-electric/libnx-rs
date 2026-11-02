//! `switch/services/fsldr.h`

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
pub struct FsCodeInfo {
    pub signature: [u8; 256],
    pub hash: [u8; 32],
    pub is_signed: bool,
    pub reserved: [u8; 3],
}

impl_zeroed_default!(FsCodeInfo);

unsafe extern "C" {
    /// Initialize fsp-ldr.
    pub fn fsldrInitialize() -> Result;

    /// Exit fsp-ldr.
    pub fn fsldrExit();

    /// Gets the Service object for the actual fsp-ldr service session.
    pub fn fsldrGetServiceSession() -> *mut Service;

    pub fn fsldrOpenCodeFileSystem(out_code_info: *mut FsCodeInfo, tid: u64, storage_id: NcmStorageId, path: *const c_char, attr: FsContentAttributes, out: *mut FsFileSystem) -> Result;

    pub fn fsldrIsArchivedProgram(pid: u64, out: *mut bool) -> Result;
}
