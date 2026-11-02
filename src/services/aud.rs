//! `switch/services/aud.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const AUD_MAX_DELAY: u64 = 0x3b9aca00;

unsafe extern "C" {
    /// Initialize aud:a. Only available on [11.0.0+].
    pub fn audaInitialize() -> Result;

    /// Exit aud:a.
    pub fn audaExit();

    /// Initialize aud:d. Only available on [11.0.0+].
    pub fn auddInitialize() -> Result;

    /// Exit aud:d.
    pub fn auddExit();

    /// Gets the Service for aud:a.
    pub fn audaGetServiceSession() -> *mut Service;

    /// Gets the Service for aud:d.
    pub fn auddGetServiceSession() -> *mut Service;

    pub fn audaRequestSuspendAudio(pid: u64, delay: u64) -> Result;

    pub fn audaRequestResumeAudio(pid: u64, delay: u64) -> Result;

    pub fn audaGetAudioOutputProcessMasterVolume(pid: u64, volume_out: *mut f32) -> Result;

    pub fn audaSetAudioOutputProcessMasterVolume(pid: u64, delay: u64, volume: f32) -> Result;

    pub fn audaGetAudioInputProcessMasterVolume(pid: u64, volume_out: *mut f32) -> Result;

    pub fn audaSetAudioInputProcessMasterVolume(pid: u64, delay: u64, volume: f32) -> Result;

    pub fn audaGetAudioOutputProcessRecordVolume(pid: u64, volume_out: *mut f32) -> Result;

    pub fn audaSetAudioOutputProcessRecordVolume(pid: u64, delay: u64, volume: f32) -> Result;

    pub fn auddRequestSuspendAudioForDebug(pid: u64, delay: u64) -> Result;

    pub fn auddRequestResumeAudioForDebug(pid: u64, delay: u64) -> Result;
}
