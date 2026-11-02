// inline: notifAlarmSettingSetIsMuted
#[inline(always)]
pub fn notifAlarmSettingSetIsMuted(alarm_setting: &mut NotifAlarmSetting, flag: bool) {
    alarm_setting.muted = flag as _;
}
// inline: notifAlarmSettingSetUid
#[inline(always)]
pub fn notifAlarmSettingSetUid(alarm_setting: &mut NotifAlarmSetting, uid: AccountUid) {
    alarm_setting.uid = uid;
}
// inline: notifGetNotificationSystemEvent
/// # Safety
/// Requires the applet service to be initialized.
#[inline(always)]
pub unsafe fn notifGetNotificationSystemEvent(out_event: &mut Event) -> Result {
    unsafe { appletGetNotificationStorageChannelEvent(out_event) }
}
