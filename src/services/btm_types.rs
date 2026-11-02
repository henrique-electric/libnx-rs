//! `switch/services/btm_types.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// BtmState
pub type BtmState = u32;
/// NotInitialized
pub const BtmState_NotInitialized: BtmState = 0;
/// RadioOff
pub const BtmState_RadioOff: BtmState = 1;
/// MinorSlept
pub const BtmState_MinorSlept: BtmState = 2;
/// RadioOffMinorSlept
pub const BtmState_RadioOffMinorSlept: BtmState = 3;
/// Slept
pub const BtmState_Slept: BtmState = 4;
/// RadioOffSlept
pub const BtmState_RadioOffSlept: BtmState = 5;
/// Initialized
pub const BtmState_Initialized: BtmState = 6;
/// Working
pub const BtmState_Working: BtmState = 7;

/// BluetoothMode
pub type BtmBluetoothMode = u32;
/// Dynamic2Slot
pub const BtmBluetoothMode_Dynamic2Slot: BtmBluetoothMode = 0;
/// StaticJoy
pub const BtmBluetoothMode_StaticJoy: BtmBluetoothMode = 1;

/// WlanMode
pub type BtmWlanMode = u32;
/// Local4
pub const BtmWlanMode_Local4: BtmWlanMode = 0;
/// Local8
pub const BtmWlanMode_Local8: BtmWlanMode = 1;
/// None
pub const BtmWlanMode_None: BtmWlanMode = 2;

/// TsiMode
pub type BtmTsiMode = u32;
/// 0Fd3Td3Si10
pub const BtmTsiMode_0Fd3Td3Si10: BtmTsiMode = 0;
/// 1Fd1Td1Si5
pub const BtmTsiMode_1Fd1Td1Si5: BtmTsiMode = 1;
/// 2Fd1Td3Si10
pub const BtmTsiMode_2Fd1Td3Si10: BtmTsiMode = 2;
/// 3Fd1Td5Si15
pub const BtmTsiMode_3Fd1Td5Si15: BtmTsiMode = 3;
/// 4Fd3Td1Si10
pub const BtmTsiMode_4Fd3Td1Si10: BtmTsiMode = 4;
/// 5Fd3Td3Si15
pub const BtmTsiMode_5Fd3Td3Si15: BtmTsiMode = 5;
/// 6Fd5Td1Si15
pub const BtmTsiMode_6Fd5Td1Si15: BtmTsiMode = 6;
/// 7Fd1Td3Si15
pub const BtmTsiMode_7Fd1Td3Si15: BtmTsiMode = 7;
/// 8Fd3Td1Si15
pub const BtmTsiMode_8Fd3Td1Si15: BtmTsiMode = 8;
/// 9Fd1Td1Si10
pub const BtmTsiMode_9Fd1Td1Si10: BtmTsiMode = 9;
/// 10Fd1Td1Si15
pub const BtmTsiMode_10Fd1Td1Si15: BtmTsiMode = 10;
/// Active
pub const BtmTsiMode_Active: BtmTsiMode = 255;

/// SlotMode
pub type BtmSlotMode = u32;
/// 2
pub const BtmSlotMode_2: BtmSlotMode = 0;
/// 4
pub const BtmSlotMode_4: BtmSlotMode = 1;
/// 6
pub const BtmSlotMode_6: BtmSlotMode = 2;
/// Active
pub const BtmSlotMode_Active: BtmSlotMode = 3;

/// Profile
pub type BtmProfile = u32;
/// None
pub const BtmProfile_None: BtmProfile = 0;
/// Hid
pub const BtmProfile_Hid: BtmProfile = 1;
/// [13.0.0+] Audio
pub const BtmProfile_Audio: BtmProfile = 2;

/// BdName
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtmBdName {
    /// Name string.
    pub name: [c_char; 32],
}

/// ClassOfDevice
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtmClassOfDevice {
    /// ClassOfDevice
    pub class_of_device: [u8; 3],
}

/// LinkKey
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtmLinkKey {
    /// LinkKey
    pub link_key: [u8; 16],
}

