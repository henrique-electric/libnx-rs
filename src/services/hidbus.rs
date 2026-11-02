//! `switch/services/hidbus.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// BusType
pub type HidbusBusType = u32;
/// LeftJoyRail
pub const HidbusBusType_LeftJoyRail: HidbusBusType = 0;
/// RightJoyRail
pub const HidbusBusType_RightJoyRail: HidbusBusType = 1;
/// [6.0.0+] RightLarkRail (for microphone).
pub const HidbusBusType_RightLarkRail: HidbusBusType = 2;

/// JoyPollingMode
pub type HidbusJoyPollingMode = u32;
/// SixAxisSensorDisable
pub const HidbusJoyPollingMode_SixAxisSensorDisable: HidbusJoyPollingMode = 0;
/// JoyEnableSixAxisPollingData
pub const HidbusJoyPollingMode_SixAxisSensorEnable: HidbusJoyPollingMode = 1;
/// [6.0.0+] ButtonOnly
pub const HidbusJoyPollingMode_ButtonOnly: HidbusJoyPollingMode = 2;

/// BusHandle
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidbusBusHandle {
    /// AbstractedPadId
    pub abstracted_pad_id: u32,
    /// InternalIndex
    pub internal_index: u8,
    /// PlayerNumber
    pub player_number: u8,
    /// BusTypeId
    pub bus_type_id: u8,
    /// IsValid
    pub is_valid: u8,
}

/// JoyPollingReceivedData
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidbusJoyPollingReceivedData {
    /// Data.
    pub data: [u8; 48],
    /// Size of data.
    pub out_size: u64,
    /// SamplingNumber
    pub sampling_number: u64,
}

/// HidbusDataAccessorHeader
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidbusDataAccessorHeader {
    /// Result.
    pub res: Result,
    /// Padding.
    pub pad: u32,
    /// Initialized sysmodule-side, not used by sdknso.
    pub unused: [u8; 24],
    /// Latest entry.
    pub latest_entry: u64,
    /// Total entries.
    pub total_entries: u64,
}

/// HidbusJoyDisableSixAxisPollingDataAccessorEntryData
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidbusJoyDisableSixAxisPollingDataAccessorEntryData {
    /// Data.
    pub data: [u8; 38],
    /// Size of data.
    pub out_size: u8,
    /// Padding.
    pub pad: u8,
    /// SamplingNumber
    pub sampling_number: u64,
}

/// HidbusJoyDisableSixAxisPollingDataAccessorEntry
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidbusJoyDisableSixAxisPollingDataAccessorEntry {
    /// SamplingNumber
    pub sampling_number: u64,
    /// **Ref:** HidbusJoyDisableSixAxisPollingDataAccessorEntryData
    pub data: HidbusJoyDisableSixAxisPollingDataAccessorEntryData,
}

/// HidbusJoyEnableSixAxisPollingDataAccessorEntryData
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidbusJoyEnableSixAxisPollingDataAccessorEntryData {
    /// Data.
    pub data: [u8; 8],
    /// Size of data.
    pub out_size: u8,
    /// Padding.
    pub pad: [u8; 7],
    /// SamplingNumber
    pub sampling_number: u64,
}

/// HidbusJoyEnableSixAxisPollingDataAccessorEntry
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidbusJoyEnableSixAxisPollingDataAccessorEntry {
    /// SamplingNumber
    pub sampling_number: u64,
    /// **Ref:** HidbusJoyEnableSixAxisPollingDataAccessorEntryData
    pub data: HidbusJoyEnableSixAxisPollingDataAccessorEntryData,
}

/// HidbusJoyButtonOnlyPollingDataAccessorEntryData
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidbusJoyButtonOnlyPollingDataAccessorEntryData {
    /// Data.
    pub data: [u8; 44],
    /// Size of data.
    pub out_size: u8,
    /// Padding.
    pub pad: [u8; 3],
    /// SamplingNumber
    pub sampling_number: u64,
}

/// HidbusJoyButtonOnlyPollingDataAccessorEntry
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidbusJoyButtonOnlyPollingDataAccessorEntry {
    /// SamplingNumber
    pub sampling_number: u64,
    /// **Ref:** HidbusJoyEnableSixAxisPollingDataAccessorEntryData
    pub data: HidbusJoyButtonOnlyPollingDataAccessorEntryData,
}

