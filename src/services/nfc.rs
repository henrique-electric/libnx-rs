//! `switch/services/nfc.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// NfpServiceType
pub type NfpServiceType = u32;
/// Initializes nfp:user.
pub const NfpServiceType_User: NfpServiceType = 0;
/// Initializes nfp:dbg.
pub const NfpServiceType_Debug: NfpServiceType = 1;
/// Initializes nfp:sys.
pub const NfpServiceType_System: NfpServiceType = 2;

/// NfcServiceType
pub type NfcServiceType = u32;
/// Initializes nfc:user.
pub const NfcServiceType_User: NfcServiceType = 0;
/// Initializes nfc:sys.
pub const NfcServiceType_System: NfcServiceType = 1;

pub type NfcState = u32;
pub const NfcState_NonInitialized: NfcState = 0;
pub const NfcState_Initialized: NfcState = 1;

pub type NfpDeviceState = u32;
pub const NfpDeviceState_Initialized: NfpDeviceState = 0;
pub const NfpDeviceState_SearchingForTag: NfpDeviceState = 1;
pub const NfpDeviceState_TagFound: NfpDeviceState = 2;
pub const NfpDeviceState_TagRemoved: NfpDeviceState = 3;
pub const NfpDeviceState_TagMounted: NfpDeviceState = 4;
pub const NfpDeviceState_Unavailable: NfpDeviceState = 5;

pub type NfcDeviceState = u32;
pub const NfcDeviceState_Initialized: NfcDeviceState = 0;
pub const NfcDeviceState_SearchingForTag: NfcDeviceState = 1;
pub const NfcDeviceState_TagFound: NfcDeviceState = 2;
pub const NfcDeviceState_TagRemoved: NfcDeviceState = 3;
pub const NfcDeviceState_TagMounted: NfcDeviceState = 4;

pub type NfcMifareDeviceState = u32;
pub const NfcMifareDeviceState_Initialized: NfcMifareDeviceState = 0;
pub const NfcMifareDeviceState_SearchingForTag: NfcMifareDeviceState = 1;
pub const NfcMifareDeviceState_TagFound: NfcMifareDeviceState = 2;
pub const NfcMifareDeviceState_TagRemoved: NfcMifareDeviceState = 3;
pub const NfcMifareDeviceState_TagMounted: NfcMifareDeviceState = 4;
pub const NfcMifareDeviceState_Unavailable: NfcMifareDeviceState = 5;

pub type NfpApplicationAreaVersion = u32;
/// Application area created by a 3DS game.
pub const NfpApplicationAreaVersion_3DS: NfpApplicationAreaVersion = 0;
/// Application area created by a Wii U game.
pub const NfpApplicationAreaVersion_WiiU: NfpApplicationAreaVersion = 1;
/// Application area created by a (new?) 3DS game.
pub const NfpApplicationAreaVersion_3DSv2: NfpApplicationAreaVersion = 2;
/// Application area created by a Switch game.
pub const NfpApplicationAreaVersion_Switch: NfpApplicationAreaVersion = 3;
/// Invalid value (application area not created).
pub const NfpApplicationAreaVersion_Invalid: NfpApplicationAreaVersion = 0xFF;

pub type NfpDeviceType = u32;
pub const NfpDeviceType_Amiibo: NfpDeviceType = 0;

pub type NfpMountTarget = u32;
pub const NfpMountTarget_Rom: NfpMountTarget = BIT(0);
pub const NfpMountTarget_Ram: NfpMountTarget = BIT(1);
pub const NfpMountTarget_All: NfpMountTarget = NfpMountTarget_Rom | NfpMountTarget_Ram;

pub type NfcProtocol = u32;
pub const NfcProtocol_None: NfcProtocol = 0;
/// ISO14443A
pub const NfcProtocol_TypeA: NfcProtocol = BIT(0);
/// ISO14443B
pub const NfcProtocol_TypeB: NfcProtocol = BIT(1);
/// Sony FeliCa
pub const NfcProtocol_TypeF: NfcProtocol = BIT(2);
pub const NfcProtocol_All: NfcProtocol = 0xFFFFFFFF;

