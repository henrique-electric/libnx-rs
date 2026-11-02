//! `switch/services/btm.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

unsafe extern "C" {
    /// Initialize btm.
    pub fn btmInitialize() -> Result;

    /// Exit btm.
    pub fn btmExit();

    /// Gets the Service object for the actual btm service session.
    pub fn btmGetServiceSession() -> *mut Service;

    /// GetState
    ///
    /// # Arguments
    /// - `out` (out): `BtmState`
    pub fn btmGetState(out: *mut BtmState) -> Result;

    /// GetHostDeviceProperty
    ///
    /// # Arguments
    /// - `out` (out): `BtmHostDeviceProperty`
    pub fn btmGetHostDeviceProperty(out: *mut BtmHostDeviceProperty) -> Result;

    /// AcquireDeviceConditionEvent
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=true.
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn btmAcquireDeviceConditionEvent(out_event: *mut Event) -> Result;

    /// GetDeviceCondition [1.0.0-12.1.0]
    ///
    /// # Arguments
    /// - `out` (out): `BtmDeviceCondition`
    pub fn btmLegacyGetDeviceCondition(out: *mut BtmDeviceCondition) -> Result;

    /// GetDeviceCondition [13.0.0+]
    ///
    /// # Arguments
    /// - `profile` (in): `BtmProfile`, when not ::BtmProfile_None entries are only returned which match this profile.
    /// - `out` (out): `BtmConnectedDeviceV13`
    /// - `count` (in): Size of the out array in entries.
    /// - `total_out` (out): Total output entries.
    pub fn btmGetDeviceCondition(profile: BtmProfile, out: *mut BtmConnectedDeviceV13, count: usize, total_out: *mut s32) -> Result;

    /// SetBurstMode
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    /// - `flag` (in): Flag
    pub fn btmSetBurstMode(addr: BtdrvAddress, flag: bool) -> Result;

    /// SetSlotMode
    ///
    /// # Arguments
    /// - `list` (in): `BtmDeviceSlotModeList`
    pub fn btmSetSlotMode(list: *const BtmDeviceSlotModeList) -> Result;

    /// SetBluetoothMode
    ///
    /// # Arguments
    /// - `mode` (in): `BtmBluetoothMode`
    ///
    /// **Note:** Only available on pre-9.0.0.
    pub fn btmSetBluetoothMode(mode: BtmBluetoothMode) -> Result;

    /// SetWlanMode
    ///
    /// # Arguments
    /// - `mode` (in): `BtmWlanMode`
    pub fn btmSetWlanMode(mode: BtmWlanMode) -> Result;

    /// AcquireDeviceInfoEvent
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=true.
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn btmAcquireDeviceInfoEvent(out_event: *mut Event) -> Result;

    /// GetDeviceInfo [1.0.0-12.1.0]
    ///
    /// # Arguments
    /// - `out` (out): `BtmDeviceInfoList`
    pub fn btmLegacyGetDeviceInfo(out: *mut BtmDeviceInfoList) -> Result;

    /// GetDeviceInfo [13.0.0+]
    ///
    /// # Arguments
    /// - `profile` (in): `BtmProfile`, when not ::BtmProfile_None entries are only returned which match this profile.
    /// - `out` (out): `BtmDeviceInfoV13`
    /// - `count` (in): Size of the out array in entries.
    /// - `total_out` (out): Total output entries.
    pub fn btmGetDeviceInfo(profile: BtmProfile, out: *mut BtmDeviceInfoV13, count: usize, total_out: *mut s32) -> Result;

    /// AddDeviceInfo
    ///
    /// # Arguments
    /// - `info` (in): `BtmDeviceInfo`
    pub fn btmAddDeviceInfo(info: *const BtmDeviceInfo) -> Result;

    /// RemoveDeviceInfo
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    pub fn btmRemoveDeviceInfo(addr: BtdrvAddress) -> Result;

    /// IncreaseDeviceInfoOrder
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    pub fn btmIncreaseDeviceInfoOrder(addr: BtdrvAddress) -> Result;

    /// LlrNotify
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    /// - `unk` (in): [9.0.0+] Unknown
    pub fn btmLlrNotify(addr: BtdrvAddress, unk: s32) -> Result;

    /// EnableRadio
    pub fn btmEnableRadio() -> Result;

    /// DisableRadio
    pub fn btmDisableRadio() -> Result;

    /// HidDisconnect
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    pub fn btmHidDisconnect(addr: BtdrvAddress) -> Result;

    /// HidSetRetransmissionMode
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    /// - `list` (in): `BtmZeroRetransmissionList`
    pub fn btmHidSetRetransmissionMode(addr: BtdrvAddress, list: *const BtmZeroRetransmissionList) -> Result;

    /// AcquireAwakeReqEvent
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=true.
    ///
    /// **Note:** Only available on [2.0.0+].
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn btmAcquireAwakeReqEvent(out_event: *mut Event) -> Result;

    /// AcquireLlrStateEvent
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=true.
    ///
    /// **Note:** Only available on [4.0.0+].
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn btmAcquireLlrStateEvent(out_event: *mut Event) -> Result;

    /// IsLlrStarted
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn btmIsLlrStarted(out: *mut bool) -> Result;

    /// EnableSlotSaving
    ///
    /// # Arguments
    /// - `flag` (in): Flag
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn btmEnableSlotSaving(flag: bool) -> Result;

    /// ProtectDeviceInfo
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    /// - `flag` (in): Flag
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btmProtectDeviceInfo(addr: BtdrvAddress, flag: bool) -> Result;

    /// AcquireBleScanEvent
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=true.
    ///
    /// **Note:** Only available on [5.0.0+].
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn btmAcquireBleScanEvent(out_event: *mut Event) -> Result;

    /// GetBleScanParameterGeneral
    ///
    /// # Arguments
    /// - `parameter_id` (in): Must be value 0x1 or 0xFFFF.
    /// - `out` (out): `BtdrvBleAdvertisePacketParameter`
    ///
    /// **Note:** Only available on [5.1.0+].
    pub fn btmGetBleScanParameterGeneral(parameter_id: u16, out: *mut BtdrvBleAdvertisePacketParameter) -> Result;

    /// GetBleScanParameterSmartDevice
    ///
    /// # Arguments
    /// - `parameter_id` (in): Must be value 0x2.
    /// - `out` (out): `BtdrvGattAttributeUuid`. The first 4-bytes is always 0.
    ///
    /// **Note:** Only available on [5.1.0+].
    pub fn btmGetBleScanParameterSmartDevice(parameter_id: u16, out: *mut BtdrvGattAttributeUuid) -> Result;

    /// StartBleScanForGeneral
    ///
    /// # Arguments
    /// - `param` (in): `BtdrvBleAdvertisePacketParameter`
    ///
    /// **Note:** Only available on [5.1.0+].
    pub fn btmStartBleScanForGeneral(param: BtdrvBleAdvertisePacketParameter) -> Result;

    /// StopBleScanForGeneral
    ///
    /// **Note:** Only available on [5.1.0+].
    pub fn btmStopBleScanForGeneral() -> Result;

    /// GetBleScanResultsForGeneral
    ///
    /// # Arguments
    /// - `results` (out): Output array of `BtdrvBleScanResult`.
    /// - `count` (in): Size of the results array in entries. The max is 10.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** Only available on [5.1.0+].
    pub fn btmGetBleScanResultsForGeneral(results: *mut BtdrvBleScanResult, count: u8, total_out: *mut u8) -> Result;

    /// StartBleScanForPaired
    ///
    /// # Arguments
    /// - `param` (in): `BtdrvBleAdvertisePacketParameter`
    ///
    /// **Note:** Only available on [5.1.0+].
    pub fn btmStartBleScanForPaired(param: BtdrvBleAdvertisePacketParameter) -> Result;

    /// StopBleScanForPaired
    ///
    /// **Note:** Only available on [5.1.0+].
    pub fn btmStopBleScanForPaired() -> Result;

    /// StartBleScanForSmartDevice
    ///
    /// # Arguments
    /// - `uuid` (in): `BtdrvGattAttributeUuid`
    ///
    /// **Note:** Only available on [5.1.0+].
    pub fn btmStartBleScanForSmartDevice(uuid: *const BtdrvGattAttributeUuid) -> Result;

    /// StopBleScanForSmartDevice
    ///
    /// **Note:** Only available on [5.1.0+].
    pub fn btmStopBleScanForSmartDevice() -> Result;

    /// GetBleScanResultsForSmartDevice
    ///
    /// # Arguments
    /// - `results` (out): Output array of `BtdrvBleScanResult`.
    /// - `count` (in): Size of the results array in entries. The max is 10.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** Only available on [5.1.0+].
    pub fn btmGetBleScanResultsForSmartDevice(results: *mut BtdrvBleScanResult, count: u8, total_out: *mut u8) -> Result;

    /// AcquireBleConnectionEvent
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=true.
    ///
    /// **Note:** Only available on [5.1.0+].
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn btmAcquireBleConnectionEvent(out_event: *mut Event) -> Result;

    /// BleConnect
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    ///
    /// **Note:** Only available on [5.0.0+].
    ///
    /// **Note:** The `BtdrvAddress` must not be already connected. A maximum of 4 devices can be connected.
    pub fn btmBleConnect(addr: BtdrvAddress) -> Result;

    /// BleOverrideConnection
    ///
    /// # Arguments
    /// - `id` (in): Same as `btmBleDisconnect`.
    ///
    /// **Note:** Only available on [5.1.0+].
    pub fn btmBleOverrideConnection(id: u32) -> Result;

    /// BleDisconnect
    ///
    /// # Arguments
    /// - `connection_handle` (in): This must match a BtdrvBleConnectionInfo::id from `btmBleGetConnectionState`. [5.1.0+] 0xFFFFFFFF is invalid.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btmBleDisconnect(connection_handle: u32) -> Result;

    /// BleGetConnectionState
    ///
    /// # Arguments
    /// - `info` (out): Output array of `BtdrvBleConnectionInfo`.
    /// - `count` (in): Size of the info array in entries. Other cmds which use this internally use count=4.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btmBleGetConnectionState(info: *mut BtdrvBleConnectionInfo, count: u8, total_out: *mut u8) -> Result;

    /// BleGetGattClientConditionList
    ///
    /// # Arguments
    /// - `list` (out): `BtmGattClientConditionList`
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btmBleGetGattClientConditionList(list: *mut BtmGattClientConditionList) -> Result;

    /// AcquireBlePairingEvent
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=true.
    ///
    /// **Note:** Only available on [5.0.0+].
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn btmAcquireBlePairingEvent(out_event: *mut Event) -> Result;

    /// BlePairDevice
    ///
    /// # Arguments
    /// - `connection_handle` (in): Same as `btmBleDisconnect`.
    /// - `param` (in): `BtdrvBleAdvertisePacketParameter`
    ///
    /// **Note:** Only available on [5.1.0+].
    pub fn btmBlePairDevice(connection_handle: u32, param: BtdrvBleAdvertisePacketParameter) -> Result;

    /// BleUnpairDeviceOnBoth
    ///
    /// # Arguments
    /// - `connection_handle` (in): Same as `btmBleDisconnect`.
    /// - `param` (in): `BtdrvBleAdvertisePacketParameter`
    ///
    /// **Note:** Only available on [5.1.0+].
    pub fn btmBleUnpairDeviceOnBoth(connection_handle: u32, param: BtdrvBleAdvertisePacketParameter) -> Result;

    /// BleUnPairDevice
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    /// - `param` (in): `BtdrvBleAdvertisePacketParameter`
    ///
    /// **Note:** Only available on [5.1.0+].
    pub fn btmBleUnPairDevice(addr: BtdrvAddress, param: BtdrvBleAdvertisePacketParameter) -> Result;

    /// BleGetPairedAddresses
    ///
    /// # Arguments
    /// - `param` (in): `BtdrvBleAdvertisePacketParameter`
    /// - `addrs` (out): Output array of `BtdrvAddress`.
    /// - `count` (in): Size of the addrs array in entries.
    /// - `total_out` (out): Total output entries. The max is 10.
    ///
    /// **Note:** Only available on [5.1.0+].
    pub fn btmBleGetPairedAddresses(param: BtdrvBleAdvertisePacketParameter, addrs: *mut BtdrvAddress, count: u8, total_out: *mut u8) -> Result;

    /// AcquireBleServiceDiscoveryEvent
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=true.
    ///
    /// **Note:** Only available on [5.1.0+].
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn btmAcquireBleServiceDiscoveryEvent(out_event: *mut Event) -> Result;

    /// GetGattServices
    ///
    /// # Arguments
    /// - `connection_handle` (in): Same as `btmBleDisconnect`.
    /// - `services` (out): Output array of `BtmGattService`.
    /// - `count` (in): Size of the services array in entries. The max is 100.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btmGetGattServices(connection_handle: u32, services: *mut BtmGattService, count: u8, total_out: *mut u8) -> Result;

    /// Same as `btmGetGattServices` except this only returns the `BtmGattService` which matches the input `BtdrvGattAttributeUuid`.
    ///
    /// # Arguments
    /// - `connection_handle` (in): Same as `btmBleDisconnect`.
    /// - `uuid` (in): `BtdrvGattAttributeUuid`
    /// - `service` (out): `BtmGattService`
    /// - `flag` (out): Whether a `BtmGattService` was returned.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btmGetGattService(connection_handle: u32, uuid: *const BtdrvGattAttributeUuid, service: *mut BtmGattService, flag: *mut bool) -> Result;

    /// Same as `btmGetGattServices` except this only returns `BtmGattService` entries where various checks pass with u16 fields.
    ///
    /// # Arguments
    /// - `connection_handle` (in): Same as `btmBleDisconnect`.
    /// - `service_handle` (in): ServiceHandle
    /// - `services` (out): `BtmGattService`
    /// - `count` (in): Size of the services array in entries. The max is 100.
    /// - `out` (out): Output value.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btmGetGattIncludedServices(connection_handle: u32, service_handle: u16, services: *mut BtmGattService, count: u8, out: *mut u8) -> Result;

    /// This is similar to `btmGetGattIncludedServices` except this only returns 1 `BtmGattService`.
    ///
    /// # Arguments
    /// - `connection_handle` (in): Same as `btmBleDisconnect`.
    /// - `attribute_handle` (in): AttributeHandle
    /// - `service` (out): `BtmGattService`
    /// - `flag` (out): Whether a `BtmGattService` was returned.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btmGetBelongingService(connection_handle: u32, attribute_handle: u16, service: *mut BtmGattService, flag: *mut bool) -> Result;

    /// GetGattCharacteristics
    ///
    /// # Arguments
    /// - `connection_handle` (in): Same as `btmBleDisconnect`.
    /// - `service_handle` (in): This controls which `BtmGattCharacteristic` entries to return.
    /// - `characteristics` (out): `BtmGattCharacteristic`
    /// - `count` (in): Size of the characteristics array in entries. The max is 100.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btmGetGattCharacteristics(connection_handle: u32, service_handle: u16, characteristics: *mut BtmGattCharacteristic, count: u8, total_out: *mut u8) -> Result;

    /// GetGattDescriptors
    ///
    /// # Arguments
    /// - `connection_handle` (in): Same as `btmBleDisconnect`.
    /// - `char_handle` (in): Characteristic handle. This controls which `BtmGattDescriptor` entries to return.
    /// - `descriptors` (out): `BtmGattDescriptor`
    /// - `count` (in): Size of the descriptors array in entries. The max is 100.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btmGetGattDescriptors(connection_handle: u32, char_handle: u16, descriptors: *mut BtmGattDescriptor, count: u8, total_out: *mut u8) -> Result;

    /// AcquireBleMtuConfigEvent
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=true.
    ///
    /// **Note:** Only available on [5.0.0+].
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn btmAcquireBleMtuConfigEvent(out_event: *mut Event) -> Result;

    /// ConfigureBleMtu
    ///
    /// # Arguments
    /// - `connection_handle` (in): Same as `btmBleDisconnect`.
    /// - `mtu` (in): MTU
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btmConfigureBleMtu(connection_handle: u32, mtu: u16) -> Result;

    /// GetBleMtu
    ///
    /// # Arguments
    /// - `connection_handle` (in): Same as `btmBleDisconnect`.
    /// - `out` (out): Output MTU.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btmGetBleMtu(connection_handle: u32, out: *mut u16) -> Result;

    /// RegisterBleGattDataPath
    ///
    /// # Arguments
    /// - `path` (in): `BtmBleDataPath`
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btmRegisterBleGattDataPath(path: *const BtmBleDataPath) -> Result;

    /// UnregisterBleGattDataPath
    ///
    /// # Arguments
    /// - `path` (in): `BtmBleDataPath`
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btmUnregisterBleGattDataPath(path: *const BtmBleDataPath) -> Result;

    /// RegisterAppletResourceUserId
    ///
    /// # Arguments
    /// - `AppletResourceUserId` (in): AppletResourceUserId
    /// - `unk` (in): Unknown
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btmRegisterAppletResourceUserId(AppletResourceUserId: u64, unk: u32) -> Result;

    /// UnregisterAppletResourceUserId
    ///
    /// # Arguments
    /// - `AppletResourceUserId` (in): AppletResourceUserId
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btmUnregisterAppletResourceUserId(AppletResourceUserId: u64) -> Result;

    /// SetAppletResourceUserId
    ///
    /// # Arguments
    /// - `AppletResourceUserId` (in): AppletResourceUserId
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btmSetAppletResourceUserId(AppletResourceUserId: u64) -> Result;
}
