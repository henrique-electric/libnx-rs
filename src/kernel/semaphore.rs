//! `switch/kernel/semaphore.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Semaphore structure.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Semaphore {
    /// Condition variable object.
    pub condvar: CondVar,
    /// Mutex object.
    pub mutex: Mutex,
    /// Internal counter.
    pub count: u64,
}

impl_zeroed_default!(Semaphore);

unsafe extern "C" {
    /// Initializes a semaphore and its internal counter.
    ///
    /// # Arguments
    /// - `s`: Semaphore object.
    /// - `initial_count`: initial value for internal counter (typically the # of free resources).
    pub fn semaphoreInit(s: *mut Semaphore, initial_count: u64);

    /// Increments the Semaphore to allow other threads to continue.
    ///
    /// # Arguments
    /// - `s`: Semaphore object.
    pub fn semaphoreSignal(s: *mut Semaphore);

    /// Decrements Semaphore and waits if 0.
    ///
    /// # Arguments
    /// - `s`: Semaphore object.
    pub fn semaphoreWait(s: *mut Semaphore);

    /// Attempts to get lock without waiting.
    ///
    /// # Arguments
    /// - `s`: Semaphore object.
    ///
    /// **Returns:** true if no wait and successful lock, false otherwise.
    pub fn semaphoreTryWait(s: *mut Semaphore) -> bool;
}