/// HidDeviceInfo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtmHidDeviceInfo {
    /// Vid
    pub vid: u16,
    /// Pid
    pub pid: u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtmHostDeviceProperty_anon0_v1 {
    /// Same as BtdrvAdapterProperty::addr.
    pub addr: BtdrvAddress,
    /// Same as BtdrvAdapterProperty::class_of_device.
    pub class_of_device: BtmClassOfDevice,
    /// Same as BtdrvAdapterProperty::name (except the last byte which is always zero).
    pub name: BtmBdName,
    /// Same as BtdrvAdapterProperty::feature_set.
    pub feature_set: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtmHostDeviceProperty_anon0_v13 {
    /// Same as BtdrvAdapterProperty::addr.
    pub addr: BtdrvAddress,
    /// Same as BtdrvAdapterProperty::class_of_device.
    pub class_of_device: BtmClassOfDevice,
    /// Same as BtdrvAdapterProperty::name (except the last byte which is always zero).
    pub name: [c_char; 249],
    /// Same as BtdrvAdapterProperty::feature_set.
    pub feature_set: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union BtmHostDeviceProperty_anon0 {
    /// [1.0.0-12.1.0]
    pub v1: BtmHostDeviceProperty_anon0_v1,
    /// [13.0.0+]
    pub v13: BtmHostDeviceProperty_anon0_v13,
}

/// HostDeviceProperty
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BtmHostDeviceProperty {
    pub anon0: BtmHostDeviceProperty_anon0,
}

/// BtmConnectedDevice [1.0.0-12.1.0]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtmConnectedDeviceV1 {
    pub address: BtdrvAddress,
    pub pad: [u8; 2],
    pub unk_x8: u32,
    pub name: [c_char; 32],
    pub unk_x2C: [u8; 28],
    pub vid: u16,
    pub pid: u16,
    pub unk_x4C: [u8; 32],
}

/// BtmConnectedDevice [13.0.0+]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtmConnectedDeviceV13 {
    pub address: BtdrvAddress,
    pub pad: [u8; 2],
    /// **Ref:** BtmProfile
    pub profile: u32,
    pub unk_xC: [u8; 64],
    pub name: [c_char; 32],
    pub unk_x6C: [u8; 217],
    pub pad2: [u8; 3],
}

/// DeviceCondition [1.0.0-5.0.2]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtmDeviceConditionV100 {
    pub unk_x0: u32,
    pub unk_x4: u32,
    pub unk_x8: u8,
    pub unk_x9: u8,
    pub max_count: u8,
    pub connected_count: u8,
    pub devices: [BtmConnectedDeviceV1; 8],
}

/// DeviceCondition [5.1.0-7.0.1]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtmDeviceConditionV510 {
    pub unk_x0: u32,
    pub unk_x4: u32,
    pub unk_x8: u8,
    pub unk_x9: [u8; 2],
    pub max_count: u8,
    pub connected_count: u8,
    pub pad: [u8; 3],
    pub devices: [BtmConnectedDeviceV1; 8],
}

/// DeviceCondition [8.0.0-8.1.1]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtmDeviceConditionV800 {
    pub unk_x0: u32,
    pub unk_x4: u32,
    pub unk_x8: u8,
    pub unk_x9: u8,
    pub max_count: u8,
    pub connected_count: u8,
    pub devices: [BtmConnectedDeviceV1; 8],
}

/// DeviceCondition [9.0.0-12.1.0]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtmDeviceConditionV900 {
    pub unk_x0: u32,
    pub unk_x4: u8,
    pub unk_x5: u8,
    pub max_count: u8,
    pub connected_count: u8,
    pub devices: [BtmConnectedDeviceV1; 8],
}

/// DeviceCondition [1.0.0-12.1.0]
#[repr(C)]
#[derive(Clone, Copy)]
pub union BtmDeviceCondition {
    pub v100: BtmDeviceConditionV100,
    pub v510: BtmDeviceConditionV510,
    pub v800: BtmDeviceConditionV800,
    pub v900: BtmDeviceConditionV900,
}

/// DeviceSlotMode
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtmDeviceSlotMode {
    /// **Ref:** BtdrvAddress
    pub addr: BtdrvAddress,
    /// Reserved
    pub reserved: [u8; 2],
    /// **Ref:** BtmSlotMode
    pub slot_mode: u32,
}

