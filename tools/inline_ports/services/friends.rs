// inline: friendsGetFriendInvitationNotificationEvent
/// # Safety
/// Requires the applet service to be initialized.
#[inline(always)]
pub unsafe fn friendsGetFriendInvitationNotificationEvent(out_event: &mut Event) -> Result {
    unsafe { appletGetFriendInvitationStorageChannelEvent(out_event) }
}
