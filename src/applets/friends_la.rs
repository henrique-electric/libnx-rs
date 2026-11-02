//! `switch/applets/friends_la.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Arg type values used with `FriendsLaArg`.
pub type FriendsLaArgType = u32;
/// ShowFriendList. Launches the applet with the "Friend List" menu initially selected.
pub const FriendsLaArgType_ShowFriendList: FriendsLaArgType = 0;
/// ShowUserDetailInfo
pub const FriendsLaArgType_ShowUserDetailInfo: FriendsLaArgType = 1;
/// StartSendingFriendRequest
pub const FriendsLaArgType_StartSendingFriendRequest: FriendsLaArgType = 2;
/// ShowMethodsOfSendingFriendRequest. Launches the applet with the "Add Friend" menu initially selected.
pub const FriendsLaArgType_ShowMethodsOfSendingFriendRequest: FriendsLaArgType = 3;
/// StartFacedFriendRequest. Launches the applet where the "Search for Local Users" menu is initially shown. Returning from this menu will exit the applet.
pub const FriendsLaArgType_StartFacedFriendRequest: FriendsLaArgType = 4;
/// ShowReceivedFriendRequestList. Launches the applet where the "Received Friend Requests" menu is initially shown. Returning from this menu will exit the applet.
pub const FriendsLaArgType_ShowReceivedFriendRequestList: FriendsLaArgType = 5;
/// ShowBlockedUserList. Launches the applet where the "Blocked-User List" menu is initially shown. Returning from this menu will exit the applet.
pub const FriendsLaArgType_ShowBlockedUserList: FriendsLaArgType = 6;
/// ShowMyProfile. Launches the applet with the "Profile" menu initially selected. ShowMyProfileForHomeMenu is identical to this except for playStartupSound=true.
pub const FriendsLaArgType_ShowMyProfile: FriendsLaArgType = 7;
/// [9.0.0+] StartFriendInvitation. Launches the applet for sending online-play invites to friends, where the friends are selected via the UI.
pub const FriendsLaArgType_StartFriendInvitation: FriendsLaArgType = 8;
/// [9.0.0+] StartSendingFriendInvitation.
pub const FriendsLaArgType_StartSendingFriendInvitation: FriendsLaArgType = 9;
/// [9.0.0+] ShowReceivedInvitationDetail.
pub const FriendsLaArgType_ShowReceivedInvitationDetail: FriendsLaArgType = 10;

/// Header for the arg struct.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FriendsLaArgHeader {
    /// **Ref:** FriendsLaArgType
    pub type_: u32,
    /// Padding.
    pub pad: u32,
    /// **Ref:** AccountUid
    pub uid: AccountUid,
}

/// Common data for the arg struct, for the pre-9.0.0 types.
/// This is only set for ::FriendsLaArgType_ShowUserDetailInfo/::FriendsLaArgType_StartSendingFriendRequest, for everything else this is cleared.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FriendsLaArgCommonData {
    /// **Ref:** AccountNetworkServiceAccountId for the other account.
    pub id: AccountNetworkServiceAccountId,
    /// First InAppScreenName.
    pub first_inAppScreenName: FriendsInAppScreenName,
    /// Second InAppScreenName.
    pub second_inAppScreenName: FriendsInAppScreenName,
}