pub type NfcTagType = u32;
pub const NfcTagType_None: NfcTagType = 0;
/// ISO14443A RW. Topaz
pub const NfcTagType_Type1: NfcTagType = BIT(0);
/// ISO14443A RW. Ultralight, NTAGX, ST25TN
pub const NfcTagType_Type2: NfcTagType = BIT(1);
/// ISO14443A RW/RO. Sony FeliCa
pub const NfcTagType_Type3: NfcTagType = BIT(2);
/// ISO14443A RW/RO. DESFire
pub const NfcTagType_Type4A: NfcTagType = BIT(3);
/// ISO14443B RW/RO. DESFire
pub const NfcTagType_Type4B: NfcTagType = BIT(4);
/// ISO15693 RW/RO. SLI, SLIX, ST25TV
pub const NfcTagType_Type5: NfcTagType = BIT(5);
/// Mifare clasic. Skylanders
pub const NfcTagType_Mifare: NfcTagType = BIT(6);
pub const NfcTagType_All: NfcTagType = 0xFFFFFFFF;

pub type NfcMifareCommand = u32;
pub const NfcMifareCommand_Read: NfcMifareCommand = 0x30;
pub const NfcMifareCommand_AuthA: NfcMifareCommand = 0x60;
pub const NfcMifareCommand_AuthB: NfcMifareCommand = 0x61;
pub const NfcMifareCommand_Write: NfcMifareCommand = 0xA0;
pub const NfcMifareCommand_Transfer: NfcMifareCommand = 0xB0;
pub const NfcMifareCommand_Decrement: NfcMifareCommand = 0xC0;
pub const NfcMifareCommand_Increment: NfcMifareCommand = 0xC1;
pub const NfcMifareCommand_Store: NfcMifareCommand = 0xC2;

pub type NfpAmiiboFlag = u32;
/// Initialized in system settings.
pub const NfpAmiiboFlag_Valid: NfpAmiiboFlag = BIT(0);
/// Application area exists.
pub const NfpAmiiboFlag_ApplicationAreaExists: NfpAmiiboFlag = BIT(1);

