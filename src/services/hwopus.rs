//! `switch/services/hwopus.h`

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
pub struct HwopusDecoder {
    pub s: Service,
    pub tmem: TransferMemory,
    pub multistream: bool,
}

/// This structure is the start of opusin for `hwopusDecodeInterleaved`, with the actual opus packet following this.
/// These fields are big-endian.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HwopusHeader {
    /// Size of the packet following this header.
    pub size: u32,
    /// Indicates the final range of the codec encoder's entropy coder. This can be left at zero.
    pub final_range: u32,
}

/// Used internally.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HwopusMultistreamState {
    pub SampleRate: s32,
    pub ChannelCount: s32,
    pub TotalStreamCount: s32,
    pub StereoStreamCount: s32,
    pub channel_mapping: [u8; 256],
}

impl_zeroed_default!(HwopusDecoder, HwopusHeader, HwopusMultistreamState);

unsafe extern "C" {
    pub fn hwopusDecoderInitialize(decoder: *mut HwopusDecoder, SampleRate: s32, ChannelCount: s32) -> Result;

    pub fn hwopusDecoderExit(decoder: *mut HwopusDecoder);

    /// Only available on [3.0.0+].
    /// See libopus multistream docs.
    pub fn hwopusDecoderMultistreamInitialize(decoder: *mut HwopusDecoder, SampleRate: s32, ChannelCount: s32, TotalStreamCount: s32, StereoStreamCount: s32, channel_mapping: *mut u8) -> Result;

    /// Decodes opus data.
    pub fn hwopusDecodeInterleaved(decoder: *mut HwopusDecoder, DecodedDataSize: *mut s32, DecodedSampleCount: *mut s32, opusin: *const c_void, opusin_size: usize, pcmbuf: *mut s16, pcmbuf_size: usize) -> Result;
}
