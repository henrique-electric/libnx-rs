//! `switch/applets/psel.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// playerSelect UI modes.
pub type PselUiMode = u32;
/// UserSelector
pub const PselUiMode_UserSelector: PselUiMode = 0;
/// UserCreator
pub const PselUiMode_UserCreator: PselUiMode = 1;
/// EnsureNetworkServiceAccountAvailable
pub const PselUiMode_EnsureNetworkServiceAccountAvailable: PselUiMode = 2;
/// UserIconEditor
pub const PselUiMode_UserIconEditor: PselUiMode = 3;
/// UserNicknameEditor
pub const PselUiMode_UserNicknameEditor: PselUiMode = 4;
/// UserCreatorForStarter
pub const PselUiMode_UserCreatorForStarter: PselUiMode = 5;
/// NintendoAccountAuthorizationRequestContext
pub const PselUiMode_NintendoAccountAuthorizationRequestContext: PselUiMode = 6;
/// IntroduceExternalNetworkServiceAccount
pub const PselUiMode_IntroduceExternalNetworkServiceAccount: PselUiMode = 7;
/// [6.0.0+] IntroduceExternalNetworkServiceAccountForRegistration
pub const PselUiMode_IntroduceExternalNetworkServiceAccountForRegistration: PselUiMode = 8;
/// [6.0.0+] NintendoAccountNnidLinker
pub const PselUiMode_NintendoAccountNnidLinker: PselUiMode = 9;
/// [6.0.0+] LicenseRequirementsForNetworkService
pub const PselUiMode_LicenseRequirementsForNetworkService: PselUiMode = 10;
/// [7.0.0+] LicenseRequirementsForNetworkServiceWithUserContextImpl
pub const PselUiMode_LicenseRequirementsForNetworkServiceWithUserContextImpl: PselUiMode = 11;
/// [7.0.0+] UserCreatorForImmediateNaLoginTest
pub const PselUiMode_UserCreatorForImmediateNaLoginTest: PselUiMode = 12;
/// [13.0.0+] UserQualificationPromoter
pub const PselUiMode_UserQualificationPromoter: PselUiMode = 13;

/// UI message text to display with ::PselUiMode_UserSelector. Invalid values are handled as ::PselUserSelectionPurpose_General.
pub type PselUserSelectionPurpose = u32;
/// "Select a user."
pub const PselUserSelectionPurpose_General: PselUserSelectionPurpose = 0;
/// [2.0.0+] "Who will receive the points?"
pub const PselUserSelectionPurpose_GameCardRegistration: PselUserSelectionPurpose = 1;
/// [2.0.0+] "Who is using Nintendo eShop?"
pub const PselUserSelectionPurpose_EShopLaunch: PselUserSelectionPurpose = 2;
/// [2.0.0+] "Who is making this purchase?"
pub const PselUserSelectionPurpose_EShopItemShow: PselUserSelectionPurpose = 3;
/// [2.0.0+] "Who is posting?"
pub const PselUserSelectionPurpose_PicturePost: PselUserSelectionPurpose = 4;
/// [2.0.0+] "Select a user to link to a Nintendo Account."
pub const PselUserSelectionPurpose_NintendoAccountLinkage: PselUserSelectionPurpose = 5;
/// [2.0.0+] "Change settings for which user?"
pub const PselUserSelectionPurpose_SettingsUpdate: PselUserSelectionPurpose = 6;
/// [2.0.0+] "Format data for which user?"
pub const PselUserSelectionPurpose_SaveDataDeletion: PselUserSelectionPurpose = 7;
/// [4.0.0+] "Which user will be transferred to another console?"
pub const PselUserSelectionPurpose_UserMigration: PselUserSelectionPurpose = 8;
/// [8.0.0+] "Send save data for which user?"
pub const PselUserSelectionPurpose_SaveDataTransfer: PselUserSelectionPurpose = 9;

/// NintendoAccountStartupDialogType
pub type PselNintendoAccountStartupDialogType = u32;
/// LoginAndCreate
pub const PselNintendoAccountStartupDialogType_LoginAndCreate: PselNintendoAccountStartupDialogType = 0;
/// Login
pub const PselNintendoAccountStartupDialogType_Login: PselNintendoAccountStartupDialogType = 1;
/// Create
pub const PselNintendoAccountStartupDialogType_Create: PselNintendoAccountStartupDialogType = 2;

