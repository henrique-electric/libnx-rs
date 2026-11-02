//! `switch/services/ncm.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// ContentStorage
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NcmContentStorage {
    /// IContentStorage
    pub s: Service,
}

/// ContentMetaDatabase
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NcmContentMetaDatabase {
    /// IContentMetaDatabase
    pub s: Service,
}

/// RightsId
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NcmRightsId {
    pub rights_id: FsRightsId,
    /// [3.0.0+]
    pub key_generation: u8,
    /// [3.0.0+]
    pub pad: [u8; 7],
}

impl_zeroed_default!(NcmContentStorage, NcmContentMetaDatabase, NcmRightsId);

unsafe extern "C" {
    /// Initialize ncm.
    pub fn ncmInitialize() -> Result;

    /// Exit ncm.
    pub fn ncmExit();

    /// Gets the Service object for the actual ncm service session.
    pub fn ncmGetServiceSession() -> *mut Service;

    pub fn ncmCreateContentStorage(storage_id: NcmStorageId) -> Result;

    pub fn ncmCreateContentMetaDatabase(storage_id: NcmStorageId) -> Result;

    pub fn ncmVerifyContentStorage(storage_id: NcmStorageId) -> Result;

    pub fn ncmVerifyContentMetaDatabase(storage_id: NcmStorageId) -> Result;

    pub fn ncmOpenContentStorage(out_content_storage: *mut NcmContentStorage, storage_id: NcmStorageId) -> Result;

    pub fn ncmOpenContentMetaDatabase(out_content_meta_database: *mut NcmContentMetaDatabase, storage_id: NcmStorageId) -> Result;

    /// [1.0.0]
    pub fn ncmCloseContentStorageForcibly(storage_id: NcmStorageId) -> Result;

    /// [1.0.0]
    pub fn ncmCloseContentMetaDatabaseForcibly(storage_id: NcmStorageId) -> Result;

    pub fn ncmCleanupContentMetaDatabase(storage_id: NcmStorageId) -> Result;

    /// [2.0.0+]
    pub fn ncmActivateContentStorage(storage_id: NcmStorageId) -> Result;

    /// [2.0.0+]
    pub fn ncmInactivateContentStorage(storage_id: NcmStorageId) -> Result;

    /// [2.0.0+]
    pub fn ncmActivateContentMetaDatabase(storage_id: NcmStorageId) -> Result;

    /// [2.0.0+]
    pub fn ncmInactivateContentMetaDatabase(storage_id: NcmStorageId) -> Result;

    /// [9.0.0+]
    pub fn ncmInvalidateRightsIdCache() -> Result;

    /// [16.0.0+]
    pub fn ncmActivateFsContentStorage(fs_storage_id: FsContentStorageId) -> Result;

    pub fn ncmContentStorageClose(cs: *mut NcmContentStorage);

    pub fn ncmContentStorageGeneratePlaceHolderId(cs: *mut NcmContentStorage, out_id: *mut NcmPlaceHolderId) -> Result;

    pub fn ncmContentStorageCreatePlaceHolder(cs: *mut NcmContentStorage, content_id: *const NcmContentId, placeholder_id: *const NcmPlaceHolderId, size: s64) -> Result;

    pub fn ncmContentStorageDeletePlaceHolder(cs: *mut NcmContentStorage, placeholder_id: *const NcmPlaceHolderId) -> Result;

    pub fn ncmContentStorageHasPlaceHolder(cs: *mut NcmContentStorage, out: *mut bool, placeholder_id: *const NcmPlaceHolderId) -> Result;

    pub fn ncmContentStorageWritePlaceHolder(cs: *mut NcmContentStorage, placeholder_id: *const NcmPlaceHolderId, offset: u64, data: *const c_void, data_size: usize) -> Result;

    pub fn ncmContentStorageRegister(cs: *mut NcmContentStorage, content_id: *const NcmContentId, placeholder_id: *const NcmPlaceHolderId) -> Result;

    pub fn ncmContentStorageDelete(cs: *mut NcmContentStorage, content_id: *const NcmContentId) -> Result;

    pub fn ncmContentStorageHas(cs: *mut NcmContentStorage, out: *mut bool, content_id: *const NcmContentId) -> Result;

    pub fn ncmContentStorageGetPath(cs: *mut NcmContentStorage, out_path: *mut c_char, out_size: usize, content_id: *const NcmContentId) -> Result;

    pub fn ncmContentStorageGetPlaceHolderPath(cs: *mut NcmContentStorage, out_path: *mut c_char, out_size: usize, placeholder_id: *const NcmPlaceHolderId) -> Result;

    pub fn ncmContentStorageCleanupAllPlaceHolder(cs: *mut NcmContentStorage) -> Result;

    pub fn ncmContentStorageListPlaceHolder(cs: *mut NcmContentStorage, out_ids: *mut NcmPlaceHolderId, count: s32, out_count: *mut s32) -> Result;

    pub fn ncmContentStorageGetContentCount(cs: *mut NcmContentStorage, out_count: *mut s32) -> Result;

    pub fn ncmContentStorageListContentId(cs: *mut NcmContentStorage, out_ids: *mut NcmContentId, count: s32, out_count: *mut s32, start_offset: s32) -> Result;

    pub fn ncmContentStorageGetSizeFromContentId(cs: *mut NcmContentStorage, out_size: *mut s64, content_id: *const NcmContentId) -> Result;

    pub fn ncmContentStorageDisableForcibly(cs: *mut NcmContentStorage) -> Result;

    /// [2.0.0+]
    pub fn ncmContentStorageRevertToPlaceHolder(cs: *mut NcmContentStorage, placeholder_id: *const NcmPlaceHolderId, old_content_id: *const NcmContentId, new_content_id: *const NcmContentId) -> Result;

    /// [2.0.0+]
    pub fn ncmContentStorageSetPlaceHolderSize(cs: *mut NcmContentStorage, placeholder_id: *const NcmPlaceHolderId, size: s64) -> Result;

    /// [2.0.0+]
    pub fn ncmContentStorageReadContentIdFile(cs: *mut NcmContentStorage, out_data: *mut c_void, out_data_size: usize, content_id: *const NcmContentId, offset: s64) -> Result;

    /// [2.0.0+]
    pub fn ncmContentStorageGetRightsIdFromPlaceHolderId(cs: *mut NcmContentStorage, out_rights_id: *mut NcmRightsId, placeholder_id: *const NcmPlaceHolderId, attr: FsContentAttributes) -> Result;

    /// [2.0.0+]
    pub fn ncmContentStorageGetRightsIdFromContentId(cs: *mut NcmContentStorage, out_rights_id: *mut NcmRightsId, content_id: *const NcmContentId, attr: FsContentAttributes) -> Result;

    /// [2.0.0+]
    pub fn ncmContentStorageWriteContentForDebug(cs: *mut NcmContentStorage, content_id: *const NcmContentId, offset: s64, data: *const c_void, data_size: usize) -> Result;

    /// [2.0.0+]
    pub fn ncmContentStorageGetFreeSpaceSize(cs: *mut NcmContentStorage, out_size: *mut s64) -> Result;

    /// [2.0.0+]
    pub fn ncmContentStorageGetTotalSpaceSize(cs: *mut NcmContentStorage, out_size: *mut s64) -> Result;

    /// [3.0.0+]
    pub fn ncmContentStorageFlushPlaceHolder(cs: *mut NcmContentStorage) -> Result;

    /// [4.0.0+]
    pub fn ncmContentStorageGetSizeFromPlaceHolderId(cs: *mut NcmContentStorage, out_size: *mut s64, placeholder_id: *const NcmPlaceHolderId) -> Result;

    /// [4.0.0+]
    pub fn ncmContentStorageRepairInvalidFileAttribute(cs: *mut NcmContentStorage) -> Result;

    /// [8.0.0+]
    pub fn ncmContentStorageGetRightsIdFromPlaceHolderIdWithCache(cs: *mut NcmContentStorage, out_rights_id: *mut NcmRightsId, placeholder_id: *const NcmPlaceHolderId, cache_content_id: *const NcmContentId, attr: FsContentAttributes) -> Result;

    /// [13.0.0+]
    pub fn ncmContentStorageRegisterPath(cs: *mut NcmContentStorage, content_id: *const NcmContentId, path: *const c_char) -> Result;

    /// [13.0.0+]
    pub fn ncmContentStorageClearRegisteredPath(cs: *mut NcmContentStorage) -> Result;

    /// [17.0.0+]
    pub fn ncmContentStorageGetProgramId(cs: *mut NcmContentStorage, out: *mut u64, content_id: *const NcmContentId, attr: FsContentAttributes) -> Result;

    pub fn ncmContentMetaDatabaseClose(db: *mut NcmContentMetaDatabase);

    pub fn ncmContentMetaDatabaseSet(db: *mut NcmContentMetaDatabase, key: *const NcmContentMetaKey, data: *const c_void, data_size: u64) -> Result;

    pub fn ncmContentMetaDatabaseGet(db: *mut NcmContentMetaDatabase, key: *const NcmContentMetaKey, out_size: *mut u64, out_data: *mut c_void, out_data_size: u64) -> Result;

    pub fn ncmContentMetaDatabaseRemove(db: *mut NcmContentMetaDatabase, key: *const NcmContentMetaKey) -> Result;

    pub fn ncmContentMetaDatabaseGetContentIdByType(db: *mut NcmContentMetaDatabase, out_content_id: *mut NcmContentId, key: *const NcmContentMetaKey, type_: NcmContentType) -> Result;

    pub fn ncmContentMetaDatabaseListContentInfo(db: *mut NcmContentMetaDatabase, out_entries_written: *mut s32, out_info: *mut NcmContentInfo, count: s32, key: *const NcmContentMetaKey, start_index: s32) -> Result;

    pub fn ncmContentMetaDatabaseList(db: *mut NcmContentMetaDatabase, out_entries_total: *mut s32, out_entries_written: *mut s32, out_keys: *mut NcmContentMetaKey, count: s32, meta_type: NcmContentMetaType, id: u64, id_min: u64, id_max: u64, install_type: NcmContentInstallType) -> Result;

    pub fn ncmContentMetaDatabaseGetLatestContentMetaKey(db: *mut NcmContentMetaDatabase, out_key: *mut NcmContentMetaKey, id: u64) -> Result;

    pub fn ncmContentMetaDatabaseListApplication(db: *mut NcmContentMetaDatabase, out_entries_total: *mut s32, out_entries_written: *mut s32, out_keys: *mut NcmApplicationContentMetaKey, count: s32, meta_type: NcmContentMetaType) -> Result;

    pub fn ncmContentMetaDatabaseHas(db: *mut NcmContentMetaDatabase, out: *mut bool, key: *const NcmContentMetaKey) -> Result;

    pub fn ncmContentMetaDatabaseHasAll(db: *mut NcmContentMetaDatabase, out: *mut bool, keys: *const NcmContentMetaKey, count: s32) -> Result;

    pub fn ncmContentMetaDatabaseGetSize(db: *mut NcmContentMetaDatabase, out_size: *mut u64, key: *const NcmContentMetaKey) -> Result;

    pub fn ncmContentMetaDatabaseGetRequiredSystemVersion(db: *mut NcmContentMetaDatabase, out_version: *mut u32, key: *const NcmContentMetaKey) -> Result;

    pub fn ncmContentMetaDatabaseGetPatchContentMetaId(db: *mut NcmContentMetaDatabase, out_patch_id: *mut u64, key: *const NcmContentMetaKey) -> Result;

    pub fn ncmContentMetaDatabaseDisableForcibly(db: *mut NcmContentMetaDatabase) -> Result;

    pub fn ncmContentMetaDatabaseLookupOrphanContent(db: *mut NcmContentMetaDatabase, out_orphaned: *mut bool, content_ids: *const NcmContentId, count: s32) -> Result;

    pub fn ncmContentMetaDatabaseCommit(db: *mut NcmContentMetaDatabase) -> Result;

    pub fn ncmContentMetaDatabaseHasContent(db: *mut NcmContentMetaDatabase, out: *mut bool, key: *const NcmContentMetaKey, content_id: *const NcmContentId) -> Result;

    pub fn ncmContentMetaDatabaseListContentMetaInfo(db: *mut NcmContentMetaDatabase, out_entries_written: *mut s32, out_meta_info: *mut c_void, count: s32, key: *const NcmContentMetaKey, start_index: s32) -> Result;

    pub fn ncmContentMetaDatabaseGetAttributes(db: *mut NcmContentMetaDatabase, key: *const NcmContentMetaKey, out: *mut u8) -> Result;

    /// [2.0.0+]
    pub fn ncmContentMetaDatabaseGetRequiredApplicationVersion(db: *mut NcmContentMetaDatabase, out_version: *mut u32, key: *const NcmContentMetaKey) -> Result;

    /// [5.0.0+]
    pub fn ncmContentMetaDatabaseGetContentIdByTypeAndIdOffset(db: *mut NcmContentMetaDatabase, out_content_id: *mut NcmContentId, key: *const NcmContentMetaKey, type_: NcmContentType, id_offset: u8) -> Result;

    /// [17.0.0+]
    pub fn ncmContentMetaDatabaseGetPlatform(db: *mut NcmContentMetaDatabase, out: *mut u8, key: *const NcmContentMetaKey) -> Result;
}
