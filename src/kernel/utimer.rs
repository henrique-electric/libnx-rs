//! `switch/kernel/utimer.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Valid types for a user-mode timer.
pub type TimerType = u32;
/// Timers of this kind fire once and then stop automatically.
pub const TimerType_OneShot: TimerType = 0;
/// Timers of this kind fire periodically.
pub const TimerType_Repeating: TimerType = 1;

/// User-mode timer object.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UTimer {
    pub waitable: Waitable,
    pub _bitfield_0: [u8; 2],
    pub next_tick: u64,
    pub interval: u64,
}

impl UTimer {
    bitfields!(_bitfield_0 {
        type_ / set_type_: 0, 8 => TimerType;
        started / set_started: 8, 1 => bool;
    });
}

/// Creates a waiter for a user-mode timer.
#[inline(always)]
pub fn waiterForUTimer(t: &mut UTimer) -> Waiter {
    Waiter { type_: WaiterType_Waitable, anon0: Waiter_anon0 { waitable: &raw mut t.waitable } }
}

impl_zeroed_default!(UTimer);

unsafe extern "C" {
    /// Creates a user-mode timer.
    ///
    /// # Arguments
    /// - `t` (out): UTimer object.
    /// - `interval` (in): Interval (in nanoseconds).
    /// - `type` (in): Type of timer to create (see `TimerType`).
    ///
    /// **Note:** The timer is stopped when it is created. Use `utimerStart` to start it.
    ///
    /// **Note:** It is safe to wait on this timer with several threads simultaneously.
    ///
    /// **Note:** If more than one thread is listening on it, at least one thread will get the signal. No other guarantees.
    ///
    /// **Note:** For a repeating timer: If the timer triggers twice before you wait on it, you will only get one signal.
    pub fn utimerCreate(t: *mut UTimer, interval: u64, type_: TimerType);

    /// Starts the timer.
    ///
    /// # Arguments
    /// - `t` (in): UTimer object.
    pub fn utimerStart(t: *mut UTimer);

    /// Stops the timer.
    ///
    /// # Arguments
    /// - `t` (in): UTimer object.
    pub fn utimerStop(t: *mut UTimer);
}
