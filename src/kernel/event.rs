//! `switch/kernel/event.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Kernel-mode event structure.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Event {
    /// Read-only event handle
    pub revent: Handle,
    /// Write-only event handle
    pub wevent: Handle,
    /// Autoclear flag
    pub autoclear: bool,
}

/// Creates a `Waiter` for a kernel-mode event.
#[inline(always)]
pub fn waiterForEvent(t: &Event) -> Waiter {
    Waiter {
        type_: if t.autoclear { WaiterType_HandleWithClear } else { WaiterType_Handle },
        anon0: Waiter_anon0 { handle: t.revent },
    }
}

/// Returns whether an `Event` is initialized.
///
/// # Arguments
/// - `t` (in): Pointer to `Event` structure.
///
/// **Returns:** Initialization status.
#[inline(always)]
pub fn eventActive(t: &Event) -> bool {
    t.revent != INVALID_HANDLE
}

impl_zeroed_default!(Event);

unsafe extern "C" {
    /// Creates a kernel-mode event.
    ///
    /// # Arguments
    /// - `t` (out): Pointer to `Event` structure.
    /// - `autoclear` (in): Autoclear flag.
    ///
    /// **Returns:** Result code.
    ///
    /// **Warning:** This is a privileged operation; in normal circumstances applications shouldn't use this function.
    pub fn eventCreate(t: *mut Event, autoclear: bool) -> Result;

    /// Loads a kernel-mode event obtained from IPC.
    ///
    /// # Arguments
    /// - `t` (out): Pointer to `Event` structure.
    /// - `handle` (in): Read-only event handle.
    /// - `autoclear` (in): Autoclear flag.
    pub fn eventLoadRemote(t: *mut Event, handle: Handle, autoclear: bool);

    /// Closes a kernel-mode event.
    ///
    /// # Arguments
    /// - `t` (in): Pointer to `Event` structure.
    pub fn eventClose(t: *mut Event);

    /// Waits on a kernel-mode event.
    ///
    /// # Arguments
    /// - `t` (in): Pointer to `Event` structure.
    /// - `timeout` (in): Timeout in nanoseconds (pass UINT64_MAX to wait indefinitely).
    ///
    /// **Returns:** Result code.
    pub fn eventWait(t: *mut Event, timeout: u64) -> Result;

    /// Signals a kernel-mode event.
    ///
    /// # Arguments
    /// - `t` (in): Pointer to `Event` structure.
    ///
    /// **Returns:** Result code.
    ///
    /// **Note:** This function only works for events initialized with `eventCreate`, it doesn't work with events initialized with `eventLoadRemote`.
    ///
    /// **Warning:** This is a privileged operation; in normal circumstances applications shouldn't use this function.
    pub fn eventFire(t: *mut Event) -> Result;

    /// Clears a kernel-mode event.
    ///
    /// # Arguments
    /// - `t` (in): Pointer to `Event` structure.
    ///
    /// **Returns:** Result code.
    ///
    /// **Note:** This function shouldn't be used on autoclear events.
    pub fn eventClear(t: *mut Event) -> Result;
}
