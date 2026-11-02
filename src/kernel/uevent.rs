//! `switch/kernel/uevent.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// User-mode event object.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UEvent {
    pub waitable: Waitable,
    pub signal: bool,
    pub auto_clear: bool,
}

/// Creates a waiter for a user-mode event.
#[inline(always)]
pub fn waiterForUEvent(e: &mut UEvent) -> Waiter {
    Waiter { type_: WaiterType_Waitable, anon0: Waiter_anon0 { waitable: &raw mut e.waitable } }
}

impl_zeroed_default!(UEvent);

unsafe extern "C" {
    /// Creates a user-mode event.
    ///
    /// # Arguments
    /// - `e` (out): UEvent object.
    /// - `auto_clear` (in): Whether to automatically clear the event.
    ///
    /// **Note:** It is safe to wait on this event with several threads simultaneously.
    ///
    /// **Note:** If more than one thread is listening on it, at least one thread will get the signal. No other guarantees.
    pub fn ueventCreate(e: *mut UEvent, auto_clear: bool);

    /// Clears the event signal.
    ///
    /// # Arguments
    /// - `e` (in): UEvent object.
    pub fn ueventClear(e: *mut UEvent);

    /// Signals the event.
    ///
    /// # Arguments
    /// - `e` (in): UEvent object.
    pub fn ueventSignal(e: *mut UEvent);
}
