//! `switch/services/audin.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub type AudioInState = u32;
pub const AudioInState_Started: AudioInState = 0;
pub const AudioInState_Stopped: AudioInState = 1;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioInBuffer {
    /// Next buffer. (Unused)
    pub next: *mut AudioInBuffer,
    /// Sample buffer (aligned to 0x1000 bytes).
    pub buffer: *mut c_void,
    /// Sample buffer size (aligned to 0x1000 bytes).
    pub buffer_size: u64,
    /// Size of data inside the buffer.
    pub data_size: u64,
    /// Offset of data inside the buffer. (Unused?)
    pub data_offset: u64,
}

impl_zeroed_default!(AudioInBuffer);

unsafe extern "C" {
    /// Initialize audin.
    pub fn audinInitialize() -> Result;

    /// Exit audin.
    pub fn audinExit();

    /// Gets the Service object for the actual audin service session.
    pub fn audinGetServiceSession() -> *mut Service;

    /// Gets the Service object for IAudioIn.
    pub fn audinGetServiceSession_AudioIn() -> *mut Service;

    pub fn audinListAudioIns(DeviceNames: *mut c_char, count: s32, DeviceNamesCount: *mut u32) -> Result;

    pub fn audinOpenAudioIn(DeviceNameIn: *const c_char, DeviceNameOut: *mut c_char, SampleRateIn: u32, ChannelCountIn: u32, SampleRateOut: *mut u32, ChannelCountOut: *mut u32, Format: *mut PcmFormat, State: *mut AudioInState) -> Result;

    pub fn audinGetAudioInState(State: *mut AudioInState) -> Result;

    pub fn audinStartAudioIn() -> Result;

    pub fn audinStopAudioIn() -> Result;

    /// Submits an `AudioInBuffer` for capturing.
    pub fn audinAppendAudioInBuffer(Buffer: *mut AudioInBuffer) -> Result;

    pub fn audinGetReleasedAudioInBuffer(Buffer: *mut *mut AudioInBuffer, ReleasedBuffersCount: *mut u32) -> Result;

    pub fn audinContainsAudioInBuffer(Buffer: *mut AudioInBuffer, ContainsBuffer: *mut bool) -> Result;

    /// Submits an audio sample data buffer for capturing and waits for it to finish capturing.
    /// Uses `audinAppendAudioInBuffer` and `audinWaitCaptureFinish` internally.
    ///
    /// # Arguments
    /// - `source`: AudioInBuffer containing the buffer to hold the captured sample data.
    /// - `released`: AudioInBuffer to receive the captured buffer after being released.
    pub fn audinCaptureBuffer(source: *mut AudioInBuffer, released: *mut *mut AudioInBuffer) -> Result;

    /// Waits for audio capture to finish.
    ///
    /// # Arguments
    /// - `released`: AudioInBuffer to receive the first captured buffer after being released.
    /// - `released_count`: Pointer to receive the number of captured buffers.
    /// - `timeout`: Timeout value, use UINT64_MAX to wait until all finished.
    pub fn audinWaitCaptureFinish(released: *mut *mut AudioInBuffer, released_count: *mut u32, timeout: u64) -> Result;

    /// Supported sample rate (48000Hz).
    pub fn audinGetSampleRate() -> u32;

    /// Supported channel count (2 channels).
    pub fn audinGetChannelCount() -> u32;

    /// Supported PCM format (Int16).
    pub fn audinGetPcmFormat() -> PcmFormat;

    /// Initial device state (stopped).
    pub fn audinGetDeviceState() -> AudioInState;
}
