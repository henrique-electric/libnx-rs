//! `switch/services/audrec.h`

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
pub struct FinalOutputRecorderBuffer {
    pub released_ns: u64,
    pub next_buffer_ptr: u64,
    pub sample_buffer_ptr: u64,
    pub sample_buffer_capacity: u64,
    pub data_size: u64,
    pub data_offset: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FinalOutputRecorderParameter {
    pub sample_rate: u32,
    pub channel_count: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FinalOutputRecorderParameterInternal {
    pub sample_rate: u32,
    pub channel_count: u32,
    pub sample_format: u32,
    pub state: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudrecRecorder {
    pub s: Service,
}

impl_zeroed_default!(FinalOutputRecorderBuffer, FinalOutputRecorderParameter, FinalOutputRecorderParameterInternal, AudrecRecorder);

unsafe extern "C" {
    pub fn audrecInitialize() -> Result;

    pub fn audrecExit();

    pub fn audrecGetServiceSession() -> *mut Service;

    pub fn audrecOpenFinalOutputRecorder(recorder_out: *mut AudrecRecorder, param_in: *mut FinalOutputRecorderParameter, aruid: u64, param_out: *mut FinalOutputRecorderParameterInternal) -> Result;

    pub fn audrecRecorderStart(recorder: *mut AudrecRecorder) -> Result;

    pub fn audrecRecorderStop(recorder: *mut AudrecRecorder) -> Result;

    pub fn audrecRecorderRegisterBufferEvent(recorder: *mut AudrecRecorder, out_event: *mut Event) -> Result;

    pub fn audrecRecorderAppendFinalOutputRecorderBuffer(recorder: *mut AudrecRecorder, buffer_client_ptr: u64, param: *mut FinalOutputRecorderBuffer) -> Result;

    pub fn audrecRecorderGetReleasedFinalOutputRecorderBuffers(recorder: *mut AudrecRecorder, out_buffers: *mut u64, inout_count: *mut u64, out_released: *mut u64) -> Result;

    pub fn audrecRecorderClose(recorder: *mut AudrecRecorder);
}
