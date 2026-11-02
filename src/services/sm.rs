//! Service manager (sm) IPC wrapper. (`switch/services/sm.h`)

use core::ffi::c_char;

use crate::sf::service::Service;
use crate::sf::tipc::TipcService;
use crate::types::*;

/// Structure representing a service name (null terminated, remaining characters set to zero).
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SmServiceName {
    pub name: [c_char; 8],
}

/// Converts a service name into a 64-bit integer.
#[inline(always)]
pub const fn smServiceNameToU64(name: SmServiceName) -> u64 {
    // SAFETY: [c_char; 8] and [u8; 8] have the same size and every bit pattern is valid.
    u64::from_ne_bytes(unsafe { core::mem::transmute::<[c_char; 8], [u8; 8]>(name.name) })
}

/// Converts a 64-bit integer into a service name.
#[inline(always)]
pub const fn smServiceNameFromU64(name: u64) -> SmServiceName {
    // SAFETY: as above.
    SmServiceName { name: unsafe { core::mem::transmute::<[u8; 8], [c_char; 8]>(name.to_ne_bytes()) } }
}

/// Checks whether two service names are equal.
#[inline(always)]
pub const fn smServiceNamesAreEqual(a: SmServiceName, b: SmServiceName) -> bool {
    smServiceNameToU64(a) == smServiceNameToU64(b)
}

/// Encodes a service name string as a [`SmServiceName`] structure.
///
/// Like the C version (which uses `strlen`), copying stops at the first NUL or after 8 bytes.
#[inline(always)]
pub const fn smEncodeName(name: &str) -> SmServiceName {
    let bytes = name.as_bytes();
    let mut name_encoded = SmServiceName { name: [0; 8] };
    let mut i = 0;
    while i < 8 && i < bytes.len() && bytes[i] != 0 {
        name_encoded.name[i] = bytes[i] as c_char;
        i += 1;
    }
    name_encoded
}

unsafe extern "C" {
    /// Initializes SM.
    ///
    /// **Note:** This function is already called in the default application startup code (before main() is called).
    pub fn smInitialize() -> Result;

    /// Uninitializes SM.
    ///
    /// **Note:** This function is already handled in the default application exit code (after main() returns).
    pub fn smExit();

    /// Requests a service from SM, allowing overrides.
    ///
    /// # Arguments
    /// - `service_out` (out): Service structure which will be filled in.
    /// - `name` (in): Name of the service to request.
    pub fn smGetServiceWrapper(service_out: *mut Service, name: SmServiceName) -> Result;

    /// Requests a service from SM, as an IPC session handle directly
    ///
    /// # Arguments
    /// - `handle_out` (out): Variable containing IPC session handle.
    /// - `name` (in): Name of the service to request.
    pub fn smGetServiceOriginal(handle_out: *mut Handle, name: SmServiceName) -> Result;

    /// Retrieves an overriden service in the homebrew environment.
    ///
    /// # Arguments
    /// - `name` (in): Name of the service to request.
    pub fn smGetServiceOverride(name: SmServiceName) -> Handle;

    /// Creates and registers a new service within SM.
    ///
    /// # Arguments
    /// - `handle_out` (out): Variable containing IPC port handle.
    /// - `name` (in): Name of the service.
    /// - `is_light` (in): "Is light"
    /// - `max_sessions` (in): Maximum number of concurrent sessions that the service will accept.
    pub fn smRegisterService(handle_out: *mut Handle, name: SmServiceName, is_light: bool, max_sessions: s32) -> Result;

    /// Unregisters a previously registered service in SM.
    ///
    /// # Arguments
    /// - `name` (in): Name of the service.
    pub fn smUnregisterService(name: SmServiceName) -> Result;

    /// Detaches the current SM session.
    ///
    /// **Note:** After this function is called, the rest of the SM API cannot be used.
    ///
    /// **Note:** Only available on [11.0.0-11.0.1], or Atmosphère.
    pub fn smDetachClient() -> Result;

    /// Gets the Service session used to communicate with SM.
    pub fn smGetServiceSession() -> *mut Service;

    /// Gets the TipcService session used to communicate with SM.
    ///
    /// **Note:** Only available on [12.0.0+], or Atmosphère.
    pub fn smGetServiceSessionTipc() -> *mut TipcService;

    /// Overrides a service with a custom IPC service handle.
    ///
    /// # Arguments
    /// - `name` (in): Name of the service.
    /// - `handle` (in): IPC session handle.
    pub fn smAddOverrideHandle(name: SmServiceName, handle: Handle);

    /// Same as [`smRegisterService`], but always using cmif serialization.
    pub fn smRegisterServiceCmif(handle_out: *mut Handle, name: SmServiceName, is_light: bool, max_sessions: s32) -> Result;

    /// Same as [`smRegisterService`], but always using tipc serialization.
    pub fn smRegisterServiceTipc(handle_out: *mut Handle, name: SmServiceName, is_light: bool, max_sessions: s32) -> Result;

    /// Same as [`smUnregisterService`], but always using cmif serialization.
    pub fn smUnregisterServiceCmif(name: SmServiceName) -> Result;

    /// Same as [`smUnregisterService`], but always using tipc serialization.
    pub fn smUnregisterServiceTipc(name: SmServiceName) -> Result;

    /// Same as [`smDetachClient`], but always using cmif serialization.
    pub fn smDetachClientCmif() -> Result;

    /// Same as [`smDetachClient`], but always using tipc serialization.
    pub fn smDetachClientTipc() -> Result;
}

/// Requests a service from SM.
///
/// # Arguments
/// - `service_out` (out): Service structure which will be filled in.
/// - `name` (in): Name of the service to request (as a string).
///
/// # Safety
/// SM must be initialized.
#[inline(always)]
pub unsafe fn smGetService(service_out: &mut Service, name: &str) -> Result {
    unsafe { smGetServiceWrapper(service_out, smEncodeName(name)) }
}
