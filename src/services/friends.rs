//! `switch/services/friends.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub type FriendsServiceType = u32;
/// Initializes friend:u
pub const FriendsServiceType_User: FriendsServiceType = 0;
/// Initializes friend:v
pub const FriendsServiceType_Viewer: FriendsServiceType = 1;
/// Initializes friend:m
pub const FriendsServiceType_Manager: FriendsServiceType = 2;
/// Initializes friend:s
pub const FriendsServiceType_System: FriendsServiceType = 3;
/// Initializes friend:a
pub const FriendsServiceType_Administrator: FriendsServiceType = 4;

/// InAppScreenName
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FriendsInAppScreenName {
    /// UTF-8 string, NUL-terminated.
    pub name: [c_char; 64],
    /// LanguageCode, see set.h.
    pub languageCode: u64,
}

/// FriendInvitationGameModeDescription
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FriendsFriendInvitationGameModeDescription {
    /// Unknown.
    pub unk_x0: [u8; 3072],
}

/// FriendInvitationId
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FriendsFriendInvitationId {
    /// Id.
    pub id: u64,
}

/// FriendInvitationGroupId
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FriendsFriendInvitationGroupId {
    /// Id.
    pub id: u64,
}

/// FriendsUserSetting
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FriendsUserSetting {
    /// User ID
    pub uid: AccountUid,
    /// Presence permission
    pub presence_permission: u32,
    /// Play log permission
    pub play_log_permission: u32,
    /// Unknown
    pub friend_request_reception: u64,
    /// Friend Code
    pub friend_code: [c_char; 32],
    /// Unknown
    pub friend_code_next_issuable_time: u64,
    /// Unknown
    pub unk_x48: [u8; 1992],
}

/// Gets an Event which is signaled when data is available with `friendsTryPopFriendInvitationNotificationInfo`.
///
/// # Arguments
/// - `out_event` (out): Output Event with autoclear=false.
///
/// **Note:** This is a wrapper for `appletGetFriendInvitationStorageChannelEvent`, see that for the usage requirements.
///
/// **Note:** The Event must be closed by the user once finished with it.
/// # Safety
/// Requires the applet service to be initialized.
#[inline(always)]
pub unsafe fn friendsGetFriendInvitationNotificationEvent(out_event: &mut Event) -> Result {
    unsafe { appletGetFriendInvitationStorageChannelEvent(out_event) }
}

impl_zeroed_default!(FriendsInAppScreenName, FriendsFriendInvitationGameModeDescription, FriendsFriendInvitationId, FriendsFriendInvitationGroupId, FriendsUserSetting);

unsafe extern "C" {
    /// Initialize friends
    pub fn friendsInitialize(service_type: FriendsServiceType) -> Result;

    /// Exit friends
    pub fn friendsExit();

    /// Gets the Service object for the friends service session.
    pub fn friendsGetServiceSession() -> *mut Service;

    /// Gets the Service object for the actual IFriendsService service session.
    pub fn friendsGetServiceSession_IFriendsService() -> *mut Service;

    /// Gets the `FriendsUserSetting` details
    ///
    /// # Arguments
    /// - `uid` (in): `User` AccountUid.
    /// - `user_setting` (out): `FriendsUserSetting`
    pub fn friendsGetUserSetting(uid: AccountUid, user_setting: *mut FriendsUserSetting) -> Result;

    /// Uses `appletTryPopFromFriendInvitationStorageChannel` then reads the data from there into the output params.
    ///
    /// # Arguments
    /// - `uid` (out): `AccountUid`. Optional, can be NULL.
    /// - `buffer` (out): Output buffer.
    /// - `size` (out): Output buffer size.
    /// - `out_size` (out): Size of the data which was written into the output buffer. Optional, can be NULL.
    ///
    /// **Note:** This is a wrapper for `appletTryPopFromFriendInvitationStorageChannel`, see that for the usage requirements.
    pub fn friendsTryPopFriendInvitationNotificationInfo(uid: *mut AccountUid, buffer: *mut c_void, size: u64, out_size: *mut u64) -> Result;
}
