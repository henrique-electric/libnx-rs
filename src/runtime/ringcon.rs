//! `switch/runtime/ringcon.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const RINGCON_CAL_MAGIC: i32 = -0x3502;

/// Whether the output data is valid.
pub type RingConDataValid = u32;
/// Valid.
pub const RingConDataValid_Ok: RingConDataValid = 0;
/// Bad CRC.
pub const RingConDataValid_CRC: RingConDataValid = 1;
/// Only used with `ringconReadUserCal`. Calibration is needed via `ringconUpdateUserCal`.
pub const RingConDataValid_Cal: RingConDataValid = 2;

pub type RingConErrorFlag = u32;
/// The output from `ringconReadUserCal` doesn't match the input used with `ringconWriteUserCal`, or the `RingConDataValid` is not ::RingConDataValid_Ok.
pub const RingConErrorFlag_BadUserCalUpdate: RingConErrorFlag = 0;
/// The output flag from `ringconCmdx00020105` when successful is invalid.
pub const RingConErrorFlag_BadFlag: RingConErrorFlag = 4;
/// BadUserCal
pub const RingConErrorFlag_BadUserCal: RingConErrorFlag = 5;
/// BadManuCal
pub const RingConErrorFlag_BadManuCal: RingConErrorFlag = 6;

/// Ring-Con firmware version.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RingConFwVersion {
    /// Main firmware version.
    pub fw_main_ver: u8,
    /// Sub firmware version.
    pub fw_sub_ver: u8,
}

/// Ring-Con manufacturer calibration.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RingConManuCal {
    /// (manu_)os_max
    pub os_max: s16,
    /// (manu_)hk_max
    pub hk_max: s16,
    /// (manu_)zero_min
    pub zero_min: s16,
    /// (manu_)zero_max
    pub zero_max: s16,
}

/// Ring-Con user calibration.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RingConUserCal {
    /// (user_)os_max
    pub os_max: s16,
    /// (user_)hk_max
    pub hk_max: s16,
    /// (user_)zero
    pub zero: s16,
    /// **Ref:** RingConDataValid
    pub data_valid: RingConDataValid,
}

/// Polling data extracted from `HidbusJoyPollingReceivedData`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RingConPollingData {
    /// Sensor state data.
    pub data: s16,
    /// SamplingNumber
    pub sampling_number: u64,
}

/// Ring-Con state object.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RingCon {
    pub bus_initialized: bool,
    pub handle: HidbusBusHandle,
    pub workbuf: *mut c_void,
    pub workbuf_size: usize,
    pub polling_last_sampling_number: u64,
    pub error_flags: u32,
    pub id_l: u64,
    pub id_h: u64,
    pub fw_ver: RingConFwVersion,
    pub flag: u32,
    pub unk_cal: s16,
    pub total_push_count: s32,
    pub manu_cal: RingConManuCal,
    pub user_cal: RingConUserCal,
}

/// Gets the error flags field.
///
/// # Arguments
/// - `c`: `RingCon`
#[inline(always)]
pub fn ringconGetErrorFlags(c: &RingCon) -> u32 {
    c.error_flags
}

/// Gets the value of an error flag, set by `ringconSetErrorFlag`.
///
/// # Arguments
/// - `c`: `RingCon`
/// - `flag` (in): `RingConErrorFlag`
#[inline(always)]
pub fn ringconGetErrorFlag(c: &RingCon, flag: RingConErrorFlag) -> bool {
    (c.error_flags & BIT(flag)) != 0
}

/// Gets the `RingConFwVersion` previously loaded by `ringconCreate`.
///
/// # Arguments
/// - `c`: `RingCon`
/// - `out` (out): `RingConFwVersion`
#[inline(always)]
pub fn ringconGetFwVersion(c: &RingCon) -> RingConFwVersion {
    c.fw_ver
}

/// Gets the Id previously loaded by `ringconCreate`.
///
/// # Arguments
/// - `c`: `RingCon`
/// - `id_l` (out): Id low.
/// - `id_h` (out): Id high.
#[inline(always)]
pub fn ringconGetId(c: &RingCon, id_l: &mut u64, id_h: &mut u64) {
    *id_l = c.id_l;
    *id_h = c.id_h;
}

/// Gets the unk_cal previously loaded by `ringconCreate` with `ringconReadUnkCal`. Only valid when the output flag from `ringconCmdx00020105` is valid.
///
/// # Arguments
/// - `c`: `RingCon`
#[inline(always)]
pub fn ringconGetUnkCal(c: &RingCon) -> s16 {
    c.unk_cal
}

/// Gets the total-push-count previously loaded by `ringconCreate`.
///
/// # Arguments
/// - `c`: `RingCon`
/// - `out` (out): total_push_count
#[inline(always)]
pub fn ringconGetTotalPushCount(c: &RingCon) -> s32 {
    c.total_push_count
}

/// Gets the `RingConManuCal` previously loaded by `ringconCreate`.
///
/// # Arguments
/// - `c`: `RingCon`
/// - `out` (out): `RingConManuCal`
#[inline(always)]
pub fn ringconGetManuCal(c: &RingCon, out: &mut RingConManuCal) {
    *out = c.manu_cal;
}

/// Gets the `RingConUserCal` previously loaded by `ringconCreate`.
///
/// # Arguments
/// - `c`: `RingCon`
/// - `out` (out): `RingConUserCal`
///
/// **Note:** The Ring-Con UserCal doesn't seem to be calibrated normally?
#[inline(always)]
pub fn ringconGetUserCal(c: &RingCon, out: &mut RingConUserCal) {
    *out = c.user_cal;
}

