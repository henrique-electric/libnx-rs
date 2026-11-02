//! `switch/services/async.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// AsyncValue
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AsyncValue {
    /// IAsyncValue
    pub s: Service,
    /// Event with autoclear=false.
    pub event: Event,
}

/// AsyncResult
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AsyncResult {
    /// IAsyncResult
    pub s: Service,
    /// Event with autoclear=false.
    pub event: Event,
}

impl_zeroed_default!(AsyncValue, AsyncResult);

unsafe extern "C" {
    /// Close a `AsyncValue`.
    ///
    /// # Arguments
    /// - `a`: `AsyncValue`
    ///
    /// **Note:** When the object is initialized, this uses `asyncValueCancel` then `asyncValueWait` with timeout=UINT64_MAX.
    pub fn asyncValueClose(a: *mut AsyncValue);

    /// Waits for the async operation to finish using the specified timeout.
    ///
    /// # Arguments
    /// - `a`: `AsyncValue`
    /// - `timeout` (in): Timeout in nanoseconds. UINT64_MAX for no timeout.
    pub fn asyncValueWait(a: *mut AsyncValue, timeout: u64) -> Result;

    /// Gets the value size.
    ///
    /// # Arguments
    /// - `a`: `AsyncValue`
    /// - `size` (out): Output size.
    pub fn asyncValueGetSize(a: *mut AsyncValue, size: *mut u64) -> Result;

    /// Gets the value.
    ///
    /// # Arguments
    /// - `a`: `AsyncValue`
    /// - `buffer` (out): Output buffer.
    /// - `size` (in): Output buffer size.
    ///
    /// **Note:** Prior to using the cmd, this uses `asyncResultWait` with timeout=UINT64_MAX.
    pub fn asyncValueGet(a: *mut AsyncValue, buffer: *mut c_void, size: usize) -> Result;

    /// Cancels the async operation.
    ///
    /// # Arguments
    /// - `a`: `AsyncValue`
    ///
    /// **Note:** Used automatically by `asyncValueClose`.
    pub fn asyncValueCancel(a: *mut AsyncValue) -> Result;

    /// Gets the `ErrorContext`.
    ///
    /// # Arguments
    /// - `a`: `AsyncValue`
    /// - `context` (out): `ErrorContext`
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn asyncValueGetErrorContext(a: *mut AsyncValue, context: *mut ErrorContext) -> Result;

    /// Close a `AsyncResult`.
    ///
    /// # Arguments
    /// - `a`: `AsyncResult`
    ///
    /// **Note:** When the object is initialized, this uses `asyncResultCancel` then `asyncResultWait` with timeout=UINT64_MAX.
    pub fn asyncResultClose(a: *mut AsyncResult);

    /// Waits for the async operation to finish using the specified timeout.
    ///
    /// # Arguments
    /// - `a`: `AsyncResult`
    /// - `timeout` (in): Timeout in nanoseconds. UINT64_MAX for no timeout.
    pub fn asyncResultWait(a: *mut AsyncResult, timeout: u64) -> Result;

    /// Gets the Result.
    ///
    /// # Arguments
    /// - `a`: `AsyncResult`
    ///
    /// **Note:** Prior to using the cmd, this uses `asyncResultWait` with timeout=UINT64_MAX.
    pub fn asyncResultGet(a: *mut AsyncResult) -> Result;

    /// Cancels the async operation.
    ///
    /// # Arguments
    /// - `a`: `AsyncResult`
    ///
    /// **Note:** Used automatically by `asyncResultClose`.
    pub fn asyncResultCancel(a: *mut AsyncResult) -> Result;

    /// Gets the `ErrorContext`.
    ///
    /// # Arguments
    /// - `a`: `AsyncResult`
    /// - `context` (out): `ErrorContext`
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn asyncResultGetErrorContext(a: *mut AsyncResult, context: *mut ErrorContext) -> Result;
}
