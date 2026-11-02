//! `switch/applets/error.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Error type for ErrorCommonHeader.type.
pub type ErrorType = u32;
/// Normal
pub const ErrorType_Normal: ErrorType = 0;
/// System
pub const ErrorType_System: ErrorType = 1;
/// Application
pub const ErrorType_Application: ErrorType = 2;
/// EULA
pub const ErrorType_Eula: ErrorType = 3;
/// Parental Controls
pub const ErrorType_Pctl: ErrorType = 4;
/// Record
pub const ErrorType_Record: ErrorType = 5;
/// SystemUpdateEula
pub const ErrorType_SystemUpdateEula: ErrorType = 8;

/// Stores error-codes which are displayed as XXXX-XXXX, low for the former and desc for the latter.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ErrorCode {
    /// The module portion of the error, normally this should be set to module + 2000.
    pub low: u32,
    /// The error description.
    pub desc: u32,
}

/// Error type for ErrorContext.type
pub type ErrorContextType = u32;
/// None
pub const ErrorContextType_None: ErrorContextType = 0;
/// Http
pub const ErrorContextType_Http: ErrorContextType = 1;
/// FileSystem
pub const ErrorContextType_FileSystem: ErrorContextType = 2;
/// WebMediaPlayer
pub const ErrorContextType_WebMediaPlayer: ErrorContextType = 3;
/// LocalContentShare
pub const ErrorContextType_LocalContentShare: ErrorContextType = 4;

/// Error context.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ErrorContext {
    /// Type, see `ErrorContextType`.
    pub type_: u8,
    /// Padding
    pub pad: [u8; 7],
    /// Data
    pub data: [u8; 500],
    /// Result
    pub res: Result,
}

/// Common header for the start of the arg storage.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ErrorCommonHeader {
    /// Type, see `ErrorType`.
    pub type_: u8,
    /// When clear, this indicates WithoutJump.
    pub jumpFlag: u8,
    /// Unknown
    pub unk_x2: [u8; 3],
    /// When set with ::ErrorType_Normal, indicates that an additional storage is pushed for `ErrorResultBacktrace`. [4.0.0+] Otherwise, when set indicates that an additional storage is pushed for `ErrorContext`.
    pub contextFlag: u8,
    /// ErrorCommonArg: When clear, errorCode is used, otherwise the applet generates the error-code from res.
    pub resultFlag: u8,
    /// Similar to contextFlag except for ErrorCommonArg, indicating `ErrorContext` is used.
    pub contextFlag2: u8,
}

/// Common error arg data.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ErrorCommonArg {
    /// Common header.
    pub hdr: ErrorCommonHeader,
    /// **Ref:** ErrorCode
    pub errorCode: ErrorCode,
    /// Result
    pub res: Result,
}

/// Error arg data for certain errors with module PCTL.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ErrorPctlArg {
    /// Common header.
    pub hdr: ErrorCommonHeader,
    /// Result
    pub res: Result,
}

/// ResultBacktrace
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ErrorResultBacktrace {
    /// Total entries in the backtrace array.
    pub count: s32,
    /// Result backtrace.
    pub backtrace: [Result; 32],
}

/// Error arg data for EULA.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ErrorEulaArg {
    /// Common header.
    pub hdr: ErrorCommonHeader,
    /// **Ref:** SetRegion
    pub regionCode: SetRegion,
}

/// Additional input storage data for `errorSystemUpdateEulaShow`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ErrorEulaData {
    /// data
    pub data: [u8; 131072],
}

/// Error arg data for Record.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ErrorRecordArg {
    /// Common header.
    pub hdr: ErrorCommonHeader,
    /// **Ref:** ErrorCode
    pub errorCode: ErrorCode,
    /// POSIX timestamp.
    pub timestamp: u64,
}

/// SystemErrorArg
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ErrorSystemArg {
    /// Common header.
    pub hdr: ErrorCommonHeader,
    /// **Ref:** ErrorCode
    pub errorCode: ErrorCode,
    /// See set.h.
    pub languageCode: u64,
    /// UTF-8 Dialog message.
    pub dialogMessage: [c_char; 2048],
    /// UTF-8 Fullscreen message (displayed when the user clicks on "Details").
    pub fullscreenMessage: [c_char; 2048],
}