/// Arg struct pushed for the applet input storage, for pre-9.0.0.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FriendsLaArgV1 {
    /// **Ref:** FriendsLaArgHeader
    pub hdr: FriendsLaArgHeader,
    /// **Ref:** FriendsLaArgCommonData
    pub data: FriendsLaArgCommonData,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FriendsLaArg_data_start_friend_invitation {
    /// **Ref:** AccountNetworkServiceAccountId count, must be 1-15.
    pub id_count: s32,
    /// Padding.
    pub pad: u32,
    /// User-data size, must be &lt;=0x400.
    pub userdata_size: u64,
    /// Arbitrary user-data, see above size.
    pub userdata: [u8; 1024],
    /// **Ref:** FriendsFriendInvitationGameModeDescription
    pub desc: FriendsFriendInvitationGameModeDescription,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FriendsLaArg_data_start_sending_friend_invitation {
    /// **Ref:** AccountNetworkServiceAccountId count, must be 1-15.
    pub id_count: s32,
    /// Padding.
    pub pad: u32,
    /// **Ref:** AccountNetworkServiceAccountId list, see above count.
    pub id_list: [AccountNetworkServiceAccountId; 16],
    /// User-data size, must be &lt;=0x400.
    pub userdata_size: u64,
    /// Arbitrary user-data, see above size.
    pub userdata: [u8; 1024],
    /// **Ref:** FriendsFriendInvitationGameModeDescription
    pub desc: FriendsFriendInvitationGameModeDescription,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FriendsLaArg_data_show_received_invitation_detail {
    /// **Ref:** FriendsFriendInvitationId
    pub invitation_id: FriendsFriendInvitationId,
    /// **Ref:** FriendsFriendInvitationGroupId
    pub invitation_group_id: FriendsFriendInvitationGroupId,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union FriendsLaArg_data {
    /// Raw data.
    pub raw: [u8; 4240],
    /// **Ref:** FriendsLaArgCommonData
    pub common: FriendsLaArgCommonData,
    /// Data for ::FriendsLaArgType_StartFriendInvitation.
    pub start_friend_invitation: FriendsLaArg_data_start_friend_invitation,
    /// Data for ::FriendsLaArgType_StartSendingFriendInvitation.
    pub start_sending_friend_invitation: FriendsLaArg_data_start_sending_friend_invitation,
    /// Data for ::FriendsLaArgType_ShowReceivedInvitationDetail.
    pub show_received_invitation_detail: FriendsLaArg_data_show_received_invitation_detail,
}

/// Arg struct pushed for the applet input storage, for [9.0.0+].
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FriendsLaArg {
    /// **Ref:** FriendsLaArgHeader
    pub hdr: FriendsLaArgHeader,
    /// Data for each `FriendsLaArgType`.
    pub data: FriendsLaArg_data,
}

impl_zeroed_default!(FriendsLaArgHeader, FriendsLaArgCommonData, FriendsLaArgV1, FriendsLaArg_data_start_friend_invitation, FriendsLaArg_data_start_sending_friend_invitation, FriendsLaArg_data_show_received_invitation_detail, FriendsLaArg_data, FriendsLaArg);

unsafe extern "C" {
    /// Launches the applet with ::FriendsLaArgType_ShowFriendList, the specified input, and playStartupSound=false.
    ///
    /// # Arguments
    /// - `uid` (in): `AccountUid`
    pub fn friendsLaShowFriendList(uid: AccountUid) -> Result;

    /// Launches the applet with ::FriendsLaArgType_ShowUserDetailInfo, the specified input, and playStartupSound=false.
    ///
    /// # Arguments
    /// - `uid` (in): `AccountUid`
    /// - `id` (in): `AccountNetworkServiceAccountId` for the user to show UserDetailInfo for.
    /// - `first_inAppScreenName` (in): First `FriendsInAppScreenName`.
    /// - `second_inAppScreenName` (in): Second `FriendsInAppScreenName`.
    pub fn friendsLaShowUserDetailInfo(uid: AccountUid, id: AccountNetworkServiceAccountId, first_inAppScreenName: *const FriendsInAppScreenName, second_inAppScreenName: *const FriendsInAppScreenName) -> Result;

    /// Launches the applet with ::FriendsLaArgType_StartSendingFriendRequest, the specified input, and playStartupSound=false. On success, this will load the output Result from the output storage.
    ///
    /// # Arguments
    /// - `uid` (in): `AccountUid`
    /// - `id` (in): `AccountNetworkServiceAccountId` to send the friend request to.
    /// - `first_inAppScreenName` (in): First `FriendsInAppScreenName`.
    /// - `second_inAppScreenName` (in): Second `FriendsInAppScreenName`.
    pub fn friendsLaStartSendingFriendRequest(uid: AccountUid, id: AccountNetworkServiceAccountId, first_inAppScreenName: *const FriendsInAppScreenName, second_inAppScreenName: *const FriendsInAppScreenName) -> Result;

    /// Launches the applet with ::FriendsLaArgType_ShowMethodsOfSendingFriendRequest, the specified input, and playStartupSound=false.
    ///
    /// # Arguments
    /// - `uid` (in): `AccountUid`
    pub fn friendsLaShowMethodsOfSendingFriendRequest(uid: AccountUid) -> Result;

    /// Launches the applet with ::FriendsLaArgType_StartFacedFriendRequest, the specified input, and playStartupSound=false.
    ///
    /// # Arguments
    /// - `uid` (in): `AccountUid`
    pub fn friendsLaStartFacedFriendRequest(uid: AccountUid) -> Result;

    /// Launches the applet with ::FriendsLaArgType_ShowReceivedFriendRequestList, the specified input, and playStartupSound=false.
    ///
    /// # Arguments
    /// - `uid` (in): `AccountUid`
    pub fn friendsLaShowReceivedFriendRequestList(uid: AccountUid) -> Result;

    /// Launches the applet with ::FriendsLaArgType_ShowBlockedUserList, the specified input, and playStartupSound=false.
    ///
    /// # Arguments
    /// - `uid` (in): `AccountUid`
    pub fn friendsLaShowBlockedUserList(uid: AccountUid) -> Result;

    /// Launches the applet with ::FriendsLaArgType_ShowMyProfile, the specified input, and playStartupSound=false.
    ///
    /// # Arguments
    /// - `uid` (in): `AccountUid`
    pub fn friendsLaShowMyProfile(uid: AccountUid) -> Result;

    /// Same as `friendsLaShowMyProfile` except with playStartupSound=true.
    ///
    /// # Arguments
    /// - `uid` (in): `AccountUid`
    pub fn friendsLaShowMyProfileForHomeMenu(uid: AccountUid) -> Result;

    /// Launches the applet with ::FriendsLaArgType_StartFriendInvitation, the specified input, and playStartupSound=false. On success, this will load the output Result from the output storage.
    ///
    /// # Arguments
    /// - `uid` (in): `AccountUid`
    /// - `id_count` (in): `AccountNetworkServiceAccountId` count, must be 1-15. Number of friends to invite.
    /// - `desc` (in): `FriendsFriendInvitationGameModeDescription`
    /// - `userdata` (in): Arbitrary user-data. Can be NULL.
    /// - `userdata_size` (in): User-data size, must be &lt;=0x400. Can be 0 if userdata is NULL.
    ///
    /// **Note:** Only available on [9.0.0+].
    pub fn friendsLaStartFriendInvitation(uid: AccountUid, id_count: s32, desc: *const FriendsFriendInvitationGameModeDescription, userdata: *const c_void, userdata_size: u64) -> Result;

    /// Launches the applet with ::FriendsLaArgType_StartSendingFriendInvitation, the specified input, and playStartupSound=false. On success, this will load the output Result from the output storage.
    ///
    /// # Arguments
    /// - `uid` (in): `AccountUid`
    /// - `id_list` (in): `AccountNetworkServiceAccountId` list.
    /// - `id_count` (in): Size of the id_list array in entries, must be 1-15. Number of friends to invite.
    /// - `desc` (in): `FriendsFriendInvitationGameModeDescription`
    /// - `userdata` (in): Arbitrary user-data. Can be NULL.
    /// - `userdata_size` (in): User-data size, must be &lt;=0x400. Can be 0 if userdata is NULL.
    ///
    /// **Note:** Only available on [9.0.0+].
    pub fn friendsLaStartSendingFriendInvitation(uid: AccountUid, id_list: *const AccountNetworkServiceAccountId, id_count: s32, desc: *const FriendsFriendInvitationGameModeDescription, userdata: *const c_void, userdata_size: u64) -> Result;

    /// Launches the applet with ::FriendsLaArgType_ShowReceivedInvitationDetail, the specified input, and playStartupSound=false.
    ///
    /// # Arguments
    /// - `uid` (in): `AccountUid`
    /// - `invitation_id` (in): `FriendsFriendInvitationId`
    /// - `invitation_group_id` (in): `FriendsFriendInvitationGroupId`
    ///
    /// **Note:** Only available on [9.0.0+].
    pub fn friendsLaShowReceivedInvitationDetail(uid: AccountUid, invitation_id: FriendsFriendInvitationId, invitation_group_id: FriendsFriendInvitationGroupId) -> Result;
}
