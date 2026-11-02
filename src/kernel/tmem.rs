//! `switch/kernel/tmem.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Transfer memory information structure.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TransferMemory {
    /// Kernel object handle.
    pub handle: Handle,
    /// Size of the transfer memory object.
    pub size: usize,
    /// Permissions of the transfer memory object.
    pub perm: Permission,
    /// Address of the source backing memory.
    pub src_addr: *mut c_void,
    /// Address to which the transfer memory object is mapped.
    pub map_addr: *mut c_void,
}

/// Retrieves the mapped address of a transfer memory object.
///
/// # Arguments
/// - `t`: Transfer memory information structure.
///
/// **Returns:** Mapped address of the transfer memory object.
#[inline(always)]
pub fn tmemGetAddr(t: &TransferMemory) -> *mut c_void {
    t.map_addr
}

impl_zeroed_default!(TransferMemory);

unsafe extern "C" {
    /// Creates a transfer memory object.
    ///
    /// # Arguments
    /// - `t`: Transfer memory information structure that will be filled in.
    /// - `size`: Size of the transfer memory object to create.
    /// - `perm`: Permissions with which to protect the transfer memory in the local process.
    ///
    /// **Returns:** Result code.
    pub fn tmemCreate(t: *mut TransferMemory, size: usize, perm: Permission) -> Result;

    /// Creates a transfer memory object from existing memory.
    ///
    /// # Arguments
    /// - `t`: Transfer memory information structure that will be filled in.
    /// - `buf`: Pointer to a page-aligned buffer.
    /// - `size`: Size of the transfer memory object to create.
    /// - `perm`: Permissions with which to protect the transfer memory in the local process.
    ///
    /// **Returns:** Result code.
    pub fn tmemCreateFromMemory(t: *mut TransferMemory, buf: *mut c_void, size: usize, perm: Permission) -> Result;

    /// Loads a transfer memory object coming from a remote process.
    ///
    /// # Arguments
    /// - `t`: Transfer memory information structure which will be filled in.
    /// - `handle`: Handle of the transfer memory object.
    /// - `size`: Size of the transfer memory object that is being loaded.
    /// - `perm`: Permissions which the transfer memory is expected to have in the process that owns the memory.
    ///
    /// **Warning:** This is a privileged operation; in normal circumstances applications shouldn't use this function.
    pub fn tmemLoadRemote(t: *mut TransferMemory, handle: Handle, size: usize, perm: Permission);

    /// Maps a transfer memory object.
    ///
    /// # Arguments
    /// - `t`: Transfer memory information structure.
    ///
    /// **Returns:** Result code.
    ///
    /// **Warning:** This is a privileged operation; in normal circumstances applications cannot use this function.
    pub fn tmemMap(t: *mut TransferMemory) -> Result;

    /// Unmaps a transfer memory object.
    ///
    /// # Arguments
    /// - `t`: Transfer memory information structure.
    ///
    /// **Returns:** Result code.
    ///
    /// **Warning:** This is a privileged operation; in normal circumstances applications cannot use this function.
    pub fn tmemUnmap(t: *mut TransferMemory) -> Result;

    /// Closes handle of a transfer memory object.
    ///
    /// # Arguments
    /// - `t`: Transfer memory information structure.
    ///
    /// **Returns:** Result code.
    pub fn tmemCloseHandle(t: *mut TransferMemory) -> Result;

    /// Waits until source backing memory permissions match perm.
    ///
    /// # Arguments
    /// - `t`: Transfer memory information structure.
    /// - `perm`: Permissions which the source backing memory is expected to have before return.
    ///
    /// **Returns:** Result code.
    pub fn tmemWaitForPermission(t: *mut TransferMemory, perm: Permission) -> Result;

    /// Frees up resources used by a transfer memory object, unmapping and closing handles, etc.
    ///
    /// # Arguments
    /// - `t`: Transfer memory information structure.
    ///
    /// **Returns:** Result code.
    pub fn tmemClose(t: *mut TransferMemory) -> Result;
}
