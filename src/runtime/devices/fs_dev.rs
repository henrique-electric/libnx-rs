//! `switch/runtime/devices/fs_dev.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// "fsdv"
pub const FSDEV_DIRITER_MAGIC: u32 = 0x66736476;

/// Open directory struct
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct fsdev_dir_t {
    /// "fsdv"
    pub magic: u32,
    /// File descriptor
    pub fd: FsDir,
    /// Current entry index
    pub index: isize,
    /// Current batch size
    pub size: usize,
}

/// Retrieves a pointer to temporary stage for reading entries
/// Returns the entry buffer that directly follows `dir` in memory.
#[inline(always)]
pub fn fsdevDirGetEntries(dir: *mut fsdev_dir_t) -> *mut FsDirectoryEntry {
    dir.wrapping_add(1).cast()
}

impl_zeroed_default!(fsdev_dir_t);

unsafe extern "C" {
    /// Initializes and mounts the sdmc device if accessible.
    pub fn fsdevMountSdmc() -> Result;

    /// Mounts the specified SaveData.
    pub fn fsdevMountSaveData(name: *const c_char, application_id: u64, uid: AccountUid) -> Result;

    /// Mounts the specified SaveData as ReadOnly.
    /// Only available on [2.0.0+].
    pub fn fsdevMountSaveDataReadOnly(name: *const c_char, application_id: u64, uid: AccountUid) -> Result;

    /// Mounts the specified BcatSaveData.
    pub fn fsdevMountBcatSaveData(name: *const c_char, application_id: u64) -> Result;

    /// Mounts the specified DeviceSaveData.
    pub fn fsdevMountDeviceSaveData(name: *const c_char, application_id: u64) -> Result;

    /// Mounts the TemporaryStorage for the current process.
    /// Only available on [3.0.0+].
    pub fn fsdevMountTemporaryStorage(name: *const c_char) -> Result;

    /// Mounts the specified CacheStorage.
    /// Only available on [3.0.0+].
    pub fn fsdevMountCacheStorage(name: *const c_char, application_id: u64, save_data_index: u16) -> Result;

    /// Mounts the specified SystemSaveData.
    pub fn fsdevMountSystemSaveData(name: *const c_char, save_data_space_id: FsSaveDataSpaceId, system_save_data_id: u64, uid: AccountUid) -> Result;

    /// Mounts the specified SystemBcatSaveData.
    /// Only available on [4.0.0+].
    pub fn fsdevMountSystemBcatSaveData(name: *const c_char, system_save_data_id: u64) -> Result;

    /// Mounts the input fs with the specified device name. fsdev will handle closing the fs when required, including when fsdevMountDevice() fails.
    /// Returns -1 when any errors occur.
    /// Input device name string shouldn't exceed 31 characters, and shouldn't have a trailing colon.
    pub fn fsdevMountDevice(name: *const c_char, fs: FsFileSystem) -> c_int;

    /// Unmounts the specified device.
    pub fn fsdevUnmountDevice(name: *const c_char) -> c_int;

    /// Uses fsFsCommit() with the specified device. This must be used after any savedata-write operations(not just file-write). This should be used after each file-close where file-writing was done.
    /// This is not used automatically at device unmount.
    pub fn fsdevCommitDevice(name: *const c_char) -> Result;

    /// Returns the FsFileSystem for the specified device. Returns NULL when the specified device isn't found.
    pub fn fsdevGetDeviceFileSystem(name: *const c_char) -> *mut FsFileSystem;

    /// Writes the FS-path to outpath (which has buffer size FS_MAX_PATH), for the input path (as used in stdio). The FsFileSystem is also written to device when not NULL.
    pub fn fsdevTranslatePath(path: *const c_char, device: *mut *mut FsFileSystem, outpath: *mut c_char) -> c_int;

    /// This calls fsFsSetConcatenationFileAttribute on the filesystem specified by the input path (as used in stdio).
    pub fn fsdevSetConcatenationFileAttribute(path: *const c_char) -> Result;

    pub fn fsdevIsValidSignedSystemPartitionOnSdCard(name: *const c_char, out: *mut bool) -> Result;

    /// This calls fsFsCreateFile on the filesystem specified by the input path (as used in stdio).
    pub fn fsdevCreateFile(path: *const c_char, size: usize, flags: u32) -> Result;

    /// Recursively deletes the directory specified by the input path (as used in stdio).
    pub fn fsdevDeleteDirectoryRecursively(path: *const c_char) -> Result;

    /// Unmounts all devices and cleans up any resources used by the FS driver.
    pub fn fsdevUnmountAll() -> Result;

    /// Retrieves the last native result code generated during a failed fsdev operation.
    pub fn fsdevGetLastResult() -> Result;
}
