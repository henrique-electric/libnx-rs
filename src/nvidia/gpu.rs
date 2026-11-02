//! `switch/nvidia/gpu.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

unsafe extern "C" {
    pub fn nvGpuInit() -> Result;

    pub fn nvGpuExit();

    pub fn nvGpuGetCharacteristics() -> *const nvioctl_gpu_characteristics;

    pub fn nvGpuGetZcullCtxSize() -> u32;

    pub fn nvGpuGetZcullInfo() -> *const nvioctl_zcull_info;

    pub fn nvGpuGetTpcMasks(num_masks_out: *mut u32) -> *const u32;

    pub fn nvGpuZbcGetActiveSlotMask(out_slot: *mut u32, out_mask: *mut u32) -> Result;

    pub fn nvGpuZbcAddColor(color_l2: *const u32, color_ds: *const u32, format: u32) -> Result;

    pub fn nvGpuZbcAddDepth(depth: f32) -> Result;

    pub fn nvGpuGetTimestamp(ts: *mut u64) -> Result;
}
