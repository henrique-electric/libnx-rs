//! `switch/nvidia/address_space.h`

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
pub struct NvAddressSpace {
    pub fd: u32,
    pub page_size: u32,
    pub has_init: bool,
}

impl_zeroed_default!(NvAddressSpace);

unsafe extern "C" {
    pub fn nvAddressSpaceCreate(a: *mut NvAddressSpace, page_size: u32) -> Result;

    pub fn nvAddressSpaceClose(a: *mut NvAddressSpace);

    pub fn nvAddressSpaceAlloc(a: *mut NvAddressSpace, sparse: bool, size: u64, iova_out: *mut iova_t) -> Result;

    pub fn nvAddressSpaceAllocFixed(a: *mut NvAddressSpace, sparse: bool, size: u64, iova: iova_t) -> Result;

    pub fn nvAddressSpaceFree(a: *mut NvAddressSpace, iova: iova_t, size: u64) -> Result;

    pub fn nvAddressSpaceMap(a: *mut NvAddressSpace, nvmap_handle: u32, is_gpu_cacheable: bool, kind: NvKind, iova_out: *mut iova_t) -> Result;

    pub fn nvAddressSpaceMapFixed(a: *mut NvAddressSpace, nvmap_handle: u32, is_gpu_cacheable: bool, kind: NvKind, iova: iova_t) -> Result;

    pub fn nvAddressSpaceModify(a: *mut NvAddressSpace, iova: iova_t, offset: u64, size: u64, kind: NvKind) -> Result;

    pub fn nvAddressSpaceUnmap(a: *mut NvAddressSpace, iova: iova_t) -> Result;
}
