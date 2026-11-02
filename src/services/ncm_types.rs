//! `switch/services/ncm_types.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// StorageId
pub type NcmStorageId = u32;
/// None
pub const NcmStorageId_None: NcmStorageId = 0;
/// Host
pub const NcmStorageId_Host: NcmStorageId = 1;
/// GameCard
pub const NcmStorageId_GameCard: NcmStorageId = 2;
/// BuiltInSystem
pub const NcmStorageId_BuiltInSystem: NcmStorageId = 3;
/// BuiltInUser
pub const NcmStorageId_BuiltInUser: NcmStorageId = 4;
/// SdCard
pub const NcmStorageId_SdCard: NcmStorageId = 5;
/// Any
pub const NcmStorageId_Any: NcmStorageId = 6;

/// ContentType
pub type NcmContentType = u32;
/// Meta
pub const NcmContentType_Meta: NcmContentType = 0;
/// Program
pub const NcmContentType_Program: NcmContentType = 1;
/// Data
pub const NcmContentType_Data: NcmContentType = 2;
/// Control
pub const NcmContentType_Control: NcmContentType = 3;
/// HtmlDocument
pub const NcmContentType_HtmlDocument: NcmContentType = 4;
/// LegalInformation
pub const NcmContentType_LegalInformation: NcmContentType = 5;
/// DeltaFragment
pub const NcmContentType_DeltaFragment: NcmContentType = 6;

/// ContentMetaType
pub type NcmContentMetaType = u32;
/// Unknown
pub const NcmContentMetaType_Unknown: NcmContentMetaType = 0x0;
/// SystemProgram
pub const NcmContentMetaType_SystemProgram: NcmContentMetaType = 0x1;
/// SystemData
pub const NcmContentMetaType_SystemData: NcmContentMetaType = 0x2;
/// SystemUpdate
pub const NcmContentMetaType_SystemUpdate: NcmContentMetaType = 0x3;
/// BootImagePackage
pub const NcmContentMetaType_BootImagePackage: NcmContentMetaType = 0x4;
/// BootImagePackageSafe
pub const NcmContentMetaType_BootImagePackageSafe: NcmContentMetaType = 0x5;
/// Application
pub const NcmContentMetaType_Application: NcmContentMetaType = 0x80;
/// Patch
pub const NcmContentMetaType_Patch: NcmContentMetaType = 0x81;
/// AddOnContent
pub const NcmContentMetaType_AddOnContent: NcmContentMetaType = 0x82;
/// Delta
pub const NcmContentMetaType_Delta: NcmContentMetaType = 0x83;
/// DataPatch
pub const NcmContentMetaType_DataPatch: NcmContentMetaType = 0x84;

/// ContentMetaAttribute
pub type NcmContentMetaAttribute = u32;
/// None
pub const NcmContentMetaAttribute_None: NcmContentMetaAttribute = 0;
/// IncludesExFatDriver
pub const NcmContentMetaAttribute_IncludesExFatDriver: NcmContentMetaAttribute = BIT(0);
/// Rebootless
pub const NcmContentMetaAttribute_Rebootless: NcmContentMetaAttribute = BIT(1);
/// Compacted
pub const NcmContentMetaAttribute_Compacted: NcmContentMetaAttribute = BIT(2);

/// ContentInstallType
pub type NcmContentInstallType = u32;
/// Full
pub const NcmContentInstallType_Full: NcmContentInstallType = 0;
/// FragmentOnly
pub const NcmContentInstallType_FragmentOnly: NcmContentInstallType = 1;
/// Unknown
pub const NcmContentInstallType_Unknown: NcmContentInstallType = 7;

/// ContentMetaPlatform
pub type NcmContentMetaPlatform = u32;
/// Nx
pub const NcmContentMetaPlatform_Nx: NcmContentMetaPlatform = 0;

/// ContentId
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NcmContentId {
    /// Id
    pub c: [u8; 16],
}

/// PlaceHolderId
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NcmPlaceHolderId {
    /// UUID
    pub uuid: Uuid,
}

/// ContentMetaKey
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NcmContentMetaKey {
    /// Id.
    pub id: u64,
    /// Version.
    pub version: u32,
    /// **Ref:** NcmContentMetaType
    pub type_: u8,
    /// **Ref:** NcmContentInstallType
    pub install_type: u8,
    /// Padding.
    pub padding: [u8; 2],
}

/// ApplicationContentMetaKey
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NcmApplicationContentMetaKey {
    /// **Ref:** NcmContentMetaKey
    pub key: NcmContentMetaKey,
    /// ApplicationId.
    pub application_id: u64,
}

/// ContentInfo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NcmContentInfo {
    /// **Ref:** NcmContentId
    pub content_id: NcmContentId,
    /// Content size (low).
    pub size_low: u32,
    /// Content size (high).
    pub size_high: u8,
    /// Content attributes.
    pub attr: u8,
    /// **Ref:** NcmContentType.
    pub content_type: u8,
    /// Offset of this content. Unused by most applications.
    pub id_offset: u8,
}

