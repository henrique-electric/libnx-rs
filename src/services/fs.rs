//! `switch/services/fs.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const FS_MAX_PATH: u32 = 0x301;
/// For use with `FsSaveDataAttribute`.
pub const FS_SAVEDATA_CURRENT_APPLICATIONID: u32 = 0;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FsRightsId {
    pub c: [u8; 16],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FsFileSystem {
    pub s: Service,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FsFile {
    pub s: Service,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FsDir {
    pub s: Service,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FsStorage {
    pub s: Service,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FsSaveDataInfoReader {
    pub s: Service,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FsEventNotifier {
    pub s: Service,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FsDeviceOperator {
    pub s: Service,
}

/// Directory entry.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FsDirectoryEntry {
    /// Entry name.
    pub name: [c_char; 769],
    pub pad: [u8; 3],
    /// See FsDirEntryType.
    pub type_: s8,
    /// ?
    pub pad2: [u8; 3],
    /// File size.
    pub file_size: s64,
}

/// SaveDataAttribute
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FsSaveDataAttribute {
    /// ApplicationId of the savedata to access when accessing other programs' savedata via SaveData, otherwise FS_SAVEDATA_CURRENT_APPLICATIONID.
    pub application_id: u64,
    /// **Ref:** AccountUid for the user-specific savedata to access, otherwise 0 for common savedata.
    pub uid: AccountUid,
    /// SystemSaveDataId, 0 for ::FsSaveDataType_Account.
    pub system_save_data_id: u64,
    /// **Ref:** FsSaveDataType
    pub save_data_type: u8,
    /// **Ref:** FsSaveDataRank
    pub save_data_rank: u8,
    /// SaveDataIndex
    pub save_data_index: u16,
    /// Padding.
    pub pad_x24: u32,
    /// 0 for ::FsSaveDataType_System/::FsSaveDataType_Account.
    pub unk_x28: u64,
    /// 0 for ::FsSaveDataType_System/::FsSaveDataType_Account.
    pub unk_x30: u64,
    /// 0 for ::FsSaveDataType_System/::FsSaveDataType_Account.
    pub unk_x38: u64,
}

/// SaveDataExtraData
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FsSaveDataExtraData {
    /// **Ref:** FsSaveDataAttribute
    pub attr: FsSaveDataAttribute,
    /// ProgramId of the owner of this save data. 0 for ::FsSaveDataType_System.
    pub owner_id: u64,
    /// POSIX timestamp.
    pub timestamp: u64,
    /// **Ref:** FsSaveDataFlags
    pub flags: u32,
    /// Normally 0. Possibly unused?
    pub unk_x54: u32,
    /// Usable save data size.
    pub data_size: s64,
    /// Journal size of the save data.
    pub journal_size: s64,
    /// Id of the latest commit.
    pub commit_id: u64,
    /// Uninitialized.
    pub unused: [u8; 400],
}

/// SaveDataMetaInfo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FsSaveDataMetaInfo {
    pub size: u32,
    /// **Ref:** FsSaveDataMetaType
    pub type_: u8,
    pub reserved: [u8; 11],
}

/// SaveDataCreationInfo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FsSaveDataCreationInfo {
    /// Size of the save data.
    pub save_data_size: s64,
    /// Journal size of the save data.
    pub journal_size: s64,
    /// AvailableSize
    pub available_size: u64,
    /// ProgramId of the owner of this save data. 0 for ::FsSaveDataType_System.
    pub owner_id: u64,
    /// **Ref:** FsSaveDataFlags
    pub flags: u32,
    /// **Ref:** FsSaveDataSpaceId
    pub save_data_space_id: u8,
    /// 0 for ::FsSaveDataType_System.
    pub unk: u8,
    /// Uninitialized for ::FsSaveDataType_System.
    pub padding: [u8; 26],
}

/// SaveDataInfo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FsSaveDataInfo {
    /// SaveDataId
    pub save_data_id: u64,
    /// **Ref:** FsSaveDataSpaceId
    pub save_data_space_id: u8,
    /// **Ref:** FsSaveDataType
    pub save_data_type: u8,
    /// Padding.
    pub pad: [u8; 6],
    /// FsSave::userID
    pub uid: AccountUid,
    /// FsSaveDataAttribute::system_save_data_id
    pub system_save_data_id: u64,
    /// ApplicationId for ::FsSaveDataType_Account.
    pub application_id: u64,
    /// Raw saveimage size.
    pub size: u64,
    /// SaveDataIndex
    pub save_data_index: u16,
    /// **Ref:** FsSaveDataRank
    pub save_data_rank: u8,
    /// Unknown. Usually zeros?
    pub unk_x3b: [u8; 37],
}

/// SaveDataFilter
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FsSaveDataFilter {
    /// Filter by `FsSaveDataAttribute`::application_id
    pub filter_by_application_id: bool,
    /// Filter by `FsSaveDataAttribute`::save_data_type
    pub filter_by_save_data_type: bool,
    /// Filter by `FsSaveDataAttribute`::uid
    pub filter_by_user_id: bool,
    /// Filter by `FsSaveDataAttribute`::system_save_data_id
    pub filter_by_system_save_data_id: bool,
    /// Filter by `FsSaveDataAttribute`::save_data_index
    pub filter_by_index: bool,
    /// **Ref:** FsSaveDataRank
    pub save_data_rank: u8,
    /// Padding
    pub padding: [u8; 2],
    /// **Ref:** FsSaveDataAttribute
    pub attr: FsSaveDataAttribute,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FsTimeStampRaw {
    /// POSIX timestamp.
    pub created: u64,
    /// POSIX timestamp.
    pub modified: u64,
    /// POSIX timestamp.
    pub accessed: u64,
    /// 0x1 when the timestamps are set.
    pub is_valid: u8,
    pub padding: [u8; 7],
}

/// This is nn::fssystem::ArchiveMacKey. Used by `setsysGetThemeKey` and `setsysSetThemeKey`. Does not appear to be in use elsewhere.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FsArchiveMacKey {
    pub key: [u8; 16],
}

/// Returned by fsFsGetEntryType.
pub type FsDirEntryType = u32;
/// Entry is a directory.
pub const FsDirEntryType_Dir: FsDirEntryType = 0;
/// Entry is a file.
pub const FsDirEntryType_File: FsDirEntryType = 1;

/// For use with fsFsOpenFile.
pub type FsOpenMode = u32;
/// Open for reading.
pub const FsOpenMode_Read: FsOpenMode = BIT(0);
/// Open for writing.
pub const FsOpenMode_Write: FsOpenMode = BIT(1);
/// Append file.
pub const FsOpenMode_Append: FsOpenMode = BIT(2);

/// For use with fsFsCreateFile.
pub type FsCreateOption = u32;
/// Creates a ConcatenationFile (dir with archive bit) instead of file.
pub const FsCreateOption_BigFile: FsCreateOption = BIT(0);

/// For use with fsFsOpenDirectory.
pub type FsDirOpenMode = u32;
/// Enable reading directory entries.
pub const FsDirOpenMode_ReadDirs: FsDirOpenMode = BIT(0);
/// Enable reading file entries.
pub const FsDirOpenMode_ReadFiles: FsDirOpenMode = BIT(1);
/// Causes result entries to not contain filesize information (always 0).
pub const FsDirOpenMode_NoFileSize: FsDirOpenMode = BIT(31);

/// For use with fsFileRead.
pub type FsReadOption = u32;
/// No option.
pub const FsReadOption_None: FsReadOption = 0;

/// For use with fsFileWrite.
pub type FsWriteOption = u32;
/// No option.
pub const FsWriteOption_None: FsWriteOption = 0;
/// Forces a flush after write.
pub const FsWriteOption_Flush: FsWriteOption = BIT(0);

pub type FsContentStorageId = u32;
/// System
pub const FsContentStorageId_System: FsContentStorageId = 0;
/// User
pub const FsContentStorageId_User: FsContentStorageId = 1;
/// SdCard
pub const FsContentStorageId_SdCard: FsContentStorageId = 2;
/// [16.0.0+] System0
pub const FsContentStorageId_System0: FsContentStorageId = 3;

pub type FsCustomStorageId = u32;
pub const FsCustomStorageId_System: FsCustomStorageId = 0;
pub const FsCustomStorageId_SdCard: FsCustomStorageId = 1;

/// ImageDirectoryId
pub type FsImageDirectoryId = u32;
pub const FsImageDirectoryId_Nand: FsImageDirectoryId = 0;
pub const FsImageDirectoryId_Sd: FsImageDirectoryId = 1;

/// SaveDataSpaceId
pub type FsSaveDataSpaceId = i32;
/// System
pub const FsSaveDataSpaceId_System: FsSaveDataSpaceId = 0;
/// User
pub const FsSaveDataSpaceId_User: FsSaveDataSpaceId = 1;
/// SdSystem
pub const FsSaveDataSpaceId_SdSystem: FsSaveDataSpaceId = 2;
/// [3.0.0+] Temporary
pub const FsSaveDataSpaceId_Temporary: FsSaveDataSpaceId = 3;
/// [4.0.0+] SdUser
pub const FsSaveDataSpaceId_SdUser: FsSaveDataSpaceId = 4;
/// [3.0.0+] ProperSystem
pub const FsSaveDataSpaceId_ProperSystem: FsSaveDataSpaceId = 100;
/// [3.0.0+] SafeMode
pub const FsSaveDataSpaceId_SafeMode: FsSaveDataSpaceId = 101;
/// Pseudo value for fsOpenSaveDataInfoReader().
pub const FsSaveDataSpaceId_All: FsSaveDataSpaceId = -1;

/// SaveDataType
pub type FsSaveDataType = u32;
/// System
pub const FsSaveDataType_System: FsSaveDataType = 0;
/// Account
pub const FsSaveDataType_Account: FsSaveDataType = 1;
/// Bcat
pub const FsSaveDataType_Bcat: FsSaveDataType = 2;
/// Device
pub const FsSaveDataType_Device: FsSaveDataType = 3;
/// [3.0.0+] Temporary
pub const FsSaveDataType_Temporary: FsSaveDataType = 4;
/// [3.0.0+] Cache
pub const FsSaveDataType_Cache: FsSaveDataType = 5;
/// [4.0.0+] SystemBcat
pub const FsSaveDataType_SystemBcat: FsSaveDataType = 6;

/// SaveDataRank
pub type FsSaveDataRank = u32;
/// Primary
pub const FsSaveDataRank_Primary: FsSaveDataRank = 0;
/// Secondary
pub const FsSaveDataRank_Secondary: FsSaveDataRank = 1;

/// SaveDataFlags
pub type FsSaveDataFlags = u32;
pub const FsSaveDataFlags_KeepAfterResettingSystemSaveData: FsSaveDataFlags = BIT(0);
pub const FsSaveDataFlags_KeepAfterRefurbishment: FsSaveDataFlags = BIT(1);
pub const FsSaveDataFlags_KeepAfterResettingSystemSaveDataWithoutUserSaveData: FsSaveDataFlags = BIT(2);
pub const FsSaveDataFlags_NeedsSecureDelete: FsSaveDataFlags = BIT(3);

/// SaveDataMetaType
pub type FsSaveDataMetaType = u32;
pub const FsSaveDataMetaType_None: FsSaveDataMetaType = 0;
pub const FsSaveDataMetaType_Thumbnail: FsSaveDataMetaType = 1;
pub const FsSaveDataMetaType_ExtensionContext: FsSaveDataMetaType = 2;

pub type FsGameCardAttribute = u32;
/// Causes the cartridge to automatically start on bootup
pub const FsGameCardAttribute_AutoBootFlag: FsGameCardAttribute = BIT(0);
/// Causes NS to throw an error on attempt to load the cartridge
pub const FsGameCardAttribute_HistoryEraseFlag: FsGameCardAttribute = BIT(1);
/// [4.0.0+] Indicates that this gamecard is a repair tool.
pub const FsGameCardAttribute_RepairToolFlag: FsGameCardAttribute = BIT(2);
/// [9.0.0+] DifferentRegionCupToTerraDeviceFlag
pub const FsGameCardAttribute_DifferentRegionCupToTerraDeviceFlag: FsGameCardAttribute = BIT(3);
/// [9.0.0+] DifferentRegionCupToGlobalDeviceFlag
pub const FsGameCardAttribute_DifferentRegionCupToGlobalDeviceFlag: FsGameCardAttribute = BIT(4);

pub type FsGameCardPartition = u32;
pub const FsGameCardPartition_Update: FsGameCardPartition = 0;
pub const FsGameCardPartition_Normal: FsGameCardPartition = 1;
pub const FsGameCardPartition_Secure: FsGameCardPartition = 2;
/// [4.0.0+]
pub const FsGameCardPartition_Logo: FsGameCardPartition = 3;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FsGameCardHandle {
    pub value: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FsGameCardUpdatePartitionInfo {
    pub version: u32,
    pub pad: [u8; 4],
    pub id: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FsRangeInfo {
    /// Contains bitflags describing how data is AES encrypted.
    pub aes_ctr_key_type: u32,
    /// Contains bitflags describing how data is emulated.
    pub speed_emulation_type: u32,
    pub reserved: [u32; 14],
}

pub type FsOperationId = u32;
/// Fill range with zero for supported file/storage.
pub const FsOperationId_Clear: FsOperationId = 0;
/// Clears signature for supported file/storage.
pub const FsOperationId_ClearSignature: FsOperationId = 1;
/// Invalidates cache for supported file/storage.
pub const FsOperationId_InvalidateCache: FsOperationId = 2;
/// Retrieves information on data for supported file/storage.
pub const FsOperationId_QueryRange: FsOperationId = 3;

/// BisPartitionId
pub type FsBisPartitionId = u32;
pub const FsBisPartitionId_BootPartition1Root: FsBisPartitionId = 0;
pub const FsBisPartitionId_BootPartition2Root: FsBisPartitionId = 10;
pub const FsBisPartitionId_UserDataRoot: FsBisPartitionId = 20;
pub const FsBisPartitionId_BootConfigAndPackage2Part1: FsBisPartitionId = 21;
pub const FsBisPartitionId_BootConfigAndPackage2Part2: FsBisPartitionId = 22;
pub const FsBisPartitionId_BootConfigAndPackage2Part3: FsBisPartitionId = 23;
pub const FsBisPartitionId_BootConfigAndPackage2Part4: FsBisPartitionId = 24;
pub const FsBisPartitionId_BootConfigAndPackage2Part5: FsBisPartitionId = 25;
pub const FsBisPartitionId_BootConfigAndPackage2Part6: FsBisPartitionId = 26;
pub const FsBisPartitionId_CalibrationBinary: FsBisPartitionId = 27;
pub const FsBisPartitionId_CalibrationFile: FsBisPartitionId = 28;
pub const FsBisPartitionId_SafeMode: FsBisPartitionId = 29;
pub const FsBisPartitionId_User: FsBisPartitionId = 30;
pub const FsBisPartitionId_System: FsBisPartitionId = 31;
pub const FsBisPartitionId_SystemProperEncryption: FsBisPartitionId = 32;
pub const FsBisPartitionId_SystemProperPartition: FsBisPartitionId = 33;
pub const FsBisPartitionId_SignedSystemPartitionOnSafeMode: FsBisPartitionId = 34;
pub const FsBisPartitionId_DeviceTreeBlob: FsBisPartitionId = 35;
pub const FsBisPartitionId_System0: FsBisPartitionId = 36;

/// FileSystemType
pub type FsFileSystemType = u32;
/// Logo
pub const FsFileSystemType_Logo: FsFileSystemType = 2;
/// ContentControl
pub const FsFileSystemType_ContentControl: FsFileSystemType = 3;
/// ContentManual
pub const FsFileSystemType_ContentManual: FsFileSystemType = 4;
/// ContentMeta
pub const FsFileSystemType_ContentMeta: FsFileSystemType = 5;
/// ContentData
pub const FsFileSystemType_ContentData: FsFileSystemType = 6;
/// ApplicationPackage
pub const FsFileSystemType_ApplicationPackage: FsFileSystemType = 7;
/// [4.0.0+] RegisteredUpdate
pub const FsFileSystemType_RegisteredUpdate: FsFileSystemType = 8;

/// FileSystemQueryId
pub type FsFileSystemQueryId = u32;
/// [4.0.0+]
pub const FsFileSystemQueryId_SetConcatenationFileAttribute: FsFileSystemQueryId = 0;
/// [8.0.0+]
pub const FsFileSystemQueryId_IsValidSignedSystemPartitionOnSdCard: FsFileSystemQueryId = 2;

/// FileSystemAttribute
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FsFileSystemAttribute {
    pub directory_name_length_max_has_value: bool,
    pub file_name_length_max_has_value: bool,
    pub directory_path_length_max_has_value: bool,
    pub file_path_length_max_has_value: bool,
    pub utf16_create_directory_path_length_max_has_value: bool,
    pub utf16_delete_directory_path_length_max_has_value: bool,
    pub utf16_rename_source_directory_path_length_max_has_value: bool,
    pub utf16_rename_destination_directory_path_length_max_has_value: bool,
    pub utf16_open_directory_path_length_max_has_value: bool,
    pub utf16_directory_name_length_max_has_value: bool,
    pub utf16_file_name_length_max_has_value: bool,
    pub utf16_directory_path_length_max_has_value: bool,
    pub utf16_file_path_length_max_has_value: bool,
    pub reserved1: [u8; 27],
    pub directory_name_length_max: s32,
    pub file_name_length_max: s32,
    pub directory_path_length_max: s32,
    pub file_path_length_max: s32,
    pub utf16_create_directory_path_length_max: s32,
    pub utf16_delete_directory_path_length_max: s32,
    pub utf16_rename_source_directory_path_length_max: s32,
    pub utf16_rename_destination_directory_path_length_max: s32,
    pub utf16_open_directory_path_length_max: s32,
    pub utf16_directory_name_length_max: s32,
    pub utf16_file_name_length_max: s32,
    pub utf16_directory_path_length_max: s32,
    pub utf16_file_path_length_max: s32,
    pub reserved2: [u8; 100],
}

/// FsPriority
pub type FsPriority = u32;
pub const FsPriority_Normal: FsPriority = 0;
pub const FsPriority_Realtime: FsPriority = 1;
pub const FsPriority_Low: FsPriority = 2;
pub const FsPriority_Background: FsPriority = 3;

/// FsContentAttributes
pub type FsContentAttributes = u32;
pub const FsContentAttributes_None: FsContentAttributes = 0x0;
pub const FsContentAttributes_All: FsContentAttributes = 0xF;

/// For use with fsOpenHostFileSystemWithOption
pub type FsMountHostOption = u32;
/// Host filesystem will be case insensitive.
pub const FsMountHostOptionFlag_None: FsMountHostOption = 0;
/// Host filesystem will be pseudo case sensitive.
pub const FsMountHostOptionFlag_PseudoCaseSensitive: FsMountHostOption = BIT(0);

/// FsStorageErrorInfo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FsStorageErrorInfo {
    pub num_activation_failures: u32,
    pub num_activation_error_corrections: u32,
    pub num_read_write_failures: u32,
    pub num_read_write_error_corrections: u32,
}

/// FatFatError
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FatFatError {
    pub error: s32,
    pub extra_error: s32,
    pub drive_id: s32,
    pub name: [c_char; 16],
    pub reserved: [u8; 4],
}

/// FatFatReportInfo1
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FatFatReportInfo1 {
    pub open_file_peak_count: u16,
    pub open_directory_peak_count: u16,
}

/// FatFatReportInfo2
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FatFatReportInfo2 {
    pub open_unique_file_entry_peak_count: u16,
    pub open_unique_directory_entry_peak_count: u16,
}

/// FatFatSafeInfo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FatFatSafeInfo {
    pub result: u32,
    pub error_number: u32,
    pub safe_error_number: u32,
}

/// FsFileSystemProxyErrorInfo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FsFileSystemProxyErrorInfo {
    pub rom_fs_remount_for_data_corruption_count: u32,
    pub rom_fs_unrecoverable_data_corruption_by_remount_count: u32,
    pub fat_fs_error: FatFatError,
    pub rom_fs_recovered_by_invalidate_cache_count: u32,
    pub save_data_index_count: u32,
    pub bis_system_fat_report_info_1: FatFatReportInfo1,
    pub bis_user_fat_report_info_1: FatFatReportInfo1,
    pub sd_card_fat_report_info_1: FatFatReportInfo1,
    pub bis_system_fat_report_info_2: FatFatReportInfo2,
    pub bis_user_fat_report_info_2: FatFatReportInfo2,
    pub sd_card_fat_report_info_2: FatFatReportInfo2,
    pub rom_fs_deep_retry_start_count: u32,
    pub rom_fs_unrecoverable_by_game_card_access_failed_count: u32,
    pub bis_system_fat_safe_info: FatFatSafeInfo,
    pub bis_user_fat_safe_info: FatFatSafeInfo,
    pub reserved: [u8; 24],
}

/// FsMemoryReportInfo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FsMemoryReportInfo {
    pub pooled_buffer_peak_free_size: u64,
    pub pooled_buffer_retried_count: u64,
    pub pooled_buffer_reduce_allocation_count: u64,
    pub buffer_manager_peak_free_size: u64,
    pub buffer_manager_retried_count: u64,
    pub exp_heap_peak_free_size: u64,
    pub buffer_pool_peak_free_size: u64,
    pub patrol_read_allocate_buffer_success_count: u64,
    pub patrol_read_allocate_buffer_failure_count: u64,
    pub buffer_manager_peak_total_allocatable_size: u64,
    pub buffer_pool_max_allocate_size: u64,
    pub pooled_buffer_failed_ideal_allocation_count_on_async_access: u64,
    pub reserved: [u8; 32],
}

/// FsGameCardErrorReportInfo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FsGameCardErrorReportInfo {
    pub game_card_crc_error_num: u16,
    pub reserved1: u16,
    pub asic_crc_error_num: u16,
    pub reserved2: u16,
    pub refresh_num: u16,
    pub reserved3: u16,
    pub retry_limit_out_num: u16,
    pub timeout_retry_num: u16,
    pub asic_reinitialize_failure_detail: u16,
    pub insertion_count: u16,
    pub removal_count: u16,
    pub asic_reinitialize_num: u16,
    pub initialize_count: u32,
    pub asic_reinitialize_failure_num: u16,
    pub awaken_failure_num: u16,
    pub reserved4: u16,
    pub refresh_succeeded_count: u16,
    pub last_read_error_page_address: u32,
    pub last_read_error_page_count: u32,
    pub awaken_count: u32,
    pub read_count_from_insert: u32,
    pub read_count_from_awaken: u32,
    pub reserved5: [u8; 8],
}

