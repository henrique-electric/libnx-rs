//! `switch/kernel/thread.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Thread information structure.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Thread {
    /// Thread handle.
    pub handle: Handle,
    /// Whether the stack memory is automatically allocated.
    pub owns_stack_mem: bool,
    /// Pointer to stack memory.
    pub stack_mem: *mut c_void,
    /// Pointer to stack memory mirror.
    pub stack_mirror: *mut c_void,
    /// Stack size.
    pub stack_sz: usize,
    pub tls_array: *mut *mut c_void,
    pub next: *mut Thread,
    pub prev_next: *mut *mut Thread,
}

/// Creates a `Waiter` for a `Thread`.
#[inline(always)]
pub fn waiterForThread(t: &Thread) -> Waiter {
    waiterForHandle(t.handle)
}

impl_zeroed_default!(Thread);

unsafe extern "C" {
    /// Creates a thread.
    ///
    /// # Arguments
    /// - `t`: Thread information structure which will be filled in.
    /// - `entry`: Entrypoint of the thread.
    /// - `arg`: Argument to pass to the entrypoint.
    /// - `stack_mem`: Memory to use as backing for stack/tls/reent. Must be page-aligned. NULL argument means to allocate new memory.
    /// - `stack_sz`: If stack_mem is NULL, size to use for stack. If stack_mem is non-NULL, size to use for stack + reent + tls (must be page-aligned).
    /// - `prio`: Thread priority (0x00~0x3F); 0x2C is the usual priority of the main thread, 0x3B is a special priority on cores 0..2 that enables preemptive multithreading (0x3F on core 3).
    /// - `cpuid`: ID of the core on which to create the thread (0~3); or -2 to use the default core for the current process.
    ///
    /// **Returns:** Result code.
    pub fn threadCreate(t: *mut Thread, entry: ThreadFunc, arg: *mut c_void, stack_mem: *mut c_void, stack_sz: usize, prio: c_int, cpuid: c_int) -> Result;

    /// Starts the execution of a thread.
    ///
    /// # Arguments
    /// - `t`: Thread information structure.
    ///
    /// **Returns:** Result code.
    pub fn threadStart(t: *mut Thread) -> Result;

    /// Exits the current thread immediately.
    pub fn threadExit() -> !;

    /// Waits for a thread to finish executing.
    ///
    /// # Arguments
    /// - `t`: Thread information structure.
    ///
    /// **Returns:** Result code.
    pub fn threadWaitForExit(t: *mut Thread) -> Result;

    /// Frees up resources associated with a thread.
    ///
    /// # Arguments
    /// - `t`: Thread information structure.
    ///
    /// **Returns:** Result code.
    pub fn threadClose(t: *mut Thread) -> Result;

    /// Pauses the execution of a thread.
    ///
    /// # Arguments
    /// - `t`: Thread information structure.
    ///
    /// **Returns:** Result code.
    pub fn threadPause(t: *mut Thread) -> Result;

    /// Resumes the execution of a thread, after having been paused.
    ///
    /// # Arguments
    /// - `t`: Thread information structure.
    ///
    /// **Returns:** Result code.
    pub fn threadResume(t: *mut Thread) -> Result;

    /// Dumps the registers of a thread paused by `threadPause` (register groups: all).
    ///
    /// # Arguments
    /// - `ctx` (out): Output thread context (register dump).
    /// - `t`: Thread information structure.
    ///
    /// **Returns:** Result code.
    ///
    /// **Warning:** Official kernel will not dump x0..x18 if the thread is currently executing a system call, and prior to 6.0.0 doesn't dump TPIDR_EL0.
    pub fn threadDumpContext(ctx: *mut ThreadContext, t: *mut Thread) -> Result;

    /// Gets a pointer to the current thread structure.
    ///
    /// **Returns:** Thread information structure.
    pub fn threadGetSelf() -> *mut Thread;

    /// Gets the raw handle to the current thread.
    ///
    /// **Returns:** The current thread's handle.
    pub fn threadGetCurHandle() -> Handle;

    /// Allocates a TLS slot.
    ///
    /// # Arguments
    /// - `destructor`: Function to run automatically when a thread exits.
    ///
    /// **Returns:** TLS slot ID on success, or a negative value on failure.
    pub fn threadTlsAlloc(destructor: Option<unsafe extern "C" fn(*mut c_void)>) -> s32;

    /// Retrieves the value stored in a TLS slot.
    ///
    /// # Arguments
    /// - `slot_id`: TLS slot ID.
    ///
    /// **Returns:** Value.
    pub fn threadTlsGet(slot_id: s32) -> *mut c_void;

    /// Stores the specified value into a TLS slot.
    ///
    /// # Arguments
    /// - `slot_id`: TLS slot ID.
    /// - `value`: Value.
    pub fn threadTlsSet(slot_id: s32, value: *mut c_void);

    /// Frees a TLS slot.
    ///
    /// # Arguments
    /// - `slot_id`: TLS slot ID.
    pub fn threadTlsFree(slot_id: s32);
}
