//! `switch/services/ins.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

unsafe extern "C" {
    /// Initialize ins:r.
    pub fn insrInitialize() -> Result;

    /// Exit ins:r.
    pub fn insrExit();

    /// Gets the Service object for the actual ins:r service session.
    pub fn insrGetServiceSession() -> *mut Service;

    /// Retrieves the last system tick the event corresponding to the ID was signaled at.
    ///
    /// # Arguments
    /// - `id` (in): Ins request ID (should be 0..4).
    /// - `tick` (out): .
    ///
    /// **Returns:** Result code.
    ///
    /// **Note:** The tick is only updated once per second at minimum.
    pub fn insrGetLastTick(id: u32, tick: *mut u64) -> Result;

    /// Retrieves the event corresponding to the ID.
    ///
    /// # Arguments
    /// - `id` (in): Ins request ID (should be 0..4).
    /// - `out` (out): .
    ///
    /// **Returns:** Result code.
    ///
    /// **Note:** The event is only signaled once per second at minimum.
    pub fn insrGetReadableEvent(id: u32, out: *mut Event) -> Result;

    /// Initialize ins:s.
    pub fn inssInitialize() -> Result;

    /// Exit ins:s.
    pub fn inssExit();

    /// Gets the Service object for the actual ins:s service session.
    pub fn inssGetServiceSession() -> *mut Service;

    /// Retrieves the event corresponding to the ID.
    ///
    /// # Arguments
    /// - `id` (in): Ins send ID (should be 0..11).
    /// - `out` (out): .
    ///
    /// **Returns:** Result code.
    ///
    /// **Note:** The returned event cannot be waited on, only signaled. Clearing is handled by the service.
    pub fn inssGetWritableEvent(id: u32, out: *mut Event) -> Result;
}
