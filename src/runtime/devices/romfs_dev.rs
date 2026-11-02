//! `switch/runtime/devices/romfs_dev.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// RomFS header.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct romfs_header {
    /// Size of the header.
    pub headerSize: u64,
    /// Offset of the directory hash table.
    pub dirHashTableOff: u64,
    /// Size of the directory hash table.
    pub dirHashTableSize: u64,
    /// Offset of the directory table.
    pub dirTableOff: u64,
    /// Size of the directory table.
    pub dirTableSize: u64,
    /// Offset of the file hash table.
    pub fileHashTableOff: u64,
    /// Size of the file hash table.
    pub fileHashTableSize: u64,
    /// Offset of the file table.
    pub fileTableOff: u64,
    /// Size of the file table.
    pub fileTableSize: u64,
    /// Offset of the file data.
    pub fileDataOff: u64,
}

/// RomFS directory.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct romfs_dir {
    /// Offset of the parent directory.
    pub parent: u32,
    /// Offset of the next sibling directory.
    pub sibling: u32,
    /// Offset of the first child directory.
    pub childDir: u32,
    /// Offset of the first file.
    pub childFile: u32,
    /// Directory hash table pointer.
    pub nextHash: u32,
    /// Name length.
    pub nameLen: u32,
    /// Name. (UTF-8)
    pub name: [u8; 0],
}

/// RomFS file.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct romfs_file {
    /// Offset of the parent directory.
    pub parent: u32,
    /// Offset of the next sibling file.
    pub sibling: u32,
    /// Offset of the file's data.
    pub dataOff: u64,
    /// Length of the file's data.
    pub dataSize: u64,
    /// File hash table pointer.
    pub nextHash: u32,
    /// Name length.
    pub nameLen: u32,
    /// Name. (UTF-8)
    pub name: [u8; 0],
}

/// Wrapper for `romfsMountSelf` with the default "romfs" device name.
/// # Safety
/// Requires the fs service to be initialized.
#[inline(always)]
pub unsafe fn romfsInit() -> Result {
    unsafe { romfsMountSelf(c"romfs".as_ptr()) }
}

/// Wrapper for `romfsUnmount` with the default "romfs" device name.
/// # Safety
/// `romfs:` must have been mounted with [`romfsInit`].
#[inline(always)]
pub unsafe fn romfsExit() -> Result {
    unsafe { romfsUnmount(c"romfs".as_ptr()) }
}

impl_zeroed_default!(romfs_header, romfs_dir, romfs_file);

unsafe extern "C" {
    /// Mounts the Application's RomFS.
    ///
    /// # Arguments
    /// - `name`: Device mount name.
    ///
    /// **Remark:** This function is intended to be used to access one's own RomFS.
    /// If the application is running as NRO, it mounts the embedded RomFS section inside the NRO.
    /// If on the other hand it's an NSO, it behaves identically to `romfsMountFromCurrentProcess`.
    pub fn romfsMountSelf(name: *const c_char) -> Result;

    /// Mounts RomFS from an open file.
    ///
    /// # Arguments
    /// - `file`: FsFile of the RomFS image.
    /// - `offset`: Offset of the RomFS within the file.
    /// - `name`: Device mount name.
    pub fn romfsMountFromFile(file: FsFile, offset: u64, name: *const c_char) -> Result;

    /// Mounts RomFS from an open storage.
    ///
    /// # Arguments
    /// - `storage`: FsStorage of the RomFS image.
    /// - `offset`: Offset of the RomFS within the storage.
    /// - `name`: Device mount name.
    pub fn romfsMountFromStorage(storage: FsStorage, offset: u64, name: *const c_char) -> Result;

    /// Mounts RomFS using the current process host program RomFS.
    ///
    /// # Arguments
    /// - `name`: Device mount name.
    pub fn romfsMountFromCurrentProcess(name: *const c_char) -> Result;

    /// Mounts RomFS of a running program.
    ///
    /// # Arguments
    /// - `program_id`: ProgramId to mount.
    /// - `name`: Device mount name.
    ///
    /// **Note:** Permission needs to be set in the NPDM.
    pub fn romfsMountDataStorageFromProgram(program_id: u64, name: *const c_char) -> Result;

    /// Mounts RomFS from a file path in a mounted fsdev device.
    ///
    /// # Arguments
    /// - `path`: File path.
    /// - `offset`: Offset of the RomFS within the file.
    /// - `name`: Device mount name.
    pub fn romfsMountFromFsdev(path: *const c_char, offset: u64, name: *const c_char) -> Result;

    /// Mounts RomFS from SystemData.
    ///
    /// # Arguments
    /// - `dataId`: SystemDataId to mount.
    /// - `storageId`: Storage ID to mount from.
    /// - `name`: Device mount name.
    pub fn romfsMountFromDataArchive(dataId: u64, storageId: NcmStorageId, name: *const c_char) -> Result;

    /// Unmounts the RomFS device.
    pub fn romfsUnmount(name: *const c_char) -> Result;
}
