//! `switch/services/btdrv.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvEventInfo_anon0_type0 {
    /// Value
    pub val: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvEventInfo_anon0_inquiry_device_anon0_v1 {
    /// Device name, NUL-terminated string.
    pub name: [c_char; 249],
    /// Device address.
    pub addr: BtdrvAddress,
    /// Reserved
    pub reserved_xFF: [u8; 16],
    /// Class of Device.
    pub class_of_device: BtdrvClassOfDevice,
    /// Set to fixed value u32 0x1.
    pub unk_x112: [u8; 4],
    /// Reserved
    pub reserved_x116: [u8; 250],
    /// Reserved
    pub reserved_x210: [u8; 92],
    /// Device name, NUL-terminated string. Same as name above, except starting at index 1.
    pub name2: [c_char; 249],
    /// s32 RSSI
    pub rssi: [u8; 4],
    /// Two bytes which are the same as name[11-12].
    pub name3: [u8; 4],
    /// Reserved
    pub reserved_x36D: [u8; 16],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvEventInfo_anon0_inquiry_device_anon0_v12 {
    /// Device address.
    pub addr: BtdrvAddress,
    /// Device name, NUL-terminated string.
    pub name: [c_char; 249],
    /// Class of Device.
    pub class_of_device: BtdrvClassOfDevice,
    /// Reserved
    pub reserved: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union BtdrvEventInfo_anon0_inquiry_device_anon0 {
    /// [1.0.0-11.0.1]
    pub v1: BtdrvEventInfo_anon0_inquiry_device_anon0_v1,
    /// [12.0.0+]
    pub v12: BtdrvEventInfo_anon0_inquiry_device_anon0_v12,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BtdrvEventInfo_anon0_inquiry_device {
    pub anon0: BtdrvEventInfo_anon0_inquiry_device_anon0,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvEventInfo_anon0_inquiry_status_anon0_v1 {
    /// **Ref:** BtdrvInquiryStatus
    pub status: BtdrvInquiryStatus,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvEventInfo_anon0_inquiry_status_anon0_v12 {
    /// **Ref:** BtdrvInquiryStatus
    pub status: u8,
    /// Padding
    pub pad: [u8; 3],
    /// Services value from `btdrvStartInquiry` when starting, otherwise this is value 0.
    pub service_mask: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union BtdrvEventInfo_anon0_inquiry_status_anon0 {
    /// [1.0.0-11.0.1]
    pub v1: BtdrvEventInfo_anon0_inquiry_status_anon0_v1,
    /// [12.0.0+]
    pub v12: BtdrvEventInfo_anon0_inquiry_status_anon0_v12,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BtdrvEventInfo_anon0_inquiry_status {
    pub anon0: BtdrvEventInfo_anon0_inquiry_status_anon0,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvEventInfo_anon0_pairing_pin_code_request {
    /// Device address.
    pub addr: BtdrvAddress,
    /// Device name, NUL-terminated string.
    pub name: [c_char; 249],
    /// Class of Device.
    pub class_of_device: BtdrvClassOfDevice,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvEventInfo_anon0_ssp_request_anon0_v1 {
    /// Device address.
    pub addr: BtdrvAddress,
    /// Device name, NUL-terminated string.
    pub name: [c_char; 249],
    /// Class of Device.
    pub class_of_device: BtdrvClassOfDevice,
    /// Padding
    pub pad: [u8; 2],
    /// 0 = SSP confirm request, 3 = SSP passkey notification.
    pub type_: u32,
    /// Passkey, only set when the above field is value 3.
    pub passkey: s32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvEventInfo_anon0_ssp_request_anon0_v12 {
    /// Device address.
    pub addr: BtdrvAddress,
    /// Device name, NUL-terminated string.
    pub name: [c_char; 249],
    /// Class of Device.
    pub class_of_device: BtdrvClassOfDevice,
    /// bool flag for Just Works. With SSP passkey notification this is always 0.
    pub flag: u8,
    /// Padding
    pub pad: u8,
    /// Passkey
    pub passkey: s32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union BtdrvEventInfo_anon0_ssp_request_anon0 {
    /// [1.0.0-11.0.1]
    pub v1: BtdrvEventInfo_anon0_ssp_request_anon0_v1,
    /// [12.0.0+]
    pub v12: BtdrvEventInfo_anon0_ssp_request_anon0_v12,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BtdrvEventInfo_anon0_ssp_request {
    pub anon0: BtdrvEventInfo_anon0_ssp_request_anon0,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvEventInfo_anon0_connection_anon0_v1 {
    /// Device address.
    pub addr: BtdrvAddress,
    /// Padding
    pub pad: [u8; 2],
    /// Status, always 0 except with ::BtdrvConnectionEventType_Status: 2 = ACL Link is now Resumed, 9 = connection failed (pairing/authentication failed, or opening the hid connection failed).
    pub status: u32,
    /// **Ref:** BtdrvConnectionEventType
    pub type_: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvEventInfo_anon0_connection_anon0_v9 {
    /// Status, always 0 except with ::BtdrvConnectionEventType_Status: 2 = ACL Link is now Resumed, 9 = connection failed (pairing/authentication failed, or opening the hid connection failed).
    pub status: u32,
    /// Device address.
    pub addr: BtdrvAddress,
    /// Padding
    pub pad: [u8; 2],
    /// **Ref:** BtdrvConnectionEventType
    pub type_: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvEventInfo_anon0_connection_anon0_v12 {
    /// **Ref:** BtdrvConnectionEventType
    pub type_: u32,
    /// Device address.
    pub addr: BtdrvAddress,
    /// Reserved
    pub reserved: [u8; 254],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union BtdrvEventInfo_anon0_connection_anon0 {
    /// [1.0.0-8.1.1]
    pub v1: BtdrvEventInfo_anon0_connection_anon0_v1,
    /// [9.0.0-11.0.1]
    pub v9: BtdrvEventInfo_anon0_connection_anon0_v9,
    /// [12.0.0+]
    pub v12: BtdrvEventInfo_anon0_connection_anon0_v12,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BtdrvEventInfo_anon0_connection {
    pub anon0: BtdrvEventInfo_anon0_connection_anon0,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvEventInfo_anon0_tsi {
    /// Device address.
    pub addr: BtdrvAddress,
    /// Status flag: 1 = success, 0 = failure.
    pub status: u8,
    /// Tsi value, when the above indicates success.
    pub value: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvEventInfo_anon0_burst_mode {
    /// Device address.
    pub addr: BtdrvAddress,
    /// Status flag: 1 = success, 0 = failure.
    pub status: u8,
    /// Input bool value from `btdrvEnableBurstMode`, when the above indicates success.
    pub value: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvEventInfo_anon0_set_zero_retransmission {
    /// Device address.
    pub addr: BtdrvAddress,
    /// Status flag: 1 = success, 0 = failure.
    pub status: u8,
    /// Bool flag, when the above indicates success.
    pub flag: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvEventInfo_anon0_pending_connections {
    /// Status flag: 1 = success, 0 = failure.
    pub status: u8,
    /// Padding
    pub pad: [u8; 3],
    /// Count value.
    pub count: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvEventInfo_anon0_move_to_secondary_piconet {
    /// Device address.
    pub addr: BtdrvAddress,
    /// Status flag: 1 = success, 0 = failure.
    pub status: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvEventInfo_anon0_bluetooth_crash {
    /// **Ref:** BtdrvFatalReason
    pub reason: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union BtdrvEventInfo_anon0 {
    /// Raw data.
    pub data: [u8; 1024],
    /// ::BtdrvEventTypeOld_Unknown0
    pub type0: BtdrvEventInfo_anon0_type0,
    /// ::BtdrvEventType_InquiryDevice
    pub inquiry_device: BtdrvEventInfo_anon0_inquiry_device,
    /// ::BtdrvEventType_InquiryStatus
    pub inquiry_status: BtdrvEventInfo_anon0_inquiry_status,
    /// ::BtdrvEventType_PairingPinCodeRequest
    pub pairing_pin_code_request: BtdrvEventInfo_anon0_pairing_pin_code_request,
    /// ::BtdrvEventType_SspRequest
    pub ssp_request: BtdrvEventInfo_anon0_ssp_request,
    /// ::BtdrvEventType_Connection
    pub connection: BtdrvEventInfo_anon0_connection,
    /// ::BtdrvEventType_Tsi
    pub tsi: BtdrvEventInfo_anon0_tsi,
    /// ::BtdrvEventType_BurstMode
    pub burst_mode: BtdrvEventInfo_anon0_burst_mode,
    /// ::BtdrvEventType_SetZeroRetransmission
    pub set_zero_retransmission: BtdrvEventInfo_anon0_set_zero_retransmission,
    /// ::BtdrvEventType_PendingConnections
    pub pending_connections: BtdrvEventInfo_anon0_pending_connections,
    /// ::BtdrvEventType_MoveToSecondaryPiconet
    pub move_to_secondary_piconet: BtdrvEventInfo_anon0_move_to_secondary_piconet,
    /// ::BtdrvEventType_BluetoothCrash
    pub bluetooth_crash: BtdrvEventInfo_anon0_bluetooth_crash,
}

/// Data for `btdrvGetEventInfo`. The data stored here depends on the `BtdrvEventType`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BtdrvEventInfo {
    pub anon0: BtdrvEventInfo_anon0,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvHidEventInfo_anon0_connection_anon0_v1 {
    /// Device address.
    pub addr: BtdrvAddress,
    /// Padding
    pub pad: [u8; 2],
    /// **Ref:** BtdrvHidConnectionStatus
    pub status: BtdrvHidConnectionStatus,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvHidEventInfo_anon0_connection_anon0_v12 {
    /// **Ref:** BtdrvHidConnectionStatus
    pub status: BtdrvHidConnectionStatus,
    /// Device address.
    pub addr: BtdrvAddress,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union BtdrvHidEventInfo_anon0_connection_anon0 {
    /// [1.0.0-11.0.1]
    pub v1: BtdrvHidEventInfo_anon0_connection_anon0_v1,
    /// [12.0.0+]
    pub v12: BtdrvHidEventInfo_anon0_connection_anon0_v12,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BtdrvHidEventInfo_anon0_connection {
    pub anon0: BtdrvHidEventInfo_anon0_connection_anon0,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvHidEventInfo_anon0_ext_anon0_set_tsi {
    /// 0 for success, non-zero for error.
    pub status: u32,
    /// Device address.
    pub addr: BtdrvAddress,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvHidEventInfo_anon0_ext_anon0_exit_tsi {
    /// 0 for success, non-zero for error.
    pub status: u32,
    /// Device address.
    pub addr: BtdrvAddress,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvHidEventInfo_anon0_ext_anon0_set_burst_mode {
    /// 0 for success, non-zero for error.
    pub status: u32,
    /// Device address.
    pub addr: BtdrvAddress,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvHidEventInfo_anon0_ext_anon0_exit_burst_mode {
    /// 0 for success, non-zero for error.
    pub status: u32,
    /// Device address.
    pub addr: BtdrvAddress,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvHidEventInfo_anon0_ext_anon0_set_zero_retransmission {
    /// 0 for success, non-zero for error.
    pub status: u32,
    /// Device address.
    pub addr: BtdrvAddress,
    /// Padding
    pub pad: [u8; 2],
    /// Flag
    pub flag: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvHidEventInfo_anon0_ext_anon0_pending_connections {
    /// 0 for success, non-zero for error.
    pub status: u32,
    /// Unused
    pub addr: BtdrvAddress,
    /// Padding
    pub pad: [u8; 2],
    /// Count value.
    pub count: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvHidEventInfo_anon0_ext_anon0_move_to_secondary_piconet {
    /// 0 for success, non-zero for error.
    pub status: u32,
    /// Device address.
    pub addr: BtdrvAddress,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union BtdrvHidEventInfo_anon0_ext_anon0 {
    /// ::BtdrvExtEventType_SetTsi
    pub set_tsi: BtdrvHidEventInfo_anon0_ext_anon0_set_tsi,
    /// ::BtdrvExtEventType_ExitTsi
    pub exit_tsi: BtdrvHidEventInfo_anon0_ext_anon0_exit_tsi,
    /// ::BtdrvExtEventType_SetBurstMode
    pub set_burst_mode: BtdrvHidEventInfo_anon0_ext_anon0_set_burst_mode,
    /// ::BtdrvExtEventType_ExitBurstMode
    pub exit_burst_mode: BtdrvHidEventInfo_anon0_ext_anon0_exit_burst_mode,
    /// ::BtdrvExtEventType_SetZeroRetransmission
    pub set_zero_retransmission: BtdrvHidEventInfo_anon0_ext_anon0_set_zero_retransmission,
    /// ::BtdrvExtEventType_PendingConnections
    pub pending_connections: BtdrvHidEventInfo_anon0_ext_anon0_pending_connections,
    /// ::BtdrvExtEventType_MoveToSecondaryPiconet
    pub move_to_secondary_piconet: BtdrvHidEventInfo_anon0_ext_anon0_move_to_secondary_piconet,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BtdrvHidEventInfo_anon0_ext {
    /// **Ref:** BtdrvExtEventType, controls which data is stored below.
    pub type_: u32,
    pub anon0: BtdrvHidEventInfo_anon0_ext_anon0,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union BtdrvHidEventInfo_anon0 {
    /// Raw data.
    pub data: [u8; 1152],
    /// ::BtdrvHidEventType_Connection
    pub connection: BtdrvHidEventInfo_anon0_connection,
    /// ::BtdrvHidEventType_Ext [1.0.0-11.0.1]
    pub ext: BtdrvHidEventInfo_anon0_ext,
}

/// Data for `btdrvGetHidEventInfo`. The data stored here depends on the `BtdrvHidEventType`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BtdrvHidEventInfo {
    pub anon0: BtdrvHidEventInfo_anon0,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvHidReportEventInfo_anon0_data_report_anon0_v1_hdr {
    pub addr: BtdrvAddress,
    pub pad: [u8; 2],
    pub res: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvHidReportEventInfo_anon0_data_report_anon0_v1 {
    pub hdr: BtdrvHidReportEventInfo_anon0_data_report_anon0_v1_hdr,
    /// Unused
    pub unused: [u8; 3],
    /// **Ref:** BtdrvAddress
    pub addr: BtdrvAddress,
    /// Unused
    pub unused2: [u8; 3],
    pub report: BtdrvHidData,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvHidReportEventInfo_anon0_data_report_anon0_v7 {
    /// Unused
    pub unused: [u8; 3],
    /// **Ref:** BtdrvAddress
    pub addr: BtdrvAddress,
    /// Unused
    pub unused2: [u8; 3],
    pub report: BtdrvHidData,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvHidReportEventInfo_anon0_data_report_anon0_v9 {
    /// Always 0.
    pub res: u32,
    /// Protocol mode. Always 0 (report mode).
    pub proto_mode: u8,
    /// **Ref:** BtdrvAddress
    pub addr: BtdrvAddress,
    /// Padding
    pub pad: u8,
    pub report: BtdrvHidReport,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union BtdrvHidReportEventInfo_anon0_data_report_anon0 {
    /// [1.0.0-6.2.0]
    pub v1: BtdrvHidReportEventInfo_anon0_data_report_anon0_v1,
    /// [7.0.0-8.1.1]
    pub v7: BtdrvHidReportEventInfo_anon0_data_report_anon0_v7,
    /// [9.0.0+]
    pub v9: BtdrvHidReportEventInfo_anon0_data_report_anon0_v9,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BtdrvHidReportEventInfo_anon0_data_report {
    pub anon0: BtdrvHidReportEventInfo_anon0_data_report_anon0,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvHidReportEventInfo_anon0_set_report_anon0_anon0 {
    /// 0 = success, non-zero = error.
    pub res: u32,
    /// **Ref:** BtdrvAddress
    pub addr: BtdrvAddress,
    /// Padding
    pub pad: [u8; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union BtdrvHidReportEventInfo_anon0_set_report_anon0 {
    /// Raw data.
    pub rawdata: [u8; 12],
    pub anon0: BtdrvHidReportEventInfo_anon0_set_report_anon0_anon0,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BtdrvHidReportEventInfo_anon0_set_report {
    pub anon0: BtdrvHidReportEventInfo_anon0_set_report_anon0,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvHidReportEventInfo_anon0_get_report_anon0_v1_anon0 {
    /// **Ref:** BtdrvAddress
    pub addr: BtdrvAddress,
    /// Padding
    pub pad: [u8; 2],
    /// Unknown. hid-sysmodule only uses the below data when this field is 0.
    pub res: u32,
    /// **Ref:** BtdrvHidData
    pub report: BtdrvHidData,
    /// Padding
    pub pad2: [u8; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union BtdrvHidReportEventInfo_anon0_get_report_anon0_v1 {
    /// Raw data.
    pub rawdata: [u8; 656],
    pub anon0: BtdrvHidReportEventInfo_anon0_get_report_anon0_v1_anon0,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvHidReportEventInfo_anon0_get_report_anon0_v9_anon0 {
    /// Unknown. hid-sysmodule only uses the below report when this field is 0.
    pub res: u32,
    /// **Ref:** BtdrvAddress
    pub addr: BtdrvAddress,
    /// **Ref:** BtdrvHidReport
    pub report: BtdrvHidReport,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union BtdrvHidReportEventInfo_anon0_get_report_anon0_v9 {
    /// Raw data.
    pub rawdata: [u8; 712],
    pub anon0: BtdrvHidReportEventInfo_anon0_get_report_anon0_v9_anon0,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union BtdrvHidReportEventInfo_anon0_get_report_anon0 {
    /// [1.0.0-8.1.1]
    pub v1: BtdrvHidReportEventInfo_anon0_get_report_anon0_v1,
    /// [9.0.0+]
    pub v9: BtdrvHidReportEventInfo_anon0_get_report_anon0_v9,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BtdrvHidReportEventInfo_anon0_get_report {
    pub anon0: BtdrvHidReportEventInfo_anon0_get_report_anon0,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union BtdrvHidReportEventInfo_anon0 {
    /// Raw data.
    pub data: [u8; 1152],
    /// ::BtdrvHidEventType_DataReport
    pub data_report: BtdrvHidReportEventInfo_anon0_data_report,
    /// ::BtdrvHidEventType_SetReport
    pub set_report: BtdrvHidReportEventInfo_anon0_set_report,
    /// ::BtdrvHidEventType_GetReport
    pub get_report: BtdrvHidReportEventInfo_anon0_get_report,
}

/// Data for `btdrvGetHidReportEventInfo`. The data stored here depends on the `BtdrvHidEventType`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BtdrvHidReportEventInfo {
    pub anon0: BtdrvHidReportEventInfo_anon0,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvHidReportEventInfoBufferData_hdr {
    /// **Ref:** BtdrvHidEventType
    pub type_: u8,
    pub pad: [u8; 7],
    pub tick: u64,
    pub size: u64,
}

/// The raw sharedmem data for HidReportEventInfo.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BtdrvHidReportEventInfoBufferData {
    pub hdr: BtdrvHidReportEventInfoBufferData_hdr,
    pub data: BtdrvHidReportEventInfo,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvAudioEventInfo_connection {
    /// Status: 0 = AV connection closed, 1 = AV connection opened, 2 = failed to open AV connection.
    pub status: u32,
    /// Device address.
    pub addr: BtdrvAddress,
    /// Padding
    pub pad: [u8; 2],
}

/// Data for `btdrvGetAudioEventInfo`. The data stored here depends on the `BtdrvAudioEventType`.
#[repr(C)]
#[derive(Clone, Copy)]
pub union BtdrvAudioEventInfo {
    /// ::BtdrvAudioEventType_Connection
    pub connection: BtdrvAudioEventInfo_connection,
}

/// CircularBuffer
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvCircularBuffer {
    pub mutex: Mutex,
    /// Not set with sharedmem.
    pub event_type: *mut c_void,
    pub data: [u8; 10000],
    pub write_offset: s32,
    pub read_offset: s32,
    pub utilization: u64,
    pub name: [c_char; 17],
    pub initialized: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvBleEventInfo_anon0_client_registration {
    /// 0 for success, non-zero for error.
    pub result: u32,
    /// Client interface handle
    pub client_if: u8,
    /// Registration status. 0 = Unregistered, 1 = Registered
    pub status: u8,
    /// Padding
    pub pad: [u8; 2],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvBleEventInfo_anon0_server_registration {
    /// 0 for success, non-zero for error.
    pub result: u32,
    /// Server interface handle. Unused
    pub server_if: u8,
    /// Registration status. 0 = Unregistered, 1 = Registered
    pub status: u8,
    /// Padding
    pub pad: [u8; 2],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvBleEventInfo_anon0_connection_update {
    /// 0 for success, non-zero for error.
    pub result: u32,
    /// Connection ID
    pub conn_id: u32,
    /// Connection interval
    pub conn_interval: u16,
    /// Connection latency
    pub conn_latency: u16,
    /// Connection supervision timeout multiplier
    pub supervision_tout: u16,
    /// Padding
    pub pad: [u8; 2],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvBleEventInfo_anon0_preferred_connection_parameters {
    /// Connection ID
    pub conn_id: u32,
    /// Minimum connection interval
    pub min_conn_interval: u16,
    /// Maximum connection interval
    pub max_conn_interval: u16,
    /// Slave latency
    pub slave_latency: u16,
    /// Connection supervision timeout multiplier
    pub supervision_tout: u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvBleEventInfo_anon0_client_connection {
    /// 0 for success, non-zero for error.
    pub result: u32,
    /// Connection status. 0 = Connected, 2 = Disconnected
    pub status: u8,
    /// Client interface handle
    pub client_if: u8,
    /// Padding
    pub pad: [u8; 2],
    /// Connection ID
    pub conn_id: u32,
    /// Device address
    pub address: BtdrvAddress,
    /// Disconnection reason
    pub reason: u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvBleEventInfo_anon0_server_connection {
    /// 0 for success, non-zero for error.
    pub result: u32,
    /// Connection status. 0 = Connected, 2 = Disconnected
    pub status: u8,
    /// Server interface handle
    pub server_if: u8,
    /// Padding
    pub pad: [u8; 2],
    /// Connection ID
    pub conn_id: u32,
    /// Device address
    pub address: BtdrvAddress,
    /// Disconnection reason
    pub reason: u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvBleEventInfo_anon0_scan_result {
    /// 0 for success, non-zero for error.
    pub result: u32,
    /// BLE scan status. 1 = Scan complete, 2 = New device found, 0xFF = Scan started
    pub status: u8,
    /// Device type. 0 = BD/EDR, 1 = BLE, 2 = Dual Mode
    pub device_type: u8,
    /// BLE address type. 0 = Public, 1 = Random, 2 = Public ID, 3 = Random ID
    pub ble_addr_type: u8,
    /// Device address
    pub address: BtdrvAddress,
    /// BLE Advertisement list `BtdrvBleAdvertisement`
    pub ad_list: [BtdrvBleAdvertisement; 10],
    /// Number of entries above
    pub count: u8,
    /// RSSI value
    pub rssi: s32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvBleEventInfo_anon0_scan_filter {
    /// 0 for success, non-zero for error.
    pub result: u32,
    /// Scan filter action. 0 = Add, 1 = Delete, 2 = Clear, 3 = Enable, 4 = Disable
    pub action: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvBleEventInfo_anon0_client_notify {
    /// 0 for success, non-zero for error.
    pub result: u32,
    /// Connection ID
    pub conn_id: u32,
    /// Notification type. 4 = Notification, 5 = Indication.
    pub type_: u8,
    /// Padding
    pub pad: [u8; 3],
    /// GATT Service UUID `BtdrvGattAttributeUuid`
    pub serv_uuid: BtdrvGattAttributeUuid,
    /// GATT Characteristic UUID `BtdrvGattAttributeUuid`
    pub char_uuid: BtdrvGattAttributeUuid,
    /// GATT Descriptor UUID `BtdrvGattAttributeUuid`
    pub desc_uuid: BtdrvGattAttributeUuid,
    /// Size of the below data
    pub size: u16,
    /// Notification data.
    pub data: [u8; 512],
    /// Padding
    pub pad2: [u8; 2],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvBleEventInfo_anon0_client_cache_save {
    /// 0 for success, non-zero for error.
    pub result: u32,
    /// Connection ID
    pub conn_id: u32,
    /// Number of attributes below.
    pub count: u32,
    /// List of Gatt attributes `BtdrvGattAttribute`
    pub attr_list: [BtdrvGattAttribute; 10],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvBleEventInfo_anon0_client_cache_load {
    /// 0 for success, non-zero for error.
    pub result: u32,
    /// Connection ID
    pub conn_id: u32,
    /// Unknown. Always 0
    pub unk_x8: u16,
    /// Unused
    pub unused: [u8; 322],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvBleEventInfo_anon0_client_configure_mtu {
    /// 0 for success, non-zero for error.
    pub result: u32,
    /// Connection ID
    pub conn_id: u32,
    /// MTU value
    pub mtu: u16,
    /// Padding
    pub pad: [u8; 2],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvBleEventInfo_anon0_server_add_attribute {
    /// 0 for success, non-zero for error.
    pub result: u32,
    /// Server interface handle
    pub server_if: u32,
    /// **Ref:** BtdrvGattAttributeUuid
    pub uuid: BtdrvGattAttributeUuid,
    /// Service ID
    pub service_id: u16,
    /// Attribute ID
    pub attr_id: u16,
    /// Attribute type `BtdrvGattAttributeType`
    pub attr_type: u8,
    /// Characteristic properties. Only set if attr_type is 1 `BtdrvGattCharacteristicProperty`
    pub property: u8,
    /// Is a primary service or not
    pub is_primary: u8,
    /// Padding
    pub pad: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BtdrvBleEventInfo_anon0_server_attribute_operation {
    /// 0 for success, non-zero for error.
    pub result: u32,
    /// Connection ID
    pub conn_id: u16,
    /// Operation. 0 = Read, 1 = Write
    pub operation: u8,
    /// Padding
    pub pad: u8,
    /// Service ID
    pub service_id: u16,
    /// Attribute ID
    pub attr_id: u16,
    /// Attribute type `BtdrvGattAttributeType`
    pub attr_type: u8,
    /// Data written during write operation
    pub data: [u8; 512],
    /// Size of the above data
    pub size: u16,
    /// Offset
    pub offset: u16,
    /// Padding
    pub pad2: [u8; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union BtdrvBleEventInfo_anon0 {
    pub data: [u8; 1024],
    /// ::BtdrvBleEventType_ClientRegistration
    pub client_registration: BtdrvBleEventInfo_anon0_client_registration,
    /// ::BtdrvBleEventType_ServerRegistration
    pub server_registration: BtdrvBleEventInfo_anon0_server_registration,
    /// ::BtdrvBleEventType_ConnectionUpdate
    pub connection_update: BtdrvBleEventInfo_anon0_connection_update,
    /// ::BtdrvBleEventType_PreferredConnectionParameters
    pub preferred_connection_parameters: BtdrvBleEventInfo_anon0_preferred_connection_parameters,
    /// ::BtdrvBleEventType_ClientConnection
    pub client_connection: BtdrvBleEventInfo_anon0_client_connection,
    /// ::BtdrvBleEventType_ServerConnection
    pub server_connection: BtdrvBleEventInfo_anon0_server_connection,
    /// ::BtdrvBleEventType_ScanResult
    pub scan_result: BtdrvBleEventInfo_anon0_scan_result,
    /// ::BtdrvBleEventType_ScanFilter
    pub scan_filter: BtdrvBleEventInfo_anon0_scan_filter,
    /// ::BtdrvBleEventType_ClientNotify
    pub client_notify: BtdrvBleEventInfo_anon0_client_notify,
    /// ::BtdrvBleEventType_ClientCacheSave
    pub client_cache_save: BtdrvBleEventInfo_anon0_client_cache_save,
    /// ::BtdrvBleEventType_ClientCacheLoad
    pub client_cache_load: BtdrvBleEventInfo_anon0_client_cache_load,
    /// ::BtdrvBleEventType_ClientConfigureMtu
    pub client_configure_mtu: BtdrvBleEventInfo_anon0_client_configure_mtu,
    /// ::BtdrvBleEventType_ServerAddAttribute
    pub server_add_attribute: BtdrvBleEventInfo_anon0_server_add_attribute,
    /// ::BtdrvBleEventType_ServerAttributeOperation
    pub server_attribute_operation: BtdrvBleEventInfo_anon0_server_attribute_operation,
}

/// Data for `btdrvGetBleManagedEventInfo` and `btdrvGetLeHidEventInfo`. The data stored here depends on the `BtdrvBleEventType`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BtdrvBleEventInfo {
    pub anon0: BtdrvBleEventInfo_anon0,
}

impl_zeroed_default!(BtdrvEventInfo_anon0_type0, BtdrvEventInfo_anon0_inquiry_device_anon0_v1, BtdrvEventInfo_anon0_inquiry_device_anon0_v12, BtdrvEventInfo_anon0_inquiry_device_anon0, BtdrvEventInfo_anon0_inquiry_device, BtdrvEventInfo_anon0_inquiry_status_anon0_v1, BtdrvEventInfo_anon0_inquiry_status_anon0_v12, BtdrvEventInfo_anon0_inquiry_status_anon0, BtdrvEventInfo_anon0_inquiry_status, BtdrvEventInfo_anon0_pairing_pin_code_request, BtdrvEventInfo_anon0_ssp_request_anon0_v1, BtdrvEventInfo_anon0_ssp_request_anon0_v12, BtdrvEventInfo_anon0_ssp_request_anon0, BtdrvEventInfo_anon0_ssp_request, BtdrvEventInfo_anon0_connection_anon0_v1, BtdrvEventInfo_anon0_connection_anon0_v9, BtdrvEventInfo_anon0_connection_anon0_v12, BtdrvEventInfo_anon0_connection_anon0, BtdrvEventInfo_anon0_connection, BtdrvEventInfo_anon0_tsi, BtdrvEventInfo_anon0_burst_mode, BtdrvEventInfo_anon0_set_zero_retransmission, BtdrvEventInfo_anon0_pending_connections, BtdrvEventInfo_anon0_move_to_secondary_piconet, BtdrvEventInfo_anon0_bluetooth_crash, BtdrvEventInfo_anon0, BtdrvEventInfo, BtdrvHidEventInfo_anon0_connection_anon0_v1, BtdrvHidEventInfo_anon0_connection_anon0_v12, BtdrvHidEventInfo_anon0_connection_anon0, BtdrvHidEventInfo_anon0_connection, BtdrvHidEventInfo_anon0_ext_anon0_set_tsi, BtdrvHidEventInfo_anon0_ext_anon0_exit_tsi, BtdrvHidEventInfo_anon0_ext_anon0_set_burst_mode, BtdrvHidEventInfo_anon0_ext_anon0_exit_burst_mode, BtdrvHidEventInfo_anon0_ext_anon0_set_zero_retransmission, BtdrvHidEventInfo_anon0_ext_anon0_pending_connections, BtdrvHidEventInfo_anon0_ext_anon0_move_to_secondary_piconet, BtdrvHidEventInfo_anon0_ext_anon0, BtdrvHidEventInfo_anon0_ext, BtdrvHidEventInfo_anon0, BtdrvHidEventInfo, BtdrvHidReportEventInfo_anon0_data_report_anon0_v1_hdr, BtdrvHidReportEventInfo_anon0_data_report_anon0_v1, BtdrvHidReportEventInfo_anon0_data_report_anon0_v7, BtdrvHidReportEventInfo_anon0_data_report_anon0_v9, BtdrvHidReportEventInfo_anon0_data_report_anon0, BtdrvHidReportEventInfo_anon0_data_report, BtdrvHidReportEventInfo_anon0_set_report_anon0_anon0, BtdrvHidReportEventInfo_anon0_set_report_anon0, BtdrvHidReportEventInfo_anon0_set_report, BtdrvHidReportEventInfo_anon0_get_report_anon0_v1_anon0, BtdrvHidReportEventInfo_anon0_get_report_anon0_v1, BtdrvHidReportEventInfo_anon0_get_report_anon0_v9_anon0, BtdrvHidReportEventInfo_anon0_get_report_anon0_v9, BtdrvHidReportEventInfo_anon0_get_report_anon0, BtdrvHidReportEventInfo_anon0_get_report, BtdrvHidReportEventInfo_anon0, BtdrvHidReportEventInfo, BtdrvHidReportEventInfoBufferData_hdr, BtdrvHidReportEventInfoBufferData, BtdrvAudioEventInfo_connection, BtdrvAudioEventInfo, BtdrvCircularBuffer, BtdrvBleEventInfo_anon0_client_registration, BtdrvBleEventInfo_anon0_server_registration, BtdrvBleEventInfo_anon0_connection_update, BtdrvBleEventInfo_anon0_preferred_connection_parameters, BtdrvBleEventInfo_anon0_client_connection, BtdrvBleEventInfo_anon0_server_connection, BtdrvBleEventInfo_anon0_scan_result, BtdrvBleEventInfo_anon0_scan_filter, BtdrvBleEventInfo_anon0_client_notify, BtdrvBleEventInfo_anon0_client_cache_save, BtdrvBleEventInfo_anon0_client_cache_load, BtdrvBleEventInfo_anon0_client_configure_mtu, BtdrvBleEventInfo_anon0_server_add_attribute, BtdrvBleEventInfo_anon0_server_attribute_operation, BtdrvBleEventInfo_anon0, BtdrvBleEventInfo);

unsafe extern "C" {
    /// Initialize btdrv.
    pub fn btdrvInitialize() -> Result;

    /// Exit btdrv.
    pub fn btdrvExit();

    /// Gets the Service object for the actual btdrv service session.
    pub fn btdrvGetServiceSession() -> *mut Service;

    /// InitializeBluetooth
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=true.
    ///
    /// **Note:** This is used by btm-sysmodule, this should not be used by other processes.
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn btdrvInitializeBluetooth(out_event: *mut Event) -> Result;

    /// EnableBluetooth
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvEnableBluetooth() -> Result;

    /// DisableBluetooth
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvDisableBluetooth() -> Result;

    /// FinalizeBluetooth
    ///
    /// **Note:** This is not used by btm-sysmodule, this should not be used by other processes.
    pub fn btdrvFinalizeBluetooth() -> Result;

    /// GetAdapterProperties [1.0.0-11.0.1]
    ///
    /// # Arguments
    /// - `properties` (out): `BtdrvAdapterPropertyOld`
    pub fn btdrvLegacyGetAdapterProperties(properties: *mut BtdrvAdapterPropertyOld) -> Result;

    /// GetAdapterProperties [12.0.0+]
    ///
    /// # Arguments
    /// - `properties` (out): `BtdrvAdapterPropertySet`
    pub fn btdrvGetAdapterProperties(properties: *mut BtdrvAdapterPropertySet) -> Result;

    /// GetAdapterProperty [1.0.0-11.0.1]
    ///
    /// # Arguments
    /// - `type` (in): `BtdrvBluetoothPropertyType`
    /// - `buffer` (out): Output buffer, see `BtdrvBluetoothPropertyType` for the contents.
    /// - `size` (in): Output buffer size.
    pub fn btdrvLegacyGetAdapterProperty(type_: BtdrvBluetoothPropertyType, buffer: *mut c_void, size: usize) -> Result;

    /// GetAdapterProperty [12.0.0+]
    ///
    /// # Arguments
    /// - `type` (in): `BtdrvAdapterPropertyType`
    /// - `property` (in): `BtdrvAdapterProperty`
    pub fn btdrvGetAdapterProperty(type_: BtdrvAdapterPropertyType, property: *mut BtdrvAdapterProperty) -> Result;

    /// SetAdapterProperty [1.0.0-11.0.1]
    ///
    /// # Arguments
    /// - `type` (in): `BtdrvBluetoothPropertyType`
    /// - `buffer` (in): Input buffer, see `BtdrvBluetoothPropertyType` for the contents.
    /// - `size` (in): Input buffer size.
    pub fn btdrvLegacySetAdapterProperty(type_: BtdrvBluetoothPropertyType, buffer: *const c_void, size: usize) -> Result;

    /// SetAdapterProperty [12.0.0+]
    ///
    /// # Arguments
    /// - `type` (in): `BtdrvAdapterPropertyType`
    /// - `property` (in): `BtdrvAdapterProperty`
    pub fn btdrvSetAdapterProperty(type_: BtdrvAdapterPropertyType, property: *const BtdrvAdapterProperty) -> Result;

    /// StartInquiry [1.0.0-11.0.1]. This starts Inquiry, the output data will be available via `btdrvGetEventInfo`. Inquiry will automatically stop in 10.24 seconds.
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvLegacyStartInquiry() -> Result;

    /// StartInquiry [12.0.0+]. This starts Inquiry, the output data will be available via `btdrvGetEventInfo`.
    ///
    /// # Arguments
    /// - `services` (in): Bitfield of allowed services. When -1 the original defaults from pre-12.0.0 are used.
    /// - `duration` (in): Inquiry duration in nanoseconds.
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvStartInquiry(services: u32, duration: s64) -> Result;

    /// This stops Inquiry which was started by `btdrvStartInquiry`, if it's still active.
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvStopInquiry() -> Result;

    /// CreateBond
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    /// - `type` (in): TransportType
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvCreateBond(addr: BtdrvAddress, type_: u32) -> Result;

    /// RemoveBond
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvRemoveBond(addr: BtdrvAddress) -> Result;

    /// CancelBond
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvCancelBond(addr: BtdrvAddress) -> Result;

    /// RespondToPinRequest [1.0.0-11.0.1]
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    /// - `flag` (in): Flag
    /// - `pin_code` (in): `BtdrvBluetoothPinCode`
    /// - `length` (in): Length of pin_code
    ///
    /// **Note:** The official sysmodule only uses the input `BtdrvAddress`.
    pub fn btdrvLegacyRespondToPinRequest(addr: BtdrvAddress, flag: bool, pin_code: *const BtdrvBluetoothPinCode, length: u8) -> Result;

    /// RespondToPinRequest [12.0.0+]
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    /// - `pin_code` (in): `BtdrvPinCode`
    pub fn btdrvRespondToPinRequest(addr: BtdrvAddress, pin_code: *const BtdrvPinCode) -> Result;

    /// RespondToSspRequest
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    /// - `variant` (in): BluetoothSspVariant
    /// - `accept` (in): Whether the request is accepted.
    /// - `passkey` (in): Passkey.
    ///
    /// **Note:** The official sysmodule only uses the input `BtdrvAddress` and the flag.
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvRespondToSspRequest(addr: BtdrvAddress, variant: u32, accept: bool, passkey: u32) -> Result;

    /// GetEventInfo
    ///
    /// # Arguments
    /// - `buffer` (out): Output buffer, see `BtdrvEventInfo`.
    /// - `size` (in): Output buffer size.
    /// - `type` (out): Output BtdrvEventType.
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvGetEventInfo(buffer: *mut c_void, size: usize, type_: *mut BtdrvEventType) -> Result;

    /// InitializeHid
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=true.
    ///
    /// **Note:** This is used by btm-sysmodule, this should not be used by other processes.
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn btdrvInitializeHid(out_event: *mut Event) -> Result;

    /// OpenHidConnection
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvOpenHidConnection(addr: BtdrvAddress) -> Result;

    /// CloseHidConnection
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvCloseHidConnection(addr: BtdrvAddress) -> Result;

    /// This sends a HID DATA transaction packet with report-type Output.
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    /// - `buffer` (in): Input `BtdrvHidReport`, on pre-9.0.0 this is `BtdrvHidData`.
    pub fn btdrvWriteHidData(addr: BtdrvAddress, buffer: *const BtdrvHidReport) -> Result;

    /// WriteHidData2
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    /// - `buffer` (in): Input buffer, same as the buffer for `btdrvWriteHidData`.
    /// - `size` (in): Input buffer size.
    pub fn btdrvWriteHidData2(addr: BtdrvAddress, buffer: *const c_void, size: usize) -> Result;

    /// This sends a HID SET_REPORT transaction packet.
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    /// - `type` (in): `BtdrvBluetoothHhReportType`
    /// - `buffer` (in): Input `BtdrvHidReport`, on pre-9.0.0 this is `BtdrvHidData`.
    pub fn btdrvSetHidReport(addr: BtdrvAddress, type_: BtdrvBluetoothHhReportType, buffer: *const BtdrvHidReport) -> Result;

    /// This sends a HID GET_REPORT transaction packet.
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    /// - `report_id` (in): This is sent in the packet for the Report Id, when non-zero.
    /// - `type` (in): `BtdrvBluetoothHhReportType`
    pub fn btdrvGetHidReport(addr: BtdrvAddress, report_id: u8, type_: BtdrvBluetoothHhReportType) -> Result;

    /// TriggerConnection
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    /// - `timeout` (in): [9.0.0+] Host trigger timeout
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvTriggerConnection(addr: BtdrvAddress, timeout: u16) -> Result;

    /// AddPairedDeviceInfo
    ///
    /// # Arguments
    /// - `settings` (in): `SetSysBluetoothDevicesSettings`
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvAddPairedDeviceInfo(settings: *const SetSysBluetoothDevicesSettings) -> Result;

    /// GetPairedDeviceInfo
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    /// - `settings` (out): `SetSysBluetoothDevicesSettings`
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvGetPairedDeviceInfo(addr: BtdrvAddress, settings: *mut SetSysBluetoothDevicesSettings) -> Result;

    /// FinalizeHid
    ///
    /// **Note:** This is not used by btm-sysmodule, this should not be used by other processes.
    pub fn btdrvFinalizeHid() -> Result;

    /// GetHidEventInfo
    ///
    /// # Arguments
    /// - `buffer` (out): Output buffer, see `BtdrvHidEventInfo`.
    /// - `size` (in): Output buffer size.
    /// - `type` (out): `BtdrvHidEventType`, always ::BtdrvHidEventType_Connection or ::BtdrvHidEventType_Ext.
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvGetHidEventInfo(buffer: *mut c_void, size: usize, type_: *mut BtdrvHidEventType) -> Result;

    /// SetTsi
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    /// - `tsi` (in): Tsi: non-value-0xFF to Set, value 0xFF to Exit. See also `BtmTsiMode`.
    ///
    /// **Note:** The response will be available via `btdrvGetHidEventInfo` ([12.0.0+] `btdrvGetEventInfo`).
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvSetTsi(addr: BtdrvAddress, tsi: u8) -> Result;

    /// EnableBurstMode
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    /// - `flag` (in): Flag: true = Set, false = Exit.
    ///
    /// **Note:** The response will be available via `btdrvGetHidEventInfo` ([12.0.0+] `btdrvGetEventInfo`).
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvEnableBurstMode(addr: BtdrvAddress, flag: bool) -> Result;

    /// SetZeroRetransmission
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    /// - `report_ids` (in): Input buffer containing an array of u8s.
    /// - `count` (in): Total u8s in the input buffer. This can be 0, the max is 5.
    ///
    /// **Note:** The response will be available via `btdrvGetHidEventInfo` ([12.0.0+] `btdrvGetEventInfo`).
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvSetZeroRetransmission(addr: BtdrvAddress, report_ids: *mut u8, count: u8) -> Result;

    /// EnableMcMode
    ///
    /// # Arguments
    /// - `flag` (in): Flag
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvEnableMcMode(flag: bool) -> Result;

    /// EnableLlrScan
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvEnableLlrScan() -> Result;

    /// DisableLlrScan
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvDisableLlrScan() -> Result;

    /// EnableRadio
    ///
    /// # Arguments
    /// - `flag` (in): Flag
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvEnableRadio(flag: bool) -> Result;

    /// SetVisibility
    ///
    /// # Arguments
    /// - `inquiry_scan` (in): Controls Inquiry Scan, whether the device can be discovered during Inquiry.
    /// - `page_scan` (in): Controls Page Scan, whether the device accepts connections.
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvSetVisibility(inquiry_scan: bool, page_scan: bool) -> Result;

    /// EnableTbfcScan
    ///
    /// # Arguments
    /// - `flag` (in): Flag
    ///
    /// **Note:** Only available on [4.0.0+].
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvEnableTbfcScan(flag: bool) -> Result;

    /// RegisterHidReportEvent
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=true. This is signaled when data is available with `btdrvGetHidReportEventInfo`.
    ///
    /// **Note:** This also does sharedmem init/handling if needed, on [7.0.0+].
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn btdrvRegisterHidReportEvent(out_event: *mut Event) -> Result;

    /// GetHidReportEventInfo
    ///
    /// # Arguments
    /// - `buffer` (out): Output buffer, see `BtdrvHidReportEventInfo`.
    /// - `size` (in): Output buffer size.
    /// - `type` (out): `BtdrvHidEventType`
    ///
    /// **Note:** `btdrvRegisterHidReportEvent` must be used before this, on [7.0.0+].
    ///
    /// **Note:** This is used by hid-sysmodule. When used by other processes, hid/user-process will conflict. No events will be received by that user-process, or it will be corrupted, etc.
    ///
    /// **Note:** [7.0.0+] When data isn't available, the type is set to ::BtdrvHidEventType_Data, with the buffer cleared to all-zero.
    pub fn btdrvGetHidReportEventInfo(buffer: *mut c_void, size: usize, type_: *mut BtdrvHidEventType) -> Result;

    /// Gets the SharedMemory addr for HidReportEventInfo (`BtdrvCircularBuffer`), only valid when `btdrvRegisterHidReportEvent` was previously used, on [7.0.0+].
    pub fn btdrvGetHidReportEventInfoSharedmemAddr() -> *mut c_void;

    /// GetLatestPlr
    ///
    /// # Arguments
    /// - `out` (out): Output `BtdrvPlrList`, on pre-9.0.0 this is `BtdrvPlrStatistics`.
    pub fn btdrvGetLatestPlr(out: *mut BtdrvPlrList) -> Result;

    /// GetPendingConnections
    ///
    /// **Note:** The output data will be available via `btdrvGetHidEventInfo` ([12.0.0+] `btdrvGetEventInfo`).
    ///
    /// **Note:** This is used by btm-sysmodule.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn btdrvGetPendingConnections() -> Result;

    /// GetChannelMap
    ///
    /// # Arguments
    /// - `out` (out): `BtdrvChannelMapList`
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn btdrvGetChannelMap(out: *mut BtdrvChannelMapList) -> Result;

    /// EnableTxPowerBoostSetting
    ///
    /// # Arguments
    /// - `flag` (in): Input flag.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn btdrvEnableTxPowerBoostSetting(flag: bool) -> Result;

    /// IsTxPowerBoostSettingEnabled
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn btdrvIsTxPowerBoostSettingEnabled(out: *mut bool) -> Result;

    /// EnableAfhSetting
    ///
    /// # Arguments
    /// - `flag` (in): Input flag.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn btdrvEnableAfhSetting(flag: bool) -> Result;

    /// IsAfhSettingEnabled
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn btdrvIsAfhSettingEnabled(out: *mut bool) -> Result;

    /// InitializeBle
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=true.
    ///
    /// **Note:** Only available on [5.0.0+].
    ///
    /// **Note:** This is used by btm-sysmodule.
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn btdrvInitializeBle(out_event: *mut Event) -> Result;

    /// EnableBle
    ///
    /// **Note:** Only available on [5.0.0+].
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvEnableBle() -> Result;

    /// DisableBle
    ///
    /// **Note:** Only available on [5.0.0+].
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvDisableBle() -> Result;

    /// FinalizeBle
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btdrvFinalizeBle() -> Result;

    /// SetBleVisibility
    ///
    /// # Arguments
    /// - `discoverable` (in): Whether the BLE device is discoverable.
    /// - `connectable` (in): Whether the BLE device is connectable.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btdrvSetBleVisibility(discoverable: bool, connectable: bool) -> Result;

    /// SetLeConnectionParameter
    ///
    /// # Arguments
    /// - `param` (in): `BtdrvLeConnectionParams`
    ///
    /// **Note:** Only available on [5.0.0-8.1.1]. This is the older version of `btdrvSetBleConnectionParameter`.
    pub fn btdrvSetLeConnectionParameter(param: *const BtdrvLeConnectionParams) -> Result;

    /// SetBleConnectionParameter
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    /// - `param` (in): `BtdrvBleConnectionParameter`
    /// - `preference` (in): Unused
    ///
    /// **Note:** Only available on [9.0.0+]. This is the newer version of `btdrvSetLeConnectionParameter`.
    pub fn btdrvSetBleConnectionParameter(addr: BtdrvAddress, param: *const BtdrvBleConnectionParameter, preference: bool) -> Result;

    /// SetLeDefaultConnectionParameter
    ///
    /// # Arguments
    /// - `param` (in): `BtdrvLeConnectionParams`
    ///
    /// **Note:** Only available on [5.0.0-8.1.1]. This is the older version of `btdrvSetBleDefaultConnectionParameter`.
    pub fn btdrvSetLeDefaultConnectionParameter(param: *const BtdrvLeConnectionParams) -> Result;

    /// SetBleDefaultConnectionParameter
    ///
    /// # Arguments
    /// - `param` (in): `BtdrvBleConnectionParameter`
    ///
    /// **Note:** Only available on [9.0.0+]. This is the newer version of `btdrvSetLeDefaultConnectionParameter`.
    pub fn btdrvSetBleDefaultConnectionParameter(param: *const BtdrvBleConnectionParameter) -> Result;

    /// SetBleAdvertiseData
    ///
    /// # Arguments
    /// - `data` (in): `BtdrvBleAdvertisePacketData`
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btdrvSetBleAdvertiseData(data: *const BtdrvBleAdvertisePacketData) -> Result;

    /// SetBleAdvertiseParameter
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    /// - `min_interval` (in): Minimum advertisement interval
    /// - `max_interval` (in): Maximum advertisement interval
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btdrvSetBleAdvertiseParameter(addr: BtdrvAddress, min_interval: u16, max_interval: u16) -> Result;

    /// StartBleScan
    ///
    /// **Note:** Only available on [5.0.0+].
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvStartBleScan() -> Result;

    /// StopBleScan
    ///
    /// **Note:** Only available on [5.0.0+].
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvStopBleScan() -> Result;

    /// AddBleScanFilterCondition
    ///
    /// # Arguments
    /// - `filter` (in): `BtdrvBleAdvertiseFilter`
    ///
    /// **Note:** Only available on [5.0.0+].
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvAddBleScanFilterCondition(filter: *const BtdrvBleAdvertiseFilter) -> Result;

    /// DeleteBleScanFilterCondition
    ///
    /// # Arguments
    /// - `filter` (in): `BtdrvBleAdvertiseFilter`
    ///
    /// **Note:** Only available on [5.0.0+].
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvDeleteBleScanFilterCondition(filter: *const BtdrvBleAdvertiseFilter) -> Result;

    /// DeleteBleScanFilter
    ///
    /// # Arguments
    /// - `index` (in): Filter index
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btdrvDeleteBleScanFilter(index: u8) -> Result;

    /// ClearBleScanFilters
    ///
    /// **Note:** Only available on [5.0.0+].
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvClearBleScanFilters() -> Result;

    /// EnableBleScanFilter
    ///
    /// # Arguments
    /// - `flag` (in): Flag
    ///
    /// **Note:** Only available on [5.0.0+].
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvEnableBleScanFilter(flag: bool) -> Result;

    /// RegisterGattClient
    ///
    /// # Arguments
    /// - `uuid` (in): `BtdrvGattAttributeUuid`
    ///
    /// **Note:** Only available on [5.0.0+].
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvRegisterGattClient(uuid: *const BtdrvGattAttributeUuid) -> Result;

    /// UnregisterGattClient
    ///
    /// # Arguments
    /// - `client_if` (in): Client interface ID
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btdrvUnregisterGattClient(client_if: u8) -> Result;

    /// UnregisterAllGattClients
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btdrvUnregisterAllGattClients() -> Result;

    /// ConnectGattServer
    ///
    /// # Arguments
    /// - `client_if` (in): Client interface ID
    /// - `addr` (in): `BtdrvAddress`
    /// - `is_direct` (in): Whether a direct connection or a background auto connection
    /// - `AppletResourceUserId` (in): AppletResourceUserId
    ///
    /// **Note:** Only available on [5.0.0+].
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvConnectGattServer(client_if: u8, addr: BtdrvAddress, is_direct: bool, AppletResourceUserId: u64) -> Result;

    /// CancelConnectGattServer
    ///
    /// # Arguments
    /// - `client_if` (in): Client interface ID
    /// - `addr` (in): `BtdrvAddress`
    /// - `is_direct` (in): Whether a direct connection or a background auto connection
    ///
    /// **Note:** Only available on [5.1.0+].
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvCancelConnectGattServer(client_if: u8, addr: BtdrvAddress, is_direct: bool) -> Result;

    /// DisconnectGattServer
    ///
    /// # Arguments
    /// - `conn_id` (in): Connection ID
    ///
    /// **Note:** Only available on [5.0.0+].
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvDisconnectGattServer(conn_id: u32) -> Result;

    /// GetGattAttribute
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`, only used on pre-9.0.0.
    /// - `conn_id` (in): Connection ID
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btdrvGetGattAttribute(addr: BtdrvAddress, conn_id: u32) -> Result;

    /// GetGattService
    ///
    /// # Arguments
    /// - `conn_id` (in): Connection ID
    /// - `uuid` (in): `BtdrvGattAttributeUuid`
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btdrvGetGattService(conn_id: u32, uuid: *const BtdrvGattAttributeUuid) -> Result;

    /// ConfigureAttMtu
    ///
    /// # Arguments
    /// - `conn_id` (in): Connection ID
    /// - `mtu` (in): MTU
    ///
    /// **Note:** Only available on [5.0.0+].
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvConfigureAttMtu(conn_id: u32, mtu: u16) -> Result;

    /// RegisterGattServer
    ///
    /// # Arguments
    /// - `uuid` (in): `BtdrvGattAttributeUuid`
    ///
    /// **Note:** Only available on [5.0.0+].
    ///
    /// **Note:** Event data generated by this call contains uninitialized junk instead of the server_if value received internally.
    pub fn btdrvRegisterGattServer(uuid: *const BtdrvGattAttributeUuid) -> Result;

    /// UnregisterGattServer
    ///
    /// # Arguments
    /// - `server_if` (in): Server interface ID
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btdrvUnregisterGattServer(server_if: u8) -> Result;

    /// ConnectGattClient
    ///
    /// # Arguments
    /// - `server_if` (in): Server interface ID
    /// - `addr` (in): `BtdrvAddress`
    /// - `is_direct` (in): Whether a direct connection or a background auto connection
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btdrvConnectGattClient(server_if: u8, addr: BtdrvAddress, is_direct: bool) -> Result;

    /// DisconnectGattClient
    ///
    /// # Arguments
    /// - `conn_id` (in): Connection ID
    /// - `addr` (in): `BtdrvAddress`, only used on pre-9.0.0.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btdrvDisconnectGattClient(conn_id: u8, addr: BtdrvAddress) -> Result;

    /// AddGattService
    ///
    /// # Arguments
    /// - `server_if` (in): Server interface ID
    /// - `uuid` (in): `BtdrvGattAttributeUuid`
    /// - `num_handle` (in): Number of handles
    /// - `is_primary` (in): Is a primary service or not
    ///
    /// **Note:** Only available on [5.0.0+].
    ///
    /// **Note:** Broken behaviour due to internal bugs.
    pub fn btdrvAddGattService(server_if: u8, uuid: *const BtdrvGattAttributeUuid, num_handle: u8, is_primary: bool) -> Result;

    /// EnableGattService
    ///
    /// # Arguments
    /// - `server_if` (in): Server interface ID
    /// - `uuid` (in): `BtdrvGattAttributeUuid`
    ///
    /// **Note:** Only available on [5.0.0+].
    ///
    /// **Note:** Broken behaviour due to internal bugs.
    pub fn btdrvEnableGattService(server_if: u8, uuid: *const BtdrvGattAttributeUuid) -> Result;

    /// AddGattCharacteristic
    ///
    /// # Arguments
    /// - `server_if` (in): Server interface ID
    /// - `serv_uuid` (in): Service UUID `BtdrvGattAttributeUuid`
    /// - `char_uuid` (in): Characteristic UUID `BtdrvGattAttributeUuid`
    /// - `permissions` (in): `BtdrvGattAttributePermission`
    /// - `property` (in): `BtdrvGattCharacteristicProperty`
    ///
    /// **Note:** Only available on [5.0.0+].
    ///
    /// **Note:** Broken behaviour due to internal bugs.
    pub fn btdrvAddGattCharacteristic(server_if: u8, serv_uuid: *const BtdrvGattAttributeUuid, char_uuid: *const BtdrvGattAttributeUuid, permissions: u16, property: u8) -> Result;

    /// AddGattDescriptor
    ///
    /// # Arguments
    /// - `server_if` (in): Server interface ID
    /// - `serv_uuid` (in): Service UUID `BtdrvGattAttributeUuid`
    /// - `desc_uuid` (in): Descriptor UUID `BtdrvGattAttributeUuid`
    /// - `permissions` (in): `BtdrvGattAttributePermission`
    ///
    /// **Note:** Only available on [5.0.0+].
    ///
    /// **Note:** Broken behaviour due to internal bugs.
    pub fn btdrvAddGattDescriptor(server_if: u8, serv_uuid: *const BtdrvGattAttributeUuid, desc_uuid: *const BtdrvGattAttributeUuid, permissions: u16) -> Result;

    /// GetBleManagedEventInfo
    ///
    /// # Arguments
    /// - `buffer` (out): Output buffer. 0x400-bytes from state is written here. See `BtdrvBleEventInfo`.
    /// - `size` (in): Output buffer size.
    /// - `type` (out): Output BtdrvBleEventType.
    ///
    /// **Note:** Only available on [5.0.0+].
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvGetBleManagedEventInfo(buffer: *mut c_void, size: usize, type_: *mut BtdrvBleEventType) -> Result;

    /// GetGattFirstCharacteristic
    ///
    /// # Arguments
    /// - `conn_id` (in): Connection ID
    /// - `serv_id` (in): Service GATT ID `BtdrvGattId`
    /// - `is_primary` (in): Is a primary service or not
    /// - `filter_uuid` (in): Characteristic filter UUID `BtdrvGattAttributeUuid`
    /// - `out_property` (out): Output property `BtdrvGattCharacteristicProperty`
    /// - `out_char_id` (out): Output characteristic GATT ID `BtdrvGattId`
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btdrvGetGattFirstCharacteristic(conn_id: u32, serv_id: *const BtdrvGattId, is_primary: bool, filter_uuid: *const BtdrvGattAttributeUuid, out_property: *mut u8, out_char_id: *mut BtdrvGattId) -> Result;

    /// GetGattNextCharacteristic
    ///
    /// # Arguments
    /// - `conn_id` (in): Connection ID
    /// - `serv_id` (in): Service ID `BtdrvGattId`
    /// - `is_primary` (in): Is a primary service or not
    /// - `char_id` (in): Previous characteristic GATT ID `BtdrvGattId`
    /// - `filter_uuid` (in): Characteristic filter UUID `BtdrvGattAttributeUuid`
    /// - `out_property` (out): Output property `BtdrvGattCharacteristicProperty`
    /// - `out_char_id` (out): Output characteristic GATT ID `BtdrvGattId`
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btdrvGetGattNextCharacteristic(conn_id: u32, serv_id: *const BtdrvGattId, is_primary: bool, char_id: *const BtdrvGattId, filter_uuid: *const BtdrvGattAttributeUuid, out_property: *mut u8, out_char_id: *mut BtdrvGattId) -> Result;

    /// GetGattFirstDescriptor
    ///
    /// # Arguments
    /// - `conn_id` (in): Connection ID
    /// - `serv_id` (in): Service GATT ID `BtdrvGattId`
    /// - `is_primary` (in): Is a primary service or not
    /// - `char_id` (in): Characteristic GATT ID `BtdrvGattId`
    /// - `filter_uuid` (in): Descriptor filter UUID `BtdrvGattAttributeUuid`
    /// - `out_desc_id` (out): Output descriptor GATT ID `BtdrvGattId`
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btdrvGetGattFirstDescriptor(conn_id: u32, serv_id: *const BtdrvGattId, is_primary: bool, char_id: *const BtdrvGattId, filter_uuid: *const BtdrvGattAttributeUuid, out_desc_id: *mut BtdrvGattId) -> Result;

    /// GetGattNextDescriptor
    ///
    /// # Arguments
    /// - `conn_id` (in): Connection ID
    /// - `serv_id` (in): Service GATT ID `BtdrvGattId`
    /// - `is_primary` (in): Is a primary service or not
    /// - `char_id` (in): Characteristic GATT ID `BtdrvGattId`
    /// - `desc_id` (in): Previous descriptor GATT ID `BtdrvGattId`
    /// - `filter_uuid` (in): Descriptor filter UUID `BtdrvGattAttributeUuid`
    /// - `out_desc_id` (out): Output descriptor GATT ID `BtdrvGattId`
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btdrvGetGattNextDescriptor(conn_id: u32, serv_id: *const BtdrvGattId, is_primary: bool, char_id: *const BtdrvGattId, desc_id: *const BtdrvGattId, filter_uuid: *const BtdrvGattAttributeUuid, out_desc_id: *mut BtdrvGattId) -> Result;

    /// RegisterGattManagedDataPath
    ///
    /// # Arguments
    /// - `uuid` (in): `BtdrvGattAttributeUuid`
    ///
    /// **Note:** Only available on [5.0.0+].
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvRegisterGattManagedDataPath(uuid: *const BtdrvGattAttributeUuid) -> Result;

    /// UnregisterGattManagedDataPath
    ///
    /// # Arguments
    /// - `uuid` (in): `BtdrvGattAttributeUuid`
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btdrvUnregisterGattManagedDataPath(uuid: *const BtdrvGattAttributeUuid) -> Result;

    /// RegisterGattHidDataPath
    ///
    /// # Arguments
    /// - `uuid` (in): `BtdrvGattAttributeUuid`
    ///
    /// **Note:** Only available on [5.0.0+].
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvRegisterGattHidDataPath(uuid: *const BtdrvGattAttributeUuid) -> Result;

    /// UnregisterGattHidDataPath
    ///
    /// # Arguments
    /// - `uuid` (in): `BtdrvGattAttributeUuid`
    ///
    /// **Note:** Only available on [5.0.0+].
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvUnregisterGattHidDataPath(uuid: *const BtdrvGattAttributeUuid) -> Result;

    /// RegisterGattDataPath
    ///
    /// # Arguments
    /// - `uuid` (in): `BtdrvGattAttributeUuid`
    ///
    /// **Note:** Only available on [5.0.0+].
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvRegisterGattDataPath(uuid: *const BtdrvGattAttributeUuid) -> Result;

    /// UnregisterGattDataPath
    ///
    /// # Arguments
    /// - `uuid` (in): `BtdrvGattAttributeUuid`
    ///
    /// **Note:** Only available on [5.0.0+].
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvUnregisterGattDataPath(uuid: *const BtdrvGattAttributeUuid) -> Result;

    /// ReadGattCharacteristic
    ///
    /// # Arguments
    /// - `connection_handle` (in): ConnectionHandle
    /// - `is_primary` (in): Is a primary service or not
    /// - `serv_id` (in): Service GATT ID `BtdrvGattId`
    /// - `char_id` (in): Characteristic GATT ID `BtdrvGattId`
    /// - `auth_req` (in): `BtdrvGattAuthReqType`
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btdrvReadGattCharacteristic(connection_handle: u32, is_primary: bool, serv_id: *const BtdrvGattId, char_id: *const BtdrvGattId, auth_req: u8) -> Result;

    /// ReadGattDescriptor
    ///
    /// # Arguments
    /// - `connection_handle` (in): ConnectionHandle
    /// - `is_primary` (in): Is a primary service or not
    /// - `serv_id` (in): Service GATT ID `BtdrvGattId`
    /// - `char_id` (in): Characteristic GATT ID `BtdrvGattId`
    /// - `desc_id` (in): Descriptor GATT ID `BtdrvGattId`
    /// - `auth_req` (in): `BtdrvGattAuthReqType`
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btdrvReadGattDescriptor(connection_handle: u32, is_primary: bool, serv_id: *const BtdrvGattId, char_id: *const BtdrvGattId, desc_id: *const BtdrvGattId, auth_req: u8) -> Result;

    /// WriteGattCharacteristic
    ///
    /// # Arguments
    /// - `connection_handle` (in): ConnectionHandle
    /// - `is_primary` (in): Is a primary service or not
    /// - `serv_id` (in): Service GATT ID `BtdrvGattId`
    /// - `char_id` (in): Characteristic GATT ID `BtdrvGattId`
    /// - `buffer` (in): Input buffer.
    /// - `size` (in): Input buffer size, must be &lt;=0x258.
    /// - `auth_req` (in): `BtdrvGattAuthReqType`
    /// - `with_response` (in): Whether to use Write-With-Response write type or not
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btdrvWriteGattCharacteristic(connection_handle: u32, is_primary: bool, serv_id: *const BtdrvGattId, char_id: *const BtdrvGattId, buffer: *const c_void, size: usize, auth_req: u8, with_response: bool) -> Result;

    /// WriteGattDescriptor
    ///
    /// # Arguments
    /// - `connection_handle` (in): ConnectionHandle
    /// - `is_primary` (in): Is a primary service or not
    /// - `serv_id` (in): Service GATT ID `BtdrvGattId`
    /// - `char_id` (in): Characteristic GATT ID `BtdrvGattId`
    /// - `desc_id` (in): Descriptor GATT ID `BtdrvGattId`
    /// - `buffer` (in): Input buffer.
    /// - `size` (in): Input buffer size, must be &lt;=0x258.
    /// - `auth_req` (in): `BtdrvGattAuthReqType`
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btdrvWriteGattDescriptor(connection_handle: u32, is_primary: bool, serv_id: *const BtdrvGattId, char_id: *const BtdrvGattId, desc_id: *const BtdrvGattId, buffer: *const c_void, size: usize, auth_req: u8) -> Result;

    /// RegisterGattNotification
    ///
    /// # Arguments
    /// - `connection_handle` (in): ConnectionHandle
    /// - `is_primary` (in): Is a primary service or not
    /// - `serv_id` (in): Service GATT ID `BtdrvGattId`
    /// - `char_id` (in): Characteristic GATT ID `BtdrvGattId`
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btdrvRegisterGattNotification(connection_handle: u32, is_primary: bool, serv_id: *const BtdrvGattId, char_id: *const BtdrvGattId) -> Result;

    /// UnregisterGattNotification
    ///
    /// # Arguments
    /// - `connection_handle` (in): ConnectionHandle
    /// - `is_primary` (in): Is a primary service or not
    /// - `serv_id` (in): Service GATT ID `BtdrvGattId`
    /// - `char_id` (in): Characteristic GATT ID `BtdrvGattId`
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btdrvUnregisterGattNotification(connection_handle: u32, is_primary: bool, serv_id: *const BtdrvGattId, char_id: *const BtdrvGattId) -> Result;

    /// GetLeHidEventInfo
    ///
    /// # Arguments
    /// - `buffer` (out): Output buffer. 0x400-bytes from state is written here. See `BtdrvBleEventInfo`.
    /// - `size` (in): Output buffer size.
    /// - `type` (out): `BtdrvBleEventType`.
    ///
    /// **Note:** Only available on [5.0.0+].
    ///
    /// **Note:** The state used by this is reset after writing the data to output.
    pub fn btdrvGetLeHidEventInfo(buffer: *mut c_void, size: usize, type_: *mut BtdrvBleEventType) -> Result;

    /// RegisterBleHidEvent
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=true.
    ///
    /// **Note:** Only available on [5.0.0+].
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn btdrvRegisterBleHidEvent(out_event: *mut Event) -> Result;

    /// SetBleScanParameter
    ///
    /// # Arguments
    /// - `scan_interval` (in): Scan interval
    /// - `scan_window` (in): Scan window
    ///
    /// **Note:** Only available on [5.1.0+].
    ///
    /// **Note:** This is used by btm-sysmodule.
    pub fn btdrvSetBleScanParameter(scan_interval: u16, scan_window: u16) -> Result;

    /// MoveToSecondaryPiconet
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    ///
    /// **Note:** The response will be available via `btdrvGetHidEventInfo` ([12.0.0+] `btdrvGetEventInfo`).
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn btdrvMoveToSecondaryPiconet(addr: BtdrvAddress) -> Result;

    /// IsBluetoothEnabled
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [12.0.0+].
    pub fn btdrvIsBluetoothEnabled(out: *mut bool) -> Result;

    /// AcquireAudioEvent
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event.
    /// - `autoclear` (in): Event autoclear.
    ///
    /// **Note:** Only available on [12.0.0+].
    pub fn btdrvAcquireAudioEvent(out_event: *mut Event, autoclear: bool) -> Result;

    /// GetAudioEventInfo
    ///
    /// # Arguments
    /// - `buffer` (out): Output buffer, see `BtdrvAudioEventInfo`.
    /// - `size` (in): Output buffer size.
    /// - `type` (out): `BtdrvAudioEventType`.
    ///
    /// **Note:** Only available on [12.0.0+].
    pub fn btdrvGetAudioEventInfo(buffer: *mut c_void, size: usize, type_: *mut BtdrvAudioEventType) -> Result;

    /// OpenAudioConnection
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    ///
    /// **Note:** Only available on [12.0.0+].
    pub fn btdrvOpenAudioConnection(addr: BtdrvAddress) -> Result;

    /// CloseAudioConnection
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    ///
    /// **Note:** Only available on [12.0.0+].
    pub fn btdrvCloseAudioConnection(addr: BtdrvAddress) -> Result;

    /// OpenAudioOut
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    /// - `audio_handle` (out): Audio handle.
    ///
    /// **Note:** Only available on [12.0.0+].
    pub fn btdrvOpenAudioOut(addr: BtdrvAddress, audio_handle: *mut u32) -> Result;

    /// CloseAudioOut
    ///
    /// # Arguments
    /// - `audio_handle` (in): Audio handle from `btdrvOpenAudioOut`.
    ///
    /// **Note:** Only available on [12.0.0+].
    pub fn btdrvCloseAudioOut(audio_handle: u32) -> Result;

    /// StartAudioOut
    ///
    /// # Arguments
    /// - `audio_handle` (in): Audio handle from `btdrvOpenAudioOut`.
    /// - `pcm_param` (in): `BtdrvPcmParameter`
    /// - `in_latency` (in): Input latency in nanoseconds.
    /// - `out_latency` (out): Output latency in nanoseconds.
    /// - `out1` (out): Unknown output.
    ///
    /// **Note:** Only available on [12.0.0+].
    pub fn btdrvStartAudioOut(audio_handle: u32, pcm_param: *const BtdrvPcmParameter, in_latency: s64, out_latency: *mut s64, out1: *mut u64) -> Result;

    /// StopAudioOut
    ///
    /// # Arguments
    /// - `audio_handle` (in): Audio handle from `btdrvOpenAudioOut`.
    ///
    /// **Note:** Only available on [12.0.0+].
    pub fn btdrvStopAudioOut(audio_handle: u32) -> Result;

    /// GetAudioOutState
    ///
    /// # Arguments
    /// - `audio_handle` (in): Audio handle from `btdrvOpenAudioOut`.
    /// - `out` (out): `BtdrvAudioOutState`
    ///
    /// **Note:** Only available on [12.0.0+].
    pub fn btdrvGetAudioOutState(audio_handle: u32, out: *mut BtdrvAudioOutState) -> Result;

    /// GetAudioOutFeedingCodec
    ///
    /// # Arguments
    /// - `audio_handle` (in): Audio handle from `btdrvOpenAudioOut`.
    /// - `out` (out): `BtdrvAudioCodec`
    ///
    /// **Note:** Only available on [12.0.0+].
    pub fn btdrvGetAudioOutFeedingCodec(audio_handle: u32, out: *mut BtdrvAudioCodec) -> Result;

    /// GetAudioOutFeedingParameter
    ///
    /// # Arguments
    /// - `audio_handle` (in): Audio handle from `btdrvOpenAudioOut`.
    /// - `out` (out): `BtdrvPcmParameter`
    ///
    /// **Note:** Only available on [12.0.0+].
    pub fn btdrvGetAudioOutFeedingParameter(audio_handle: u32, out: *mut BtdrvPcmParameter) -> Result;

    /// AcquireAudioOutStateChangedEvent
    ///
    /// # Arguments
    /// - `audio_handle` (in): Audio handle from `btdrvOpenAudioOut`.
    /// - `out_event` (out): Output Event.
    /// - `autoclear` (in): Event autoclear.
    ///
    /// **Note:** Only available on [12.0.0+].
    pub fn btdrvAcquireAudioOutStateChangedEvent(audio_handle: u32, out_event: *mut Event, autoclear: bool) -> Result;

    /// AcquireAudioOutBufferAvailableEvent
    ///
    /// # Arguments
    /// - `audio_handle` (in): Audio handle from `btdrvOpenAudioOut`.
    /// - `out_event` (out): Output Event.
    /// - `autoclear` (in): Event autoclear.
    ///
    /// **Note:** Only available on [12.0.0+].
    pub fn btdrvAcquireAudioOutBufferAvailableEvent(audio_handle: u32, out_event: *mut Event, autoclear: bool) -> Result;

    /// SendAudioData
    ///
    /// # Arguments
    /// - `audio_handle` (in): Audio handle from `btdrvOpenAudioOut`.
    /// - `buffer` (in): Input buffer.
    /// - `size` (in): Input buffer size.
    /// - `Output` (out): transferred size. This is always either 0 (error occured) or the buffer size.
    ///
    /// **Note:** Only available on [12.0.0+].
    pub fn btdrvSendAudioData(audio_handle: u32, buffer: *const c_void, size: usize, transferred_size: *mut u64) -> Result;

    /// AcquireAudioControlInputStateChangedEvent
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event.
    /// - `autoclear` (in): Event autoclear.
    ///
    /// **Note:** Only available on [12.0.0+].
    pub fn btdrvAcquireAudioControlInputStateChangedEvent(out_event: *mut Event, autoclear: bool) -> Result;

    /// GetAudioControlInputState
    ///
    /// # Arguments
    /// - `states` (out): Output array of `BtdrvAudioControlButtonState`.
    /// - `count` (in): Size of the states array in entries, the maximum is 0xF.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** Only available on [12.0.0+].
    pub fn btdrvGetAudioControlInputState(states: *mut BtdrvAudioControlButtonState, count: s32, total_out: *mut s32) -> Result;

    /// AcquireAudioConnectionStateChangedEvent
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event.
    /// - `autoclear` (in): Event autoclear.
    ///
    /// **Note:** Only available on [12.0.0-13.2.1].
    pub fn btdrvAcquireAudioConnectionStateChangedEvent(out_event: *mut Event, autoclear: bool) -> Result;

    /// GetConnectedAudioDevice
    ///
    /// # Arguments
    /// - `addrs` (out): Output array of `BtdrvAddress`.
    /// - `count` (in): Size of the addrs array in entries, the maximum is 0x8.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** Only available on [12.0.0-13.2.1].
    pub fn btdrvGetConnectedAudioDevice(addrs: *mut BtdrvAddress, count: s32, total_out: *mut s32) -> Result;

    /// CloseAudioControlInput
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    ///
    /// **Note:** Only available on [13.0.0+].
    pub fn btdrvCloseAudioControlInput(addr: BtdrvAddress) -> Result;

    /// RegisterAudioControlNotification
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    /// - `event_type` (in): AvrcEventType
    ///
    /// **Note:** Only available on [13.0.0+].
    pub fn btdrvRegisterAudioControlNotification(addr: BtdrvAddress, event_type: u32) -> Result;

    /// SendAudioControlPassthroughCommand
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    /// - `op_id` (in): AvrcOperationId
    /// - `state_type` (in): AvrcStateType
    ///
    /// **Note:** Only available on [13.0.0+].
    pub fn btdrvSendAudioControlPassthroughCommand(addr: BtdrvAddress, op_id: u32, state_type: u32) -> Result;

    /// SendAudioControlSetAbsoluteVolumeCommand
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    /// - `val` (in): Input value
    ///
    /// **Note:** Only available on [13.0.0+].
    pub fn btdrvSendAudioControlSetAbsoluteVolumeCommand(addr: BtdrvAddress, val: s32) -> Result;

    /// IsManufacturingMode
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn btdrvIsManufacturingMode(out: *mut bool) -> Result;

    /// EmulateBluetoothCrash
    ///
    /// # Arguments
    /// - `reason` (in): `BtdrvFatalReason`
    ///
    /// **Note:** Only available on [7.0.0+].
    pub fn btdrvEmulateBluetoothCrash(reason: BtdrvFatalReason) -> Result;

    /// GetBleChannelMap
    ///
    /// # Arguments
    /// - `out` (out): `BtdrvChannelMapList`
    ///
    /// **Note:** Only available on [9.0.0+].
    pub fn btdrvGetBleChannelMap(out: *mut BtdrvChannelMapList) -> Result;

    /// Read
    ///
    /// # Arguments
    /// - `c`: `BtdrvCircularBuffer`
    ///
    /// **Note:** Used by `btdrvGetHidReportEventInfo` on [7.0.0+].
    pub fn btdrvCircularBufferRead(c: *mut BtdrvCircularBuffer) -> *mut c_void;

    /// Free
    ///
    /// # Arguments
    /// - `c`: `BtdrvCircularBuffer`
    ///
    /// **Note:** Used by `btdrvGetHidReportEventInfo` on [7.0.0+].
    pub fn btdrvCircularBufferFree(c: *mut BtdrvCircularBuffer) -> bool;
}
