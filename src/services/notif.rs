//! `switch/services/notif.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Maximum alarms that can be registered at the same time by the host Application.
pub const NOTIF_MAX_ALARMS: u32 = 8;

/// ServiceType for `notifInitialize`.
pub type NotifServiceType = u32;
/// Initializes notif:a, for Application.
pub const NotifServiceType_Application: NotifServiceType = 0;
/// Initializes notif:s, for System.
pub const NotifServiceType_System: NotifServiceType = 1;

/// Data extracted from NotifWeeklyScheduleAlarmSetting::settings. This uses local-time.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NotifAlarmTime {
    /// Hour.
    pub hour: s32,
    /// Minute.
    pub minute: s32,
}

/// WeeklyScheduleAlarmSetting
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NotifWeeklyScheduleAlarmSetting {
    /// Unknown.
    pub unk_x0: [u8; 10],
    /// Schedule settings for each day of the week, Sun-Sat. High byte is the hour, low byte is the minute. This uses local-time.
    pub settings: [s16; 7],
}

/// AlarmSetting
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NotifAlarmSetting {
    /// AlarmSettingId
    pub alarm_setting_id: u16,
    /// Kind: 0 = WeeklySchedule.
    pub kind: u8,
    /// u8 bool flag for whether this AlarmSetting is muted (non-zero = AlarmSetting turned off, zero = on).
    pub muted: u8,
    /// Padding.
    pub pad: [u8; 4],
    /// **Ref:** AccountUid. User account associated with this AlarmSetting. Used for the preselected_user (`accountGetPreselectedUser`) when launching the Application when the system was previously in sleep-mode, instead of launching the applet for selecting the user.
    pub uid: AccountUid,
    /// ApplicationId
    pub application_id: u64,
    /// Unknown.
    pub unk_x20: u64,
    /// **Ref:** NotifWeeklyScheduleAlarmSetting
    pub schedule: NotifWeeklyScheduleAlarmSetting,
}

/// Sets whether the `NotifAlarmSetting` is muted.
///
/// # Arguments
/// - `alarm_setting`: `NotifAlarmSetting`
/// - `flag` (in): Whether the alarm is muted (true = Alarm turned off, false = on).
///
/// **Note:** By default (`notifAlarmSettingCreate`) this is false.
#[inline(always)]
pub fn notifAlarmSettingSetIsMuted(alarm_setting: &mut NotifAlarmSetting, flag: bool) {
    alarm_setting.muted = flag as _;
}

/// Sets the `AccountUid` for the `NotifAlarmSetting`, see NotifAlarmSetting::uid.
///
/// # Arguments
/// - `alarm_setting`: `NotifAlarmSetting`
/// - `uid` (in): `AccountUid`. If want to clear the uid after it was previously set, you can use an all-zero uid to reset to the default (`notifAlarmSettingCreate`).
#[inline(always)]
pub fn notifAlarmSettingSetUid(alarm_setting: &mut NotifAlarmSetting, uid: AccountUid) {
    alarm_setting.uid = uid;
}

/// Gets an Event which is signaled when data is available with `notifTryPopNotifiedApplicationParameter`.
///
/// # Arguments
/// - `out_event` (out): Output Event with autoclear=false.
///
/// **Note:** This is a wrapper for `appletGetNotificationStorageChannelEvent`, see that for the usage requirements.
///
/// **Note:** Some official apps don't use this.
///
/// **Note:** The Event must be closed by the user once finished with it.
/// # Safety
/// Requires the applet service to be initialized.
#[inline(always)]
pub unsafe fn notifGetNotificationSystemEvent(out_event: &mut Event) -> Result {
    unsafe { appletGetNotificationStorageChannelEvent(out_event) }
}

impl_zeroed_default!(NotifAlarmTime, NotifWeeklyScheduleAlarmSetting, NotifAlarmSetting);