/// HidbusJoyDisableSixAxisPollingDataAccessor
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidbusJoyDisableSixAxisPollingDataAccessor {
    /// **Ref:** HidbusDataAccessorHeader
    pub hdr: HidbusDataAccessorHeader,
    /// **Ref:** HidbusJoyDisableSixAxisPollingDataAccessorEntry
    pub entries: [HidbusJoyDisableSixAxisPollingDataAccessorEntry; 11],
}

/// HidbusJoyEnableSixAxisPollingDataAccessor
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidbusJoyEnableSixAxisPollingDataAccessor {
    /// **Ref:** HidbusDataAccessorHeader
    pub hdr: HidbusDataAccessorHeader,
    /// **Ref:** HidbusJoyEnableSixAxisPollingDataAccessorEntry
    pub entries: [HidbusJoyEnableSixAxisPollingDataAccessorEntry; 11],
}

/// HidbusJoyButtonOnlyPollingDataAccessor
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidbusJoyButtonOnlyPollingDataAccessor {
    /// **Ref:** HidbusDataAccessorHeader
    pub hdr: HidbusDataAccessorHeader,
    /// **Ref:** HidbusJoyButtonOnlyPollingDataAccessorEntry
    pub entries: [HidbusJoyButtonOnlyPollingDataAccessorEntry; 11],
}

/// Common data for HidbusStatusManagerEntry*.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidbusStatusManagerEntryCommon {
    /// IsConnected
    pub is_connected: u8,
    /// Padding.
    pub pad: [u8; 3],
    /// IsConnectedResult
    pub is_connected_result: Result,
    /// Flag indicating whether a device is enabled (`hidbusEnableExternalDevice`).
    pub is_enabled: u8,
    /// Flag indicating whether this entry is valid.
    pub is_in_focus: u8,
    /// Flag indicating whether polling is enabled (`hidbusEnableJoyPollingReceiveMode`).
    pub is_polling_mode: u8,
    /// Reserved
    pub reserved: u8,
    /// **Ref:** HidbusJoyPollingMode
    pub polling_mode: u32,
}

/// HidbusStatusManagerEntry on 5.x.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidbusStatusManagerEntryV5 {
    /// **Ref:** HidbusStatusManagerEntryCommon
    pub common: HidbusStatusManagerEntryCommon,
    /// Ignored by official sw.
    pub unk_x10: [u8; 240],
}

/// HidbusStatusManagerEntry
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidbusStatusManagerEntry {
    /// **Ref:** HidbusStatusManagerEntryCommon
    pub common: HidbusStatusManagerEntryCommon,
    /// Ignored by official sw.
    pub unk_x10: [u8; 112],
}

/// StatusManager on 5.x.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidbusStatusManagerV5 {
    /// **Ref:** HidbusStatusManagerEntryV5
    pub entries: [HidbusStatusManagerEntryV5; 16],
}

/// StatusManager
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidbusStatusManager {
    /// **Ref:** HidbusStatusManagerEntry
    pub entries: [HidbusStatusManagerEntry; 19],
    /// Unused.
    pub unused: [u8; 1664],
}

impl_zeroed_default!(HidbusBusHandle, HidbusJoyPollingReceivedData, HidbusDataAccessorHeader, HidbusJoyDisableSixAxisPollingDataAccessorEntryData, HidbusJoyDisableSixAxisPollingDataAccessorEntry, HidbusJoyEnableSixAxisPollingDataAccessorEntryData, HidbusJoyEnableSixAxisPollingDataAccessorEntry, HidbusJoyButtonOnlyPollingDataAccessorEntryData, HidbusJoyButtonOnlyPollingDataAccessorEntry, HidbusJoyDisableSixAxisPollingDataAccessor, HidbusJoyEnableSixAxisPollingDataAccessor, HidbusJoyButtonOnlyPollingDataAccessor, HidbusStatusManagerEntryCommon, HidbusStatusManagerEntryV5, HidbusStatusManagerEntry, HidbusStatusManagerV5, HidbusStatusManager);