impl_zeroed_default!(RingConFwVersion, RingConManuCal, RingConUserCal, RingConPollingData, RingCon);

unsafe extern "C" {
    /// Creates a `RingCon` object, and handles the various initialization for it.
    ///
    /// # Arguments
    /// - `c`: `RingCon`
    /// - `id` (in): `HidNpadIdType`. A Ring-Con must be attached to this controller.
    pub fn ringconCreate(c: *mut RingCon, id: HidNpadIdType) -> Result;

    /// Close a `RingCon`.
    ///
    /// # Arguments
    /// - `c`: `RingCon`
    pub fn ringconClose(c: *mut RingCon);

    /// Updates the `RingConUserCal`.
    ///
    /// # Arguments
    /// - `c`: `RingCon`
    /// - `cal` (in): `RingConUserCal`
    ///
    /// **Note:** The input `RingConUserCal` is used with `ringconWriteUserCal`, and the output from `ringconReadUserCal` is verified with the input `RingConUserCal`. This does not update the `RingConUserCal` returned by `ringconGetUserCal`.
    ///
    /// **Note:** The Ring-Con UserCal doesn't seem to be calibrated normally?
    pub fn ringconUpdateUserCal(c: *mut RingCon, cal: RingConUserCal) -> Result;

    /// Reads the `RingConFwVersion`.
    ///
    /// # Arguments
    /// - `c`: `RingCon`
    /// - `out` (out): `RingConFwVersion`
    ///
    /// **Note:** This is used internally by `ringconCreate`. Normally you should use `ringconGetFwVersion` instead.
    pub fn ringconReadFwVersion(c: *mut RingCon, out: *mut RingConFwVersion) -> Result;

    /// Reads the Id.
    ///
    /// # Arguments
    /// - `c`: `RingCon`
    /// - `id_l` (out): Id low.
    /// - `id_h` (out): Id high.
    ///
    /// **Note:** This is used internally by `ringconCreate`. Normally you should use `ringconGetId` instead.
    pub fn ringconReadId(c: *mut RingCon, id_l: *mut u64, id_h: *mut u64) -> Result;

    /// Gets the `RingConPollingData`. Only returns entries which are new since the last time this was called (or if not previously called, all available entries up to count).
    ///
    /// # Arguments
    /// - `c`: `RingCon`
    /// - `out` (out): Output array of `RingConPollingData`. Entry order is newest -> oldest.
    /// - `count` (in): Total size of the out array in entries, max value is 0x9.
    /// - `total_out` (out): Total output entries.
    pub fn ringconGetPollingData(c: *mut RingCon, out: *mut RingConPollingData, count: s32, total_out: *mut s32) -> Result;

    /// Uses cmd 0x00020105.
    ///
    /// # Arguments
    /// - `c`: `RingCon`
    /// - `out` (out): Output value.
    ///
    /// **Note:** Used internally by `ringconCreate`.
    pub fn ringconCmdx00020105(c: *mut RingCon, out: *mut u32) -> Result;

    /// Reads the `RingConManuCal`.
    ///
    /// # Arguments
    /// - `c`: `RingCon`
    /// - `out` (out): `RingConManuCal`
    ///
    /// **Note:** Used internally by `ringconCreate` and `ringconReadUnkCal`.
    pub fn ringconReadManuCal(c: *mut RingCon, out: *mut RingConManuCal) -> Result;

    /// Gets the unknown value derived from the output of cmd 0x00020504 and `ringconReadManuCal`.
    ///
    /// # Arguments
    /// - `c`: `RingCon`
    /// - `out` (out): Output value.
    ///
    /// **Note:** Used internally by `ringconCreate`.
    pub fn ringconReadUnkCal(c: *mut RingCon, out: *mut s16) -> Result;

    /// Reads the `RingConUserCal`.
    ///
    /// # Arguments
    /// - `c`: `RingCon`
    /// - `out` (out): `RingConUserCal`
    ///
    /// **Note:** Used internally by `ringconCreate` and `ringconUpdateUserCal`.
    pub fn ringconReadUserCal(c: *mut RingCon, out: *mut RingConUserCal) -> Result;

    /// Reads the rep-count for Multitask Mode.
    ///
    /// # Arguments
    /// - `c`: `RingCon`
    /// - `out` (out): Output value. Official sw using this clamps the output to range 0-500.
    /// - `data_valid` (out): `RingConDataValid`
    pub fn ringconReadRepCount(c: *mut RingCon, out: *mut s32, data_valid: *mut RingConDataValid) -> Result;

    /// Reads the total-push-count, for Multitask Mode.
    ///
    /// # Arguments
    /// - `c`: `RingCon`
    /// - `out` (out): Output value.
    /// - `data_valid` (out): `RingConDataValid`
    ///
    /// **Note:** Used internally by `ringconCreate`. Normally `ringconGetTotalPushCount` should be used instead.
    pub fn ringconReadTotalPushCount(c: *mut RingCon, out: *mut s32, data_valid: *mut RingConDataValid) -> Result;

    /// This resets the value returned by `ringconReadRepCount` to 0.
    ///
    /// # Arguments
    /// - `c`: `RingCon`
    pub fn ringconResetRepCount(c: *mut RingCon) -> Result;

    /// Writes the `RingConUserCal`.
    ///
    /// # Arguments
    /// - `c`: `RingCon`
    /// - `cal` (in): `RingConUserCal`
    ///
    /// **Note:** Used internally by `ringconUpdateUserCal`.
    pub fn ringconWriteUserCal(c: *mut RingCon, cal: RingConUserCal) -> Result;
}
