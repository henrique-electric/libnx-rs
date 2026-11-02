//! `switch/services/acc.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const ACC_USER_LIST_SIZE: u32 = 8;

pub type AccountServiceType = u32;
/// Initializes acc:u0.
pub const AccountServiceType_Application: AccountServiceType = 0;
/// Initializes acc:u1.
pub const AccountServiceType_System: AccountServiceType = 1;
/// Initializes acc:su.
pub const AccountServiceType_Administrator: AccountServiceType = 2;

/// Profile
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AccountProfile {
    /// IProfile
    pub s: Service,
}

/// Account UserId.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AccountUid {
    /// UserId. All-zero is invalid / Uid not set. See also `accountUidIsValid`.
    pub uid: [u64; 2],
}

/// UserData
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AccountUserData {
    /// Unknown.
    pub unk_x0: u32,
    /// Icon ID. 0 = Mii, the rest are character icon IDs.
    pub iconID: u32,
    /// Profile icon background color ID
    pub iconBackgroundColorID: u8,
    /// Unknown.
    pub unk_x9: [u8; 7],
    /// Some ID related to the Mii? All zeros when a character icon is used.
    pub miiID: [u8; 16],
    /// Usually zeros?
    pub unk_x20: [u8; 96],
}

/// ProfileBase
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AccountProfileBase {
    /// **Ref:** AccountUid
    pub uid: AccountUid,
    /// POSIX UTC timestamp, for the last account edit.
    pub lastEditTimestamp: u64,
    /// UTF-8 Nickname.
    pub nickname: [c_char; 32],
}

/// NetworkServiceAccountId
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AccountNetworkServiceAccountId {
    /// Id.
    pub id: u64,
}

/// Checks whether the specified `AccountUid` is valid/set (non-zero).
///
/// # Arguments
/// - `Uid` (in): `AccountUid`
#[inline(always)]
pub const fn accountUidIsValid(Uid: &AccountUid) -> bool {
    Uid.uid[0] != 0 || Uid.uid[1] != 0
}

impl_zeroed_default!(AccountProfile, AccountUid, AccountUserData, AccountProfileBase, AccountNetworkServiceAccountId);

unsafe extern "C" {
    /// Initialize account.
    pub fn accountInitialize(service_type: AccountServiceType) -> Result;

    /// Exit account.
    pub fn accountExit();

    /// Gets the Service object for the actual account service session.
    pub fn accountGetServiceSession() -> *mut Service;

    /// Get the total number of user profiles.
    pub fn accountGetUserCount(user_count: *mut s32) -> Result;

    /// Get a list of all userIds. The returned list will never be larger than ACC_USER_LIST_SIZE.
    ///
    /// # Arguments
    /// - `uids`: Pointer to array of userIds.
    /// - `max_uids`: Maximum number of userIds to return.
    /// - `actual_total`: The actual total number of userIds found.
    pub fn accountListAllUsers(uids: *mut AccountUid, max_uids: s32, actual_total: *mut s32) -> Result;

    /// Get the userId for the last opened user.
    pub fn accountGetLastOpenedUser(uid: *mut AccountUid) -> Result;

    /// Get an AccountProfile for the specified userId.
    pub fn accountGetProfile(out: *mut AccountProfile, uid: AccountUid) -> Result;

    /// IsUserRegistrationRequestPermitted
    pub fn accountIsUserRegistrationRequestPermitted(out: *mut bool) -> Result;

    /// TrySelectUserWithoutInteraction
    pub fn accountTrySelectUserWithoutInteraction(uid: *mut AccountUid, is_network_service_account_required: bool) -> Result;

    /// Close the AccountProfile.
    pub fn accountProfileClose(profile: *mut AccountProfile);

    /// Get `AccountUserData` and `AccountProfileBase` for the specified profile, userdata is optional (can be NULL).
    pub fn accountProfileGet(profile: *mut AccountProfile, userdata: *mut AccountUserData, profilebase: *mut AccountProfileBase) -> Result;

    /// Get the icon image size.
    pub fn accountProfileGetImageSize(profile: *mut AccountProfile, image_size: *mut u32) -> Result;

    /// Load the JPEG profile icon, valid for both Miis and character icons. The output image_size is the same as the one from `accountProfileGetImageSize`.
    pub fn accountProfileLoadImage(profile: *mut AccountProfile, buf: *mut c_void, len: usize, image_size: *mut u32) -> Result;

    /// Gets the userId which was selected by the profile-selector applet (if any), prior to launching the currently running Application.
    /// This gets the cached PreselectedUser loaded during accountInitialize, when PreselectedUser is available.
    pub fn accountGetPreselectedUser(uid: *mut AccountUid) -> Result;
}
