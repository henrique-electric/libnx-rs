//! `switch/applets/hid_la.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Mode values for HidLaControllerSupportArgPrivate::mode.
pub type HidLaControllerSupportMode = u32;
/// ShowControllerSupport
pub const HidLaControllerSupportMode_ShowControllerSupport: HidLaControllerSupportMode = 0;
/// [3.0.0+] ShowControllerStrapGuide
pub const HidLaControllerSupportMode_ShowControllerStrapGuide: HidLaControllerSupportMode = 1;
/// [3.0.0+] ShowControllerFirmwareUpdate
pub const HidLaControllerSupportMode_ShowControllerFirmwareUpdate: HidLaControllerSupportMode = 2;
/// [11.0.0+] ShowControllerKeyRemappingForSystem
pub const HidLaControllerSupportMode_ShowControllerKeyRemappingForSystem: HidLaControllerSupportMode = 4;

/// ControllerSupportCaller
pub type HidLaControllerSupportCaller = u32;
/// Application, this is the default.
pub const HidLaControllerSupportCaller_Application: HidLaControllerSupportCaller = 0;
/// System. Skips the firmware-update confirmation dialog. This has the same affect as using the controller-update option from qlaunch System Settings.
pub const HidLaControllerSupportCaller_System: HidLaControllerSupportCaller = 1;

/// ControllerSupportArgPrivate
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidLaControllerSupportArgPrivate {
    /// Size of this ControllerSupportArgPrivate struct.
    pub private_size: u32,
    /// Size of the storage following this one (`HidLaControllerSupportArg` or `HidLaControllerFirmwareUpdateArg`).
    pub arg_size: u32,
    /// Flag0
    pub flag0: u8,
    /// Flag1
    pub flag1: u8,
    /// **Ref:** HidLaControllerSupportMode
    pub mode: u8,
    /// **Ref:** HidLaControllerSupportCaller. Always zero except with `hidLaShowControllerFirmwareUpdateForSystem`, which sets this to the input param.
    pub controller_support_caller: u8,
    /// Output from `hidGetSupportedNpadStyleSet`. With ShowControllerSupportForSystem on pre-3.0.0 this is value 0.
    pub npad_style_set: u32,
    /// Output from `hidGetNpadJoyHoldType`. With ShowControllerSupportForSystem on pre-3.0.0 this is value 1.
    pub npad_joy_hold_type: u32,
}

/// Common header used by HidLaControllerSupportArg*.
/// max_supported_players is 4 on pre-8.0.0, 8 on [8.0.0+]. player_count_min and player_count_max are overriden with value 4 when larger than value 4, during conversion handling for `HidLaControllerSupportArg` on pre-8.0.0.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidLaControllerSupportArgHeader {
    /// playerCountMin. Must be >=0 and &lt;=max_supported_players.
    pub player_count_min: s8,
    /// playerCountMax. Must be >=1 and &lt;=max_supported_players.
    pub player_count_max: s8,
    /// enableTakeOverConnection, non-zero to enable. Disconnects the controllers when not enabled.
    pub enable_take_over_connection: u8,
    /// enableLeftJustify, non-zero to enable.
    pub enable_left_justify: u8,
    /// enablePermitJoyDual, non-zero to enable.
    pub enable_permit_joy_dual: u8,
    /// enableSingleMode, non-zero to enable. Enables using a single player in handheld-mode, dual-mode, or single-mode (player_count_* are overridden). Using handheld-mode is not allowed if this is not enabled.
    pub enable_single_mode: u8,
    /// When non-zero enables using identification_color.
    pub enable_identification_color: u8,
}

/// Identification color used by HidLaControllerSupportArg*. When HidLaControllerSupportArgHeader::enable_identification_color is set this controls the color of the UI player box outline.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidLaControllerSupportArgColor {
    /// Red color component.
    pub r: u8,
    /// Green color component.
    pub g: u8,
    /// Blue color component.
    pub b: u8,
    /// Alpha color component.
    pub a: u8,
}

