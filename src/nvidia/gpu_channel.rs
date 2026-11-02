//! `switch/nvidia/gpu_channel.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const GPFIFO_QUEUE_SIZE: u32 = 0x800;
pub const GPFIFO_ENTRY_NOT_MAIN: u32 = 512;
pub const GPFIFO_ENTRY_NO_PREFETCH: u32 = 0x80000000;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NvGpuChannel {
    pub base: NvChannel,
    pub error_event: Event,
    pub object_id: u64,
    pub fence: NvFence,
    pub fence_incr: u32,
    pub entries: [nvioctl_gpfifo_entry; 2048],
    pub num_entries: u32,
}

#[inline(always)]
pub fn nvGpuChannelGetSyncpointId(c: &NvGpuChannel) -> u32 {
    c.fence.id
}

#[inline(always)]
pub fn nvGpuChannelGetFence(c: &NvGpuChannel, fence_out: &mut NvFence) {
    fence_out.id = c.fence.id;
    fence_out.value = c.fence.value.wrapping_add(c.fence_incr);
}

#[inline(always)]
pub fn nvGpuChannelIncrFence(c: &mut NvGpuChannel) {
    c.fence_incr = c.fence_incr.wrapping_add(1);
}

impl_zeroed_default!(NvGpuChannel);

unsafe extern "C" {
    pub fn nvGpuChannelCreate(c: *mut NvGpuChannel, as_: *mut NvAddressSpace, prio: NvChannelPriority) -> Result;

    pub fn nvGpuChannelClose(c: *mut NvGpuChannel);

    pub fn nvGpuChannelZcullBind(c: *mut NvGpuChannel, iova: iova_t) -> Result;

    pub fn nvGpuChannelAppendEntry(c: *mut NvGpuChannel, start: iova_t, num_cmds: usize, flags: u32, flush_threshold: u32) -> Result;

    pub fn nvGpuChannelKickoff(c: *mut NvGpuChannel) -> Result;

    pub fn nvGpuChannelGetErrorNotification(c: *mut NvGpuChannel, notif: *mut NvNotification) -> Result;

    pub fn nvGpuChannelGetErrorInfo(c: *mut NvGpuChannel, error: *mut NvError) -> Result;
}
