//! `switch/services/btdrv_types.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// BluetoothPropertyType [1.0.0-11.0.1]
pub type BtdrvBluetoothPropertyType = u32;
/// Name. String, max length 0xF8 excluding NUL-terminator.
pub const BtdrvBluetoothPropertyType_Name: BtdrvBluetoothPropertyType = 1;
/// **Ref:** BtdrvAddress
pub const BtdrvBluetoothPropertyType_Address: BtdrvBluetoothPropertyType = 2;
/// Only available with `btdrvSetAdapterProperty`. Unknown, `BtdrvAddress`.
pub const BtdrvBluetoothPropertyType_Unknown3: BtdrvBluetoothPropertyType = 3;
/// 3-bytes, Class of Device.
pub const BtdrvBluetoothPropertyType_ClassOfDevice: BtdrvBluetoothPropertyType = 5;
/// 1-byte, FeatureSet. The default is value 0x68.
pub const BtdrvBluetoothPropertyType_FeatureSet: BtdrvBluetoothPropertyType = 6;

/// AdapterPropertyType [12.0.0+]
pub type BtdrvAdapterPropertyType = u32;
/// **Ref:** BtdrvAddress
pub const BtdrvAdapterPropertyType_Address: BtdrvAdapterPropertyType = 0;
/// Name. String, max length 0xF8 excluding NUL-terminator.
pub const BtdrvAdapterPropertyType_Name: BtdrvAdapterPropertyType = 1;
/// 3-bytes, Class of Device.
pub const BtdrvAdapterPropertyType_ClassOfDevice: BtdrvAdapterPropertyType = 2;
/// Only available with `btdrvSetAdapterProperty`. Unknown, `BtdrvAddress`.
pub const BtdrvAdapterPropertyType_Unknown3: BtdrvAdapterPropertyType = 3;

/// EventType
pub type BtdrvEventType = u32;
/// Device found during Inquiry.
pub const BtdrvEventType_InquiryDevice: BtdrvEventType = 0;
/// Inquiry status changed.
pub const BtdrvEventType_InquiryStatus: BtdrvEventType = 1;
/// Pairing PIN code request.
pub const BtdrvEventType_PairingPinCodeRequest: BtdrvEventType = 2;
/// SSP confirm request / SSP passkey notification.
pub const BtdrvEventType_SspRequest: BtdrvEventType = 3;
/// Connection
pub const BtdrvEventType_Connection: BtdrvEventType = 4;
/// SetTsi (`btdrvSetTsi`)
pub const BtdrvEventType_Tsi: BtdrvEventType = 5;
/// SetBurstMode (`btdrvEnableBurstMode`)
pub const BtdrvEventType_BurstMode: BtdrvEventType = 6;
/// **Ref:** btdrvSetZeroRetransmission
pub const BtdrvEventType_SetZeroRetransmission: BtdrvEventType = 7;
/// **Ref:** btdrvGetPendingConnections
pub const BtdrvEventType_PendingConnections: BtdrvEventType = 8;
/// **Ref:** btdrvMoveToSecondaryPiconet
pub const BtdrvEventType_MoveToSecondaryPiconet: BtdrvEventType = 9;
/// BluetoothCrash
pub const BtdrvEventType_BluetoothCrash: BtdrvEventType = 10;
/// Unused
pub const BtdrvEventTypeOld_Unknown0: BtdrvEventType = 0;
/// Device found during Inquiry.
pub const BtdrvEventTypeOld_InquiryDevice: BtdrvEventType = 3;
/// Inquiry status changed.
pub const BtdrvEventTypeOld_InquiryStatus: BtdrvEventType = 4;
/// Pairing PIN code request.
pub const BtdrvEventTypeOld_PairingPinCodeRequest: BtdrvEventType = 5;
/// SSP confirm request / SSP passkey notification.
pub const BtdrvEventTypeOld_SspRequest: BtdrvEventType = 6;
/// Connection
pub const BtdrvEventTypeOld_Connection: BtdrvEventType = 7;
/// BluetoothCrash
pub const BtdrvEventTypeOld_BluetoothCrash: BtdrvEventType = 13;

