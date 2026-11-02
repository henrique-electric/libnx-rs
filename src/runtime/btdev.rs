//! `switch/runtime/btdev.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// GattAttribute
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdevGattAttribute {
    /// Type
    pub type_: u8,
    /// **Ref:** BtdrvGattAttributeUuid
    pub uuid: BtdrvGattAttributeUuid,
    /// Handle
    pub handle: u16,
    /// ConnectionHandle
    pub connection_handle: u32,
}

/// GattService
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdevGattService {
    /// **Ref:** BtdevGattAttribute
    pub attr: BtdevGattAttribute,
    /// InstanceId
    pub instance_id: u16,
    /// EndGroupHandle
    pub end_group_handle: u16,
    /// PrimaryService
    pub primary_service: bool,
}

/// GattCharacteristic
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdevGattCharacteristic {
    /// **Ref:** BtdevGattAttribute
    pub attr: BtdevGattAttribute,
    /// InstanceId
    pub instance_id: u16,
    /// Properties
    pub properties: u8,
    /// Size of value.
    pub value_size: u64,
    /// Value
    pub value: [u8; 512],
}

/// GattDescriptor
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdevGattDescriptor {
    /// **Ref:** BtdevGattAttribute
    pub attr: BtdevGattAttribute,
    /// Size of value.
    pub value_size: u64,
    /// Value
    pub value: [u8; 512],
}

/// Gets the Type.
///
/// # Arguments
/// - `a`: `BtdevGattAttribute`
#[inline(always)]
pub fn btdevGattAttributeGetType(a: &BtdevGattAttribute) -> u8 {
    a.type_
}

/// Gets the Uuid.
///
/// # Arguments
/// - `a`: `BtdevGattAttribute`
/// - `out` (out): `BtdrvGattAttributeUuid`
#[inline(always)]
pub fn btdevGattAttributeGetUuid(a: &BtdevGattAttribute, out: &mut BtdrvGattAttributeUuid) {
    *out = a.uuid;
}

/// Gets the Handle.
///
/// # Arguments
/// - `a`: `BtdevGattAttribute`
#[inline(always)]
pub fn btdevGattAttributeGetHandle(a: &BtdevGattAttribute) -> u16 {
    a.handle
}

/// Gets the ConnectionHandle.
///
/// # Arguments
/// - `a`: `BtdevGattAttribute`
#[inline(always)]
pub fn btdevGattAttributeGetConnectionHandle(a: &BtdevGattAttribute) -> u32 {
    a.connection_handle
}

/// Gets the InstanceId.
///
/// # Arguments
/// - `s`: `BtdevGattService`
#[inline(always)]
pub fn btdevGattServiceGetInstanceId(s: &BtdevGattService) -> u16 {
    s.instance_id
}

/// Gets the EndGroupHandle.
///
/// # Arguments
/// - `s`: `BtdevGattService`
#[inline(always)]
pub fn btdevGattServiceGetEndGroupHandle(s: &BtdevGattService) -> u16 {
    s.end_group_handle
}

/// Gets whether this is the PrimaryService.
///
/// # Arguments
/// - `s`: `BtdevGattService`
#[inline(always)]
pub fn btdevGattServiceIsPrimaryService(s: &BtdevGattService) -> u16 {
    s.primary_service as u16
}

/// Gets the InstanceId.
///
/// # Arguments
/// - `c`: `BtdevGattCharacteristic`
#[inline(always)]
pub fn btdevGattCharacteristicGetInstanceId(c: &BtdevGattCharacteristic) -> u16 {
    c.instance_id
}

/// Gets the Properties.
///
/// # Arguments
/// - `c`: `BtdevGattCharacteristic`
#[inline(always)]
pub fn btdevGattCharacteristicGetProperties(c: &BtdevGattCharacteristic) -> u8 {
    c.properties
}

impl_zeroed_default!(BtdevGattAttribute, BtdevGattService, BtdevGattCharacteristic, BtdevGattDescriptor);

