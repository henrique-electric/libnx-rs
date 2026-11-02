//! `switch/kernel/rwlock.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Read/write lock structure.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RwLock {
    pub mutex: Mutex,
    pub condvar_reader_wait: CondVar,
    pub condvar_writer_wait: CondVar,
    pub read_lock_count: u32,
    pub read_waiter_count: u32,
    pub write_lock_count: u32,
    pub write_waiter_count: u32,
    pub write_owner_tag: u32,
}

impl_zeroed_default!(RwLock);

unsafe extern "C" {
    /// Initializes the read/write lock.
    ///
    /// # Arguments
    /// - `r`: Read/write lock object.
    pub fn rwlockInit(r: *mut RwLock);

    /// Locks the read/write lock for reading.
    ///
    /// # Arguments
    /// - `r`: Read/write lock object.
    pub fn rwlockReadLock(r: *mut RwLock);

    /// Attempts to lock the read/write lock for reading without waiting.
    ///
    /// # Arguments
    /// - `r`: Read/write lock object.
    ///
    /// **Returns:** 1 if the mutex has been acquired successfully, and 0 on contention.
    pub fn rwlockTryReadLock(r: *mut RwLock) -> bool;

    /// Unlocks the read/write lock for reading.
    ///
    /// # Arguments
    /// - `r`: Read/write lock object.
    pub fn rwlockReadUnlock(r: *mut RwLock);

    /// Locks the read/write lock for writing.
    ///
    /// # Arguments
    /// - `r`: Read/write lock object.
    pub fn rwlockWriteLock(r: *mut RwLock);

    /// Attempts to lock the read/write lock for writing without waiting.
    ///
    /// # Arguments
    /// - `r`: Read/write lock object.
    ///
    /// **Returns:** 1 if the mutex has been acquired successfully, and 0 on contention.
    pub fn rwlockTryWriteLock(r: *mut RwLock) -> bool;

    /// Unlocks the read/write lock for writing.
    ///
    /// # Arguments
    /// - `r`: Read/write lock object.
    pub fn rwlockWriteUnlock(r: *mut RwLock);

    /// Checks if the write lock is held by the current thread.
    ///
    /// # Arguments
    /// - `r`: Read/write lock object.
    ///
    /// **Returns:** 1 if the current hold holds the write lock, and 0 if it does not.
    pub fn rwlockIsWriteLockHeldByCurrentThread(r: *mut RwLock) -> bool;

    /// Checks if the read/write lock is owned by the current thread.
    ///
    /// # Arguments
    /// - `r`: Read/write lock object.
    ///
    /// **Returns:** 1 if the current hold holds the write lock or if it holds read locks acquired
    /// while it held the write lock, and 0 if it does not.
    pub fn rwlockIsOwnedByCurrentThread(r: *mut RwLock) -> bool;
}