/// BtdrvInquiryStatus
pub type BtdrvInquiryStatus = u32;
/// Inquiry stopped.
pub const BtdrvInquiryStatus_Stopped: BtdrvInquiryStatus = 0;
/// Inquiry started.
pub const BtdrvInquiryStatus_Started: BtdrvInquiryStatus = 1;

/// ConnectionEventType
pub type BtdrvConnectionEventType = u32;
/// BtdrvEventInfo::connection::status
pub const BtdrvConnectionEventType_Status: BtdrvConnectionEventType = 0;
/// SSP confirm request.
pub const BtdrvConnectionEventType_SspConfirmRequest: BtdrvConnectionEventType = 1;
/// ACL Link is now Suspended.
pub const BtdrvConnectionEventType_Suspended: BtdrvConnectionEventType = 2;

/// ExtEventType [1.0.0-11.0.1]
pub type BtdrvExtEventType = u32;
/// SetTsi (`btdrvSetTsi`)
pub const BtdrvExtEventType_SetTsi: BtdrvExtEventType = 0;
/// ExitTsi (`btdrvSetTsi`)
pub const BtdrvExtEventType_ExitTsi: BtdrvExtEventType = 1;
/// SetBurstMode (`btdrvEnableBurstMode`)
pub const BtdrvExtEventType_SetBurstMode: BtdrvExtEventType = 2;
/// ExitBurstMode (`btdrvEnableBurstMode`)
pub const BtdrvExtEventType_ExitBurstMode: BtdrvExtEventType = 3;
/// **Ref:** btdrvSetZeroRetransmission
pub const BtdrvExtEventType_SetZeroRetransmission: BtdrvExtEventType = 4;
/// **Ref:** btdrvGetPendingConnections
pub const BtdrvExtEventType_PendingConnections: BtdrvExtEventType = 5;
/// **Ref:** btdrvMoveToSecondaryPiconet
pub const BtdrvExtEventType_MoveToSecondaryPiconet: BtdrvExtEventType = 6;

/// BluetoothHhReportType
/// Bit0-1 directly control the HID bluetooth transaction report-type value.
/// Bit2-3: these directly control the Parameter Reserved field for SetReport, for GetReport these control the Parameter Reserved and Size bits.
pub type BtdrvBluetoothHhReportType = u32;
/// Other
pub const BtdrvBluetoothHhReportType_Other: BtdrvBluetoothHhReportType = 0;
/// Input
pub const BtdrvBluetoothHhReportType_Input: BtdrvBluetoothHhReportType = 1;
/// Output
pub const BtdrvBluetoothHhReportType_Output: BtdrvBluetoothHhReportType = 2;
/// Feature
pub const BtdrvBluetoothHhReportType_Feature: BtdrvBluetoothHhReportType = 3;

/// HidEventType
pub type BtdrvHidEventType = u32;
/// Connection. Only used with `btdrvGetHidEventInfo`.
pub const BtdrvHidEventType_Connection: BtdrvHidEventType = 0;
/// DATA report on the Interrupt channel.
pub const BtdrvHidEventType_Data: BtdrvHidEventType = 1;
/// Response to SET_REPORT.
pub const BtdrvHidEventType_SetReport: BtdrvHidEventType = 2;
/// Response to GET_REPORT.
pub const BtdrvHidEventType_GetReport: BtdrvHidEventType = 3;
/// Connection. Only used with `btdrvGetHidEventInfo`.
pub const BtdrvHidEventTypeOld_Connection: BtdrvHidEventType = 0;
/// DATA report on the Interrupt channel.
pub const BtdrvHidEventTypeOld_Data: BtdrvHidEventType = 4;
/// Response for extensions. Only used with `btdrvGetHidEventInfo`.
pub const BtdrvHidEventTypeOld_Ext: BtdrvHidEventType = 7;
/// Response to SET_REPORT.
pub const BtdrvHidEventTypeOld_SetReport: BtdrvHidEventType = 8;
/// Response to GET_REPORT.
pub const BtdrvHidEventTypeOld_GetReport: BtdrvHidEventType = 9;