/// DeviceSlotModeList
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtmDeviceSlotModeList {
    /// DeviceCount
    pub device_count: u8,
    /// Reserved
    pub reserved: [u8; 3],
    /// Array of `BtmDeviceSlotMode` with the above count.
    pub devices: [BtmDeviceSlotMode; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union BtmDeviceInfoV1_profile_info {
    /// Empty (Profile = None)
    pub data: [u8; 4],
    /// **Ref:** BtmHidDeviceInfo (Profile = Hid)
    pub hid_device_info: BtmHidDeviceInfo,
}

/// DeviceInfo [1.0.0-12.1.0]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BtmDeviceInfoV1 {
    /// **Ref:** BtdrvAddress
    pub addr: BtdrvAddress,
    /// ClassOfDevice
    pub class_of_device: BtmClassOfDevice,
    /// BdName
    pub name: BtmBdName,
    /// LinkKey
    pub link_key: BtmLinkKey,
    /// Reserved
    pub reserved: [u8; 3],
    /// **Ref:** BtmProfile
    pub profile: u32,
    pub profile_info: BtmDeviceInfoV1_profile_info,
    /// Reserved
    pub reserved2: [u8; 28],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union BtmDeviceInfoV13_profile_info {
    /// Empty (Profile = None)
    pub data: [u8; 4],
    /// **Ref:** BtmHidDeviceInfo (Profile = Hid)
    pub hid_device_info: BtmHidDeviceInfo,
}

/// DeviceInfo [13.0.0+]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BtmDeviceInfoV13 {
    /// **Ref:** BtdrvAddress
    pub addr: BtdrvAddress,
    /// ClassOfDevice
    pub class_of_device: BtmClassOfDevice,
    /// LinkKey
    pub link_key: BtmLinkKey,
    /// Reserved
    pub reserved: [u8; 3],
    /// **Ref:** BtmProfile
    pub profile: u32,
    pub profile_info: BtmDeviceInfoV13_profile_info,
    /// Reserved
    pub reserved2: [u8; 28],
    /// Name
    pub name: [c_char; 249],
    /// Padding
    pub pad: [u8; 3],
}

/// DeviceInfo [1.0.0-13.0.0]
#[repr(C)]
#[derive(Clone, Copy)]
pub union BtmDeviceInfo {
    pub v1: BtmDeviceInfoV1,
    pub v13: BtmDeviceInfoV13,
}

/// DeviceInfoList
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BtmDeviceInfoList {
    /// DeviceCount
    pub device_count: u8,
    /// Reserved
    pub reserved: [u8; 3],
    /// Array of `BtmDeviceInfoV1` with the above count.
    pub devices: [BtmDeviceInfoV1; 10],
}

/// DeviceProperty
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtmDeviceProperty {
    /// **Ref:** BtdrvAddress
    pub addr: BtdrvAddress,
    /// ClassOfDevice
    pub class_of_device: BtmClassOfDevice,
    /// BdName
    pub name: BtmBdName,
}

/// DevicePropertyList
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtmDevicePropertyList {
    /// DeviceCount
    pub device_count: u8,
    /// Array of `BtmDeviceProperty`.
    pub devices: [BtmDeviceProperty; 15],
}

/// ZeroRetransmissionList
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtmZeroRetransmissionList {
    /// EnabledReportIdCount
    pub enabled_report_id_count: u8,
    /// Array of EnabledReportId.
    pub enabled_report_id: [u8; 16],
}

/// GattClientConditionList
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtmGattClientConditionList {
    /// Unknown
    pub unk_x0: [u8; 116],
}

/// GattService
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtmGattService {
    /// Unknown
    pub unk_x0: [u8; 4],
    /// **Ref:** BtdrvGattAttributeUuid
    pub uuid: BtdrvGattAttributeUuid,
    /// Handle
    pub handle: u16,
    /// Unknown
    pub unk_x1A: [u8; 2],
    /// InstanceId
    pub instance_id: u16,
    /// EndGroupHandle
    pub end_group_handle: u16,
    /// PrimaryService
    pub primary_service: u8,
    /// Padding
    pub pad: [u8; 3],
}

/// GattCharacteristic
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtmGattCharacteristic {
    /// Unknown
    pub unk_x0: [u8; 4],
    /// **Ref:** BtdrvGattAttributeUuid
    pub uuid: BtdrvGattAttributeUuid,
    /// Handle
    pub handle: u16,
    /// Unknown
    pub unk_x1A: [u8; 2],
    /// InstanceId
    pub instance_id: u16,
    /// Properties
    pub properties: u8,
    /// Unknown
    pub unk_x1F: [u8; 5],
}

/// GattDescriptor
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtmGattDescriptor {
    /// Unknown
    pub unk_x0: [u8; 4],
    /// **Ref:** BtdrvGattAttributeUuid
    pub uuid: BtdrvGattAttributeUuid,
    /// Handle
    pub handle: u16,
    /// Unknown
    pub unk_x1A: [u8; 6],
}

/// BleDataPath
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtmBleDataPath {
    /// Unknown
    pub unk_x0: u8,
    /// Padding
    pub pad: [u8; 3],
    /// **Ref:** BtdrvGattAttributeUuid
    pub uuid: BtdrvGattAttributeUuid,
}

/// AudioDevice
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtmAudioDevice {
    /// Device address
    pub addr: BtdrvAddress,
    /// Device name
    pub name: [c_char; 249],
}

impl_zeroed_default!(BtmBdName, BtmClassOfDevice, BtmLinkKey, BtmHidDeviceInfo, BtmHostDeviceProperty_anon0_v1, BtmHostDeviceProperty_anon0_v13, BtmHostDeviceProperty_anon0, BtmHostDeviceProperty, BtmConnectedDeviceV1, BtmConnectedDeviceV13, BtmDeviceConditionV100, BtmDeviceConditionV510, BtmDeviceConditionV800, BtmDeviceConditionV900, BtmDeviceCondition, BtmDeviceSlotMode, BtmDeviceSlotModeList, BtmDeviceInfoV1_profile_info, BtmDeviceInfoV1, BtmDeviceInfoV13_profile_info, BtmDeviceInfoV13, BtmDeviceInfo, BtmDeviceInfoList, BtmDeviceProperty, BtmDevicePropertyList, BtmZeroRetransmissionList, BtmGattClientConditionList, BtmGattService, BtmGattCharacteristic, BtmGattDescriptor, BtmBleDataPath, BtmAudioDevice);
