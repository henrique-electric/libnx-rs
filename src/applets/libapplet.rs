//! `switch/applets/libapplet.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// CommonArguments
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LibAppletArgs {
    /// **Ref:** libappletArgsCreate sets this to 1, and `libappletArgsPop` requires value 1. v0 is not supported.
    pub CommonArgs_version: u32,
    /// Size of this struct.
    pub CommonArgs_size: u32,
    /// LibraryApplet API version.
    pub LaVersion: u32,
    /// Set to the output from `appletGetThemeColorType` by `libappletArgsCreate`.
    pub ExpectedThemeColor: s32,
    /// bool flag, default is false.
    pub PlayStartupSound: u8,
    /// Padding.
    pub pad: [u8; 7],
    /// System tick. Set to the output from `armGetSystemTick` during `libappletArgsPush`.
    pub tick: u64,
}

impl_zeroed_default!(LibAppletArgs);

unsafe extern "C" {
    /// Creates a LibAppletArgs struct.
    ///
    /// # Arguments
    /// - `a`: LibAppletArgs struct.
    /// - `version`: LaVersion for `LibAppletArgs`.
    pub fn libappletArgsCreate(a: *mut LibAppletArgs, version: u32);

    /// Sets the PlayStartupSound field in `LibAppletArgs`.
    ///
    /// # Arguments
    /// - `a`: LibAppletArgs struct.
    /// - `flag`: Value for `LibAppletArgs` PlayStartupSound.
    pub fn libappletArgsSetPlayStartupSound(a: *mut LibAppletArgs, flag: bool);

    /// Creates an AppletStorage with the specified size and writes the buffer contents to that storage at offset 0.
    ///
    /// # Arguments
    /// - `s` (out): Storage object.
    /// - `buffer`: Input buffer.
    /// - `size`: Size to write.
    pub fn libappletCreateWriteStorage(s: *mut AppletStorage, buffer: *const c_void, size: usize) -> Result;

    /// Reads data from offset 0 from the specified storage into the buffer. If the storage-size is smaller than the size param, the storage-size is used instead.
    ///
    /// # Arguments
    /// - `s`: Storage object.
    /// - `buffer`: Output buffer.
    /// - `size`: Size to read.
    /// - `transfer_size`: Optional output size field for the actual size used for the read, can be NULL.
    pub fn libappletReadStorage(s: *mut AppletStorage, buffer: *mut c_void, size: usize, transfer_size: *mut usize) -> Result;

    /// Sets the tick field in LibAppletArgs, then creates a storage with it which is pushed to the AppletHolder via `appletHolderPushInData`.
    ///
    /// # Arguments
    /// - `a`: LibAppletArgs struct.
    /// - `h`: AppletHolder object.
    pub fn libappletArgsPush(a: *mut LibAppletArgs, h: *mut AppletHolder) -> Result;

    /// Uses `appletPopInData` and reads it to the specified LibAppletArgs. The LibAppletArgs is validated, an error is thrown when invalid.
    ///
    /// # Arguments
    /// - `a` (out): LibAppletArgs struct.
    pub fn libappletArgsPop(a: *mut LibAppletArgs) -> Result;

    /// Creates a storage using the input buffer which is pushed to the AppletHolder via `appletHolderPushInData`.
    ///
    /// # Arguments
    /// - `h`: AppletHolder object.
    /// - `buffer`: Input data buffer.
    /// - `size`: Input data size.
    pub fn libappletPushInData(h: *mut AppletHolder, buffer: *const c_void, size: usize) -> Result;

    /// Pops a storage via `appletHolderPopOutData`, uses `libappletReadStorage`, then closes the storage.
    ///
    /// # Arguments
    /// - `h`: AppletHolder object.
    /// - `buffer`: Output buffer.
    /// - `size`: Size to read.
    /// - `transfer_size`: Optional output size field for the actual size used for the read, can be NULL.
    pub fn libappletPopOutData(h: *mut AppletHolder, buffer: *mut c_void, size: usize, transfer_size: *mut usize) -> Result;

    /// Sets whether `libappletStart` uses `appletHolderJump`.
    ///
    /// # Arguments
    /// - `flag`: Flag. Value true should not be used unless running as AppletType_LibraryApplet.
    pub fn libappletSetJumpFlag(flag: bool);

    /// If the flag from `libappletSetJumpFlag` is set, this just uses `appletHolderJump`. Otherwise, starts the applet and waits for it to finish, then checks the `LibAppletExitReason`.
    ///
    /// # Arguments
    /// - `h`: AppletHolder object.
    ///
    /// **Note:** Uses `appletHolderStart` and `appletHolderJoin`.
    pub fn libappletStart(h: *mut AppletHolder) -> Result;

    /// Creates a LibraryApplet with the specified input storage data, uses `libappletStart`, and reads the output storage reply data via `libappletPopOutData`.
    ///
    /// # Arguments
    /// - `id`: `AppletId`
    /// - `commonargs`: `LibAppletArgs` struct.
    /// - `arg`: Input storage data buffer. Optional, can be NULL.
    /// - `arg_size`: Size of the arg buffer.
    /// - `reply`: Output storage data buffer. Optional, can be NULL.
    /// - `reply_size`: Size to read for the reply buffer.
    /// - `out_reply_size`: Actual read reply data size, see `libappletPopOutData`.
    pub fn libappletLaunch(id: AppletId, commonargs: *mut LibAppletArgs, arg: *const c_void, arg_size: usize, reply: *mut c_void, reply_size: usize, out_reply_size: *mut usize) -> Result;

    /// Wrapper for `appletPushToGeneralChannel`, see appletPushToGeneralChannel regarding the requirements for using this.
    /// Returns to the main Home Menu, equivalent to pressing the HOME button.
    pub fn libappletRequestHomeMenu() -> Result;

    /// Wrapper for `appletPushToGeneralChannel`, see appletPushToGeneralChannel regarding the requirements for using this.
    /// Equivalent to entering "System Update" under System Settings. When leaving this, it returns to the main Home Menu.
    pub fn libappletRequestJumpToSystemUpdate() -> Result;

    /// Wrapper for `appletPushToGeneralChannel`, see appletPushToGeneralChannel regarding the requirements for using this.
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId
    /// - `uid` (in): `AccountUid`
    /// - `buffer` (in): Input buffer.
    /// - `size` (in): Input buffer size.
    /// - `sender` (in): LaunchApplicationRequestSender
    ///
    /// **Note:** Only available on [11.0.0+].
    pub fn libappletRequestToLaunchApplication(application_id: u64, uid: AccountUid, buffer: *const c_void, size: usize, sender: u32) -> Result;

    /// Wrapper for `appletPushToGeneralChannel`, see appletPushToGeneralChannel regarding the requirements for using this.
    ///
    /// # Arguments
    /// - `uid` (in): `AccountUid`
    /// - `application_id` (in): Optional ApplicationId, can be 0.
    ///
    /// **Note:** Only available on [11.0.0+].
    pub fn libappletRequestJumpToStory(uid: AccountUid, application_id: u64) -> Result;
}