/// HidConnectionStatus [12.0.0+]
pub type BtdrvHidConnectionStatus = u32;
pub const BtdrvHidConnectionStatus_Closed: BtdrvHidConnectionStatus = 0;
pub const BtdrvHidConnectionStatus_Opened: BtdrvHidConnectionStatus = 1;
pub const BtdrvHidConnectionStatus_Failed: BtdrvHidConnectionStatus = 2;
pub const BtdrvHidConnectionStatusOld_Opened: BtdrvHidConnectionStatus = 0;
pub const BtdrvHidConnectionStatusOld_Closed: BtdrvHidConnectionStatus = 2;
pub const BtdrvHidConnectionStatusOld_Failed: BtdrvHidConnectionStatus = 8;

/// This determines the u16 data to write into a CircularBuffer.
pub type BtdrvFatalReason = u32;
/// Only for `BtdrvEventInfo`: invalid.
pub const BtdrvFatalReason_Invalid: BtdrvFatalReason = 0;
/// Can only be triggered by `btdrvEmulateBluetoothCrash`, not triggered by the sysmodule otherwise.
pub const BtdrvFatalReason_Unknown1: BtdrvFatalReason = 1;
/// HCI command timeout.
pub const BtdrvFatalReason_CommandTimeout: BtdrvFatalReason = 2;
/// HCI event HCI_Hardware_Error occurred.
pub const BtdrvFatalReason_HardwareError: BtdrvFatalReason = 3;
/// Only for `BtdrvEventInfo`: triggered after enabling bluetooth, depending on the value of a global state field.
pub const BtdrvFatalReason_Enable: BtdrvFatalReason = 7;
/// [12.0.0+] Only for `BtdrvEventInfo`: triggered by Audio cmds in some cases.
pub const BtdrvFatalReason_Audio: BtdrvFatalReason = 9;

/// BleEventType
pub type BtdrvBleEventType = u32;
/// GATT client registration.
pub const BtdrvBleEventType_ClientRegistration: BtdrvBleEventType = 0;
/// GATT server registration.
pub const BtdrvBleEventType_ServerRegistration: BtdrvBleEventType = 1;
/// Connection update.
pub const BtdrvBleEventType_ConnectionUpdate: BtdrvBleEventType = 2;
/// Preferred connection parameters.
pub const BtdrvBleEventType_PreferredConnectionParameters: BtdrvBleEventType = 3;
/// GATT client connection.
pub const BtdrvBleEventType_ClientConnection: BtdrvBleEventType = 4;
/// GATT server connection.
pub const BtdrvBleEventType_ServerConnection: BtdrvBleEventType = 5;
/// Scan result.
pub const BtdrvBleEventType_ScanResult: BtdrvBleEventType = 6;
/// Scan filter status.
pub const BtdrvBleEventType_ScanFilter: BtdrvBleEventType = 7;
/// GATT client notify.
pub const BtdrvBleEventType_ClientNotify: BtdrvBleEventType = 8;
/// GATT client cache save.
pub const BtdrvBleEventType_ClientCacheSave: BtdrvBleEventType = 9;
/// GATT client cache load.
pub const BtdrvBleEventType_ClientCacheLoad: BtdrvBleEventType = 10;
/// GATT client configure MTU.
pub const BtdrvBleEventType_ClientConfigureMtu: BtdrvBleEventType = 11;
/// GATT server add attribute.
pub const BtdrvBleEventType_ServerAddAttribute: BtdrvBleEventType = 12;
/// GATT server attribute operation.
pub const BtdrvBleEventType_ServerAttributeOperation: BtdrvBleEventType = 13;

