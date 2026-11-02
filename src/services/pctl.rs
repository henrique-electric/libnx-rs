//! `switch/services/pctl.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PctlRestrictionSettings {
    pub rating_age: u8,
    pub sns_post_restriction: bool,
    pub free_communication_restriction: bool,
}

impl_zeroed_default!(PctlRestrictionSettings);

unsafe extern "C" {
    /// Initialize pctl.
    pub fn pctlInitialize() -> Result;

    /// Exit pctl.
    pub fn pctlExit();

    /// Gets the Service object for the actual pctl service session.
    pub fn pctlGetServiceSession() -> *mut Service;

    /// Gets the Service object for IParentalControlService.
    pub fn pctlGetServiceSession_Service() -> *mut Service;

    /// Gets whether Parental Controls restrictions are temporarily unlocked.
    pub fn pctlIsRestrictionTemporaryUnlocked(flag: *mut bool) -> Result;

    /// Confirm whether VrMode is allowed. Only available with [4.0.0+].
    pub fn pctlConfirmStereoVisionPermission() -> Result;

    /// Gets whether Parental Controls are enabled.
    pub fn pctlIsRestrictionEnabled(flag: *mut bool) -> Result;

    /// Gets whether Parental Controls are enabled.
    pub fn pctlGetSafetyLevel(safety_level: *mut u32) -> Result;

    /// Returns the current restrictions settings.
    pub fn pctlGetCurrentSettings(settings: *mut PctlRestrictionSettings) -> Result;

    /// Gets the count of applications that have free communication.
    pub fn pctlGetFreeCommunicationApplicationListCount(count: *mut u32) -> Result;

    /// Reset the confirmation done by `pctlConfirmStereoVisionPermission`. Only available with [5.0.0+].
    pub fn pctlResetConfirmedStereoVisionPermission() -> Result;

    /// Gets whether VrMode is allowed. Only available with [5.0.0+].
    pub fn pctlIsStereoVisionPermitted(flag: *mut bool) -> Result;

    /// Confirm whether pairing is active.
    pub fn pctlIsPairingActive(flag: *mut bool) -> Result;

    /// Returns the synchronization event.
    pub fn pctlGetSynchronizationEvent(out_event: *mut Event) -> Result;

    /// Returns the supension event.
    pub fn pctlGetPlayTimerEventToRequestSuspension(out_event: *mut Event) -> Result;

    /// Confirm whether play timer alarm is disabled. Only available with [4.0.0+].
    pub fn pctlIsPlayTimerAlarmDisabled(flag: *mut bool) -> Result;

    /// Returns the unlinked event.
    pub fn pctlGetUnlinkedEvent(out_event: *mut Event) -> Result;
}
