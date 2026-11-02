//! `switch/nvidia/fence.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub type NvFence = nvioctl_fence;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NvMultiFence {
    pub num_fences: u32,
    pub fences: [NvFence; 4],
}

#[inline(always)]
pub fn nvMultiFenceCreate(mf: &mut NvMultiFence, fence: &NvFence) {
    mf.num_fences = 1;
    mf.fences[0] = *fence;
}

impl_zeroed_default!(NvMultiFence);

unsafe extern "C" {
    pub fn nvFenceInit() -> Result;

    pub fn nvFenceExit();

    pub fn nvFenceGetFd() -> u32;

    pub fn nvFenceWait(f: *mut NvFence, timeout_us: s32) -> Result;

    pub fn nvMultiFenceWait(mf: *mut NvMultiFence, timeout_us: s32) -> Result;
}