/// GattAttributeType
pub type BtdrvGattAttributeType = u32;
/// Included service
pub const BtdrvGattAttributeType_IncludedService: BtdrvGattAttributeType = 0;
/// Characteristic
pub const BtdrvGattAttributeType_Characteristic: BtdrvGattAttributeType = 1;
/// Descriptor
pub const BtdrvGattAttributeType_Descriptor: BtdrvGattAttributeType = 2;
/// Service
pub const BtdrvGattAttributeType_Service: BtdrvGattAttributeType = 3;

/// GattAttributePermission
pub type BtdrvGattAttributePermission = u32;
pub const BtdrvGattAttributePermission_Read: BtdrvGattAttributePermission = BIT(0);
pub const BtdrvGattAttributePermission_ReadEncrypted: BtdrvGattAttributePermission = BIT(1);
pub const BtdrvGattAttributePermission_ReadEncryptedMitm: BtdrvGattAttributePermission = BIT(2);
pub const BtdrvGattAttributePermission_Write: BtdrvGattAttributePermission = BIT(4);
pub const BtdrvGattAttributePermission_WriteEncrypted: BtdrvGattAttributePermission = BIT(5);
pub const BtdrvGattAttributePermission_WriteEncryptedMitm: BtdrvGattAttributePermission = BIT(6);
pub const BtdrvGattAttributePermission_WriteSigned: BtdrvGattAttributePermission = BIT(7);
pub const BtdrvGattAttributePermission_WriteSignedMitm: BtdrvGattAttributePermission = BIT(8);
pub const BtdrvGattAttributePermission_ReadAllowed: BtdrvGattAttributePermission = BtdrvGattAttributePermission_Read | BtdrvGattAttributePermission_ReadEncrypted | BtdrvGattAttributePermission_ReadEncryptedMitm;
pub const BtdrvGattAttributePermission_ReadAuthRequired: BtdrvGattAttributePermission = BtdrvGattAttributePermission_ReadEncrypted;
pub const BtdrvGattAttributePermission_ReadMitmRequired: BtdrvGattAttributePermission = BtdrvGattAttributePermission_ReadEncryptedMitm;
pub const BtdrvGattAttributePermission_ReadEncryptedRequired: BtdrvGattAttributePermission = BtdrvGattAttributePermission_ReadEncrypted | BtdrvGattAttributePermission_ReadEncryptedMitm;
pub const BtdrvGattAttributePermission_WriteAllowed: BtdrvGattAttributePermission = BtdrvGattAttributePermission_Write | BtdrvGattAttributePermission_WriteEncrypted | BtdrvGattAttributePermission_WriteEncryptedMitm | BtdrvGattAttributePermission_WriteSigned | BtdrvGattAttributePermission_WriteSignedMitm;
pub const BtdrvGattAttributePermission_WriteAuthRequired: BtdrvGattAttributePermission = BtdrvGattAttributePermission_WriteEncrypted | BtdrvGattAttributePermission_WriteSigned;
pub const BtdrvGattAttributePermission_WriteMitmRequired: BtdrvGattAttributePermission = BtdrvGattAttributePermission_WriteEncryptedMitm | BtdrvGattAttributePermission_WriteSignedMitm;
pub const BtdrvGattAttributePermission_WriteEncryptedRequired: BtdrvGattAttributePermission = BtdrvGattAttributePermission_WriteEncrypted | BtdrvGattAttributePermission_WriteEncryptedMitm;
pub const BtdrvGattAttributePermission_WriteSignedRequired: BtdrvGattAttributePermission = BtdrvGattAttributePermission_WriteSigned | BtdrvGattAttributePermission_WriteSignedMitm;