/// Base UI settings for playerSelect.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PselUiSettingsV1 {
    /// **Ref:** PselUiMode
    pub mode: u32,
    /// Padding.
    pub pad: u32,
    /// List of `AccountUid`. TODO: This is only correct for ::PselUiMode_UserSelector, for other modes this is a single uid, followed by mode-specific data (if any).
    pub invalid_uid_list: [AccountUid; 8],
    /// ApplicationId with `pselShowUserSelectorForLauncher`.
    pub application_id: u64,
    /// PselUserSelectionSettings::is_network_service_account_required.
    pub is_network_service_account_required: u8,
    /// PselUserSelectionSettings::is_skip_enabled
    pub is_skip_enabled: u8,
    /// Set to value 1 by `pselShowUserSelectorForSystem` / `pselShowUserSelectorForLauncher`.
    pub unk_x92: u8,
    /// isPermitted. With ::PselUiMode_UserSelector: enables the option to create a new user. Set to the output from `accountIsUserRegistrationRequestPermitted` with pselShowUserSelector*. When not set, a dialog will be displayed when the user attempts to create an user.
    pub is_permitted: u8,
    /// PselUserSelectionSettings::show_skip_button
    pub show_skip_button: u8,
    /// PselUserSelectionSettings::additional_select
    pub additional_select: u8,
    /// [2.0.0+] Set to PselUserSelectionSettingsForSystemService::enable_user_creation_button. `pselShowUserSelectorForLauncher` / `pselShowUserSelector` sets this to value 1.
    pub unk_x96: u8,
    /// [6.0.0+] Set to PselUserSelectionSettings::is_unqualified_user_selectable ^ 1.
    pub unk_x97: u8,
}

/// UI settings for versions starting with 0x10000.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PselUiSettings {
    /// **Ref:** PselUiSettingsV1
    pub settings: PselUiSettingsV1,
    /// [2.0.0+] Set to PselUserSelectionSettingsForSystemService::purpose.
    pub unk_x98: u32,
    /// Unknown.
    pub unk_x9c: [u8; 4],
}

/// UserSelectionSettings
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PselUserSelectionSettings {
    /// invalidUidList.
    pub invalid_uid_list: [AccountUid; 8],
    /// isSkipEnabled. When set, the first user in invalid_uid_list must not be set, and additional_select must be 0. When enabled `accountTrySelectUserWithoutInteraction` will be used to select the user, in this case the applet will only be launched if `accountTrySelectUserWithoutInteraction` doesn't return an user.
    pub is_skip_enabled: u8,
    /// isNetworkServiceAccountRequired. Whether the user needs to be linked to a Nintendo account.
    pub is_network_service_account_required: u8,
    /// showSkipButton. Enables the option to skip user selection with a button.
    pub show_skip_button: u8,
    /// additionalSelect.
    pub additional_select: u8,
    /// [6.0.0+] isUnqualifiedUserSelectable
    pub is_unqualified_user_selectable: u8,
}

/// [2.0.0+] UserSelectionSettingsForSystemService
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PselUserSelectionSettingsForSystemService {
    /// **Ref:** PselUserSelectionPurpose
    pub purpose: u32,
    /// Enables the user-creation button when set. Whether user-creation when pressing the button is actually allowed is controlled by PselUiSettingsV1::is_permitted.
    pub enable_user_creation_button: u8,
    /// Padding.
    pub pad: [u8; 3],
}

/// Return data sent after execution.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PselUiReturnArg {
    /// Result.
    pub res: Result,
    /// Selected `AccountUid`.
    pub user_id: AccountUid,
}

/// Sets whether users can be created in the applet.
///
/// # Arguments
/// - `ui`: PselUiSettings struct.
/// - `flag`: Flag value.
///
/// **Note:** Only used for ::PselUiMode_SelectUser.
#[inline(always)]
pub fn pselUiSetAllowUserCreation(ui: &mut PselUiSettings, flag: bool) {
    if ui.settings.mode == PselUiMode_UserSelector {
        ui.settings.is_permitted = flag as u8;
    }
}

/// Sets whether users need to be linked to a Nintendo account.
///
/// # Arguments
/// - `ui`: PselUiSettings struct.
/// - `flag`: Flag value.
#[inline(always)]
pub fn pselUiSetNetworkServiceRequired(ui: &mut PselUiSettings, flag: bool) {
    ui.settings.is_network_service_account_required = flag as u8;
}

/// Sets whether selection can be skipped with a button.
///
/// # Arguments
/// - `ui`: PselUiSettings struct.
/// - `flag`: Flag value.
#[inline(always)]
pub fn pselUiSetSkipButtonEnabled(ui: &mut PselUiSettings, flag: bool) {
    ui.settings.show_skip_button = flag as u8;
}

