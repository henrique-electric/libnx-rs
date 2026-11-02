//! `switch/kernel/jit.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// JIT implementation type.
pub type JitType = u32;
/// JIT supported using svcSetProcessMemoryPermission
pub const JitType_SetProcessMemoryPermission: JitType = 0;
/// JIT supported using [4.0.0+] CodeMemory syscalls
pub const JitType_CodeMemory: JitType = 1;

#[repr(C)]
#[derive(Clone, Copy)]
pub union Jit_anon0 {
    pub handle: Handle,
    pub rv: *mut VirtmemReservation,
}

/// JIT buffer object.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Jit {
    pub type_: JitType,
    pub size: usize,
    pub src_addr: *mut c_void,
    pub rx_addr: *mut c_void,
    pub rw_addr: *mut c_void,
    pub is_executable: bool,
    pub anon0: Jit_anon0,
}

/// Gets the address of the writable memory alias of a JIT buffer.
///
/// # Arguments
/// - `j`: JIT buffer.
///
/// **Returns:** Pointer to alias of the JIT buffer that can be written to.
#[inline(always)]
pub fn jitGetRwAddr(j: &Jit) -> *mut c_void {
    j.rw_addr
}

/// Gets the address of the executable memory alias of a JIT buffer.
///
/// # Arguments
/// - `j`: JIT buffer.
///
/// **Returns:** Pointer to alias of the JIT buffer that can be executed.
#[inline(always)]
pub fn jitGetRxAddr(j: &Jit) -> *mut c_void {
    j.rx_addr
}

impl_zeroed_default!(Jit_anon0, Jit);

unsafe extern "C" {
    /// Creates a JIT buffer.
    ///
    /// # Arguments
    /// - `j`: JIT buffer.
    /// - `size`: Size of the JIT buffer.
    ///
    /// **Returns:** Result code.
    pub fn jitCreate(j: *mut Jit, size: usize) -> Result;

    /// Transition a JIT buffer to have writable permission.
    ///
    /// # Arguments
    /// - `j`: JIT buffer.
    ///
    /// **Returns:** Result code.
    pub fn jitTransitionToWritable(j: *mut Jit) -> Result;

    /// Transition a JIT buffer to have executable permission.
    ///
    /// # Arguments
    /// - `j`: JIT buffer.
    ///
    /// **Returns:** Result code.
    pub fn jitTransitionToExecutable(j: *mut Jit) -> Result;

    /// Destroys a JIT buffer.
    ///
    /// # Arguments
    /// - `j`: JIT buffer.
    ///
    /// **Returns:** Result code.
    pub fn jitClose(j: *mut Jit) -> Result;
}