/// GattCharacteristicProperty
pub type BtdrvGattCharacteristicProperty = u32;
pub const BtdrvGattCharacteristicProperty_Broadcast: BtdrvGattCharacteristicProperty = BIT(0);
pub const BtdrvGattCharacteristicProperty_Read: BtdrvGattCharacteristicProperty = BIT(1);
pub const BtdrvGattCharacteristicProperty_WriteNoResponse: BtdrvGattCharacteristicProperty = BIT(2);
pub const BtdrvGattCharacteristicProperty_Write: BtdrvGattCharacteristicProperty = BIT(3);
pub const BtdrvGattCharacteristicProperty_Notify: BtdrvGattCharacteristicProperty = BIT(4);
pub const BtdrvGattCharacteristicProperty_Indicate: BtdrvGattCharacteristicProperty = BIT(5);
pub const BtdrvGattCharacteristicProperty_Authentication: BtdrvGattCharacteristicProperty = BIT(6);
pub const BtdrvGattCharacteristicProperty_ExtendedProperties: BtdrvGattCharacteristicProperty = BIT(7);

/// GattAuthReqType
pub type BtdrvGattAuthReqType = u32;
pub const BtdrvGattAuthReqType_None: BtdrvGattAuthReqType = 0;
pub const BtdrvGattAuthReqType_NoMitm: BtdrvGattAuthReqType = 1;
pub const BtdrvGattAuthReqType_Mitm: BtdrvGattAuthReqType = 2;
pub const BtdrvGattAuthReqType_SignedNoMitm: BtdrvGattAuthReqType = 3;
pub const BtdrvGattAuthReqType_SignedMitm: BtdrvGattAuthReqType = 4;

/// BtdrvBleAdBit
pub type BtdrvBleAdBit = u32;
pub const BtdrvBleAdBit_DeviceName: BtdrvBleAdBit = BIT(0);
pub const BtdrvBleAdBit_Flags: BtdrvBleAdBit = BIT(1);
pub const BtdrvBleAdBit_Manufacturer: BtdrvBleAdBit = BIT(2);
pub const BtdrvBleAdBit_TxPower: BtdrvBleAdBit = BIT(3);
pub const BtdrvBleAdBit_Service32: BtdrvBleAdBit = BIT(4);
pub const BtdrvBleAdBit_IntRange: BtdrvBleAdBit = BIT(5);
pub const BtdrvBleAdBit_Service: BtdrvBleAdBit = BIT(6);
pub const BtdrvBleAdBit_ServiceSol: BtdrvBleAdBit = BIT(7);
pub const BtdrvBleAdBit_ServiceData: BtdrvBleAdBit = BIT(8);
pub const BtdrvBleAdBit_SignData: BtdrvBleAdBit = BIT(9);
pub const BtdrvBleAdBit_Service128Sol: BtdrvBleAdBit = BIT(10);
pub const BtdrvBleAdBit_Appearance: BtdrvBleAdBit = BIT(11);
pub const BtdrvBleAdBit_PublicAddress: BtdrvBleAdBit = BIT(12);
pub const BtdrvBleAdBit_RandomAddress: BtdrvBleAdBit = BIT(13);
pub const BtdrvBleAdBit_Service32Sol: BtdrvBleAdBit = BIT(14);
pub const BtdrvBleAdBit_Proprietary: BtdrvBleAdBit = BIT(15);
pub const BtdrvBleAdBit_Service128: BtdrvBleAdBit = BIT(16);

/// BtdrvBleAdFlag
pub type BtdrvBleAdFlag = u32;
pub const BtdrvBleAdFlag_None: BtdrvBleAdFlag = 0;
pub const BtdrvBleAdFlag_LimitedDiscovery: BtdrvBleAdFlag = BIT(0);
pub const BtdrvBleAdFlag_GeneralDiscovery: BtdrvBleAdFlag = BIT(1);
pub const BtdrvBleAdFlag_BrEdrNotSupported: BtdrvBleAdFlag = BIT(2);
pub const BtdrvBleAdFlag_DualModeControllerSupport: BtdrvBleAdFlag = BIT(3);
pub const BtdrvBleAdFlag_DualModeHostSupport: BtdrvBleAdFlag = BIT(4);