/// PackagedContentInfo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NcmPackagedContentInfo {
    pub hash: [u8; 32],
    pub info: NcmContentInfo,
}

/// ContentMetaInfo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NcmContentMetaInfo {
    /// Id.
    pub id: u64,
    /// Version.
    pub version: u32,
    /// **Ref:** NcmContentMetaType
    pub type_: u8,
    /// **Ref:** NcmContentMetaAttribute
    pub attr: u8,
    /// Padding.
    pub padding: [u8; 2],
}

/// ContentMetaHeader
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NcmContentMetaHeader {
    /// Size of optional struct that comes after this one.
    pub extended_header_size: u16,
    /// Number of NcmContentInfos after the extra bytes.
    pub content_count: u16,
    /// Number of NcmContentMetaInfos that come after the NcmContentInfos.
    pub content_meta_count: u16,
    /// Usually None (0).
    pub attributes: u8,
    /// Usually None (0).
    pub storage_id: u8,
}

/// ApplicationMetaExtendedHeader
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NcmApplicationMetaExtendedHeader {
    /// PatchId of this application's patch.
    pub patch_id: u64,
    /// Firmware version required by this application.
    pub required_system_version: u32,
    /// [9.0.0+] Owner application version required by this application. Previously padding.
    pub required_application_version: u32,
}

/// PatchMetaExtendedHeader
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NcmPatchMetaExtendedHeader {
    /// ApplicationId of this patch's corresponding application.
    pub application_id: u64,
    /// Firmware version required by this patch.
    pub required_system_version: u32,
    /// Size of the extended data following the NcmContentInfos.
    pub extended_data_size: u32,
    /// Unused.
    pub reserved: [u8; 8],
}

/// AddOnContentMetaExtendedHeader [15.0.0+]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NcmAddOnContentMetaExtendedHeader {
    /// ApplicationId of this add-on-content's corresponding application.
    pub application_id: u64,
    /// Version of the application required by this add-on-content.
    pub required_application_version: u32,
    /// Content accessibilities.
    pub content_accessibilities: u8,
    /// Padding.
    pub padding: [u8; 3],
    /// DataPatchId of this add-on-content's corresponding data patch.
    pub data_patch_id: u64,
}

/// LegacyAddOnContentMetaExtendedHeader [1.0.0-14.1.2]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NcmLegacyAddOnContentMetaExtendedHeader {
    /// ApplicationId of this add-on-content's corresponding application.
    pub application_id: u64,
    /// Version of the application required by this add-on-content.
    pub required_application_version: u32,
    /// Padding.
    pub padding: u32,
}

/// DataPatchMetaExtendedHeader
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NcmDataPatchMetaExtendedHeader {
    /// DataId of this data patch's corresponding add-on-content.
    pub data_id: u64,
    /// ApplicationId of this data patch's add-on-content's corresponding application.
    pub application_id: u64,
    /// Version of the application required by this data patch.
    pub required_application_version: u32,
    /// Size of the extended data following the NcmContentInfos.
    pub extended_data_size: u32,
    /// Padding.
    pub padding: u64,
}

/// SystemUpdateMetaExtendedHeader
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NcmSystemUpdateMetaExtendedHeader {
    /// Size of the extended data after NcmContentInfos and NcmContentMetaInfos.
    pub extended_data_size: u32,
}

/// ProgramLocation
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NcmProgramLocation {
    /// ProgramId
    pub program_id: u64,
    /// **Ref:** NcmStorageId
    pub storageID: u8,
    /// Padding
    pub pad: [u8; 7],
}

/// Retrieves the content size from a `NcmContentInfo` struct.
///
/// # Arguments
/// - `info` (in): Pointer to `NcmContentInfo` struct.
/// - `out` (out): Output size.
#[inline(always)]
pub fn ncmContentInfoSizeToU64(info: &NcmContentInfo, out: &mut u64) {
    *out = ((info.size_high as u64) << 32) | info.size_low as u64;
}

/// Updates the content size from a `NcmContentInfo` struct.
///
/// # Arguments
/// - `size` (in): Input size.
/// - `out` (out): Pointer to `NcmContentInfo` struct.
#[inline(always)]
pub fn ncmU64ToContentInfoSize(size: u64, info: &mut NcmContentInfo) {
    info.size_low = (size & 0xFFFFFFFF) as _;
    info.size_high = (size >> 32) as u8;
}

impl_zeroed_default!(NcmContentId, NcmPlaceHolderId, NcmContentMetaKey, NcmApplicationContentMetaKey, NcmContentInfo, NcmPackagedContentInfo, NcmContentMetaInfo, NcmContentMetaHeader, NcmApplicationMetaExtendedHeader, NcmPatchMetaExtendedHeader, NcmAddOnContentMetaExtendedHeader, NcmLegacyAddOnContentMetaExtendedHeader, NcmDataPatchMetaExtendedHeader, NcmSystemUpdateMetaExtendedHeader, NcmProgramLocation);