/// ControllerSupportArg for [1.0.0+].
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidLaControllerSupportArgV3 {
    /// **Ref:** HidLaControllerSupportArgHeader
    pub hdr: HidLaControllerSupportArgHeader,
    /// **Ref:** HidLaControllerSupportArgColor for each player, see HidLaControllerSupportArgHeader::enable_identification_color.
    pub identification_color: [HidLaControllerSupportArgColor; 4],
    /// Enables using the ExplainText data when non-zero.
    pub enable_explain_text: u8,
    /// ExplainText for each player, NUL-terminated UTF-8 strings.
    pub explain_text: [[c_char; 129]; 4],
}

/// ControllerSupportArg for [8.0.0+], converted to `HidLaControllerSupportArgV3` on pre-8.0.0.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidLaControllerSupportArg {
    /// **Ref:** HidLaControllerSupportArgHeader
    pub hdr: HidLaControllerSupportArgHeader,
    /// **Ref:** HidLaControllerSupportArgColor for each player, see HidLaControllerSupportArgHeader::enable_identification_color.
    pub identification_color: [HidLaControllerSupportArgColor; 8],
    /// Enables using the ExplainText data when non-zero.
    pub enable_explain_text: u8,
    /// ExplainText for each player, NUL-terminated UTF-8 strings.
    pub explain_text: [[c_char; 129]; 8],
}

/// ControllerFirmwareUpdateArg
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidLaControllerFirmwareUpdateArg {
    /// enableForceUpdate, non-zero to enable. Default is 0. Forces a firmware update when enabled, without an UI option to skip it.
    pub enable_force_update: u8,
    /// Padding.
    pub pad: [u8; 3],
}

/// ControllerKeyRemappingArg
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidLaControllerKeyRemappingArg {
    /// Unknown
    pub unk_x0: u64,
    /// Unknown
    pub unk_x8: u32,
    /// Padding
    pub pad: [u8; 4],
}

/// ControllerSupportResultInfo. First 8-bytes from the applet output storage.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidLaControllerSupportResultInfo {
    /// playerCount.
    pub player_count: s8,
    /// Padding.
    pub pad: [u8; 3],
    /// **Ref:** HidNpadIdType, selectedId.
    pub selected_id: u32,
}

/// Struct for the applet output storage.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidLaControllerSupportResultInfoInternal {
    /// **Ref:** HidLaControllerSupportResultInfo
    pub info: HidLaControllerSupportResultInfo,
    /// Output res value.
    pub res: u32,
}

impl_zeroed_default!(HidLaControllerSupportArgPrivate, HidLaControllerSupportArgHeader, HidLaControllerSupportArgColor, HidLaControllerSupportArgV3, HidLaControllerSupportArg, HidLaControllerFirmwareUpdateArg, HidLaControllerKeyRemappingArg, HidLaControllerSupportResultInfo, HidLaControllerSupportResultInfoInternal);

