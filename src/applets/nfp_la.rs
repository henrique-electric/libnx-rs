//! `switch/applets/nfp_la.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Values for NfpLaStartParamForAmiiboSettings::type.
pub type NfpLaStartParamTypeForAmiiboSettings = u32;
/// NicknameAndOwnerSettings
pub const NfpLaStartParamTypeForAmiiboSettings_NicknameAndOwnerSettings: NfpLaStartParamTypeForAmiiboSettings = 0;
/// GameDataEraser
pub const NfpLaStartParamTypeForAmiiboSettings_GameDataEraser: NfpLaStartParamTypeForAmiiboSettings = 1;
/// Restorer
pub const NfpLaStartParamTypeForAmiiboSettings_Restorer: NfpLaStartParamTypeForAmiiboSettings = 2;
/// Formatter
pub const NfpLaStartParamTypeForAmiiboSettings_Formatter: NfpLaStartParamTypeForAmiiboSettings = 3;

/// AmiiboSettingsStartParam
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NfpLaAmiiboSettingsStartParam {
    /// Unknown
    pub unk_x0: [u8; 8],
    /// Unknown
    pub unk_x8: [u8; 32],
    /// Unknown
    pub unk_x28: u8,
}

/// StartParamForAmiiboSettings
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NfpLaStartParamForAmiiboSettings {
    /// Unknown
    pub unk_x0: u8,
    /// **Ref:** NfpLaStartParamTypeForAmiiboSettings
    pub type_: u8,
    /// Flags
    pub flags: u8,
    /// NfpLaAmiiboSettingsStartParam::unk_x28
    pub unk_x3: u8,
    /// NfpLaAmiiboSettingsStartParam::unk_x0
    pub unk_x4: [u8; 8],
    /// **Ref:** NfpTagInfo, only enabled when flags bit1 is set.
    pub tag_info: NfpTagInfo,
    /// **Ref:** NfpRegisterInfo, only enabled when flags bit2 is set.
    pub register_info: NfpRegisterInfo,
    /// NfpLaAmiiboSettingsStartParam::unk_x8
    pub unk_x164: [u8; 32],
    /// Unknown
    pub unk_x184: [u8; 36],
}

/// ReturnValueForAmiiboSettings
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NfpLaReturnValueForAmiiboSettings {
    /// 0 = error, non-zero = success.
    pub flags: u8,
    /// Padding
    pub pad: [u8; 3],
    /// **Ref:** NfcDeviceHandle
    pub handle: NfcDeviceHandle,
    /// **Ref:** NfpTagInfo
    pub tag_info: NfpTagInfo,
    /// **Ref:** NfpRegisterInfo, only available when flags bit2 is set.
    pub register_info: NfpRegisterInfo,
    /// Unknown
    pub unk_x164: [u8; 36],
}

impl_zeroed_default!(NfpLaAmiiboSettingsStartParam, NfpLaStartParamForAmiiboSettings, NfpLaReturnValueForAmiiboSettings);

unsafe extern "C" {
    /// Launches the applet for NicknameAndOwnerSettings.
    ///
    /// # Arguments
    /// - `in_param` (in): `NfpLaAmiiboSettingsStartParam`
    /// - `in_tag_info` (in): `NfpTagInfo`. Optional, can be NULL. If specified, this must match the scanned amiibo.
    /// - `in_reg_info` (in): `NfpRegisterInfo`. Optional, can be NULL. If specified, this sets the `NfpRegisterInfo` which will be used for writing, with an option for the user to change it.
    /// - `out_tag_info` (out): `NfpTagInfo`. Optional, can be NULL.
    /// - `handle` (out): `NfcDeviceHandle`
    /// - `reg_info_flag` (out): Flag indicating whether the data for out_reg_info is set. Optional, can be NULL.
    /// - `out_reg_info` (out): `NfpRegisterInfo`, see reg_info_flag. Optional, can be NULL.
    ///
    /// **Note:** Official sw does not expose functionality for using input/output `NfpTagInfo` at the same time.
    pub fn nfpLaStartNicknameAndOwnerSettings(in_param: *const NfpLaAmiiboSettingsStartParam, in_tag_info: *const NfpTagInfo, in_reg_info: *const NfpRegisterInfo, out_tag_info: *mut NfpTagInfo, handle: *mut NfcDeviceHandle, reg_info_flag: *mut bool, out_reg_info: *mut NfpRegisterInfo) -> Result;

    /// Launches the applet for GameDataEraser.
    ///
    /// # Arguments
    /// - `in_param` (in): `NfpLaAmiiboSettingsStartParam`
    /// - `in_tag_info` (in): `NfpTagInfo`. Optional, can be NULL. If specified, this must match the scanned amiibo.
    /// - `out_tag_info` (out): `NfpTagInfo`. Optional, can be NULL.
    /// - `handle` (out): `NfcDeviceHandle`
    ///
    /// **Note:** Official sw does not expose functionality for using input/output `NfpTagInfo` at the same time.
    pub fn nfpLaStartGameDataEraser(in_param: *const NfpLaAmiiboSettingsStartParam, in_tag_info: *const NfpTagInfo, out_tag_info: *mut NfpTagInfo, handle: *mut NfcDeviceHandle) -> Result;

    /// Launches the applet for Restorer.
    ///
    /// # Arguments
    /// - `in_param` (in): `NfpLaAmiiboSettingsStartParam`
    /// - `in_tag_info` (in): `NfpTagInfo`. Optional, can be NULL. If specified, this must match the scanned amiibo.
    /// - `out_tag_info` (out): `NfpTagInfo`. Optional, can be NULL.
    /// - `handle` (out): `NfcDeviceHandle`
    ///
    /// **Note:** Official sw does not expose functionality for using input/output `NfpTagInfo` at the same time.
    pub fn nfpLaStartRestorer(in_param: *const NfpLaAmiiboSettingsStartParam, in_tag_info: *const NfpTagInfo, out_tag_info: *mut NfpTagInfo, handle: *mut NfcDeviceHandle) -> Result;

    /// Launches the applet for Formatter.
    ///
    /// # Arguments
    /// - `in_param` (in): `NfpLaAmiiboSettingsStartParam`
    /// - `out_tag_info` (out): `NfpTagInfo`
    /// - `handle` (out): `NfcDeviceHandle`
    pub fn nfpLaStartFormatter(in_param: *const NfpLaAmiiboSettingsStartParam, out_tag_info: *mut NfpTagInfo, handle: *mut NfcDeviceHandle) -> Result;
}
