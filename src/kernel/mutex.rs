//! `switch/kernel/mutex.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Mutex datatype, defined in newlib.
pub type Mutex = _LOCK_T;

/// Recursive mutex datatype, defined in newlib.
pub type RMutex = _LOCK_RECURSIVE_T;

/// Initializes a mutex.
///
/// # Arguments
/// - `m`: Mutex object.
///
/// **Note:** A mutex can also be statically initialized by assigning 0 to it.
#[inline(always)]
pub fn mutexInit(m: &mut Mutex) {
    *m = INVALID_HANDLE;
}

/// Initializes a recursive mutex.
///
/// # Arguments
/// - `m`: Recursive mutex object.
///
/// **Note:** A recursive mutex can also be statically initialized by assigning {0,0,0} to it.
#[inline(always)]
pub fn rmutexInit(m: &mut RMutex) {
    m.lock = 0;
    m.counter = 0;
}

unsafe extern "C" {
    /// Locks a mutex.
    ///
    /// # Arguments
    /// - `m`: Mutex object.
    pub fn mutexLock(m: *mut Mutex);

    /// Attempts to lock a mutex without waiting.
    ///
    /// # Arguments
    /// - `m`: Mutex object.
    ///
    /// **Returns:** 1 if the mutex has been acquired successfully, and 0 on contention.
    pub fn mutexTryLock(m: *mut Mutex) -> bool;

    /// Unlocks a mutex.
    ///
    /// # Arguments
    /// - `m`: Mutex object.
    pub fn mutexUnlock(m: *mut Mutex);

    /// Gets whether the current thread owns the mutex.
    ///
    /// # Arguments
    /// - `m`: Mutex object.
    ///
    /// **Returns:** 1 if the mutex is locked by the current thread, and 0 otherwise.
    pub fn mutexIsLockedByCurrentThread(m: *const Mutex) -> bool;

    /// Locks a recursive mutex.
    ///
    /// # Arguments
    /// - `m`: Recursive mutex object.
    pub fn rmutexLock(m: *mut RMutex);

    /// Attempts to lock a recursive mutex without waiting.
    ///
    /// # Arguments
    /// - `m`: Recursive mutex object.
    ///
    /// **Returns:** 1 if the mutex has been acquired successfully, and 0 on contention.
    pub fn rmutexTryLock(m: *mut RMutex) -> bool;

    /// Unlocks a recursive mutex.
    ///
    /// # Arguments
    /// - `m`: Recursive mutex object.
    pub fn rmutexUnlock(m: *mut RMutex);
}