/// Error system config.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ErrorSystemConfig {
    /// Arg data.
    pub arg: ErrorSystemArg,
    /// Optional error context.
    pub ctx: ErrorContext,
}

/// ApplicationErrorArg
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ErrorApplicationArg {
    /// Common header.
    pub hdr: ErrorCommonHeader,
    /// Raw decimal error number which is displayed in the dialog.
    pub errorNumber: u32,
    /// See set.h.
    pub languageCode: u64,
    /// UTF-8 Dialog message.
    pub dialogMessage: [c_char; 2048],
    /// UTF-8 Fullscreen message (displayed when the user clicks on "Details").
    pub fullscreenMessage: [c_char; 2048],
}

/// Error application config.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ErrorApplicationConfig {
    /// Arg data.
    pub arg: ErrorApplicationArg,
}

/// Creates an `ErrorCode`.
///
/// # Arguments
/// - `low`: The module portion of the error, normally this should be set to module + 2000.
/// - `desc`: The error description.
#[inline(always)]
pub const fn errorCodeCreate(low: u32, desc: u32) -> ErrorCode {
    ErrorCode { low, desc }
}

/// Creates an `ErrorCode` with the input Result. Wrapper for `errorCodeCreate`.
///
/// # Arguments
/// - `res`: Input Result.
#[inline(always)]
pub const fn errorCodeCreateResult(res: Result) -> ErrorCode {
    errorCodeCreate(2000 + R_MODULE(res), R_DESCRIPTION(res))
}

/// Creates an invalid `ErrorCode`.
#[inline(always)]
pub const fn errorCodeCreateInvalid() -> ErrorCode {
    ErrorCode { low: 0, desc: 0 }
}

/// Checks whether the input ErrorCode is valid.
///
/// # Arguments
/// - `errorCode`: `ErrorCode`
#[inline(always)]
pub const fn errorCodeIsValid(errorCode: ErrorCode) -> bool {
    errorCode.low != 0
}

/// Launches the applet for displaying an error full-screen, using the specified Result and timestamp.
///
/// # Arguments
/// - `res`: Result
/// - `timestamp`: POSIX timestamp.
///
/// **Note:** Wrapper for `errorCodeRecordShow`, see `errorCodeRecordShow` notes.
/// # Safety
/// Requires the applet service to be initialized.
#[inline(always)]
pub unsafe fn errorResultRecordShow(res: Result, timestamp: u64) -> Result {
    unsafe { errorCodeRecordShow(errorCodeCreateResult(res), timestamp) }
}

/// Sets the error code.
///
/// # Arguments
/// - `c`: ErrorSystemConfig struct.
/// - `errorCode`: `ErrorCode`
#[inline(always)]
pub fn errorSystemSetCode(c: &mut ErrorSystemConfig, errorCode: ErrorCode) {
    c.arg.errorCode = errorCode;
}

/// Sets the error code, using the input Result. Wrapper for `errorSystemSetCode`.
///
/// # Arguments
/// - `c`: ErrorSystemConfig struct.
/// - `res`: The Result to set.
#[inline(always)]
pub fn errorSystemSetResult(c: &mut ErrorSystemConfig, res: Result) {
    errorSystemSetCode(c, errorCodeCreateResult(res));
}

/// Sets the LanguageCode.
///
/// # Arguments
/// - `c`: ErrorSystemConfig struct.
/// - `LanguageCode`: LanguageCode, see set.h.
#[inline(always)]
pub fn errorSystemSetLanguageCode(c: &mut ErrorSystemConfig, LanguageCode: u64) {
    c.arg.languageCode = LanguageCode;
}

/// Sets the error code number.
///
/// # Arguments
/// - `c`: ErrorApplicationConfig struct.
/// - `errorNumber`: Error code number. Raw decimal error number which is displayed in the dialog.
#[inline(always)]
pub fn errorApplicationSetNumber(c: &mut ErrorApplicationConfig, errorNumber: u32) {
    c.arg.errorNumber = errorNumber;
}

