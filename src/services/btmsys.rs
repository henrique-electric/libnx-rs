//! `switch/services/btmsys.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

unsafe extern "C" {
    /// Initialize btm:sys.
    pub fn btmsysInitialize() -> Result;

    /// Exit btm:sys.
    pub fn btmsysExit();

    /// Gets the Service object for the actual btm:sys service session. This object must be closed by the user once finished using cmds with this.
    pub fn btmsysGetServiceSession(srv_out: *mut Service) -> Result;

    /// Gets the Service object for IBtmSystemCore.
    pub fn btmsysGetServiceSession_IBtmSystemCore() -> *mut Service;

    /// StartGamepadPairing
    pub fn btmsysStartGamepadPairing() -> Result;

    /// CancelGamepadPairing
    pub fn btmsysCancelGamepadPairing() -> Result;

    /// ClearGamepadPairingDatabase
    pub fn btmsysClearGamepadPairingDatabase() -> Result;

    /// GetPairedGamepadCount
    ///
    /// # Arguments
    /// - `out` (out): Output count.
    pub fn btmsysGetPairedGamepadCount(out: *mut u8) -> Result;

    /// EnableRadio
    pub fn btmsysEnableRadio() -> Result;

    /// DisableRadio
    pub fn btmsysDisableRadio() -> Result;

    /// GetRadioOnOff
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    pub fn btmsysGetRadioOnOff(out: *mut bool) -> Result;

    /// AcquireRadioEvent
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=true.
    ///
    /// **Note:** Only available on [3.0.0+].
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn btmsysAcquireRadioEvent(out_event: *mut Event) -> Result;

    /// AcquireGamepadPairingEvent
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=true.
    ///
    /// **Note:** Only available on [3.0.0+].
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn btmsysAcquireGamepadPairingEvent(out_event: *mut Event) -> Result;

    /// IsGamepadPairingStarted
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn btmsysIsGamepadPairingStarted(out: *mut bool) -> Result;

    /// StartAudioDeviceDiscovery
    ///
    /// **Note:** Only available on [13.0.0+].
    pub fn btmsysStartAudioDeviceDiscovery() -> Result;

    /// StopAudioDeviceDiscovery
    ///
    /// **Note:** Only available on [13.0.0+].
    pub fn btmsysStopAudioDeviceDiscovery() -> Result;

    /// IsDiscoveryingAudioDevice
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [13.0.0+].
    pub fn btmsysIsDiscoveryingAudioDevice(out: *mut bool) -> Result;

    /// GetDiscoveredAudioDevice
    ///
    /// # Arguments
    /// - `out` (out): Output array of `BtmAudioDevice`.
    /// - `count` (in): Size of the out array in entries. The max is 15.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** Only available on [13.0.0+].
    pub fn btmsysGetDiscoveredAudioDevice(out: *mut BtmAudioDevice, count: s32, total_out: *mut s32) -> Result;

    /// AcquireAudioDeviceConnectionEvent
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=true.
    ///
    /// **Note:** Only available on [13.0.0+].
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn btmsysAcquireAudioDeviceConnectionEvent(out_event: *mut Event) -> Result;

    /// ConnectAudioDevice
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    ///
    /// **Note:** Only available on [13.0.0+].
    pub fn btmsysConnectAudioDevice(addr: BtdrvAddress) -> Result;

    /// IsConnectingAudioDevice
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [13.0.0+].
    pub fn btmsysIsConnectingAudioDevice(out: *mut bool) -> Result;

    /// GetConnectedAudioDevices
    ///
    /// # Arguments
    /// - `out` (out): Output array of `BtmAudioDevice`.
    /// - `count` (in): Size of the out array in entries. The max is 8.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** Only available on [13.0.0+].
    pub fn btmsysGetConnectedAudioDevices(out: *mut BtmAudioDevice, count: s32, total_out: *mut s32) -> Result;

    /// DisconnectAudioDevice
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    ///
    /// **Note:** Only available on [13.0.0+].
    pub fn btmsysDisconnectAudioDevice(addr: BtdrvAddress) -> Result;

    /// AcquirePairedAudioDeviceInfoChangedEvent
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=true.
    ///
    /// **Note:** Only available on [13.0.0+].
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn btmsysAcquirePairedAudioDeviceInfoChangedEvent(out_event: *mut Event) -> Result;

    /// GetPairedAudioDevices
    ///
    /// # Arguments
    /// - `out` (out): Output array of `BtmAudioDevice`.
    /// - `count` (in): Size of the out array in entries. The max is 10.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** Only available on [13.0.0+].
    pub fn btmsysGetPairedAudioDevices(out: *mut BtmAudioDevice, count: s32, total_out: *mut s32) -> Result;

    /// RemoveAudioDevicePairing
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    ///
    /// **Note:** Only available on [13.0.0+].
    pub fn btmsysRemoveAudioDevicePairing(addr: BtdrvAddress) -> Result;

    /// RequestAudioDeviceConnectionRejection
    ///
    /// **Note:** Only available on [13.0.0+].
    pub fn btmsysRequestAudioDeviceConnectionRejection() -> Result;

    /// CancelAudioDeviceConnectionRejection
    ///
    /// **Note:** Only available on [13.0.0+].
    pub fn btmsysCancelAudioDeviceConnectionRejection() -> Result;
}
