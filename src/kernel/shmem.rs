//! `switch/kernel/shmem.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Shared memory information structure.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SharedMemory {
    /// Kernel object handle.
    pub handle: Handle,
    /// Size of the shared memory object.
    pub size: usize,
    /// Permissions.
    pub perm: Permission,
    /// Address to which the shared memory object is mapped.
    pub map_addr: *mut c_void,
}

/// Retrieves the mapped address of a shared memory object.
///
/// # Arguments
/// - `s`: Shared memory information structure.
///
/// **Returns:** Mapped address of the shared memory object.
#[inline(always)]
pub fn shmemGetAddr(s: &SharedMemory) -> *mut c_void {
    s.map_addr
}

impl_zeroed_default!(SharedMemory);

unsafe extern "C" {
    /// Creates a shared memory object.
    ///
    /// # Arguments
    /// - `s`: Shared memory information structure which will be filled in.
    /// - `size`: Size of the shared memory object to create.
    /// - `local_perm`: Permissions with which the shared memory object will be mapped in the local process.
    /// - `remote_perm`: Permissions with which the shared memory object will be mapped in the remote process (can be Perm_DontCare).
    ///
    /// **Returns:** Result code.
    ///
    /// **Warning:** This is a privileged operation; in normal circumstances applications cannot use this function.
    pub fn shmemCreate(s: *mut SharedMemory, size: usize, local_perm: Permission, remote_perm: Permission) -> Result;

    /// Loads a shared memory object coming from a remote process.
    ///
    /// # Arguments
    /// - `s`: Shared memory information structure which will be filled in.
    /// - `handle`: Handle of the shared memory object.
    /// - `size`: Size of the shared memory object that is being loaded.
    /// - `perm`: Permissions with which the shared memory object will be mapped in the local process.
    pub fn shmemLoadRemote(s: *mut SharedMemory, handle: Handle, size: usize, perm: Permission);

    /// Maps a shared memory object.
    ///
    /// # Arguments
    /// - `s`: Shared memory information structure.
    ///
    /// **Returns:** Result code.
    pub fn shmemMap(s: *mut SharedMemory) -> Result;

    /// Unmaps a shared memory object.
    ///
    /// # Arguments
    /// - `s`: Shared memory information structure.
    ///
    /// **Returns:** Result code.
    pub fn shmemUnmap(s: *mut SharedMemory) -> Result;

    /// Frees up resources used by a shared memory object, unmapping and closing handles, etc.
    ///
    /// # Arguments
    /// - `s`: Shared memory information structure.
    ///
    /// **Returns:** Result code.
    pub fn shmemClose(s: *mut SharedMemory) -> Result;
}