/// AudioEventType
pub type BtdrvAudioEventType = u32;
/// None
pub const BtdrvAudioEventType_None: BtdrvAudioEventType = 0;
/// Connection
pub const BtdrvAudioEventType_Connection: BtdrvAudioEventType = 1;

/// AudioOutState
pub type BtdrvAudioOutState = u32;
/// Stopped
pub const BtdrvAudioOutState_Stopped: BtdrvAudioOutState = 0;
/// Started
pub const BtdrvAudioOutState_Started: BtdrvAudioOutState = 1;

/// AudioCodec
pub type BtdrvAudioCodec = u32;
/// Raw PCM
pub const BtdrvAudioCodec_Pcm: BtdrvAudioCodec = 0;

/// Address
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvAddress {
    /// Address
    pub address: [u8; 6],
}

/// ClassOfDevice
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvClassOfDevice {
    /// ClassOfDevice
    pub class_of_device: [u8; 3],
}

/// AdapterProperty [1.0.0-11.0.1]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvAdapterPropertyOld {
    /// Same as the data for ::BtdrvBluetoothPropertyType_Address.
    pub addr: BtdrvAddress,
    /// Same as the data for ::BtdrvBluetoothPropertyType_ClassOfDevice.
    pub class_of_device: BtdrvClassOfDevice,
    /// Same as the data for ::BtdrvBluetoothPropertyType_Name (last byte is not initialized).
    pub name: [c_char; 249],
    /// Set to hard-coded value 0x68 (same as the data for ::BtdrvBluetoothPropertyType_FeatureSet).
    pub feature_set: u8,
}

/// AdapterProperty [12.0.0+]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvAdapterProperty {
    /// **Ref:** BtdrvAdapterPropertyType
    pub type_: u8,
    /// Data size.
    pub size: u8,
    /// Data (above size), as specified by the type.
    pub data: [u8; 256],
}

/// AdapterPropertySet [12.0.0+]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvAdapterPropertySet {
    /// Same as the data for ::BtdrvBluetoothPropertyType_Address.
    pub addr: BtdrvAddress,
    /// Same as the data for ::BtdrvBluetoothPropertyType_ClassOfDevice.
    pub class_of_device: BtdrvClassOfDevice,
    /// Same as the data for ::BtdrvBluetoothPropertyType_Name.
    pub name: [c_char; 249],
}

/// BluetoothPinCode [1.0.0-11.0.1]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvBluetoothPinCode {
    /// PinCode
    pub code: [c_char; 16],
}

/// BtdrvPinCode [12.0.0+]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvPinCode {
    /// PinCode
    pub code: [c_char; 16],
    /// Length
    pub length: u8,
}

/// HidData [1.0.0-8.1.1]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvHidData {
    /// Size of data.
    pub size: u16,
    /// Data
    pub data: [u8; 640],
}

/// HidReport [9.0.0+].
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvHidReport {
    /// Size of data.
    pub size: u16,
    /// Data
    pub data: [u8; 700],
}

/// PlrStatistics
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvPlrStatistics {
    /// Unknown
    pub unk_x0: [u8; 132],
}

/// PlrList
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvPlrList {
    /// Unknown
    pub unk_x0: [u8; 164],
}

/// ChannelMapList
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvChannelMapList {
    /// Unknown
    pub unk_x0: [u8; 136],
}

/// GattAttributeUuid
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvGattAttributeUuid {
    /// UUID size, must be 0x2, 0x4, or 0x10.
    pub size: u32,
    /// UUID with the above size.
    pub uuid: [u8; 16],
}

/// GattId
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvGattId {
    /// InstanceId
    pub instance_id: u8,
    /// Padding
    pub pad: [u8; 3],
    /// **Ref:** BtdrvGattAttributeUuid
    pub uuid: BtdrvGattAttributeUuid,
}

