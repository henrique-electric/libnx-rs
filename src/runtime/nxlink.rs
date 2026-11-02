//! `switch/runtime/nxlink.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// nxlink TCP server port
pub const NXLINK_SERVER_PORT: u32 = 28280;
/// nxlink TCP client port
pub const NXLINK_CLIENT_PORT: u32 = 28771;

/// Same as `nxlinkConnectToHost` but redirecting both stdout/stderr.
/// # Safety
/// Requires the socket service to be initialized.
#[inline(always)]
pub unsafe fn nxlinkStdio() -> c_int {
    unsafe { nxlinkConnectToHost(true, true) }
}

/// Same as `nxlinkConnectToHost` but redirecting only stderr.
/// # Safety
/// Requires the socket service to be initialized.
#[inline(always)]
pub unsafe fn nxlinkStdioForDebug() -> c_int {
    unsafe { nxlinkConnectToHost(false, true) }
}

unsafe extern "C" {
    /// Address of the host connected through nxlink
    pub static mut __nxlink_host: in_addr;

    /// Connects to the nxlink host, setting up an output stream.
    ///
    /// # Arguments
    /// - `redirStdout` (in): Whether to redirect stdout to nxlink output.
    /// - `redirStderr` (in): Whether to redirect stderr to nxlink output.
    ///
    /// **Returns:** Socket fd on success, negative number on failure.
    ///
    /// **Note:** The socket should be closed with close() during application cleanup.
    pub fn nxlinkConnectToHost(redirStdout: bool, redirStderr: bool) -> c_int;
}
