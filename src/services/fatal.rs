//! `switch/services/fatal.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Type of thrown fatal error.
pub type FatalPolicy = u32;
pub const FatalPolicy_ErrorReportAndErrorScreen: FatalPolicy = 0;
pub const FatalPolicy_ErrorReport: FatalPolicy = 1;
/// Only available with [3.0.0+]. If specified, FatalPolicy_ErrorReportAndErrorScreen will be used instead on pre-3.0.0.
pub const FatalPolicy_ErrorScreen: FatalPolicy = 2;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FatalAarch64Context_anon0_anon0 {
    pub _x: [u64; 29],
    pub fp: u64,
    pub lr: u64,
    pub sp: u64,
    pub pc: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union FatalAarch64Context_anon0 {
    pub x: [u64; 32],
    pub anon0: FatalAarch64Context_anon0_anon0,
}

/// Struct for fatal Cpu context, 64-bit.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FatalAarch64Context {
    pub anon0: FatalAarch64Context_anon0,
    pub pstate: u64,
    pub afsr0: u64,
    pub afsr1: u64,
    pub esr: u64,
    pub far: u64,
    pub stack_trace: [u64; 32],
    /// Address of first NSO loaded (generally, process entrypoint).
    pub start_address: u64,
    /// Bitmask, bit i indicates GPR i has a value.
    pub register_set_flags: u64,
    pub stack_trace_size: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FatalAarch32Context_anon0_anon0 {
    pub _r: [u32; 11],
    pub fp: u32,
    pub ip: u32,
    pub sp: u32,
    pub lr: u32,
    pub pc: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union FatalAarch32Context_anon0 {
    pub r: [u32; 16],
    pub anon0: FatalAarch32Context_anon0_anon0,
}

/// Struct for fatal Cpu context, 32-bit.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FatalAarch32Context {
    pub anon0: FatalAarch32Context_anon0,
    pub pstate: u32,
    pub afsr0: u32,
    pub afsr1: u32,
    pub esr: u32,
    pub far: u32,
    pub stack_trace: [u32; 32],
    pub stack_trace_size: u32,
    /// Address of first NSO loaded (generally, process entrypoint).
    pub start_address: u32,
    /// Bitmask, bit i indicates GPR i has a value.
    pub register_set_flags: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union FatalCpuContext_anon0 {
    pub aarch64_ctx: FatalAarch64Context,
    pub aarch32_ctx: FatalAarch32Context,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FatalCpuContext {
    pub anon0: FatalCpuContext_anon0,
    pub is_aarch32: bool,
    pub type_: u32,
}

impl_zeroed_default!(FatalAarch64Context_anon0_anon0, FatalAarch64Context_anon0, FatalAarch64Context, FatalAarch32Context_anon0_anon0, FatalAarch32Context_anon0, FatalAarch32Context, FatalCpuContext_anon0, FatalCpuContext);

unsafe extern "C" {
    /// Triggers a system fatal error.
    ///
    /// # Arguments
    /// - `err` (in): Result code to throw.
    ///
    /// **Note:** This function does not return.
    ///
    /// **Note:** This uses `fatalThrowWithPolicy` with `FatalPolicy_ErrorScreen` internally.
    pub fn fatalThrow(err: Result) -> !;

    /// Triggers a system fatal error with a custom `FatalPolicy`.
    ///
    /// # Arguments
    /// - `err` (in): Result code to throw.
    /// - `type` (in): Type of fatal error to throw.
    ///
    /// **Note:** This function may not return, depending on `FatalPolicy`.
    pub fn fatalThrowWithPolicy(err: Result, type_: FatalPolicy);

    /// Triggers a system fatal error with a custom `FatalPolicy` and `FatalCpuContext`.
    ///
    /// # Arguments
    /// - `err` (in): Result code to throw.
    /// - `type` (in): Type of fatal error to throw.
    /// - `ctx` (in): Cpu context for fatal error to throw.
    ///
    /// **Note:** This function may not return, depending on `FatalPolicy`.
    pub fn fatalThrowWithContext(err: Result, type_: FatalPolicy, ctx: *mut FatalCpuContext);
}