/// GattAttribute
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvGattAttribute {
    /// **Ref:** BtdrvGattId
    pub id: BtdrvGattId,
    /// **Ref:** BtdrvGattAttributeType
    pub type_: u16,
    pub handle: u16,
    pub group_end_handle: u16,
    /// Only used when type is characteristic. `BtdrvGattCharacteristicProperty`
    pub property: u8,
    /// Only used when type is service
    pub is_primary: bool,
}

/// LeConnectionParams [5.0.0-8.1.1]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvLeConnectionParams {
    /// **Ref:** BtdrvAddress
    pub addr: BtdrvAddress,
    /// Minimum connection interval
    pub min_conn_interval: u16,
    /// Maximum connection interval
    pub max_conn_interval: u16,
    /// Minimum connection event length
    pub min_ce_length: u16,
    /// Maximum connection event length
    pub max_ce_length: u16,
    /// Slave latency
    pub slave_latency: u16,
    /// Connection supervision timeout multiplier
    pub supervision_tout: u16,
    /// Unused
    pub preference: u8,
    /// Padding
    pub pad: u8,
}

/// BleConnectionParameter [9.0.0+]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvBleConnectionParameter {
    /// Minimum connection interval
    pub min_conn_interval: u16,
    /// Maximum connection interval
    pub max_conn_interval: u16,
    /// Minimum connection event length
    pub min_ce_length: u16,
    /// Maximum connection event length
    pub max_ce_length: u16,
    /// Slave latency
    pub slave_latency: u16,
    /// Connection supervision timeout multiplier
    pub supervision_tout: u16,
}

/// BtdrvBleAdvertisePacketData
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvBleAdvertisePacketData {
    /// Bitmask of following AD data to be included in advertising packets `BtdrvBleAdBit`
    pub adv_data_mask: u32,
    /// AD flag value to be advertised `BtdrvBleAdFlag`. Included with BtdrvBleAdBit_Flags
    pub flag: u8,
    /// Size of manu_data below
    pub manu_data_len: u8,
    /// Manufacturer-specific data to be advertised. Included with BtdrvBleAdBit_Manufacturer
    pub manu_data: [u8; 31],
    /// Padding
    pub pad: [u8; 1],
    /// Device appearance data to be advertised `BtdrvAppearanceType`. Included with BtdrvBleAdBit_Appearance
    pub appearance_data: u16,
    /// Number of services in uuid_val array below
    pub num_service: u8,
    /// Padding
    pub pad2: [u8; 3],
    /// Array of 16-bit UUIDs to be advertised `BtdrvGattAttributeUuid`. Included with BtdrvBleAdBit_Service
    pub uuid_val: [BtdrvGattAttributeUuid; 6],
    /// Size of service_data below
    pub service_data_len: u8,
    /// Padding
    pub pad3: [u8; 1],
    /// 16-bit UUID of service_data below
    pub service_data_uuid: u16,
    /// Service data to be advertised. Included with BtdrvBleAdBit_ServiceData
    pub service_data: [u8; 31],
    /// Whether this is an inquiry scan response or advertising data
    pub is_scan_rsp: bool,
    /// Inquiry transmit power to be advertised. Included with BtdrvBleAdBit_TxPower
    pub tx_power: u8,
    /// Padding
    pub pad4: [u8; 3],
}

/// BleAdvertisement
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvBleAdvertisement {
    /// Size of data below.
    pub size: u8,
    /// **Ref:** BtdrvAdType
    pub type_: u8,
    /// Advertisement data
    pub data: [u8; 29],
}

/// BleAdvertiseFilter
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvBleAdvertiseFilter {
    /// Filter index
    pub index: u8,
    /// **Ref:** BtdrvBleAdvertisement
    pub adv: BtdrvBleAdvertisement,
    /// Mask for advertisement data above
    pub mask: [u8; 29],
    /// Size of mask above.
    pub mask_size: u8,
}

/// BleAdvertisePacketParameter
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvBleAdvertisePacketParameter {
    pub company_id: u16,
    pub pattern_data: [u8; 6],
}