unsafe extern "C" {
    /// Gets the Service object for the actual hidbus service session. This object must be closed by the user once finished using cmds with this.
    pub fn hidbusGetServiceSession(srv_out: *mut Service) -> Result;

    /// Gets the SharedMemory addr (`HidbusStatusManagerV5` on 5.x, otherwise `HidbusStatusManager`). Only valid when at least one BusHandle is currently initialized (`hidbusInitialize`).
    pub fn hidbusGetSharedmemAddr() -> *mut c_void;

    /// GetBusHandle
    ///
    /// # Arguments
    /// - `handle` (out): `HidbusBusHandle`
    /// - `flag` (out): Output flag indicating whether the handle is valid.
    /// - `id` (in): `HidNpadIdType`
    /// - `bus_type` (in): `HidbusBusType`
    pub fn hidbusGetBusHandle(handle: *mut HidbusBusHandle, flag: *mut bool, id: HidNpadIdType, bus_type: HidbusBusType) -> Result;

    /// Initialize
    ///
    /// # Arguments
    /// - `handle` (in): `HidbusBusHandle`
    pub fn hidbusInitialize(handle: HidbusBusHandle) -> Result;

    /// Finalize
    ///
    /// # Arguments
    /// - `handle` (in): `HidbusBusHandle`
    pub fn hidbusFinalize(handle: HidbusBusHandle) -> Result;

    /// EnableExternalDevice
    ///
    /// # Arguments
    /// - `handle` (in): `HidbusBusHandle`
    /// - `flag` (in): Whether to enable the device (true = enable, false = disable). When false, this will internally use `hidbusDisableJoyPollingReceiveMode` if needed.
    /// - `device_id` (in): ExternalDeviceId which must match the connected device. Only used when flag is set.
    ///
    /// **Note:** This uses `hidLaShowControllerFirmwareUpdate` if needed.
    pub fn hidbusEnableExternalDevice(handle: HidbusBusHandle, flag: bool, device_id: u32) -> Result;

    /// SendAndReceive
    ///
    /// # Arguments
    /// - `handle` (in): `HidbusBusHandle`
    /// - `inbuf` (in): Input buffer, containing the command data.
    /// - `inbuf_size` (in): Input buffer size, must be &lt;0x26.
    /// - `outbuf` (out): Output buffer, containing the command reply data.
    /// - `outbuf_size` (in): Output buffer max size.
    /// - `out_size` (out): Actual output size.
    pub fn hidbusSendAndReceive(handle: HidbusBusHandle, inbuf: *const c_void, inbuf_size: usize, outbuf: *mut c_void, outbuf_size: usize, out_size: *mut u64) -> Result;

    /// EnableJoyPollingReceiveMode
    ///
    /// # Arguments
    /// - `handle` (in): `HidbusBusHandle`
    /// - `inbuf` (in): Input buffer, containing the command data.
    /// - `inbuf_size` (in): Input buffer size, must be &lt;0x26.
    /// - `workbuf` (out): TransferMemory buffer, must be 0x1000-byte aligned. This buffer must not be written to until after `hidbusDisableJoyPollingReceiveMode` is used.
    /// - `workbuf_size` (in): TransferMemory buffer size, must be 0x1000-byte aligned.
    /// - `polling_mode` (in): `HidbusJoyPollingMode`
    pub fn hidbusEnableJoyPollingReceiveMode(handle: HidbusBusHandle, inbuf: *const c_void, inbuf_size: usize, workbuf: *mut c_void, workbuf_size: usize, polling_mode: HidbusJoyPollingMode) -> Result;

    /// DisableJoyPollingReceiveMode
    ///
    /// # Arguments
    /// - `handle` (in): `HidbusBusHandle`
    ///
    /// **Note:** This can also be used via `hidbusEnableExternalDevice` with flag=false.
    pub fn hidbusDisableJoyPollingReceiveMode(handle: HidbusBusHandle) -> Result;

    /// GetJoyPollingReceivedData
    ///
    /// # Arguments
    /// - `handle` (in): `HidbusBusHandle`
    /// - `recv_data` (out): Output array of `HidbusJoyPollingReceivedData`.
    /// - `count` (in): Total entries for the recv_data array. The maximum is 0xa. Official apps use range 0x1-0x9.
    pub fn hidbusGetJoyPollingReceivedData(handle: HidbusBusHandle, recv_data: *mut HidbusJoyPollingReceivedData, count: s32) -> Result;
}