impl_zeroed_default!(FsRightsId, FsFileSystem, FsFile, FsDir, FsStorage, FsSaveDataInfoReader, FsEventNotifier, FsDeviceOperator, FsDirectoryEntry, FsSaveDataAttribute, FsSaveDataExtraData, FsSaveDataMetaInfo, FsSaveDataCreationInfo, FsSaveDataInfo, FsSaveDataFilter, FsTimeStampRaw, FsArchiveMacKey, FsGameCardHandle, FsGameCardUpdatePartitionInfo, FsRangeInfo, FsFileSystemAttribute, FsStorageErrorInfo, FatFatError, FatFatReportInfo1, FatFatReportInfo2, FatFatSafeInfo, FsFileSystemProxyErrorInfo, FsMemoryReportInfo, FsGameCardErrorReportInfo);

unsafe extern "C" {
    /// Initialize fsp-srv. Used automatically during app startup.
    pub fn fsInitialize() -> Result;

    /// Exit fsp-srv. Used automatically during app exit.
    pub fn fsExit();

    /// Gets the Service object for the actual fsp-srv service session.
    pub fn fsGetServiceSession() -> *mut Service;

    /// [5.0.0+] Configures the `FsPriority` of all filesystem commands issued within the current thread.
    pub fn fsSetPriority(prio: FsPriority);

    /// same as calling fsOpenFileSystemWithId with 0 as id
    pub fn fsOpenFileSystem(out: *mut FsFileSystem, fsType: FsFileSystemType, contentPath: *const c_char) -> Result;

    pub fn fsOpenDataFileSystemByCurrentProcess(out: *mut FsFileSystem) -> Result;

    /// [2.0.0+], like OpenFileSystemWithId but without content path.
    pub fn fsOpenFileSystemWithPatch(out: *mut FsFileSystem, id: u64, fsType: FsFileSystemType) -> Result;

    /// works on all firmwares, id is ignored on [1.0.0], attr is ignored before [16.0.0]
    pub fn fsOpenFileSystemWithId(out: *mut FsFileSystem, id: u64, fsType: FsFileSystemType, contentPath: *const c_char, attr: FsContentAttributes) -> Result;

    /// [3.0.0+]
    pub fn fsOpenDataFileSystemByProgramId(out: *mut FsFileSystem, program_id: u64) -> Result;

    pub fn fsOpenBisFileSystem(out: *mut FsFileSystem, partitionId: FsBisPartitionId, string: *const c_char) -> Result;

    pub fn fsOpenBisStorage(out: *mut FsStorage, partitionId: FsBisPartitionId) -> Result;

    /// Do not call this directly, see fs_dev.h.
    pub fn fsOpenSdCardFileSystem(out: *mut FsFileSystem) -> Result;

    pub fn fsOpenHostFileSystem(out: *mut FsFileSystem, path: *const c_char) -> Result;

    /// [9.0.0+]
    pub fn fsOpenHostFileSystemWithOption(out: *mut FsFileSystem, path: *const c_char, flags: u32) -> Result;

    pub fn fsDeleteSaveDataFileSystem(application_id: u64) -> Result;

    pub fn fsCreateSaveDataFileSystem(attr: *const FsSaveDataAttribute, creation_info: *const FsSaveDataCreationInfo, meta: *const FsSaveDataMetaInfo) -> Result;

    pub fn fsCreateSaveDataFileSystemBySystemSaveDataId(attr: *const FsSaveDataAttribute, creation_info: *const FsSaveDataCreationInfo) -> Result;

    /// [2.0.0+]
    pub fn fsDeleteSaveDataFileSystemBySaveDataSpaceId(save_data_space_id: FsSaveDataSpaceId, saveID: u64) -> Result;

    /// [4.0.0+]
    pub fn fsDeleteSaveDataFileSystemBySaveDataAttribute(save_data_space_id: FsSaveDataSpaceId, attr: *const FsSaveDataAttribute) -> Result;

    pub fn fsIsExFatSupported(out: *mut bool) -> Result;

    pub fn fsOpenGameCardFileSystem(out: *mut FsFileSystem, handle: *const FsGameCardHandle, partition: FsGameCardPartition) -> Result;

    /// [3.0.0+]
    pub fn fsExtendSaveDataFileSystem(save_data_space_id: FsSaveDataSpaceId, saveID: u64, dataSize: s64, journalSize: s64) -> Result;

    pub fn fsOpenSaveDataFileSystem(out: *mut FsFileSystem, save_data_space_id: FsSaveDataSpaceId, attr: *const FsSaveDataAttribute) -> Result;

    pub fn fsOpenSaveDataFileSystemBySystemSaveDataId(out: *mut FsFileSystem, save_data_space_id: FsSaveDataSpaceId, attr: *const FsSaveDataAttribute) -> Result;

    /// [2.0.0+].
    pub fn fsOpenReadOnlySaveDataFileSystem(out: *mut FsFileSystem, save_data_space_id: FsSaveDataSpaceId, attr: *const FsSaveDataAttribute) -> Result;

    pub fn fsReadSaveDataFileSystemExtraDataBySaveDataSpaceId(buf: *mut c_void, len: usize, save_data_space_id: FsSaveDataSpaceId, saveID: u64) -> Result;

    pub fn fsReadSaveDataFileSystemExtraData(buf: *mut c_void, len: usize, saveID: u64) -> Result;

    pub fn fsWriteSaveDataFileSystemExtraData(buf: *const c_void, len: usize, save_data_space_id: FsSaveDataSpaceId, saveID: u64) -> Result;

    pub fn fsOpenSaveDataInfoReader(out: *mut FsSaveDataInfoReader, save_data_space_id: FsSaveDataSpaceId) -> Result;

    /// [6.0.0+]
    pub fn fsOpenSaveDataInfoReaderWithFilter(out: *mut FsSaveDataInfoReader, save_data_space_id: FsSaveDataSpaceId, save_data_filter: *const FsSaveDataFilter) -> Result;

    pub fn fsOpenImageDirectoryFileSystem(out: *mut FsFileSystem, image_directory_id: FsImageDirectoryId) -> Result;

    pub fn fsOpenContentStorageFileSystem(out: *mut FsFileSystem, content_storage_id: FsContentStorageId) -> Result;

    /// [7.0.0+]
    pub fn fsOpenCustomStorageFileSystem(out: *mut FsFileSystem, custom_storage_id: FsCustomStorageId) -> Result;

    pub fn fsOpenDataStorageByCurrentProcess(out: *mut FsStorage) -> Result;

    pub fn fsOpenDataStorageByProgramId(out: *mut FsStorage, program_id: u64) -> Result;

    pub fn fsOpenDataStorageByDataId(out: *mut FsStorage, dataId: u64, storageId: NcmStorageId) -> Result;

    pub fn fsOpenPatchDataStorageByCurrentProcess(out: *mut FsStorage) -> Result;

    pub fn fsOpenDeviceOperator(out: *mut FsDeviceOperator) -> Result;

    pub fn fsOpenSdCardDetectionEventNotifier(out: *mut FsEventNotifier) -> Result;

    pub fn fsIsSignedSystemPartitionOnSdCardValid(out: *mut bool) -> Result;

    /// [17.0.0+]
    pub fn fsGetProgramId(out: *mut u64, path: *const c_char, attr: FsContentAttributes) -> Result;

    /// Retrieves the rights id corresponding to the content path. Only available on [2.0.0-15.0.1].
    pub fn fsGetRightsIdByPath(path: *const c_char, out_rights_id: *mut FsRightsId) -> Result;

    /// Retrieves the rights id and key generation corresponding to the content path. Only available on [3.0.0+], attr is ignored before [16.0.0].
    pub fn fsGetRightsIdAndKeyGenerationByPath(path: *const c_char, attr: FsContentAttributes, out_key_generation: *mut u8, out_rights_id: *mut FsRightsId) -> Result;

    /// [19.0.0+]
    pub fn fsGetContentStorageInfoIndex(out: *mut s32) -> Result;

    pub fn fsDisableAutoSaveDataCreation() -> Result;

    pub fn fsSetGlobalAccessLogMode(mode: u32) -> Result;

    pub fn fsGetGlobalAccessLogMode(out_mode: *mut u32) -> Result;

    pub fn fsOutputAccessLogToSdCard(log: *const c_char, size: usize) -> Result;

    /// [2.0.0+]
    pub fn fsGetAndClearErrorInfo(out: *mut FsFileSystemProxyErrorInfo) -> Result;

    /// [4.0.0+]
    pub fn fsGetAndClearMemoryReportInfo(out: *mut FsMemoryReportInfo) -> Result;

    /// Only available on [7.0.0+].
    pub fn fsGetProgramIndexForAccessLog(out_program_index: *mut u32, out_program_count: *mut u32) -> Result;

    pub fn fsCreate_TemporaryStorage(application_id: u64, owner_id: u64, size: s64, flags: u32) -> Result;

    pub fn fsCreate_SystemSaveDataWithOwner(save_data_space_id: FsSaveDataSpaceId, system_save_data_id: u64, uid: AccountUid, owner_id: u64, size: s64, journal_size: s64, flags: u32) -> Result;

    pub fn fsCreate_SystemSaveData(save_data_space_id: FsSaveDataSpaceId, system_save_data_id: u64, size: s64, journal_size: s64, flags: u32) -> Result;

    /// Wrapper for fsOpenSaveDataFileSystem.
    /// See `FsSaveDataAttribute` for application_id and uid.
    pub fn fsOpen_SaveData(out: *mut FsFileSystem, application_id: u64, uid: AccountUid) -> Result;

    /// Wrapper for fsOpenReadOnlySaveDataFileSystem.
    /// Only available on [2.0.0+].
    /// See `FsSaveDataAttribute` for application_id and uid.
    pub fn fsOpen_SaveDataReadOnly(out: *mut FsFileSystem, application_id: u64, uid: AccountUid) -> Result;

    /// Wrapper for fsOpenSaveDataFileSystem, for opening BcatSaveData.
    pub fn fsOpen_BcatSaveData(out: *mut FsFileSystem, application_id: u64) -> Result;

    /// Wrapper for fsOpenSaveDataFileSystem, for opening DeviceSaveData.
    /// See `FsSaveDataAttribute` for application_id.
    pub fn fsOpen_DeviceSaveData(out: *mut FsFileSystem, application_id: u64) -> Result;

    /// Wrapper for fsOpenSaveDataFileSystem, for opening TemporaryStorage.
    /// Only available on [3.0.0+].
    pub fn fsOpen_TemporaryStorage(out: *mut FsFileSystem) -> Result;

    /// Wrapper for fsOpenSaveDataFileSystem, for opening CacheStorage.
    /// Only available on [3.0.0+].
    /// See `FsSaveDataAttribute` for application_id.
    pub fn fsOpen_CacheStorage(out: *mut FsFileSystem, application_id: u64, save_data_index: u16) -> Result;

    /// Wrapper for fsOpenSaveDataFileSystemBySystemSaveDataId, for opening SystemSaveData.
    /// WARNING: You can brick when writing to SystemSaveData, if the data is corrupted etc.
    pub fn fsOpen_SystemSaveData(out: *mut FsFileSystem, save_data_space_id: FsSaveDataSpaceId, system_save_data_id: u64, uid: AccountUid) -> Result;

    /// Wrapper for fsOpenSaveDataFileSystemBySystemSaveDataId, for opening SystemBcatSaveData.
    /// Only available on [4.0.0+].
    pub fn fsOpen_SystemBcatSaveData(out: *mut FsFileSystem, system_save_data_id: u64) -> Result;

    pub fn fsFsCreateFile(fs: *mut FsFileSystem, path: *const c_char, size: s64, option: u32) -> Result;

    pub fn fsFsDeleteFile(fs: *mut FsFileSystem, path: *const c_char) -> Result;

    pub fn fsFsCreateDirectory(fs: *mut FsFileSystem, path: *const c_char) -> Result;

    pub fn fsFsDeleteDirectory(fs: *mut FsFileSystem, path: *const c_char) -> Result;

    pub fn fsFsDeleteDirectoryRecursively(fs: *mut FsFileSystem, path: *const c_char) -> Result;

    pub fn fsFsRenameFile(fs: *mut FsFileSystem, cur_path: *const c_char, new_path: *const c_char) -> Result;

    pub fn fsFsRenameDirectory(fs: *mut FsFileSystem, cur_path: *const c_char, new_path: *const c_char) -> Result;

    pub fn fsFsGetEntryType(fs: *mut FsFileSystem, path: *const c_char, out: *mut FsDirEntryType) -> Result;

    pub fn fsFsOpenFile(fs: *mut FsFileSystem, path: *const c_char, mode: u32, out: *mut FsFile) -> Result;

    pub fn fsFsOpenDirectory(fs: *mut FsFileSystem, path: *const c_char, mode: u32, out: *mut FsDir) -> Result;

    pub fn fsFsCommit(fs: *mut FsFileSystem) -> Result;

    pub fn fsFsGetFreeSpace(fs: *mut FsFileSystem, path: *const c_char, out: *mut s64) -> Result;

    pub fn fsFsGetTotalSpace(fs: *mut FsFileSystem, path: *const c_char, out: *mut s64) -> Result;

    /// [3.0.0+]
    pub fn fsFsGetFileTimeStampRaw(fs: *mut FsFileSystem, path: *const c_char, out: *mut FsTimeStampRaw) -> Result;

    /// [3.0.0+]
    pub fn fsFsCleanDirectoryRecursively(fs: *mut FsFileSystem, path: *const c_char) -> Result;

    /// [4.0.0+]
    pub fn fsFsQueryEntry(fs: *mut FsFileSystem, out: *mut c_void, out_size: usize, in_: *const c_void, in_size: usize, path: *const c_char, query_id: FsFileSystemQueryId) -> Result;

    /// [15.0.0+]
    pub fn fsFsGetFileSystemAttribute(fs: *mut FsFileSystem, out: *mut FsFileSystemAttribute) -> Result;

    pub fn fsFsClose(fs: *mut FsFileSystem);

    /// Uses `fsFsQueryEntry` to set the archive bit on the specified absolute directory path.
    /// This will cause HOS to treat the directory as if it were a file containing the directory's concatenated contents.
    pub fn fsFsSetConcatenationFileAttribute(fs: *mut FsFileSystem, path: *const c_char) -> Result;

    /// Wrapper for fsFsQueryEntry with FsFileSystemQueryId_IsValidSignedSystemPartitionOnSdCard.
    /// Only available on [8.0.0+].
    pub fn fsFsIsValidSignedSystemPartitionOnSdCard(fs: *mut FsFileSystem, out: *mut bool) -> Result;

    pub fn fsFileRead(f: *mut FsFile, off: s64, buf: *mut c_void, read_size: u64, option: u32, bytes_read: *mut u64) -> Result;

    pub fn fsFileWrite(f: *mut FsFile, off: s64, buf: *const c_void, write_size: u64, option: u32) -> Result;

    pub fn fsFileFlush(f: *mut FsFile) -> Result;

    pub fn fsFileSetSize(f: *mut FsFile, sz: s64) -> Result;

    pub fn fsFileGetSize(f: *mut FsFile, out: *mut s64) -> Result;

    /// [4.0.0+]
    pub fn fsFileOperateRange(f: *mut FsFile, op_id: FsOperationId, off: s64, len: s64, out: *mut FsRangeInfo) -> Result;

    pub fn fsFileClose(f: *mut FsFile);

    pub fn fsDirRead(d: *mut FsDir, total_entries: *mut s64, max_entries: usize, buf: *mut FsDirectoryEntry) -> Result;

    pub fn fsDirGetEntryCount(d: *mut FsDir, count: *mut s64) -> Result;

    pub fn fsDirClose(d: *mut FsDir);

    pub fn fsStorageRead(s: *mut FsStorage, off: s64, buf: *mut c_void, read_size: u64) -> Result;

    pub fn fsStorageWrite(s: *mut FsStorage, off: s64, buf: *const c_void, write_size: u64) -> Result;

    pub fn fsStorageFlush(s: *mut FsStorage) -> Result;

    pub fn fsStorageSetSize(s: *mut FsStorage, sz: s64) -> Result;

    pub fn fsStorageGetSize(s: *mut FsStorage, out: *mut s64) -> Result;

    /// [4.0.0+]
    pub fn fsStorageOperateRange(s: *mut FsStorage, op_id: FsOperationId, off: s64, len: s64, out: *mut FsRangeInfo) -> Result;

    pub fn fsStorageClose(s: *mut FsStorage);

    /// Read FsSaveDataInfo data into the buf array.
    pub fn fsSaveDataInfoReaderRead(s: *mut FsSaveDataInfoReader, buf: *mut FsSaveDataInfo, max_entries: usize, total_entries: *mut s64) -> Result;

    pub fn fsSaveDataInfoReaderClose(s: *mut FsSaveDataInfoReader);

    pub fn fsEventNotifierGetEventHandle(e: *mut FsEventNotifier, out: *mut Event, autoclear: bool) -> Result;

    pub fn fsEventNotifierClose(e: *mut FsEventNotifier);

    pub fn fsDeviceOperatorIsSdCardInserted(d: *mut FsDeviceOperator, out: *mut bool) -> Result;

    pub fn fsDeviceOperatorGetSdCardSpeedMode(d: *mut FsDeviceOperator, out: *mut s64) -> Result;

    pub fn fsDeviceOperatorGetSdCardCid(d: *mut FsDeviceOperator, dst: *mut c_void, dst_size: usize, size: s64) -> Result;

    pub fn fsDeviceOperatorGetSdCardUserAreaSize(d: *mut FsDeviceOperator, out: *mut s64) -> Result;

    pub fn fsDeviceOperatorGetSdCardProtectedAreaSize(d: *mut FsDeviceOperator, out: *mut s64) -> Result;

    pub fn fsDeviceOperatorGetAndClearSdCardErrorInfo(d: *mut FsDeviceOperator, out: *mut FsStorageErrorInfo, out_log_size: *mut s64, dst: *mut c_void, dst_size: usize, size: s64) -> Result;

    pub fn fsDeviceOperatorGetMmcCid(d: *mut FsDeviceOperator, dst: *mut c_void, dst_size: usize, size: s64) -> Result;

    pub fn fsDeviceOperatorGetMmcSpeedMode(d: *mut FsDeviceOperator, out: *mut s64) -> Result;

    pub fn fsDeviceOperatorGetMmcPatrolCount(d: *mut FsDeviceOperator, out: *mut u32) -> Result;

    pub fn fsDeviceOperatorGetAndClearMmcErrorInfo(d: *mut FsDeviceOperator, out: *mut FsStorageErrorInfo, out_log_size: *mut s64, dst: *mut c_void, dst_size: usize, size: s64) -> Result;

    pub fn fsDeviceOperatorGetMmcExtendedCsd(d: *mut FsDeviceOperator, dst: *mut c_void, dst_size: usize, size: s64) -> Result;

    pub fn fsDeviceOperatorIsGameCardInserted(d: *mut FsDeviceOperator, out: *mut bool) -> Result;

    pub fn fsDeviceOperatorGetGameCardHandle(d: *mut FsDeviceOperator, out: *mut FsGameCardHandle) -> Result;

    pub fn fsDeviceOperatorGetGameCardUpdatePartitionInfo(d: *mut FsDeviceOperator, handle: *const FsGameCardHandle, out: *mut FsGameCardUpdatePartitionInfo) -> Result;

    pub fn fsDeviceOperatorGetGameCardAttribute(d: *mut FsDeviceOperator, handle: *const FsGameCardHandle, out: *mut u8) -> Result;

    pub fn fsDeviceOperatorGetGameCardDeviceCertificate(d: *mut FsDeviceOperator, handle: *const FsGameCardHandle, dst: *mut c_void, dst_size: usize, out_size: *mut s64, size: s64) -> Result;

    pub fn fsDeviceOperatorGetGameCardIdSet(d: *mut FsDeviceOperator, dst: *mut c_void, dst_size: usize, size: s64) -> Result;

    pub fn fsDeviceOperatorGetGameCardErrorReportInfo(d: *mut FsDeviceOperator, out: *mut FsGameCardErrorReportInfo) -> Result;

    pub fn fsDeviceOperatorGetGameCardDeviceId(d: *mut FsDeviceOperator, dst: *mut c_void, dst_size: usize, size: s64) -> Result;

    pub fn fsDeviceOperatorChallengeCardExistence(d: *mut FsDeviceOperator, handle: *const FsGameCardHandle, dst: *mut c_void, dst_size: usize, seed: *mut c_void, seed_size: usize, value: *mut c_void, value_size: usize) -> Result;

    pub fn fsDeviceOperatorClose(d: *mut FsDeviceOperator);
}
