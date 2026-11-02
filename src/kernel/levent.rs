//! `switch/kernel/levent.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// User-mode light event structure.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LEvent {
    pub counter: u32,
    pub autoclear: bool,
}

/// Initializes a user-mode light event.
///
/// # Arguments
/// - `le` (out): Pointer to `LEvent` structure.
/// - `signaled` (in): Whether the event starts off in signaled state.
/// - `autoclear` (in): Autoclear flag.
#[inline(always)]
pub fn leventInit(le: &mut LEvent, signaled: bool, autoclear: bool) {
    le.counter = if signaled { 2 } else { 0 };
    le.autoclear = autoclear;
}

impl_zeroed_default!(LEvent);

unsafe extern "C" {
    /// Waits on a user-mode light event.
    ///
    /// # Arguments
    /// - `le` (in): Pointer to `LEvent` structure.
    /// - `timeout_ns` (in): Timeout in nanoseconds (pass UINT64_MAX to wait indefinitely).
    ///
    /// **Returns:** true if wait succeeded, false if wait timed out.
    pub fn leventWait(le: *mut LEvent, timeout_ns: u64) -> bool;

    /// Polls a user-mode light event.
    ///
    /// # Arguments
    /// - `le` (in): Pointer to `LEvent` structure.
    ///
    /// **Returns:** true if event is signaled, false otherwise.
    pub fn leventTryWait(le: *mut LEvent) -> bool;

    /// Signals a user-mode light event.
    ///
    /// # Arguments
    /// - `le` (in): Pointer to `LEvent` structure.
    pub fn leventSignal(le: *mut LEvent);

    /// Clears a user-mode light event.
    ///
    /// # Arguments
    /// - `le` (in): Pointer to `LEvent` structure.
    pub fn leventClear(le: *mut LEvent);
}
