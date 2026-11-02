//! `switch/services/mm.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub type MmuModuleId = u32;
pub const MmuModuleId_Ram: MmuModuleId = 2;
pub const MmuModuleId_Nvenc: MmuModuleId = 5;
pub const MmuModuleId_Nvdec: MmuModuleId = 6;
pub const MmuModuleId_Nvjpg: MmuModuleId = 7;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MmuRequest {
    pub module: MmuModuleId,
    pub id: u32,
}

impl_zeroed_default!(MmuRequest);

unsafe extern "C" {
    pub fn mmuInitialize() -> Result;

    pub fn mmuExit();

    pub fn mmuGetServiceSession() -> *mut Service;

    /// unk is ignored by official software
    pub fn mmuRequestInitialize(request: *mut MmuRequest, module: MmuModuleId, unk: u32, autoclear: bool) -> Result;

    pub fn mmuRequestFinalize(request: *const MmuRequest) -> Result;

    pub fn mmuRequestGet(request: *const MmuRequest, out_freq_hz: *mut u32) -> Result;

    pub fn mmuRequestSetAndWait(request: *const MmuRequest, freq_hz: u32, timeout: s32) -> Result;
}
