//! `switch/services/auddev.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

unsafe extern "C" {
    /// Initialize IAudioDevice.
    pub fn auddevInitialize() -> Result;

    /// Exit IAudioDevice.
    pub fn auddevExit();

    /// Gets the Service object for IAudioDevice.
    pub fn auddevGetServiceSession() -> *mut Service;

    pub fn auddevListAudioDeviceName(DeviceNames: *mut AudioDeviceName, max_names: s32, total_names: *mut s32) -> Result;

    pub fn auddevSetAudioDeviceOutputVolume(DeviceName: *const AudioDeviceName, volume: f32) -> Result;

    pub fn auddevGetAudioDeviceOutputVolume(DeviceName: *const AudioDeviceName, volume: *mut f32) -> Result;

    pub fn auddevGetActiveAudioDeviceName(DeviceName: *mut AudioDeviceName) -> Result;
}