unsafe extern "C" {
    /// Initializes a `HidLaControllerSupportArg` with the defaults.
    ///
    /// # Arguments
    /// - `arg` (out): `HidLaControllerSupportArg`
    ///
    /// **Note:** This clears the arg, then does: HidLaControllerSupportArgHeader::player_count_min = 0, HidLaControllerSupportArgHeader::player_count_max = 4, HidLaControllerSupportArgHeader::enable_take_over_connection = 1, HidLaControllerSupportArgHeader::enable_left_justify = 1, and HidLaControllerSupportArgHeader::enable_permit_joy_dual = 1.
    ///
    /// **Note:** If preferred, you can also memset `HidLaControllerSupportArg` manually and initialize it yourself.
    pub fn hidLaCreateControllerSupportArg(arg: *mut HidLaControllerSupportArg);

    /// Initializes a `HidLaControllerFirmwareUpdateArg` with the defaults.
    ///
    /// # Arguments
    /// - `arg` (out): `HidLaControllerFirmwareUpdateArg`
    ///
    /// **Note:** This just uses memset() with the arg.
    pub fn hidLaCreateControllerFirmwareUpdateArg(arg: *mut HidLaControllerFirmwareUpdateArg);

    /// Initializes a `HidLaControllerKeyRemappingArg` with the defaults.
    ///
    /// # Arguments
    /// - `arg` (out): `HidLaControllerKeyRemappingArg`
    ///
    /// **Note:** This just uses memset() with the arg.
    pub fn hidLaCreateControllerKeyRemappingArg(arg: *mut HidLaControllerKeyRemappingArg);

    /// Sets the ExplainText for the specified player and `HidLaControllerSupportArg`.
    ///
    /// # Arguments
    /// - `arg`: `HidLaControllerSupportArg`
    /// - `str` (in): Input ExplainText UTF-8 string, max length is 0x80 excluding NUL-terminator.
    ///   + @oaram\[in\] id Player controller, must be &lt;8.
    ///
    /// **Note:** This string is displayed in the UI box for the player.
    ///
    /// **Note:** HidLaControllerSupportArg::enable_explain_text must be set, otherwise this ExplainText is ignored.
    pub fn hidLaSetExplainText(arg: *mut HidLaControllerSupportArg, str: *const c_char, id: HidNpadIdType) -> Result;

    /// Launches the applet for ControllerSupport.
    ///
    /// # Arguments
    /// - `result_info` (out): `HidLaControllerSupportResultInfo`. Optional, can be NULL.
    /// - `arg` (in): `HidLaControllerSupportArg`
    ///
    /// **Note:** This seems to only display the applet UI when doing so is actually needed? This doesn't apply to `hidLaShowControllerSupportForSystem`.
    pub fn hidLaShowControllerSupport(result_info: *mut HidLaControllerSupportResultInfo, arg: *const HidLaControllerSupportArg) -> Result;

    /// Launches the applet for ControllerStrapGuide.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn hidLaShowControllerStrapGuide() -> Result;

    /// Launches the applet for ControllerFirmwareUpdate.
    ///
    /// # Arguments
    /// - `arg` (in): `HidLaControllerFirmwareUpdateArg`
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn hidLaShowControllerFirmwareUpdate(arg: *const HidLaControllerFirmwareUpdateArg) -> Result;

    /// This is the system version of `hidLaShowControllerSupport`.
    ///
    /// # Arguments
    /// - `result_info` (out): `HidLaControllerSupportResultInfo`. Optional, can be NULL.
    /// - `arg` (in): `HidLaControllerSupportArg`
    /// - `flag` (in): Input flag. When true, the applet displays the menu as if launched by qlaunch.
    pub fn hidLaShowControllerSupportForSystem(result_info: *mut HidLaControllerSupportResultInfo, arg: *const HidLaControllerSupportArg, flag: bool) -> Result;

    /// This is the system version of `hidLaShowControllerFirmwareUpdate`.
    ///
    /// # Arguments
    /// - `arg` (in): `HidLaControllerFirmwareUpdateArg`
    /// - `caller` (in): `HidLaControllerSupportCaller`
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn hidLaShowControllerFirmwareUpdateForSystem(arg: *const HidLaControllerFirmwareUpdateArg, caller: HidLaControllerSupportCaller) -> Result;

    /// Launches the applet for ControllerKeyRemappingForSystem.
    ///
    /// # Arguments
    /// - `arg` (in): `HidLaControllerKeyRemappingArg`
    /// - `caller` (in): `HidLaControllerSupportCaller`
    ///
    /// **Note:** Only available on [11.0.0+].
    pub fn hidLaShowControllerKeyRemappingForSystem(arg: *const HidLaControllerKeyRemappingArg, caller: HidLaControllerSupportCaller) -> Result;
}
