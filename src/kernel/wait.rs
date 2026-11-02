//! `switch/kernel/wait.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

#[repr(C)]
#[derive(Debug)]
pub struct WaitableMethods {
    _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WaitableNode {
    pub prev: *mut WaitableNode,
    pub next: *mut WaitableNode,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Waitable {
    pub vt: *const WaitableMethods,
    pub list: WaitableNode,
    pub mutex: Mutex,
}

pub type WaiterType = u32;
pub const WaiterType_Handle: WaiterType = 0;
pub const WaiterType_HandleWithClear: WaiterType = 1;
pub const WaiterType_Waitable: WaiterType = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub union Waiter_anon0 {
    pub handle: Handle,
    pub waitable: *mut Waitable,
}

/// Waiter structure, representing any generic waitable synchronization object; both kernel-mode and user-mode.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Waiter {
    pub type_: WaiterType,
    pub anon0: Waiter_anon0,
}

/// Creates a `Waiter` for a kernel-mode `Handle`.
#[inline(always)]
pub fn waiterForHandle(h: Handle) -> Waiter {
    Waiter { type_: WaiterType_Handle, anon0: Waiter_anon0 { handle: h } }
}

/// Waits on a single generic waitable synchronization object, optionally with a timeout.
///
/// # Arguments
/// - `w` (in): `Waiter` structure.
/// - `timeout` (in): Timeout (in nanoseconds).
/// # Safety
/// `w` must refer to a valid handle or waitable object.
#[inline(always)]
pub unsafe fn waitSingle(w: Waiter, timeout: u64) -> Result {
    let mut idx: s32 = 0;
    unsafe { waitObjects(&mut idx, &w, 1, timeout) }
}

/// Waits for a single kernel synchronization object, optionally with a timeout.
///
/// # Arguments
/// - `h` (in): `Handle` of the object.
/// - `timeout` (in): Timeout (in nanoseconds).
/// # Safety
/// `h` must be a valid synchronization handle.
#[inline(always)]
pub unsafe fn waitSingleHandle(h: Handle, timeout: u64) -> Result {
    let mut idx: s32 = 0;
    unsafe { waitHandles(&mut idx, &h, 1, timeout) }
}

impl_zeroed_default!(WaitableNode, Waitable, Waiter_anon0, Waiter);

/// Helper for [`waitObjects`] that takes the waiters as a slice (replaces the variadic C macro).
///
/// **Note:** The number of objects must not be greater than `MAX_WAIT_OBJECTS`. This is a Horizon kernel limitation.
///
/// # Safety
/// Every waiter must refer to a valid handle or waitable object.
#[inline(always)]
pub unsafe fn waitMulti(idx_out: &mut s32, timeout: u64, objects: &[Waiter]) -> Result {
    unsafe { waitObjects(idx_out, objects.as_ptr(), objects.len() as s32, timeout) }
}

/// Helper for [`waitHandles`] that takes the handles as a slice (replaces the variadic C macro).
///
/// **Note:** The number of objects must not be greater than `MAX_WAIT_OBJECTS`. This is a Horizon kernel limitation.
///
/// # Safety
/// Every handle must be a valid synchronization handle.
#[inline(always)]
pub unsafe fn waitMultiHandle(idx_out: &mut s32, timeout: u64, handles: &[Handle]) -> Result {
    unsafe { waitHandles(idx_out, handles.as_ptr(), handles.len() as s32, timeout) }
}

unsafe extern "C" {
    /// Waits for an arbitrary number of generic waitable synchronization objects, optionally with a timeout.
    ///
    /// # Arguments
    /// - `idx_out` (out): Variable that will received the index of the signalled object.
    /// - `objects` (in): Array containing `Waiter` structures.
    /// - `num_objects` (in): Number of objects in the array.
    /// - `timeout` (in): Timeout (in nanoseconds).
    ///
    /// **Returns:** Result code.
    ///
    /// **Note:** The number of objects must not be greater than `MAX_WAIT_OBJECTS`. This is a Horizon kernel limitation.
    pub fn waitObjects(idx_out: *mut s32, objects: *const Waiter, num_objects: s32, timeout: u64) -> Result;

    /// Waits for an arbitrary number of kernel synchronization objects, optionally with a timeout. This function replaces `svcWaitSynchronization`.
    ///
    /// # Arguments
    /// - `idx_out` (out): Variable that will received the index of the signalled object.
    /// - `handles` (in): Array containing handles.
    /// - `num_handles` (in): Number of handles in the array.
    /// - `timeout` (in): Timeout (in nanoseconds).
    ///
    /// **Returns:** Result code.
    ///
    /// **Note:** The number of objects must not be greater than `MAX_WAIT_OBJECTS`. This is a Horizon kernel limitation.
    pub fn waitHandles(idx_out: *mut s32, handles: *const Handle, num_handles: s32, timeout: u64) -> Result;
}
