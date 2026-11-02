//! `switch/runtime/devices/socket.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// BSD service type used by the socket driver.
pub type BsdServiceType = u32;
/// Uses bsd:u (default).
pub const BsdServiceType_User: BsdServiceType = BIT(0);
/// Uses bsd:s.
pub const BsdServiceType_System: BsdServiceType = BIT(1);
/// Tries to use bsd:s first, and if that fails uses bsd:u (official software behavior).
pub const BsdServiceType_Auto: BsdServiceType = BsdServiceType_User | BsdServiceType_System;

/// Configuration structure for socketInitalize
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SocketInitConfig {
    /// Size of the TCP transfer (send) buffer (initial or fixed).
    pub tcp_tx_buf_size: u32,
    /// Size of the TCP receive buffer (initial or fixed).
    pub tcp_rx_buf_size: u32,
    /// Maximum size of the TCP transfer (send) buffer. If it is 0, the size of the buffer is fixed to its initial value.
    pub tcp_tx_buf_max_size: u32,
    /// Maximum size of the TCP receive buffer. If it is 0, the size of the buffer is fixed to its initial value.
    pub tcp_rx_buf_max_size: u32,
    /// Size of the UDP transfer (send) buffer (typically 0x2400 bytes).
    pub udp_tx_buf_size: u32,
    /// Size of the UDP receive buffer (typically 0xA500 bytes).
    pub udp_rx_buf_size: u32,
    /// Number of buffers for each socket (standard values range from 1 to 8).
    pub sb_efficiency: u32,
    /// Number of BSD service sessions (typically 3).
    pub num_bsd_sessions: u32,
    /// BSD service type (typically `BsdServiceType_User`).
    pub bsd_service_type: BsdServiceType,
}

/// Initalize the socket driver using the default configuration.
/// # Safety
/// Must not be called while the socket service is already initialized.
#[inline(always)]
pub unsafe fn socketInitializeDefault() -> Result {
    unsafe { socketInitialize(core::ptr::null()) }
}

impl_zeroed_default!(SocketInitConfig);

unsafe extern "C" {
    /// Fetch the default configuration for the socket driver.
    pub fn socketGetDefaultInitConfig() -> *const SocketInitConfig;

    /// Initalize the socket driver.
    pub fn socketInitialize(config: *const SocketInitConfig) -> Result;

    /// Fetch the last bsd:u/s Switch result code (thread-local).
    pub fn socketGetLastResult() -> Result;

    /// Deinitialize the socket driver.
    pub fn socketExit();

    /// Wrapper for `sslConnectionSetSocketDescriptor`. Returns the output sockfd on success and -1 on error. errno==ENOENT indicates that no sockfd was returned, this error must be ignored.
    pub fn socketSslConnectionSetSocketDescriptor(c: *mut SslConnection, sockfd: c_int) -> c_int;

    /// Wrapper for `sslConnectionGetSocketDescriptor`. Returns the output sockfd on success and -1 on error.
    pub fn socketSslConnectionGetSocketDescriptor(c: *mut SslConnection) -> c_int;

    /// Wrapper for `nifmRequestRegisterSocketDescriptor`. Returns 0 on success and -1 on error.
    pub fn socketNifmRequestRegisterSocketDescriptor(r: *mut NifmRequest, sockfd: c_int) -> c_int;

    /// Wrapper for `nifmRequestUnregisterSocketDescriptor`. Returns 0 on success and -1 on error.
    pub fn socketNifmRequestUnregisterSocketDescriptor(r: *mut NifmRequest, sockfd: c_int) -> c_int;
}