unsafe extern "C" {
    /// Initialize bt/btmu.
    pub fn btdevInitialize() -> Result;

    /// Exit bt/btmu.
    pub fn btdevExit();

    /// Compares two `BtdrvGattAttributeUuid`, returning whether these match.
    pub fn btdevGattAttributeUuidIsSame(a: *const BtdrvGattAttributeUuid, b: *const BtdrvGattAttributeUuid) -> bool;

    /// Wrapper for `btmuAcquireBleScanEvent`.
    pub fn btdevAcquireBleScanEvent(out_event: *mut Event) -> Result;

    /// Wrapper for `btmuGetBleScanFilterParameter`.
    pub fn btdevGetBleScanParameter(parameter_id: u16, out: *mut BtdrvBleAdvertisePacketParameter) -> Result;

    /// Wrapper for `btmuGetBleScanFilterParameter2`.
    pub fn btdevGetBleScanParameter2(parameter_id: u16, out: *mut BtdrvGattAttributeUuid) -> Result;

    /// Wrapper for `btdevStartBleScanGeneral`.
    pub fn btdevStartBleScanGeneral(param: BtdrvBleAdvertisePacketParameter) -> Result;

    /// Wrapper for `btmuStopBleScanForGeneral`.
    pub fn btdevStopBleScanGeneral() -> Result;

    /// Wrapper for `btmuGetBleScanResultsForGeneral` and `btmuGetBleScanResultsForSmartDevice`.
    ///
    /// # Arguments
    /// - `results` (out): Output array of `BtdrvBleScanResult`.
    /// - `count` (in): Size of the results array in entries.
    /// - `total_out` (out): Total output entries.
    pub fn btdevGetBleScanResult(results: *mut BtdrvBleScanResult, count: u8, total_out: *mut u8) -> Result;

    /// Wrapper for `btmuStartBleScanForPaired`.
    pub fn btdevEnableBleAutoConnection(param: BtdrvBleAdvertisePacketParameter) -> Result;

    /// Wrapper for `btmuStopBleScanForPaired`.
    pub fn btdevDisableBleAutoConnection() -> Result;

    /// Wrapper for `btmuStartBleScanForSmartDevice`.
    pub fn btdevStartBleScanSmartDevice(uuid: *const BtdrvGattAttributeUuid) -> Result;

    /// Wrapper for `btmuStopBleScanForSmartDevice`.
    pub fn btdevStopBleScanSmartDevice() -> Result;

    /// Wrapper for `btmuAcquireBleConnectionEvent`.
    pub fn btdevAcquireBleConnectionStateChangedEvent(out_event: *mut Event) -> Result;

    /// Wrapper for `btmuBleConnect`.
    pub fn btdevConnectToGattServer(addr: BtdrvAddress) -> Result;

    /// Wrapper for `btmuBleDisconnect`.
    pub fn btdevDisconnectFromGattServer(connection_handle: u32) -> Result;

    /// Wrapper for `btmuBleGetConnectionState`.
    pub fn btdevGetBleConnectionInfoList(info: *mut BtdrvBleConnectionInfo, count: u8, total_out: *mut u8) -> Result;

    /// Wrapper for `btmuAcquireBleServiceDiscoveryEvent`.
    pub fn btdevAcquireBleServiceDiscoveryEvent(out_event: *mut Event) -> Result;

    /// Wrapper for `btmuGetGattServices`.
    ///
    /// # Arguments
    /// - `connection_handle` (in): ConnectionHandle
    /// - `services` (out): Output array of `BtdevGattService`.
    /// - `count` (in): Size of the services array in entries. The max is 100.
    /// - `total_out` (out): Total output entries.
    pub fn btdevGetGattServices(connection_handle: u32, services: *mut BtdevGattService, count: u8, total_out: *mut u8) -> Result;

    /// Wrapper for `btmuGetGattService`.
    ///
    /// # Arguments
    /// - `connection_handle` (in): ConnectionHandle
    /// - `uuid` (in): `BtdrvGattAttributeUuid`
    /// - `service` (out): `BtdevGattService`
    /// - `flag` (out): Whether a `BtdevGattService` was returned.
    pub fn btdevGetGattService(connection_handle: u32, uuid: *const BtdrvGattAttributeUuid, service: *mut BtdevGattService, flag: *mut bool) -> Result;

    /// Wrapper for `btmuAcquireBlePairingEvent`.
    pub fn btdevAcquireBlePairingEvent(out_event: *mut Event) -> Result;

    /// Wrapper for `btmuBlePairDevice`.
    pub fn btdevPairGattServer(connection_handle: u32, param: BtdrvBleAdvertisePacketParameter) -> Result;

    /// Wrapper for `btmuBleUnPairDevice`.
    pub fn btdevUnpairGattServer(connection_handle: u32, param: BtdrvBleAdvertisePacketParameter) -> Result;

    /// Wrapper for `btmuBleUnPairDevice2`.
    pub fn btdevUnpairGattServer2(addr: BtdrvAddress, param: BtdrvBleAdvertisePacketParameter) -> Result;

    /// Wrapper for `btmuBleGetPairedDevices`.
    pub fn btdevGetPairedGattServerAddress(param: BtdrvBleAdvertisePacketParameter, addrs: *mut BtdrvAddress, count: u8, total_out: *mut u8) -> Result;

    /// Wrapper for `btmuAcquireBleMtuConfigEvent`.
    pub fn btdevAcquireBleMtuConfigEvent(out_event: *mut Event) -> Result;

    /// Wrapper for `btmuConfigureBleMtu`.
    ///
    /// # Arguments
    /// - `connection_handle` (in): Same as `btmuBleDisconnect`.
    /// - `mtu` (in): MTU, must be 0x18-0x200.
    pub fn btdevConfigureBleMtu(connection_handle: u32, mtu: u16) -> Result;

    /// Wrapper for `btmuGetBleMtu`.
    pub fn btdevGetBleMtu(connection_handle: u32, out: *mut u16) -> Result;

    /// Wrapper for `btRegisterBleEvent`.
    pub fn btdevAcquireBleGattOperationEvent(out_event: *mut Event) -> Result;

    /// Wrapper for `btmuRegisterBleGattDataPath`.
    ///
    /// # Arguments
    /// - `uuid` (in): `BtdrvGattAttributeUuid`
    pub fn btdevRegisterGattOperationNotification(uuid: *const BtdrvGattAttributeUuid) -> Result;

    /// Wrapper for `btmuUnregisterBleGattDataPath`.
    ///
    /// # Arguments
    /// - `uuid` (in): `BtdrvGattAttributeUuid`
    pub fn btdevUnregisterGattOperationNotification(uuid: *const BtdrvGattAttributeUuid) -> Result;

    /// Wrapper for `btGetLeEventInfo`.
    ///
    /// # Arguments
    /// - `out` (out): `BtdrvBleClientGattOperationInfo`
    pub fn btdevGetGattOperationResult(out: *mut BtdrvBleClientGattOperationInfo) -> Result;

    /// Wrapper for `btLeClientReadCharacteristic`.
    ///
    /// # Arguments
    /// - `c`: `BtdevGattCharacteristic`
    ///
    /// **Note:** An error is thrown if the properties from `btdevGattCharacteristicGetProperties` don't allow using this.
    pub fn btdevReadGattCharacteristic(c: *mut BtdevGattCharacteristic) -> Result;

    /// Wrapper for `btLeClientWriteCharacteristic`.
    ///
    /// # Arguments
    /// - `c`: `BtdevGattCharacteristic`
    ///
    /// **Note:** An error is thrown if the properties from `btdevGattCharacteristicGetProperties` don't allow using this.
    ///
    /// **Note:** This uses the Value from `btdevGattCharacteristicSetValue`.
    pub fn btdevWriteGattCharacteristic(c: *mut BtdevGattCharacteristic) -> Result;

    /// Wrapper for `btLeClientRegisterNotification` / `btLeClientDeregisterNotification`.
    ///
    /// # Arguments
    /// - `c`: `BtdevGattCharacteristic`
    /// - `flag` (in): Whether to enable/disable, controls which func to call.
    ///
    /// **Note:** An error is thrown if the properties from `btdevGattCharacteristicGetProperties` don't allow using this.
    pub fn btdevEnableGattCharacteristicNotification(c: *mut BtdevGattCharacteristic, flag: bool) -> Result;

    /// Wrapper for `btLeClientReadDescriptor`.
    ///
    /// # Arguments
    /// - `d`: `BtdevGattDescriptor`
    pub fn btdevReadGattDescriptor(d: *mut BtdevGattDescriptor) -> Result;

    /// Wrapper for `btLeClientWriteDescriptor`.
    ///
    /// # Arguments
    /// - `d`: `BtdevGattDescriptor`
    ///
    /// **Note:** This uses the Value from `btdevGattDescriptorSetValue`.
    pub fn btdevWriteGattDescriptor(d: *mut BtdevGattDescriptor) -> Result;

    /// Creates a `BtdevGattAttribute` object. This is intended for internal use.
    ///
    /// # Arguments
    /// - `a`: `BtdevGattAttribute`
    /// - `uuid` (in): `BtdrvGattAttributeUuid`
    /// - `handle` (in): Handle
    /// - `connection_handle` (in): ConnectionHandle
    pub fn btdevGattAttributeCreate(a: *mut BtdevGattAttribute, uuid: *const BtdrvGattAttributeUuid, handle: u16, connection_handle: u32);

    /// Creates a `BtdevGattService` object. This is intended for internal use.
    ///
    /// # Arguments
    /// - `s`: `BtdevGattService`
    /// - `uuid` (in): `BtdrvGattAttributeUuid`
    /// - `handle` (in): Handle
    /// - `connection_handle` (in): ConnectionHandle
    /// - `instance_id` (in): InstanceId
    /// - `end_group_handle` (in): EndGroupHandle
    /// - `primary_service` (in): PrimaryService
    pub fn btdevGattServiceCreate(s: *mut BtdevGattService, uuid: *const BtdrvGattAttributeUuid, handle: u16, connection_handle: u32, instance_id: u16, end_group_handle: u16, primary_service: bool);

    /// Wrapper for `btmuGetGattIncludedServices`.
    ///
    /// # Arguments
    /// - `s`: `BtdevGattService`
    /// - `services` (out): Output array of `BtdevGattService`.
    /// - `count` (in): Size of the services array in entries. The max is 100.
    /// - `total_out` (out): Total output entries.
    pub fn btdevGattServiceGetIncludedServices(s: *mut BtdevGattService, services: *mut BtdevGattService, count: u8, total_out: *mut u8) -> Result;

    /// Wrapper for `btmuGetGattCharacteristics`.
    ///
    /// # Arguments
    /// - `s`: `BtdevGattService`
    /// - `characteristics` (out): Output array of `BtdevGattCharacteristic`.
    /// - `count` (in): Size of the characteristics array in entries. The max is 100.
    /// - `total_out` (out): Total output entries.
    pub fn btdevGattServiceGetCharacteristics(s: *mut BtdevGattService, characteristics: *mut BtdevGattCharacteristic, count: u8, total_out: *mut u8) -> Result;

    /// Same as `btdevGattServiceGetCharacteristics` except this only returns the `BtdevGattCharacteristic` which contains a matching `BtdrvGattAttributeUuid`.
    ///
    /// # Arguments
    /// - `s`: `BtdevGattService`
    /// - `uuid` (in): `BtdrvGattAttributeUuid`
    /// - `characteristic` (out): `BtdevGattCharacteristic`
    /// - `flag` (out): Whether a `BtdevGattService` was returned.
    pub fn btdevGattServiceGetCharacteristic(s: *mut BtdevGattService, uuid: *const BtdrvGattAttributeUuid, characteristic: *mut BtdevGattCharacteristic, flag: *mut bool) -> Result;

    /// Creates a `BtdevGattCharacteristic` object. This is intended for internal use.
    ///
    /// # Arguments
    /// - `c`: `BtdevGattCharacteristic`
    /// - `uuid` (in): `BtdrvGattAttributeUuid`
    /// - `handle` (in): Handle
    /// - `connection_handle` (in): ConnectionHandle
    /// - `instance_id` (in): InstanceId
    /// - `properties` (in): Properties
    pub fn btdevGattCharacteristicCreate(c: *mut BtdevGattCharacteristic, uuid: *const BtdrvGattAttributeUuid, handle: u16, connection_handle: u32, instance_id: u16, properties: u8);

    /// Wrapper for `btmuGetBelongingGattService`.
    ///
    /// # Arguments
    /// - `c`: `BtdevGattCharacteristic`.
    /// - `service` (out): `BtdevGattService`
    ///
    /// **Note:** Gets the `BtdevGattService` which belongs to this object.
    pub fn btdevGattCharacteristicGetService(c: *mut BtdevGattCharacteristic, service: *mut BtdevGattService) -> Result;

    /// Wrapper for `btmuGetGattDescriptors`.
    ///
    /// # Arguments
    /// - `c`: `BtdevGattCharacteristic`
    /// - `descriptors` (out): Output array of `BtdevGattDescriptor`.
    /// - `count` (in): Size of the descriptors array in entries. The max is 100.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** Gets the descriptors which belongs to this object.
    pub fn btdevGattCharacteristicGetDescriptors(c: *mut BtdevGattCharacteristic, descriptors: *mut BtdevGattDescriptor, count: u8, total_out: *mut u8) -> Result;

    /// Same as `btdevGattCharacteristicGetDescriptors` except this only returns a `BtdevGattDescriptor` which contains a matching `BtdrvGattAttributeUuid`.
    ///
    /// # Arguments
    /// - `c`: `BtdevGattCharacteristic`
    /// - `uuid` (in): `BtdrvGattAttributeUuid`
    /// - `descriptor` (out): `BtdevGattDescriptor`
    /// - `flag` (out): Whether a `BtdevGattDescriptor` was returned.
    pub fn btdevGattCharacteristicGetDescriptor(c: *mut BtdevGattCharacteristic, uuid: *const BtdrvGattAttributeUuid, descriptor: *mut BtdevGattDescriptor, flag: *mut bool) -> Result;

    /// Sets the Value in the object.
    ///
    /// # Arguments
    /// - `c`: `BtdevGattCharacteristic`
    /// - `buffer` (in): Input buffer.
    /// - `size` (in): Input buffer size, max is 0x200.
    ///
    /// **Note:** See also `btdevWriteGattCharacteristic`.
    pub fn btdevGattCharacteristicSetValue(c: *mut BtdevGattCharacteristic, buffer: *const c_void, size: usize);

    /// Gets the Value in the object, returns the copied value size.
    ///
    /// # Arguments
    /// - `c`: `BtdevGattCharacteristic`
    /// - `buffer` (out): Output buffer.
    /// - `size` (in): Output buffer size, max is 0x200.
    pub fn btdevGattCharacteristicGetValue(c: *mut BtdevGattCharacteristic, buffer: *mut c_void, size: usize) -> u64;

    /// Creates a `BtdevGattDescriptor` object. This is intended for internal use.
    ///
    /// # Arguments
    /// - `d`: `BtdevGattDescriptor`
    /// - `uuid` (in): `BtdrvGattAttributeUuid`
    /// - `handle` (in): Handle
    /// - `connection_handle` (in): ConnectionHandle
    pub fn btdevGattDescriptorCreate(d: *mut BtdevGattDescriptor, uuid: *const BtdrvGattAttributeUuid, handle: u16, connection_handle: u32);

    /// Wrapper for `btmuGetBelongingGattService`.
    ///
    /// # Arguments
    /// - `d`: `BtdevGattDescriptor`
    /// - `service` (out): `BtdevGattService`
    ///
    /// **Note:** Gets the `BtdevGattService` which belongs to this object.
    pub fn btdevGattDescriptorGetService(d: *mut BtdevGattDescriptor, service: *mut BtdevGattService) -> Result;

    /// Wrapper for `btmuGetGattCharacteristics`.
    ///
    /// # Arguments
    /// - `d`: `BtdevGattDescriptor`
    /// - `characteristic` (out): `BtdevGattCharacteristic`
    ///
    /// **Note:** Gets the `BtdevGattCharacteristic` which belongs to this object.
    pub fn btdevGattDescriptorGetCharacteristic(d: *mut BtdevGattDescriptor, characteristic: *mut BtdevGattCharacteristic) -> Result;

    /// Sets the Value in the object.
    ///
    /// # Arguments
    /// - `d`: `BtdevGattDescriptor`
    /// - `buffer` (in): Input buffer.
    /// - `size` (in): Input buffer size, max is 0x200.
    ///
    /// **Note:** See also `btdevWriteGattDescriptor`.
    pub fn btdevGattDescriptorSetValue(d: *mut BtdevGattDescriptor, buffer: *const c_void, size: usize);

    /// Gets the Value in the object, returns the copied value size.
    ///
    /// # Arguments
    /// - `d`: `BtdevGattDescriptor`
    /// - `buffer` (out): Output buffer.
    /// - `size` (in): Output buffer size, max is 0x200.
    pub fn btdevGattDescriptorGetValue(d: *mut BtdevGattDescriptor, buffer: *mut c_void, size: usize) -> u64;
}
