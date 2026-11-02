//! `switch/services/btmu.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

unsafe extern "C" {
    /// Initialize btm:u.
    pub fn btmuInitialize() -> Result;

    /// Exit btm:u.
    pub fn btmuExit();

    /// Gets the Service object for the actual btm:u service session. This object must be closed by the user once finished using cmds with this.
    pub fn btmuGetServiceSession(srv_out: *mut Service) -> Result;

    /// Gets the Service object for IBtmUserCore.
    pub fn btmuGetServiceSession_IBtmUserCore() -> *mut Service;

    /// AcquireBleScanEvent
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=true.
    ///
    /// **Note:** This is similar to `btmAcquireBleScanEvent`.
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn btmuAcquireBleScanEvent(out_event: *mut Event) -> Result;

    /// GetBleScanFilterParameter
    ///
    /// # Arguments
    /// - `parameter_id` (in): Must be value 0x1 or 0xFFFF.
    /// - `out` (out): `BtdrvBleAdvertisePacketParameter`
    ///
    /// **Note:** This is the same as `btmGetBleScanParameterGeneral`.
    pub fn btmuGetBleScanFilterParameter(parameter_id: u16, out: *mut BtdrvBleAdvertisePacketParameter) -> Result;

    /// GetBleScanFilterParameter2
    ///
    /// # Arguments
    /// - `parameter_id` (in): Must be value 0x2.
    /// - `out` (out): `BtdrvGattAttributeUuid`. The first 4-bytes is always 0.
    ///
    /// **Note:** This is the same as `btmGetBleScanParameterSmartDevice`.
    pub fn btmuGetBleScanFilterParameter2(parameter_id: u16, out: *mut BtdrvGattAttributeUuid) -> Result;

    /// StartBleScanForGeneral
    ///
    /// # Arguments
    /// - `param` (in): `BtdrvBleAdvertisePacketParameter`
    ///
    /// **Note:** This is similar to `btmStartBleScanForGeneral`.
    pub fn btmuStartBleScanForGeneral(param: BtdrvBleAdvertisePacketParameter) -> Result;

    /// StopBleScanForGeneral
    ///
    /// **Note:** This is similar to `btmStopBleScanForGeneral`.
    pub fn btmuStopBleScanForGeneral() -> Result;

    /// GetBleScanResultsForGeneral
    ///
    /// # Arguments
    /// - `results` (out): Output array of `BtdrvBleScanResult`.
    /// - `count` (in): Size of the results array in entries. The max is 10.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** This is similar to `btmGetBleScanResultsForGeneral`.
    pub fn btmuGetBleScanResultsForGeneral(results: *mut BtdrvBleScanResult, count: u8, total_out: *mut u8) -> Result;

    /// StartBleScanForPaired
    ///
    /// # Arguments
    /// - `param` (in): `BtdrvBleAdvertisePacketParameter`
    ///
    /// **Note:** This is similar to `btmStartBleScanForPaired`.
    pub fn btmuStartBleScanForPaired(param: BtdrvBleAdvertisePacketParameter) -> Result;

    /// StopBleScanForPaired
    ///
    /// **Note:** This is similar to `btmStopBleScanForPaired`.
    pub fn btmuStopBleScanForPaired() -> Result;

    /// StartBleScanForSmartDevice
    ///
    /// # Arguments
    /// - `uuid` (in): `BtdrvGattAttributeUuid`
    ///
    /// **Note:** This is similar to `btmStartBleScanForSmartDevice`.
    pub fn btmuStartBleScanForSmartDevice(uuid: *const BtdrvGattAttributeUuid) -> Result;

    /// StopBleScanForSmartDevice
    ///
    /// **Note:** This is similar to `btmStopBleScanForSmartDevice`.
    pub fn btmuStopBleScanForSmartDevice() -> Result;

    /// GetBleScanResultsForSmartDevice
    ///
    /// # Arguments
    /// - `results` (out): Output array of `BtdrvBleScanResult`.
    /// - `count` (in): Size of the results array in entries. The max is 10.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** This is similar to `btmGetBleScanResultsForSmartDevice`.
    pub fn btmuGetBleScanResultsForSmartDevice(results: *mut BtdrvBleScanResult, count: u8, total_out: *mut u8) -> Result;

    /// AcquireBleConnectionEvent
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=true.
    ///
    /// **Note:** This is similar to `btmAcquireBleConnectionEvent`.
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn btmuAcquireBleConnectionEvent(out_event: *mut Event) -> Result;

    /// BleConnect
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    ///
    /// **Note:** This is similar to `btmBleConnect`.
    pub fn btmuBleConnect(addr: BtdrvAddress) -> Result;

    /// BleDisconnect
    ///
    /// # Arguments
    /// - `connection_handle` (in): This must match a BtdrvBleConnectionInfo::connection_handle from `btmuBleGetConnectionState`. [5.1.0+] 0xFFFFFFFF is invalid.
    ///
    /// **Note:** This is similar to `btmBleDisconnect`.
    pub fn btmuBleDisconnect(connection_handle: u32) -> Result;

    /// BleGetConnectionState
    ///
    /// # Arguments
    /// - `info` (out): Output array of `BtdrvBleConnectionInfo`.
    /// - `count` (in): Size of the info array in entries. Other cmds which use this internally use count=4.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** This is similar to `btmBleGetConnectionState`.
    pub fn btmuBleGetConnectionState(info: *mut BtdrvBleConnectionInfo, count: u8, total_out: *mut u8) -> Result;

    /// AcquireBlePairingEvent
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=true.
    ///
    /// **Note:** This is similar to `btmAcquireBlePairingEvent`.
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn btmuAcquireBlePairingEvent(out_event: *mut Event) -> Result;

    /// BlePairDevice
    ///
    /// # Arguments
    /// - `connection_handle` (in): Same as `btmuBleDisconnect`.
    /// - `param` (in): `BtdrvBleAdvertisePacketParameter`
    ///
    /// **Note:** This is similar to `btmBlePairDevice`.
    pub fn btmuBlePairDevice(connection_handle: u32, param: BtdrvBleAdvertisePacketParameter) -> Result;

    /// BleUnPairDevice
    ///
    /// # Arguments
    /// - `connection_handle` (in): Same as `btmuBleDisconnect`.
    /// - `param` (in): `BtdrvBleAdvertisePacketParameter`
    ///
    /// **Note:** This is similar to `btmBleUnpairDeviceOnBoth`.
    pub fn btmuBleUnPairDevice(connection_handle: u32, param: BtdrvBleAdvertisePacketParameter) -> Result;

    /// BleUnPairDevice2
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    /// - `param` (in): `BtdrvBleAdvertisePacketParameter`
    ///
    /// **Note:** This is similar to `btmBleUnPairDevice`.
    pub fn btmuBleUnPairDevice2(addr: BtdrvAddress, param: BtdrvBleAdvertisePacketParameter) -> Result;

    /// BleGetPairedDevices
    ///
    /// # Arguments
    /// - `param` (in): `BtdrvBleAdvertisePacketParameter`
    /// - `addrs` (out): Output array of `BtdrvAddress`.
    /// - `count` (in): Size of the addrs array in entries.
    /// - `total_out` (out): Total output entries. The max is 10.
    ///
    /// **Note:** This is similar to `btmBleGetPairedAddresses`.
    pub fn btmuBleGetPairedDevices(param: BtdrvBleAdvertisePacketParameter, addrs: *mut BtdrvAddress, count: u8, total_out: *mut u8) -> Result;

    /// AcquireBleServiceDiscoveryEvent
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=true.
    ///
    /// **Note:** This is similar to `btmAcquireBleServiceDiscoveryEvent`.
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn btmuAcquireBleServiceDiscoveryEvent(out_event: *mut Event) -> Result;

    /// GetGattServices
    ///
    /// # Arguments
    /// - `connection_handle` (in): Same as `btmuBleDisconnect`.
    /// - `services` (out): Output array of `BtmGattService`.
    /// - `count` (in): Size of the services array in entries. The max is 100.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** This is similar to `btmGetGattServices`.
    pub fn btmuGetGattServices(connection_handle: u32, services: *mut BtmGattService, count: u8, total_out: *mut u8) -> Result;

    /// Same as `btmuGetGattServices` except this only returns the `BtmGattService` which matches the input `BtdrvGattAttributeUuid`.
    ///
    /// # Arguments
    /// - `connection_handle` (in): Same as `btmuBleDisconnect`.
    /// - `uuid` (in): `BtdrvGattAttributeUuid`
    /// - `service` (out): `BtmGattService`
    /// - `flag` (out): Whether a `BtmGattService` was returned.
    ///
    /// **Note:** This is similar to `btmGetGattService`.
    pub fn btmuGetGattService(connection_handle: u32, uuid: *const BtdrvGattAttributeUuid, service: *mut BtmGattService, flag: *mut bool) -> Result;

    /// Same as `btmuGetGattServices` except this only returns `BtmGattService` entries where various checks pass with u16 fields.
    ///
    /// # Arguments
    /// - `connection_handle` (in): Same as `btmuBleDisconnect`.
    /// - `service_handle` (in): ServiceHandle
    /// - `services` (out): `BtmGattService`
    /// - `count` (in): Size of the services array in entries. The max is 100.
    /// - `out` (out): Output value.
    ///
    /// **Note:** This is similar to `btmGetGattIncludedServices`.
    pub fn btmuGetGattIncludedServices(connection_handle: u32, service_handle: u16, services: *mut BtmGattService, count: u8, out: *mut u8) -> Result;

    /// This is similar to `btmuGetGattIncludedServices` except this only returns 1 `BtmGattService`.
    ///
    /// # Arguments
    /// - `connection_handle` (in): Same as `btmuBleDisconnect`.
    /// - `attribute_handle` (in): AttributeHandle
    /// - `service` (out): `BtmGattService`
    /// - `flag` (out): Whether a `BtmGattService` was returned.
    ///
    /// **Note:** This is similar to `btmGetBelongingService`.
    pub fn btmuGetBelongingGattService(connection_handle: u32, attribute_handle: u16, service: *mut BtmGattService, flag: *mut bool) -> Result;

    /// GetGattCharacteristics
    ///
    /// # Arguments
    /// - `connection_handle` (in): Same as `btmuBleDisconnect`.
    /// - `service_handle` (in): This controls which `BtmGattCharacteristic` entries to return.
    /// - `characteristics` (out): `BtmGattCharacteristic`
    /// - `count` (in): Size of the characteristics array in entries. The max is 100.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** This is similar to `btmGetGattCharacteristics`.
    pub fn btmuGetGattCharacteristics(connection_handle: u32, service_handle: u16, characteristics: *mut BtmGattCharacteristic, count: u8, total_out: *mut u8) -> Result;

    /// GetGattDescriptors
    ///
    /// # Arguments
    /// - `connection_handle` (in): Same as `btmuBleDisconnect`.
    /// - `char_handle` (in): Characteristic handle. This controls which `BtmGattDescriptor` entries to return.
    /// - `descriptors` (out): `BtmGattDescriptor`
    /// - `count` (in): Size of the descriptors array in entries. The max is 100.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** This is similar to `btmGetGattDescriptors`.
    pub fn btmuGetGattDescriptors(connection_handle: u32, char_handle: u16, descriptors: *mut BtmGattDescriptor, count: u8, total_out: *mut u8) -> Result;

    /// AcquireBleMtuConfigEvent
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=true.
    ///
    /// **Note:** This is similar to `btmAcquireBleMtuConfigEvent`.
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn btmuAcquireBleMtuConfigEvent(out_event: *mut Event) -> Result;

    /// ConfigureBleMtu
    ///
    /// # Arguments
    /// - `connection_handle` (in): Same as `btmuBleDisconnect`.
    /// - `mtu` (in): MTU
    ///
    /// **Note:** This is similar to `btmConfigureBleMtu`.
    pub fn btmuConfigureBleMtu(connection_handle: u32, mtu: u16) -> Result;

    /// GetBleMtu
    ///
    /// # Arguments
    /// - `connection_handle` (in): Same as `btmuBleDisconnect`.
    /// - `out` (out): Output MTU.
    ///
    /// **Note:** This is similar to `btmGetBleMtu`.
    pub fn btmuGetBleMtu(connection_handle: u32, out: *mut u16) -> Result;

    /// RegisterBleGattDataPath
    ///
    /// # Arguments
    /// - `path` (in): `BtmBleDataPath`
    ///
    /// **Note:** This is similar to `btmRegisterBleGattDataPath`.
    pub fn btmuRegisterBleGattDataPath(path: *const BtmBleDataPath) -> Result;

    /// UnregisterBleGattDataPath
    ///
    /// # Arguments
    /// - `path` (in): `BtmBleDataPath`
    ///
    /// **Note:** This is similar to `btmUnregisterBleGattDataPath`.
    pub fn btmuUnregisterBleGattDataPath(path: *const BtmBleDataPath) -> Result;
}
