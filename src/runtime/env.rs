//! `switch/runtime/env.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Structure representing an entry in the homebrew environment configuration.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ConfigEntry {
    /// Type of entry
    pub Key: u32,
    /// Entry flags
    pub Flags: u32,
    /// Entry arguments (type-specific)
    pub Value: [u64; 2],
}

/// Entry flags
/// Specifies that the entry **must** be processed by the homebrew application.
pub const EntryFlag_IsMandatory: u32 = BIT(0);

/// Entry list terminator.
pub const EntryType_EndOfList: u32 = 0;
/// Provides the handle to the main thread.
pub const EntryType_MainThreadHandle: u32 = 1;
/// Provides a buffer containing information about the next homebrew application to load.
pub const EntryType_NextLoadPath: u32 = 2;
/// Provides heap override information.
pub const EntryType_OverrideHeap: u32 = 3;
/// Provides service override information.
pub const EntryType_OverrideService: u32 = 4;
/// Provides argv.
pub const EntryType_Argv: u32 = 5;
/// Provides syscall availability hints (SVCs 0x00..0x7F).
pub const EntryType_SyscallAvailableHint: u32 = 6;
/// Provides APT applet type.
pub const EntryType_AppletType: u32 = 7;
/// Indicates that APT is broken and should not be used.
pub const EntryType_AppletWorkaround: u32 = 8;
/// Unused/reserved entry type, formerly used by StdioSockets.
pub const EntryType_Reserved9: u32 = 9;
/// Provides the process handle.
pub const EntryType_ProcessHandle: u32 = 10;
/// Provides the last load result.
pub const EntryType_LastLoadResult: u32 = 11;
/// Provides random data used to seed the pseudo-random number generator.
pub const EntryType_RandomSeed: u32 = 14;
/// Provides persistent storage for the preselected user id.
pub const EntryType_UserIdStorage: u32 = 15;
/// Provides the currently running Horizon OS version.
pub const EntryType_HosVersion: u32 = 16;
/// Provides syscall availability hints (SVCs 0x80..0xBF).
pub const EntryType_SyscallAvailableHint2: u32 = 17;

/// Use AppletType_Application instead of AppletType_SystemApplication.
pub const EnvAppletFlags_ApplicationOverride: u32 = BIT(0);

/// Loader return function.
pub type LoaderReturnFn = Option<unsafe extern "C" fn(c_int)>;

impl_zeroed_default!(ConfigEntry);

unsafe extern "C" {
    /// Parses the homebrew loader environment block (internally called).
    ///
    /// # Arguments
    /// - `ctx`: Reserved.
    /// - `main_thread`: Reserved.
    /// - `saved_lr`: Reserved.
    pub fn envSetup(ctx: *mut c_void, main_thread: Handle, saved_lr: LoaderReturnFn);

    /// Returns information text about the loader, if present.
    pub fn envGetLoaderInfo() -> *const c_char;

    /// Returns the size of the loader information text.
    pub fn envGetLoaderInfoSize() -> u64;

    /// Retrieves the handle to the main thread.
    pub fn envGetMainThreadHandle() -> Handle;

    /// Returns true if the application is running as NSO, otherwise NRO.
    pub fn envIsNso() -> bool;

    /// Returns true if the environment has a heap override.
    pub fn envHasHeapOverride() -> bool;

    /// Returns the address of the overriden heap.
    pub fn envGetHeapOverrideAddr() -> *mut c_void;

    /// Returns the size of the overriden heap.
    pub fn envGetHeapOverrideSize() -> u64;

    /// Returns true if the environment has an argv array.
    pub fn envHasArgv() -> bool;

    /// Returns the pointer to the argv array.
    pub fn envGetArgv() -> *mut c_void;

    /// Returns whether a syscall is hinted to be available.
    ///
    /// # Arguments
    /// - `svc`: Syscall number to test.
    ///
    /// **Returns:** true if the syscall is available.
    pub fn envIsSyscallHinted(svc: c_uint) -> bool;

    /// Returns the handle to the running homebrew process.
    pub fn envGetOwnProcessHandle() -> Handle;

    /// Returns the loader's return function, to be called on program exit.
    pub fn envGetExitFuncPtr() -> LoaderReturnFn;

    /// Sets the return function to be called on program exit.
    pub fn envSetExitFuncPtr(addr: LoaderReturnFn);

    /// Configures the next homebrew application to load.
    ///
    /// # Arguments
    /// - `path`: Path to the next homebrew application to load (.nro).
    /// - `argv`: Argument string to pass.
    pub fn envSetNextLoad(path: *const c_char, argv: *const c_char) -> Result;

    /// Returns true if the environment supports envSetNextLoad.
    pub fn envHasNextLoad() -> bool;

    /// Returns the Result from the last NRO.
    pub fn envGetLastLoadResult() -> Result;

    /// Returns true if the environment provides a random seed.
    pub fn envHasRandomSeed() -> bool;

    /// Retrieves the random seed provided by the environment.
    ///
    /// # Arguments
    /// - `out`: Pointer to a u64\[2\] buffer which will contain the random seed on return.
    pub fn envGetRandomSeed(out: *mut u64);

    /// Returns a pointer to the user id storage area (if present).
    pub fn envGetUserIdStorage() -> *mut AccountUid;
}