/// BleScanResult
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvBleScanResult {
    /// Unknown
    pub unk_x0: u8,
    /// **Ref:** BtdrvAddress
    pub addr: BtdrvAddress,
    /// Unknown
    pub unk_x7: [u8; 313],
    /// Unknown
    pub count: s32,
    /// Unknown
    pub unk_x144: s32,
}

/// BleConnectionInfo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvBleConnectionInfo {
    /// ConnectionHandle, 0xFFFFFFFF ([5.0.0-5.0.2] 0xFFFF) is invalid.
    pub connection_handle: u32,
    /// **Ref:** BtdrvAddress
    pub addr: BtdrvAddress,
    /// Padding
    pub pad: [u8; 2],
}

/// LeEventInfo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvLeEventInfo {
    /// Unknown
    pub unk_x0: u32,
    /// Unknown
    pub unk_x4: u32,
    /// Unknown
    pub unk_x8: u8,
    /// Padding
    pub pad: [u8; 3],
    /// **Ref:** BtdrvGattAttributeUuid
    pub uuid0: BtdrvGattAttributeUuid,
    /// **Ref:** BtdrvGattAttributeUuid
    pub uuid1: BtdrvGattAttributeUuid,
    /// **Ref:** BtdrvGattAttributeUuid
    pub uuid2: BtdrvGattAttributeUuid,
    /// Size of the below data.
    pub size: u16,
    /// Data.
    pub data: [u8; 950],
}

/// BleClientGattOperationInfo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvBleClientGattOperationInfo {
    /// Converted from BtdrvLeEventInfo::unk_x0.
    pub unk_x0: u8,
    /// Padding
    pub pad: [u8; 3],
    /// BtdrvLeEventInfo::unk_x4
    pub unk_x4: u32,
    /// BtdrvLeEventInfo::unk_x8
    pub unk_x8: u8,
    /// Padding
    pub pad2: [u8; 3],
    /// BtdrvLeEventInfo::uuid0
    pub uuid0: BtdrvGattAttributeUuid,
    /// BtdrvLeEventInfo::uuid1
    pub uuid1: BtdrvGattAttributeUuid,
    /// BtdrvLeEventInfo::uuid2
    pub uuid2: BtdrvGattAttributeUuid,
    /// BtdrvLeEventInfo::size
    pub size: u64,
    /// BtdrvLeEventInfo::data
    pub data: [u8; 512],
}

/// PcmParameter
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvPcmParameter {
    /// Must be 0-3. Controls number of channels: 0 = mono, non-zero = stereo.
    pub unk_x0: u32,
    /// Sample rate. Must be one of the following: 16000, 32000, 44100, 48000.
    pub sample_rate: s32,
    /// Bits per sample. Must be 8 or 16.
    pub bits_per_sample: u32,
}

/// AudioControlButtonState
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvAudioControlButtonState {
    /// Unknown
    pub unk_x0: [u8; 16],
}

impl_zeroed_default!(BtdrvAddress, BtdrvClassOfDevice, BtdrvAdapterPropertyOld, BtdrvAdapterProperty, BtdrvAdapterPropertySet, BtdrvBluetoothPinCode, BtdrvPinCode, BtdrvHidData, BtdrvHidReport, BtdrvPlrStatistics, BtdrvPlrList, BtdrvChannelMapList, BtdrvGattAttributeUuid, BtdrvGattId, BtdrvGattAttribute, BtdrvLeConnectionParams, BtdrvBleConnectionParameter, BtdrvBleAdvertisePacketData, BtdrvBleAdvertisement, BtdrvBleAdvertiseFilter, BtdrvBleAdvertisePacketParameter, BtdrvBleScanResult, BtdrvBleConnectionInfo, BtdrvLeEventInfo, BtdrvBleClientGattOperationInfo, BtdrvPcmParameter, BtdrvAudioControlButtonState);