impl_zeroed_default!(PselUiSettingsV1, PselUiSettings, PselUserSelectionSettings, PselUserSelectionSettingsForSystemService, PselUiReturnArg);

unsafe extern "C" {
    /// Creates a new UI config for the playerSelect applet with the specified mode.
    ///
    /// # Arguments
    /// - `ui`: PseluiSettings struct.
    /// - `mode`: playerSelect UI mode.
    pub fn pselUiCreate(ui: *mut PselUiSettings, mode: PselUiMode) -> Result;

    /// Adds an user to the user list of the applet.
    ///
    /// # Arguments
    /// - `ui`: PselUiSettings struct.
    /// - `user_id` (in): user ID.
    ///
    /// **Note:** The users will be treated as invalid users for user selection mode, and as the input user for other modes.
    pub fn pselUiAddUser(ui: *mut PselUiSettings, user_id: AccountUid);

    /// Shows the applet with the specified UI settings.
    ///
    /// # Arguments
    /// - `ui`: PselUiSettings struct.
    /// - `out_user`: Selected user ID.
    ///
    /// **Note:** If user skips (see `pselUiSetSkipEnabled`) this will return successfully but the output ID will be 0.
    pub fn pselUiShow(ui: *mut PselUiSettings, out_user: *mut AccountUid) -> Result;

    /// This is the System version of `pselShowUserSelector`.
    ///
    /// # Arguments
    /// - `out_user` (out): Returned selected user ID.
    /// - `settings` (in): `PselUserSelectionSettings`
    /// - `settings_system` (in): [2.0.0+] `PselUserSelectionSettingsForSystemService`, ignored on prior versions.
    ///
    /// **Note:** This uses `accountIsUserRegistrationRequestPermitted`, hence `accountInitialize` must be used prior to this. See also the docs for PselUserSelectionSettings::is_skip_enabled.
    pub fn pselShowUserSelectorForSystem(out_user: *mut AccountUid, settings: *const PselUserSelectionSettings, settings_system: *const PselUserSelectionSettingsForSystemService) -> Result;

    /// This is the Launcher version of `pselShowUserSelector`.
    ///
    /// # Arguments
    /// - `out_user` (out): Returned selected user ID.
    /// - `settings` (in): `PselUserSelectionSettings`
    /// - `application_id` (in): ApplicationId
    ///
    /// **Note:** This uses `accountIsUserRegistrationRequestPermitted`, hence `accountInitialize` must be used prior to this. See also the docs for PselUserSelectionSettings::is_skip_enabled.
    pub fn pselShowUserSelectorForLauncher(out_user: *mut AccountUid, settings: *const PselUserSelectionSettings, application_id: u64) -> Result;

    /// Shows the applet to select a user.
    ///
    /// # Arguments
    /// - `out_user` (out): Returned selected user ID.
    /// - `settings` (in): `PselUserSelectionSettings`
    ///
    /// **Note:** This uses `accountIsUserRegistrationRequestPermitted`, hence `accountInitialize` must be used prior to this. See also the docs for PselUserSelectionSettings::is_skip_enabled.
    pub fn pselShowUserSelector(out_user: *mut AccountUid, settings: *const PselUserSelectionSettings) -> Result;

    /// Shows the applet to create a user.
    ///
    /// **Note:** This uses `accountIsUserRegistrationRequestPermitted`, hence `accountInitialize` must be used prior to this. If the output flag is 0, an error will be thrown.
    pub fn pselShowUserCreator() -> Result;

    /// Shows the applet to change a user's icon.
    ///
    /// # Arguments
    /// - `user` (in): Input user ID.
    pub fn pselShowUserIconEditor(user: AccountUid) -> Result;

    /// Shows the applet to change a user's nickname.
    ///
    /// # Arguments
    /// - `user` (in): Input user ID.
    pub fn pselShowUserNicknameEditor(user: AccountUid) -> Result;

    /// Shows the applet to create a user. Used by the starter applet during system setup.
    pub fn pselShowUserCreatorForStarter() -> Result;

    /// Shows the applet for Nintendo Account Nnid linking.
    ///
    /// # Arguments
    /// - `user` (in): Input user ID.
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn pselShowNintendoAccountNnidLinker(user: AccountUid) -> Result;

    /// Shows the applet for UserQualificationPromoter.
    ///
    /// # Arguments
    /// - `user` (in): Input user ID.
    ///
    /// **Note:** Only available on [13.0.0+].
    pub fn pselShowUserQualificationPromoter(user: AccountUid) -> Result;
}
