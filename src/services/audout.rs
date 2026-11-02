//! `switch/services/audout.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const AUDOUT_MAX_DELAY: u64 = 0x3b9aca00;

pub type AudioOutState = u32;
pub const AudioOutState_Started: AudioOutState = 0;
pub const AudioOutState_Stopped: AudioOutState = 1;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioOutBuffer {
    /// Next buffer. (Unused)
    pub next: *mut AudioOutBuffer,
    /// Sample buffer (aligned to 0x1000 bytes).
    pub buffer: *mut c_void,
    /// Sample buffer size (aligned to 0x1000 bytes).
    pub buffer_size: u64,
    /// Size of data inside the buffer.
    pub data_size: u64,
    /// Offset of data inside the buffer. (Unused?)
    pub data_offset: u64,
}

impl_zeroed_default!(AudioOutBuffer);

unsafe extern "C" {
    /// Initialize audout.
    pub fn audoutInitialize() -> Result;

    /// Exit audout.
    pub fn audoutExit();

    /// Initialize audout:a. Removed in [11.0.0].
    pub fn audoutaInitialize() -> Result;

    /// Exit audout:a.
    pub fn audoutaExit();

    /// Initialize audout:d. Removed in [11.0.0].
    pub fn audoutdInitialize() -> Result;

    /// Exit audout:d.
    pub fn audoutdExit();

    /// Gets the Service object for the actual audout service session.
    pub fn audoutGetServiceSession() -> *mut Service;

    /// Gets the Service object for IAudioOut.
    pub fn audoutGetServiceSession_AudioOut() -> *mut Service;

    /// Gets the Service for audout:a.
    pub fn audoutaGetServiceSession() -> *mut Service;

    /// Gets the Service for audout:d.
    pub fn audoutdGetServiceSession() -> *mut Service;

    pub fn audoutListAudioOuts(DeviceNames: *mut c_char, count: s32, DeviceNamesCount: *mut u32) -> Result;

    pub fn audoutOpenAudioOut(DeviceNameIn: *const c_char, DeviceNameOut: *mut c_char, SampleRateIn: u32, ChannelCountIn: u32, SampleRateOut: *mut u32, ChannelCountOut: *mut u32, Format: *mut PcmFormat, State: *mut AudioOutState) -> Result;

    pub fn audoutGetAudioOutState(State: *mut AudioOutState) -> Result;

    pub fn audoutStartAudioOut() -> Result;

    pub fn audoutStopAudioOut() -> Result;

    /// Submits an `AudioOutBuffer` for playing.
    pub fn audoutAppendAudioOutBuffer(Buffer: *mut AudioOutBuffer) -> Result;

    pub fn audoutGetReleasedAudioOutBuffer(Buffer: *mut *mut AudioOutBuffer, ReleasedBuffersCount: *mut u32) -> Result;

    pub fn audoutContainsAudioOutBuffer(Buffer: *mut AudioOutBuffer, ContainsBuffer: *mut bool) -> Result;

    /// Only available with [4.0.0+].
    pub fn audoutGetAudioOutBufferCount(count: *mut u32) -> Result;

    /// Only available with [4.0.0+].
    pub fn audoutGetAudioOutPlayedSampleCount(count: *mut u64) -> Result;

    /// Only available with [4.0.0+].
    pub fn audoutFlushAudioOutBuffers(flushed: *mut bool) -> Result;

    /// Only available with [6.0.0+].
    pub fn audoutSetAudioOutVolume(volume: f32) -> Result;

    /// Only available with [6.0.0+].
    pub fn audoutGetAudioOutVolume(volume: *mut f32) -> Result;

    /// Submits an audio sample data buffer for playing and waits for it to finish playing.
    /// Uses `audoutAppendAudioOutBuffer` and `audoutWaitPlayFinish` internally.
    ///
    /// # Arguments
    /// - `source`: AudioOutBuffer containing the source sample data to be played.
    /// - `released`: AudioOutBuffer to receive the played buffer after being released.
    pub fn audoutPlayBuffer(source: *mut AudioOutBuffer, released: *mut *mut AudioOutBuffer) -> Result;

    /// Waits for audio playback to finish.
    ///
    /// # Arguments
    /// - `released`: AudioOutBuffer to receive the first played buffer after being released.
    /// - `released_count`: Pointer to receive the number of played buffers.
    /// - `timeout`: Timeout value, use UINT64_MAX to wait until all finished.
    pub fn audoutWaitPlayFinish(released: *mut *mut AudioOutBuffer, released_count: *mut u32, timeout: u64) -> Result;

    /// Supported sample rate (48000Hz).
    pub fn audoutGetSampleRate() -> u32;

    /// Supported channel count (2 channels).
    pub fn audoutGetChannelCount() -> u32;

    /// Supported PCM format (Int16).
    pub fn audoutGetPcmFormat() -> PcmFormat;

    /// Initial device state (stopped).
    pub fn audoutGetDeviceState() -> AudioOutState;

    pub fn audoutaRequestSuspend(pid: u64, delay: u64) -> Result;

    pub fn audoutaRequestResume(pid: u64, delay: u64) -> Result;

    pub fn audoutaGetProcessMasterVolume(pid: u64, volume_out: *mut f32) -> Result;

    pub fn audoutaSetProcessMasterVolume(pid: u64, delay: u64, volume: f32) -> Result;

    pub fn audoutaGetProcessRecordVolume(pid: u64, volume_out: *mut f32) -> Result;

    pub fn audoutaSetProcessRecordVolume(pid: u64, delay: u64, volume: f32) -> Result;

    pub fn audoutdRequestSuspendForDebug(pid: u64, delay: u64) -> Result;

    pub fn audoutdRequestResumeForDebug(pid: u64, delay: u64) -> Result;
}