unsafe extern "C" {
    /// Initialize notif. Only available on [9.0.0+].
    pub fn notifInitialize(service_type: NotifServiceType) -> Result;

    /// Exit notif.
    pub fn notifExit();

    /// Gets the Service object for the actual notif:* service session.
    pub fn notifGetServiceSession() -> *mut Service;

    /// Creates a `NotifAlarmSetting`.
    ///
    /// # Arguments
    /// - `alarm_setting` (out): `NotifAlarmSetting`
    ///
    /// **Note:** This clears the struct, with all schedule settings set the same as `notifAlarmSettingDisable`.
    pub fn notifAlarmSettingCreate(alarm_setting: *mut NotifAlarmSetting);

    /// Gets whether the schedule setting for the specified day_of_week is enabled, for the `NotifAlarmSetting`.
    ///
    /// # Arguments
    /// - `alarm_setting`: `NotifAlarmSetting`
    /// - `day_of_week` (in): Day-of-week, must be 0-6 (Sun-Sat).
    /// - `out` (out): Whether the setting is enabled.
    pub fn notifAlarmSettingIsEnabled(alarm_setting: *mut NotifAlarmSetting, day_of_week: u32, out: *mut bool) -> Result;

    /// Gets the schedule setting for the specified day_of_week, for the `NotifAlarmSetting`.
    ///
    /// # Arguments
    /// - `alarm_setting`: `NotifAlarmSetting`
    /// - `day_of_week` (in): Day-of-week, must be 0-6 (Sun-Sat).
    /// - `out` (out): `NotifAlarmTime`
    ///
    /// **Note:** Should not be used if the output from `notifAlarmSettingIsEnabled` is false.
    pub fn notifAlarmSettingGet(alarm_setting: *mut NotifAlarmSetting, day_of_week: u32, out: *mut NotifAlarmTime) -> Result;

    /// Enables the schedule setting for the specified day_of_week, for the `NotifAlarmSetting`. This uses local-time.
    ///
    /// # Arguments
    /// - `alarm_setting`: `NotifAlarmSetting`
    /// - `day_of_week` (in): Day-of-week, must be 0-6 (Sun-Sat).
    /// - `hour` (in): Hour.
    /// - `minute` (in): Minute.
    pub fn notifAlarmSettingEnable(alarm_setting: *mut NotifAlarmSetting, day_of_week: u32, hour: s32, minute: s32) -> Result;

    /// Disables the schedule setting for the specified day_of_week, for the `NotifAlarmSetting`.
    ///
    /// # Arguments
    /// - `alarm_setting`: `NotifAlarmSetting`
    /// - `day_of_week` (in): Day-of-week, must be 0-6 (Sun-Sat).
    ///
    /// **Note:** Schedule settings are disabled by default (`notifAlarmSettingCreate`).
    pub fn notifAlarmSettingDisable(alarm_setting: *mut NotifAlarmSetting, day_of_week: u32) -> Result;

    /// Registers the specified AlarmSetting.
    ///
    /// # Arguments
    /// - `alarm_setting_id` (out): AlarmSettingId
    /// - `alarm_setting` (in): `NotifAlarmSetting`
    /// - `buffer` (in): Input buffer containing the ApplicationParameter. Optional, can be NULL.
    /// - `size` (in): Input buffer size, must be &lt;=0x400. Optional, can be 0.
    ///
    /// **Note:** See `NOTIF_MAX_ALARMS` for the maximum alarms.
    ///
    /// **Note:** When indicated by the output from `hidIsFirmwareUpdateNeededForNotification`, this will use `hidLaShowControllerFirmwareUpdate`.
    pub fn notifRegisterAlarmSetting(alarm_setting_id: *mut u16, alarm_setting: *const NotifAlarmSetting, buffer: *const c_void, size: usize) -> Result;

    /// Updates the specified AlarmSetting.
    ///
    /// # Arguments
    /// - `alarm_setting` (in): `NotifAlarmSetting`
    /// - `buffer` (in): Input buffer containing the ApplicationParameter. Optional, can be NULL.
    /// - `size` (in): Input buffer size, must be &lt;=0x400. Optional, can be 0.
    pub fn notifUpdateAlarmSetting(alarm_setting: *const NotifAlarmSetting, buffer: *const c_void, size: usize) -> Result;

    /// Gets a listing of AlarmSettings.
    ///
    /// # Arguments
    /// - `alarm_settings` (out): Output `NotifAlarmSetting` array.
    /// - `count` (in): Total entries in the alarm_settings array.
    /// - `total_out` (out): Total output entries.
    pub fn notifListAlarmSettings(alarm_settings: *mut NotifAlarmSetting, count: s32, total_out: *mut s32) -> Result;

    /// Loads the ApplicationParameter for the specified AlarmSetting.
    ///
    /// # Arguments
    /// - `alarm_setting_id` (in): AlarmSettingId
    /// - `buffer` (out): Output buffer containing the ApplicationParameter.
    /// - `size` (in): Output buffer size.
    /// - `actual_size` (out): Actual output size.
    pub fn notifLoadApplicationParameter(alarm_setting_id: u16, buffer: *mut c_void, size: usize, actual_size: *mut u32) -> Result;

    /// Deletes the specified AlarmSetting.
    ///
    /// # Arguments
    /// - `alarm_setting_id` (in): AlarmSettingId
    pub fn notifDeleteAlarmSetting(alarm_setting_id: u16) -> Result;

    /// Uses `appletTryPopFromNotificationStorageChannel` then reads the data from there into the output params.
    ///
    /// # Arguments
    /// - `buffer` (out): Output buffer.
    /// - `size` (out): Output buffer size.
    /// - `out_size` (out): Size of the data which was written into the output buffer. Optional, can be NULL.
    ///
    /// **Note:** This is a wrapper for `appletTryPopFromNotificationStorageChannel`, see that for the usage requirements.
    ///
    /// **Note:** The system will only push data for this when launching the Application when the Alarm was triggered, where the system was previously in sleep-mode.
    ///
    /// **Note:** Some official apps don't use this.
    pub fn notifTryPopNotifiedApplicationParameter(buffer: *mut c_void, size: u64, out_size: *mut u64) -> Result;
}
