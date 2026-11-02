//! `switch/runtime/resolver.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

unsafe extern "C" {
    /// Fetches the last resolver Switch result code of the current thread.
    pub fn resolverGetLastResult() -> Result;

    /// Retrieves a handle used to cancel the next resolver command on the current thread.
    pub fn resolverGetCancelHandle() -> u32;

    /// Retrieves whether service discovery is enabled for resolver commands on the current thread.
    pub fn resolverGetEnableServiceDiscovery() -> bool;

    /// [5.0.0+] Retrieves whether the DNS cache is used to resolve queries on the current thread (not implemented).
    pub fn resolverGetEnableDnsCache() -> bool;

    /// Enables or disables service discovery for the current thread.
    pub fn resolverSetEnableServiceDiscovery(enable: bool);

    /// [5.0.0+] Enables or disables the usage of the DNS cache on the current thread (not implemented).
    pub fn resolverSetEnableDnsCache(enable: bool);

    /// Cancels a previous resolver command (handle obtained with `resolverGetCancelHandle` prior to calling the command).
    pub fn resolverCancel(handle: u32) -> Result;

    /// [5.0.0+] Removes a hostname from the DNS cache (not implemented).
    pub fn resolverRemoveHostnameFromCache(hostname: *const c_char) -> Result;

    /// [5.0.0+] Removes an IP address from the DNS cache (not implemented).
    pub fn resolverRemoveIpAddressFromCache(ip: u32) -> Result;
}
