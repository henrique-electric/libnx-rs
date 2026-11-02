//! `switch/nvidia/map.h`

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
pub struct NvMap {
    pub handle: u32,
    pub id: u32,
    pub size: u32,
    pub cpu_addr: *mut c_void,
    pub kind: NvKind,
    pub has_init: bool,
    pub is_cpu_cacheable: bool,
}

#[inline(always)]
pub fn nvMapGetHandle(m: &NvMap) -> u32 {
    m.handle
}

#[inline(always)]
pub fn nvMapGetId(m: &NvMap) -> u32 {
    m.id
}

#[inline(always)]
pub fn nvMapGetSize(m: &NvMap) -> u32 {
    m.size
}

#[inline(always)]
pub fn nvMapGetCpuAddr(m: &NvMap) -> *mut c_void {
    m.cpu_addr
}

#[inline(always)]
pub fn nvMapIsRemote(m: &NvMap) -> bool {
    m.cpu_addr.is_null()
}

#[inline(always)]
pub fn nvMapGetKind(m: &NvMap) -> NvKind {
    m.kind
}

impl_zeroed_default!(NvMap);

unsafe extern "C" {
    pub fn nvMapInit() -> Result;

    pub fn nvMapGetFd() -> u32;

    pub fn nvMapExit();

    pub fn nvMapCreate(m: *mut NvMap, cpu_addr: *mut c_void, size: u32, align: u32, kind: NvKind, is_cpu_cacheable: bool) -> Result;

    pub fn nvMapLoadRemote(m: *mut NvMap, id: u32) -> Result;

    pub fn nvMapClose(m: *mut NvMap);
}