/// Sets the LanguageCode.
///
/// # Arguments
/// - `c`: ErrorApplicationConfig struct.
/// - `LanguageCode`: LanguageCode, see set.h.
#[inline(always)]
pub fn errorApplicationSetLanguageCode(c: &mut ErrorApplicationConfig, LanguageCode: u64) {
    c.arg.languageCode = LanguageCode;
}

impl_zeroed_default!(ErrorCode, ErrorContext, ErrorCommonHeader, ErrorCommonArg, ErrorPctlArg, ErrorResultBacktrace, ErrorEulaArg, ErrorEulaData, ErrorRecordArg, ErrorSystemArg, ErrorSystemConfig, ErrorApplicationArg, ErrorApplicationConfig);

unsafe extern "C" {
    /// Launches the applet for displaying the specified Result.
    ///
    /// # Arguments
    /// - `res`: Result
    /// - `jumpFlag`: Jump flag, normally this is true.
    /// - `ctx`: Optional `ErrorContext`, can be NULL. Unused when jumpFlag=false. Ignored on pre-4.0.0, since it's only available for [4.0.0+].
    ///
    /// **Note:** Sets the following fields: jumpFlag and contextFlag2. Uses ::ErrorType_Normal normally.
    ///
    /// **Note:** For module=PCTL errors with desc 100-119 this sets uses ::ErrorType_Pctl, in which case the applet will display the following special dialog: "This software is restricted by Parental Controls".
    ///
    /// **Note:** If the input Result is 0xC8A2, the applet will display a special dialog regarding the current application requiring a software update, with buttons "Later" and "Restart".
    ///
    /// **Note:** [3.0.0+] If the input Result is 0xCAA2, the applet will display a special dialog related to DLC version.
    ///
    /// **Warning:** This applet creates an error report that is logged in the system, when not handling the above special dialogs. Proceed at your own risk!
    pub fn errorResultShow(res: Result, jumpFlag: bool, ctx: *const ErrorContext) -> Result;

    /// Launches the applet for displaying the specified ErrorCode.
    ///
    /// # Arguments
    /// - `errorCode`: `ErrorCode`
    /// - `jumpFlag`: Jump flag, normally this is true.
    /// - `ctx`: Optional `ErrorContext`, can be NULL. Unused when jumpFlag=false. Ignored on pre-4.0.0, since it's only available for [4.0.0+].
    ///
    /// **Note:** Sets the following fields: jumpFlag and contextFlag2. resultFlag=1. Uses ::ErrorType_Normal.
    ///
    /// **Warning:** This applet creates an error report that is logged in the system. Proceed at your own risk!
    pub fn errorCodeShow(errorCode: ErrorCode, jumpFlag: bool, ctx: *const ErrorContext) -> Result;

    /// Creates an ErrorResultBacktrace struct.
    ///
    /// # Arguments
    /// - `backtrace`: `ErrorResultBacktrace` struct.
    /// - `count`: Total number of entries.
    /// - `entries`: Input array of Result.
    pub fn errorResultBacktraceCreate(backtrace: *mut ErrorResultBacktrace, count: s32, entries: *const Result) -> Result;

    /// Launches the applet for `ErrorResultBacktrace`.
    ///
    /// # Arguments
    /// - `backtrace`: ErrorResultBacktrace struct.
    /// - `res`: Result
    ///
    /// **Note:** Sets the following fields: jumpFlag=1, contextFlag=1, and uses ::ErrorType_Normal.
    ///
    /// **Warning:** This applet creates an error report that is logged in the system. Proceed at your own risk!
    pub fn errorResultBacktraceShow(res: Result, backtrace: *const ErrorResultBacktrace) -> Result;

    /// Launches the applet for displaying the EULA.
    ///
    /// # Arguments
    /// - `RegionCode`: `SetRegion`
    ///
    /// **Note:** Sets the following fields: jumpFlag=1, regionCode, and uses ::ErrorType_Eula.
    pub fn errorEulaShow(RegionCode: SetRegion) -> Result;

    /// Launches the applet for displaying the system-update EULA.
    ///
    /// # Arguments
    /// - `RegionCode`: `SetRegion`
    /// - `eula`: EULA data. Address must be 0x1000-byte aligned.
    ///
    /// **Note:** Sets the following fields: jumpFlag=1, regionCode, and uses ::ErrorType_SystemUpdateEula.
    pub fn errorSystemUpdateEulaShow(RegionCode: SetRegion, eula: *const ErrorEulaData) -> Result;

    /// Launches the applet for displaying an error full-screen, using the specified ErrorCode and timestamp.
    ///
    /// # Arguments
    /// - `errorCode`: `ErrorCode`
    /// - `timestamp`: POSIX timestamp.
    ///
    /// **Note:** Sets the following fields: jumpFlag=1, errorCode, timestamp, and uses ::ErrorType_Record.
    ///
    /// **Note:** The applet does not log an error report for this. error*RecordShow is used by qlaunch for displaying previously logged error reports.
    pub fn errorCodeRecordShow(errorCode: ErrorCode, timestamp: u64) -> Result;

    /// Creates an ErrorSystemConfig struct.
    ///
    /// # Arguments
    /// - `c`: ErrorSystemConfig struct.
    /// - `dialog_message`: UTF-8 dialog message.
    /// - `fullscreen_message`: UTF-8 fullscreen message, displayed when the user clicks on "Details". Optional, can be NULL (which disables displaying Details).
    ///
    /// **Note:** Sets the following fields: {strings}, and uses ::ErrorType_System. The rest are cleared.
    ///
    /// **Note:** On pre-5.0.0 this will initialize languageCode by using: setInitialize(), setMakeLanguageCode(SetLanguage_ENUS, ...), and setExit(). This is needed since an empty languageCode wasn't supported until [5.0.0+] (which would also use SetLanguage_ENUS).
    ///
    /// **Warning:** This applet creates an error report that is logged in the system. Proceed at your own risk!
    pub fn errorSystemCreate(c: *mut ErrorSystemConfig, dialog_message: *const c_char, fullscreen_message: *const c_char) -> Result;

    /// Launches the applet with the specified config.
    ///
    /// # Arguments
    /// - `c`: ErrorSystemConfig struct.
    pub fn errorSystemShow(c: *mut ErrorSystemConfig) -> Result;

    /// Sets the ErrorContext.
    ///
    /// # Arguments
    /// - `c`: ErrorSystemConfig struct.
    /// - `ctx`: ErrorContext, NULL to clear it.
    ///
    /// **Note:** Only available on [4.0.0+], on older versions this will return without setting the context.
    pub fn errorSystemSetContext(c: *mut ErrorSystemConfig, ctx: *const ErrorContext);

    /// Creates an ErrorApplicationConfig struct.
    ///
    /// # Arguments
    /// - `c`: ErrorApplicationConfig struct.
    /// - `dialog_message`: UTF-8 dialog message.
    /// - `fullscreen_message`: UTF-8 fullscreen message, displayed when the user clicks on "Details". Optional, can be NULL (which disables displaying Details).
    ///
    /// **Note:** Sets the following fields: jumpFlag=1, {strings}, and uses ::ErrorType_Application. The rest are cleared.
    ///
    /// **Note:** On pre-5.0.0 this will initialize languageCode by using: setInitialize(), setMakeLanguageCode(SetLanguage_ENUS, ...), and setExit(). This is needed since an empty languageCode wasn't supported until [5.0.0+] (which would also use SetLanguage_ENUS).
    ///
    /// **Note:** With [10.0.0+] this must only be used when running under an Application, since otherwise the applet will trigger a fatalerr.
    ///
    /// **Warning:** This applet creates an error report that is logged in the system. Proceed at your own risk!
    pub fn errorApplicationCreate(c: *mut ErrorApplicationConfig, dialog_message: *const c_char, fullscreen_message: *const c_char) -> Result;

    /// Launches the applet with the specified config.
    ///
    /// # Arguments
    /// - `c`: ErrorApplicationConfig struct.
    pub fn errorApplicationShow(c: *mut ErrorApplicationConfig) -> Result;
}
