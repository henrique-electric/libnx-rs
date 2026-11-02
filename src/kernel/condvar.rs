//! `switch/kernel/condvar.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Condition variable.
pub type CondVar = u32;

/// Initializes a condition variable.
///
/// # Arguments
/// - `c` (in): Condition variable object.
#[inline(always)]
pub fn condvarInit(c: &mut CondVar) {
    *c = 0;
}

/// Waits on a condition variable.
///
/// # Arguments
/// - `c` (in): Condition variable object.
/// - `m` (in): Mutex object to use inside the condition variable.
///
/// **Returns:** Result code.
///
/// **Remark:** On function return, the underlying mutex is acquired.
/// # Safety
/// `m` must be locked by the calling thread.
#[inline(always)]
pub unsafe fn condvarWait(c: &mut CondVar, m: &mut Mutex) -> Result {
    unsafe { condvarWaitTimeout(c, m, u64::MAX) }
}

/// Wakes up up to the specified number of threads waiting on a condition variable.
///
/// # Arguments
/// - `c` (in): Condition variable object.
/// - `num` (in): Maximum number of threads to wake up (or -1 to wake them all up).
///
/// **Returns:** Result code.
/// # Safety
/// `c` must be a valid condition variable.
#[inline(always)]
pub unsafe fn condvarWake(c: &mut CondVar, num: c_int) -> Result {
    unsafe { svcSignalProcessWideKey(c, num) };
    0
}

/// Wakes up a single thread waiting on a condition variable.
///
/// # Arguments
/// - `c` (in): Condition variable object.
///
/// **Returns:** Result code.
/// # Safety
/// See [`condvarWake`].
#[inline(always)]
pub unsafe fn condvarWakeOne(c: &mut CondVar) -> Result {
    unsafe { condvarWake(c, 1) }
}

/// Wakes up all thread waiting on a condition variable.
///
/// # Arguments
/// - `c` (in): Condition variable object.
///
/// **Returns:** Result code.
/// # Safety
/// See [`condvarWake`].
#[inline(always)]
pub unsafe fn condvarWakeAll(c: &mut CondVar) -> Result {
    unsafe { condvarWake(c, -1) }
}

unsafe extern "C" {
    /// Waits on a condition variable with a timeout.
    ///
    /// # Arguments
    /// - `c` (in): Condition variable object.
    /// - `m` (in): Mutex object to use inside the condition variable.
    /// - `timeout` (in): Timeout in nanoseconds.
    ///
    /// **Returns:** Result code (0xEA01 on timeout).
    ///
    /// **Remark:** On function return, the underlying mutex is acquired.
    pub fn condvarWaitTimeout(c: *mut CondVar, m: *mut Mutex, timeout: u64) -> Result;
}
