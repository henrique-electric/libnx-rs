//! `switch/display/buffer_producer.h`

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
pub struct BqRect {
    pub left: s32,
    pub top: s32,
    pub right: s32,
    pub bottom: s32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BqBufferInput_anon0 {
    pub timestamp: s64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BqBufferInput {
    pub anon0: BqBufferInput_anon0,
    pub isAutoTimestamp: s32,
    pub crop: BqRect,
    pub scalingMode: s32,
    pub transform: u32,
    pub stickyTransform: u32,
    pub unk: u32,
    pub swapInterval: u32,
    pub fence: NvMultiFence,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BqBufferOutput {
    pub width: u32,
    pub height: u32,
    pub transformHint: u32,
    pub numPendingBuffers: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BqGraphicBuffer {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: u32,
    pub usage: u32,
    pub native_handle: *mut NativeHandle,
}

impl_zeroed_default!(BqRect, BqBufferInput_anon0, BqBufferInput, BqBufferOutput, BqGraphicBuffer);

unsafe extern "C" {
    pub fn bqRequestBuffer(b: *mut Binder, bufferIdx: s32, buf: *mut BqGraphicBuffer) -> Result;

    pub fn bqDequeueBuffer(b: *mut Binder, async_: bool, width: u32, height: u32, format: s32, usage: u32, buf: *mut s32, fence: *mut NvMultiFence) -> Result;

    pub fn bqDetachBuffer(b: *mut Binder, slot: s32) -> Result;

    pub fn bqQueueBuffer(b: *mut Binder, buf: s32, input: *const BqBufferInput, output: *mut BqBufferOutput) -> Result;

    pub fn bqCancelBuffer(b: *mut Binder, buf: s32, fence: *const NvMultiFence) -> Result;

    pub fn bqQuery(b: *mut Binder, what: s32, value: *mut s32) -> Result;

    pub fn bqConnect(b: *mut Binder, api: s32, producerControlledByApp: bool, output: *mut BqBufferOutput) -> Result;

    pub fn bqDisconnect(b: *mut Binder, api: s32) -> Result;

    pub fn bqSetPreallocatedBuffer(b: *mut Binder, buf: s32, input: *const BqGraphicBuffer) -> Result;
}