pub type NfpBreakType = u32;
pub const NfpBreakType_Flush: NfpBreakType = 0;
pub const NfpBreakType_Break1: NfpBreakType = 1;
pub const NfpBreakType_Break2: NfpBreakType = 2;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NfpDate {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NfcTagUid {
    /// UUID.
    pub uid: [u8; 10],
    /// UUID length.
    pub uid_length: u8,
    pub reserved: [u8; 21],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NfpTagInfo {
    /// UUID.
    pub uid: NfcTagUid,
    /// **Ref:** NfcProtocol
    pub protocol: u32,
    /// **Ref:** NfcTagType
    pub tag_type: u32,
    pub reserved: [u8; 48],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NfcTagInfo {
    /// UUID.
    pub uid: NfcTagUid,
    /// **Ref:** NfcProtocol
    pub protocol: u32,
    /// **Ref:** NfcTagType
    pub tag_type: u32,
    pub reserved: [u8; 48],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NfpCommonInfo {
    pub last_write_date: NfpDate,
    pub write_counter: u16,
    pub version: u16,
    pub application_area_size: u32,
    pub reserved: [u8; 52],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NfpModelInfo_anon0_anon0 {
    pub game_character_id: u16,
    pub character_variant: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NfpModelInfo_anon0 {
    pub character_id: [u8; 3],
    pub anon0: NfpModelInfo_anon0_anon0,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NfpModelInfo {
    pub anon0: NfpModelInfo_anon0,
    /// Series.
    pub series_id: u8,
    /// Model number.
    pub numbering_id: u16,
    /// Figure type.
    pub nfp_type: u8,
    pub reserved: [u8; 57],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NfpRegisterInfo {
    pub mii: MiiCharInfo,
    pub first_write_date: NfpDate,
    /// Amiibo name (utf-8, null-terminated).
    pub amiibo_name: [c_char; 41],
    pub font_region: u8,
    pub reserved: [u8; 122],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NfpRegisterInfoPrivate {
    pub mii_store_data: MiiStoreData,
    pub first_write_date: NfpDate,
    /// Amiibo name (utf-8, null-terminated).
    pub amiibo_name: [c_char; 41],
    pub font_region: u8,
    pub reserved: [u8; 142],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NfpAdminInfo {
    pub application_id: u64,
    pub access_id: u32,
    pub crc32_change_counter: u16,
    pub flags: u8,
    pub tag_type: u8,
    pub application_area_version: u8,
    pub reserved: [u8; 47],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NfpData {
    /// Tag magic (always 0xA5: <https://www.3dbrew.org/wiki/Amiibo#Page_layout>).
    pub tag_magic: u8,
    pub reserved1: [u8; 1],
    /// Incremented every tag write.
    pub tag_write_counter: u16,
    /// CRC32 of some internal 8-byte data.
    pub crc32_1: u32,
    pub reserved2: [u8; 56],
    /// Updated every write.
    pub last_write_date: NfpDate,
    /// Incremented every write, until it maxes out at 0xFFFF.
    pub write_counter: u16,
    /// Version.
    pub version: u16,
    /// Size of the application area.
    pub application_area_size: u32,
    pub reserved3: [u8; 52],
    /// Ver3StoreData (Mii format used in 3DS).
    pub mii_v3: MiiVer3StoreData,
    pub pad: [u8; 2],
    /// CRC16 of Ver3StoreData.
    pub mii_v3_crc16: u16,
    /// StoreDataExtension
    pub mii_store_data_extension: MiiNfpStoreDataExtension,
    /// Set when the amiibo is first written to.
    pub first_write_date: NfpDate,
    /// Amiibo name (utf-16, null-terminated).
    pub amiibo_name: [u16; 11],
    /// Font region.
    pub font_region: u8,
    /// Normally zero
    pub unknown1: u8,
    /// CRC32 of Ver3StoreData + application_id_byte + unknown1 + StoreDataExtension + unknown2 (0x7E bytes total)
    pub crc32_2: u32,
    /// Normally zero
    pub unknown2: [u32; 5],
    pub reserved4: [u8; 100],
    /// Modified application ID (Application ID & 0xFFFFFFFF0FFFFFFF | 0x30000000)
    pub application_id: u64,
    /// Application area access ID
    pub access_id: u32,
    pub settings_crc32_change_counter: u16,
    /// **Ref:** NfpAmiiboFlag
    pub flags: u8,
    /// **Ref:** NfcTagType
    pub tag_type: u8,
    /// **Ref:** NfpApplicationAreaVersion
    pub application_area_version: u8,
    /// Application ID byte ((Application ID >> 28) & 0xFF)
    pub application_id_byte: u8,
    pub reserved5: [u8; 46],
    /// Application area.
    pub application_area: [u8; 216],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NfcSectorKey {
    pub mifare_command: u8,
    /// Usually 1
    pub unknown: u8,
    pub reserved1: [u8; 6],
    pub sector_key: [u8; 6],
    pub reserved2: [u8; 2],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NfcMifareReadBlockParameter {
    pub sector_number: u8,
    pub reserved: [u8; 7],
    pub sector_key: NfcSectorKey,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NfcMifareReadBlockData {
    pub data: [u8; 16],
    pub sector_number: u8,
    pub reserved: [u8; 7],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NfcMifareWriteBlockParameter {
    pub data: [u8; 16],
    pub sector_number: u8,
    pub reserved: [u8; 7],
    pub sector_key: NfcSectorKey,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NfcRequiredMcuVersionData {
    pub version: u64,
    pub reserved: [u64; 3],
}

/// Nfc/Nfp DeviceHandle
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NfcDeviceHandle {
    /// Handle.
    pub handle: [u8; 8],
}

impl_zeroed_default!(NfpDate, NfcTagUid, NfpTagInfo, NfcTagInfo, NfpCommonInfo, NfpModelInfo_anon0_anon0, NfpModelInfo_anon0, NfpModelInfo, NfpRegisterInfo, NfpRegisterInfoPrivate, NfpAdminInfo, NfpData, NfcSectorKey, NfcMifareReadBlockParameter, NfcMifareReadBlockData, NfcMifareWriteBlockParameter, NfcRequiredMcuVersionData, NfcDeviceHandle);

unsafe extern "C" {
    /// Initialize nfp:*.
    pub fn nfpInitialize(service_type: NfpServiceType) -> Result;

    /// Exit nfp:*.
    pub fn nfpExit();

    /// Initialize nfc:*.
    pub fn nfcInitialize(service_type: NfcServiceType) -> Result;

    /// Exit nfc:*.
    pub fn nfcExit();

    /// Initialize nfc:mf:u.
    pub fn nfcMfInitialize() -> Result;

    /// Exit nfc:mf:u.
    pub fn nfcMfExit();

    /// Gets the Service object for the actual nfp:* service session.
    pub fn nfpGetServiceSession() -> *mut Service;

    /// Gets the Service object for the interface from nfp:*.
    pub fn nfpGetServiceSession_Interface() -> *mut Service;

    /// Gets the Service object for the actual nfc:* service session.
    pub fn nfcGetServiceSession() -> *mut Service;

    /// Gets the Service object for the interface from nfc:*.
    pub fn nfcGetServiceSession_Interface() -> *mut Service;

    /// Gets the Service object for the actual nfc:mf:u service session.
    pub fn nfcMfGetServiceSession() -> *mut Service;

    /// Gets the Service object for the interface from nfc:mf:u.
    pub fn nfcMfGetServiceSession_Interface() -> *mut Service;

    pub fn nfpListDevices(total_out: *mut s32, out: *mut NfcDeviceHandle, count: s32) -> Result;

    pub fn nfpStartDetection(handle: *const NfcDeviceHandle) -> Result;

    pub fn nfpStopDetection(handle: *const NfcDeviceHandle) -> Result;

    pub fn nfpMount(handle: *const NfcDeviceHandle, device_type: NfpDeviceType, mount_target: NfpMountTarget) -> Result;

    pub fn nfpUnmount(handle: *const NfcDeviceHandle) -> Result;

    /// Only available with [4.0.0+].
    pub fn nfcListDevices(total_out: *mut s32, out: *mut NfcDeviceHandle, count: s32) -> Result;

    /// Only available with [4.0.0+].
    pub fn nfcStartDetection(handle: *const NfcDeviceHandle, protocol: NfcProtocol) -> Result;

    /// Only available with [4.0.0+].
    pub fn nfcStopDetection(handle: *const NfcDeviceHandle) -> Result;

    pub fn nfcMfListDevices(total_out: *mut s32, out: *mut NfcDeviceHandle, count: s32) -> Result;

    pub fn nfcMfStartDetection(handle: *const NfcDeviceHandle) -> Result;

    pub fn nfcMfStopDetection(handle: *const NfcDeviceHandle) -> Result;

    /// Not available with ::NfpServiceType_System.
    /// Requires the amiibo to be mounted with ::NfpMountTarget_Ram.
    pub fn nfpOpenApplicationArea(handle: *const NfcDeviceHandle, app_id: u32) -> Result;

    /// Not available with ::NfpServiceType_System.
    /// Requires the amiibo to be mounted with ::NfpMountTarget_Ram, and the application area to be opened.
    pub fn nfpGetApplicationArea(handle: *const NfcDeviceHandle, buf: *mut c_void, buf_size: usize, out_size: *mut u32) -> Result;

    /// Not available with ::NfpServiceType_System.
    /// Requires the amiibo to be mounted with ::NfpMountTarget_Ram, and the application area to be opened.
    pub fn nfpSetApplicationArea(handle: *const NfcDeviceHandle, buf: *const c_void, buf_size: usize) -> Result;

    /// Requires the amiibo to be mounted with ::NfpMountTarget_Ram.
    pub fn nfpFlush(handle: *const NfcDeviceHandle) -> Result;

    pub fn nfpRestore(handle: *const NfcDeviceHandle) -> Result;

    /// Not available with ::NfpServiceType_System.
    /// Requires the amiibo to be mounted with ::NfpMountTarget_Ram.
    pub fn nfpCreateApplicationArea(handle: *const NfcDeviceHandle, app_id: u32, buf: *const c_void, buf_size: usize) -> Result;

    /// Not available with ::NfpServiceType_System.
    /// Only available with [3.0.0+].
    /// Requires the amiibo to be mounted with ::NfpMountTarget_Ram, and the application area to be opened.
    pub fn nfpRecreateApplicationArea(handle: *const NfcDeviceHandle, app_id: u32, buf: *const c_void, buf_size: usize) -> Result;

    /// Not available with ::NfpServiceType_System.
    pub fn nfpGetApplicationAreaSize(handle: *const NfcDeviceHandle, out_app_area_size: *mut u32) -> Result;

    /// Not available with ::NfpServiceType_User.
    pub fn nfpDeleteApplicationArea(handle: *const NfcDeviceHandle) -> Result;

    /// Not available with ::NfpServiceType_User.
    pub fn nfpExistsApplicationArea(handle: *const NfcDeviceHandle, out: *mut bool) -> Result;

    pub fn nfpGetTagInfo(handle: *const NfcDeviceHandle, out: *mut NfpTagInfo) -> Result;

    /// Requires the amiibo to be mounted with ::NfpMountTarget_Ram.
    pub fn nfpGetRegisterInfo(handle: *const NfcDeviceHandle, out: *mut NfpRegisterInfo) -> Result;

    /// Requires the amiibo to be mounted with ::NfpMountTarget_Ram.
    pub fn nfpGetCommonInfo(handle: *const NfcDeviceHandle, out: *mut NfpCommonInfo) -> Result;

    /// Requires the amiibo to be mounted with ::NfpMountTarget_Rom.
    pub fn nfpGetModelInfo(handle: *const NfcDeviceHandle, out: *mut NfpModelInfo) -> Result;

    /// Not available with ::NfpServiceType_User.
    /// Requires the amiibo to be mounted with ::NfpMountTarget_Ram.
    pub fn nfpGetAdminInfo(handle: *const NfcDeviceHandle, out: *mut NfpAdminInfo) -> Result;

    /// Only available with [4.0.0+].
    pub fn nfcGetTagInfo(handle: *const NfcDeviceHandle, out: *mut NfcTagInfo) -> Result;

    pub fn nfcMfGetTagInfo(handle: *const NfcDeviceHandle, out: *mut NfcTagInfo) -> Result;

    /// Returned event will have autoclear off.
    pub fn nfpAttachActivateEvent(handle: *const NfcDeviceHandle, out_event: *mut Event) -> Result;

    /// Returned event will have autoclear off.
    pub fn nfpAttachDeactivateEvent(handle: *const NfcDeviceHandle, out_event: *mut Event) -> Result;

    /// Returned event will have autoclear off.
    /// Only available with [4.0.0+].
    pub fn nfcAttachActivateEvent(handle: *const NfcDeviceHandle, out_event: *mut Event) -> Result;

    /// Returned event will have autoclear off.
    /// Only available with [4.0.0+].
    pub fn nfcAttachDeactivateEvent(handle: *const NfcDeviceHandle, out_event: *mut Event) -> Result;

    /// Returned event will have autoclear off.
    pub fn nfcMfAttachActivateEvent(handle: *const NfcDeviceHandle, out_event: *mut Event) -> Result;

    /// Returned event will have autoclear off.
    pub fn nfcMfAttachDeactivateEvent(handle: *const NfcDeviceHandle, out_event: *mut Event) -> Result;

    pub fn nfpGetState(out: *mut NfcState) -> Result;

    pub fn nfpGetDeviceState(handle: *const NfcDeviceHandle, out: *mut NfpDeviceState) -> Result;

    pub fn nfpGetNpadId(handle: *const NfcDeviceHandle, out: *mut u32) -> Result;

    /// Only available with [4.0.0+].
    pub fn nfcGetState(out: *mut NfcState) -> Result;

    /// Only available with [4.0.0+].
    pub fn nfcGetDeviceState(handle: *const NfcDeviceHandle, out: *mut NfcDeviceState) -> Result;

    /// Only available with [4.0.0+].
    pub fn nfcGetNpadId(handle: *const NfcDeviceHandle, out: *mut u32) -> Result;

    pub fn nfcMfGetState(out: *mut NfcState) -> Result;

    pub fn nfcMfGetDeviceState(handle: *const NfcDeviceHandle, out: *mut NfcMifareDeviceState) -> Result;

    pub fn nfcMfGetNpadId(handle: *const NfcDeviceHandle, out: *mut u32) -> Result;

    /// Returned event will have autoclear on.
    /// Only available with [3.0.0+].
    pub fn nfpAttachAvailabilityChangeEvent(out_event: *mut Event) -> Result;

    /// Returned event will have autoclear on.
    /// Only available with [4.0.0+].
    pub fn nfcAttachAvailabilityChangeEvent(out_event: *mut Event) -> Result;

    /// Returned event will have autoclear on.
    pub fn nfcMfAttachAvailabilityChangeEvent(out_event: *mut Event) -> Result;

    /// Not available with ::NfpServiceType_User.
    pub fn nfpFormat(handle: *const NfcDeviceHandle) -> Result;

    /// Not available with ::NfpServiceType_User.
    /// Requires the amiibo to be mounted with ::NfpMountTarget_Ram.
    pub fn nfpGetRegisterInfoPrivate(handle: *const NfcDeviceHandle, out: *mut NfpRegisterInfoPrivate) -> Result;

    /// Not available with ::NfpServiceType_User.
    /// Requires the amiibo to be mounted with ::NfpMountTarget_Ram.
    pub fn nfpSetRegisterInfoPrivate(handle: *const NfcDeviceHandle, register_info_private: *const NfpRegisterInfoPrivate) -> Result;

    /// Not available with ::NfpServiceType_User.
    /// Requires the amiibo to be mounted with ::NfpMountTarget_Ram.
    pub fn nfpDeleteRegisterInfo(handle: *const NfcDeviceHandle) -> Result;

    /// Only available with ::NfpServiceType_Debug.
    /// Requires the amiibo to be mounted with ::NfpMountTarget_Ram.
    pub fn nfpGetAll(handle: *const NfcDeviceHandle, out: *mut NfpData) -> Result;

    /// Only available with ::NfpServiceType_Debug.
    /// Requires the amiibo to be mounted with ::NfpMountTarget_Ram.
    pub fn nfpSetAll(handle: *const NfcDeviceHandle, nfp_data: *const NfpData) -> Result;

    /// Only available with ::NfpServiceType_Debug.
    /// Requires the amiibo to be mounted with ::NfpMountTarget_Ram.
    pub fn nfpFlushDebug(handle: *const NfcDeviceHandle) -> Result;

    /// Only available with ::NfpServiceType_Debug.
    /// Requires the amiibo to be mounted with ::NfpMountTarget_Ram.
    pub fn nfpBreakTag(handle: *const NfcDeviceHandle, break_type: NfpBreakType) -> Result;

    /// Only available with ::NfpServiceType_Debug.
    pub fn nfpReadBackupData(handle: *const NfcDeviceHandle, out_buf: *mut c_void, buf_size: usize, out_size: *mut u32) -> Result;

    /// Only available with ::NfpServiceType_Debug.
    pub fn nfpWriteBackupData(handle: *const NfcDeviceHandle, buf: *const c_void, buf_size: usize) -> Result;

    /// Only available with ::NfpServiceType_Debug.
    pub fn nfpWriteNtf(handle: *const NfcDeviceHandle, write_type: u32, buf: *const c_void, buf_size: usize) -> Result;

    /// This uses nfc:*.
    pub fn nfcIsNfcEnabled(out: *mut bool) -> Result;

    /// Only available with [4.0.0+].
    pub fn nfcReadMifare(handle: *const NfcDeviceHandle, out_block_data: *mut NfcMifareReadBlockData, read_block_parameter: *const NfcMifareReadBlockParameter, count: s32) -> Result;

    /// Only available with [4.0.0+].
    pub fn nfcWriteMifare(handle: *const NfcDeviceHandle, write_block_parameter: *const NfcMifareWriteBlockParameter, count: s32) -> Result;

    pub fn nfcMfReadMifare(handle: *const NfcDeviceHandle, out_block_data: *mut NfcMifareReadBlockData, read_block_parameter: *const NfcMifareReadBlockParameter, count: s32) -> Result;

    pub fn nfcMfWriteMifare(handle: *const NfcDeviceHandle, write_block_parameter: *const NfcMifareWriteBlockParameter, count: s32) -> Result;

    /// Only available with [4.0.0+].
    pub fn nfcSendCommandByPassThrough(handle: *const NfcDeviceHandle, timeout: u64, cmd_buf: *const c_void, cmd_buf_size: usize, reply_buf: *mut c_void, reply_buf_size: usize, out_size: *mut u64) -> Result;

    /// Only available with [4.0.0+].
    pub fn nfcKeepPassThroughSession(handle: *const NfcDeviceHandle) -> Result;

    /// Only available with [4.0.0+].
    pub fn nfcReleasePassThroughSession(handle: *const NfcDeviceHandle) -> Result;
}
