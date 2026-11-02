//! `switch/services/audctl.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub type AudioTarget = u32;
pub const AudioTarget_Invalid: AudioTarget = 0;
pub const AudioTarget_Speaker: AudioTarget = 1;
pub const AudioTarget_Headphone: AudioTarget = 2;
pub const AudioTarget_Tv: AudioTarget = 3;
pub const AudioTarget_UsbOutputDevice: AudioTarget = 4;
pub const AudioTarget_Bluetooth: AudioTarget = 5;

pub type AudioOutputMode = u32;
pub const AudioOutputMode_Invalid: AudioOutputMode = 0;
pub const AudioOutputMode_Pcm1ch: AudioOutputMode = 1;
pub const AudioOutputMode_Pcm2ch: AudioOutputMode = 2;
pub const AudioOutputMode_Pcm6ch: AudioOutputMode = 3;
pub const AudioOutputMode_PcmAuto: AudioOutputMode = 4;

pub type AudioForceMutePolicy = u32;
pub const AudioForceMutePolicy_Disable: AudioForceMutePolicy = 0;
pub const AudioForceMutePolicy_SpeakerMuteOnHeadphoneUnplugged: AudioForceMutePolicy = 1;

pub type AudioHeadphoneOutputLevelMode = u32;
pub const AudioHeadphoneOutputLevelMode_Normal: AudioHeadphoneOutputLevelMode = 0;
pub const AudioHeadphoneOutputLevelMode_HighPower: AudioHeadphoneOutputLevelMode = 1;

unsafe extern "C" {
    pub fn audctlInitialize() -> Result;

    pub fn audctlExit();

    pub fn audctlGetServiceSession() -> *mut Service;

    pub fn audctlGetTargetVolume(volume_out: *mut s32, target: AudioTarget) -> Result;

    pub fn audctlSetTargetVolume(target: AudioTarget, volume: s32) -> Result;

    pub fn audctlGetTargetVolumeMin(volume_out: *mut s32) -> Result;

    pub fn audctlGetTargetVolumeMax(volume_out: *mut s32) -> Result;

    pub fn audctlIsTargetMute(mute_out: *mut bool, target: AudioTarget) -> Result;

    pub fn audctlSetTargetMute(target: AudioTarget, mute: bool) -> Result;

    /// [1.0.0-17.0.1]
    pub fn audctlIsTargetConnected(connected_out: *mut bool, target: AudioTarget) -> Result;

    pub fn audctlSetDefaultTarget(target: AudioTarget, fade_in_ns: u64, fade_out_ns: u64) -> Result;

    pub fn audctlGetDefaultTarget(target_out: *mut AudioTarget) -> Result;

    pub fn audctlGetAudioOutputMode(mode_out: *mut AudioOutputMode, target: AudioTarget) -> Result;

    pub fn audctlSetAudioOutputMode(target: AudioTarget, mode: AudioOutputMode) -> Result;

    /// [1.0.0-13.2.1]
    pub fn audctlSetForceMutePolicy(policy: AudioForceMutePolicy) -> Result;

    /// [1.0.0-13.2.1]
    pub fn audctlGetForceMutePolicy(policy_out: *mut AudioForceMutePolicy) -> Result;

    pub fn audctlGetOutputModeSetting(mode_out: *mut AudioOutputMode, target: AudioTarget) -> Result;

    pub fn audctlSetOutputModeSetting(target: AudioTarget, mode: AudioOutputMode) -> Result;

    pub fn audctlSetOutputTarget(target: AudioTarget) -> Result;

    pub fn audctlSetInputTargetForceEnabled(enable: bool) -> Result;

    /// [3.0.0+]
    pub fn audctlSetHeadphoneOutputLevelMode(mode: AudioHeadphoneOutputLevelMode) -> Result;

    /// [3.0.0+]
    pub fn audctlGetHeadphoneOutputLevelMode(mode_out: *mut AudioHeadphoneOutputLevelMode) -> Result;

    /// [3.0.0-13.2.1]
    pub fn audctlAcquireAudioVolumeUpdateEventForPlayReport(event_out: *mut Event) -> Result;

    /// [3.0.0-13.2.1]
    pub fn audctlAcquireAudioOutputDeviceUpdateEventForPlayReport(event_out: *mut Event) -> Result;

    /// [3.0.0+]
    pub fn audctlGetAudioOutputTargetForPlayReport(target_out: *mut AudioTarget) -> Result;

    /// [3.0.0+]
    pub fn audctlNotifyHeadphoneVolumeWarningDisplayedEvent() -> Result;

    /// [4.0.0+]
    pub fn audctlSetSystemOutputMasterVolume(volume: f32) -> Result;

    /// [4.0.0+]
    pub fn audctlGetSystemOutputMasterVolume(volume_out: *mut f32) -> Result;

    pub fn audctlGetActiveOutputTarget(target: *mut AudioTarget) -> Result;
}
