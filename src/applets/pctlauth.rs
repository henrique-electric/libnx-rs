//! `switch/applets/pctlauth.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Type values for PctlAuthArg::type.
pub type PctlAuthType = u32;
/// ShowParentalAuthentication
pub const PctlAuthType_Show: PctlAuthType = 0;
/// RegisterParentalPasscode
pub const PctlAuthType_RegisterPasscode: PctlAuthType = 1;
/// ChangeParentalPasscode
pub const PctlAuthType_ChangePasscode: PctlAuthType = 2;

/// Input arg storage for the applet.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PctlAuthArg {
    /// Always set to 0 by the user-process.
    pub unk_x0: u32,
    /// **Ref:** PctlAuthType
    pub type_: PctlAuthType,
    /// Arg0
    pub arg0: u8,
    /// Arg1
    pub arg1: u8,
    /// Arg2
    pub arg2: u8,
    /// Padding
    pub pad: u8,
}

impl_zeroed_default!(PctlAuthArg);

unsafe extern "C" {
    /// Launches the applet.
    ///
    /// # Arguments
    /// - `flag`: Input flag. false = temporarily disable Parental Controls. true = validate the input PIN.
    ///
    /// **Note:** Should not be used if a PIN is not already registered. See `pctlIsRestrictionEnabled`.
    pub fn pctlauthShow(flag: bool) -> Result;

    /// Launches the applet. Only available with [4.0.0+].
    ///
    /// # Arguments
    /// - `arg0`: Value for PctlAuthArg.arg0.
    /// - `arg1`: Value for PctlAuthArg.arg1.
    /// - `arg2`: Value for PctlAuthArg.arg2.
    pub fn pctlauthShowEx(arg0: u8, arg1: u8, arg2: u8) -> Result;

    /// Just calls: pctlauthShowEx(1, 0, 1). Launches the applet for checking the PIN, used when changing system-settings.
    ///
    /// **Note:** Should not be used if a PIN is not already registered. See `pctlIsRestrictionEnabled`.
    pub fn pctlauthShowForConfiguration() -> Result;

    /// Launches the applet for registering the Parental Controls PIN.
    pub fn pctlauthRegisterPasscode() -> Result;

    /// Launches the applet for changing the Parental Controls PIN.
    ///
    /// **Note:** Should not be used if a PIN is not already registered. See `pctlIsRestrictionEnabled`.
    pub fn pctlauthChangePasscode() -> Result;
}
