//! `switch/applets/mii_la.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// AppletMode
pub type MiiLaAppletMode = u32;
/// ShowMiiEdit
pub const MiiLaAppletMode_ShowMiiEdit: MiiLaAppletMode = 0;
/// AppendMii
pub const MiiLaAppletMode_AppendMii: MiiLaAppletMode = 1;
/// AppendMiiImage
pub const MiiLaAppletMode_AppendMiiImage: MiiLaAppletMode = 2;
/// UpdateMiiImage
pub const MiiLaAppletMode_UpdateMiiImage: MiiLaAppletMode = 3;
/// [10.2.0+] CreateMii
pub const MiiLaAppletMode_CreateMii: MiiLaAppletMode = 4;
/// [10.2.0+] EditMii
pub const MiiLaAppletMode_EditMii: MiiLaAppletMode = 5;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MiiLaAppletInput_anon0_char_info {
    /// **Ref:** MiiCharInfo
    pub char_info: MiiCharInfo,
    /// Unused
    pub unused_x64: [u8; 40],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union MiiLaAppletInput_anon0 {
    /// ValidUuidArray. Only used with `MiiLaAppletMode` ::NfpLaMiiLaAppletMode_AppendMiiImage / ::NfpLaMiiLaAppletMode_UpdateMiiImage.
    pub valid_uuid_array: [Uuid; 8],
    pub char_info: MiiLaAppletInput_anon0_char_info,
}

/// AppletInput
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MiiLaAppletInput {
    /// Version
    pub version: s32,
    /// **Ref:** MiiLaAppletMode
    pub mode: u32,
    /// **Ref:** MiiSpecialKeyCode
    pub special_key_code: s32,
    pub anon0: MiiLaAppletInput_anon0,
    /// UsedUuid. Only used with `MiiLaAppletMode` ::NfpLaMiiLaAppletMode_UpdateMiiImage.
    pub used_uuid: Uuid,
    /// Unused
    pub unk_x9C: [u8; 100],
}

/// AppletOutput
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MiiLaAppletOutput {
    /// Result: 0 = Success, 1 = Cancel.
    pub res: u32,
    /// Index. Only set when Result is Success, where `MiiLaAppletMode` isn't ::NfpLaMiiLaAppletMode_ShowMiiEdit.
    pub index: s32,
    /// Unused
    pub unk_x8: [u8; 24],
}

/// AppletOutputForCharInfoEditing
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MiiLaAppletOutputForCharInfoEditing {
    /// MiiLaAppletOutput::res
    pub res: u32,
    /// **Ref:** MiiCharInfo
    pub char_info: MiiCharInfo,
    /// Unused
    pub unused: [u8; 36],
}

impl_zeroed_default!(MiiLaAppletInput_anon0_char_info, MiiLaAppletInput_anon0, MiiLaAppletInput, MiiLaAppletOutput, MiiLaAppletOutputForCharInfoEditing);

unsafe extern "C" {
    /// Launches the applet for ShowMiiEdit.
    ///
    /// # Arguments
    /// - `special_key_code` (in): `MiiSpecialKeyCode`
    pub fn miiLaShowMiiEdit(special_key_code: MiiSpecialKeyCode) -> Result;

    /// Launches the applet for AppendMii.
    ///
    /// # Arguments
    /// - `special_key_code` (in): `MiiSpecialKeyCode`
    /// - `index` (out): Output Index.
    pub fn miiLaAppendMii(special_key_code: MiiSpecialKeyCode, index: *mut s32) -> Result;

    /// Launches the applet for AppendMiiImage.
    ///
    /// # Arguments
    /// - `special_key_code` (in): `MiiSpecialKeyCode`
    /// - `valid_uuid_array` (in): Input array of Uuid.
    /// - `count` (in): Total entries for the valid_uuid_array. Must be 0-8.
    /// - `index` (out): Output Index.
    pub fn miiLaAppendMiiImage(special_key_code: MiiSpecialKeyCode, valid_uuid_array: *const Uuid, count: s32, index: *mut s32) -> Result;

    /// Launches the applet for UpdateMiiImage.
    ///
    /// # Arguments
    /// - `special_key_code` (in): `MiiSpecialKeyCode`
    /// - `valid_uuid_array` (in): Input array of Uuid.
    /// - `count` (in): Total entries for the valid_uuid_array. Must be 0-8.
    /// - `used_uuid` (in): UsedUuid
    /// - `index` (out): Output Index.
    pub fn miiLaUpdateMiiImage(special_key_code: MiiSpecialKeyCode, valid_uuid_array: *const Uuid, count: s32, used_uuid: Uuid, index: *mut s32) -> Result;

    /// Launches the applet for CreateMii.
    ///
    /// # Arguments
    /// - `special_key_code` (in): `MiiSpecialKeyCode`
    /// - `out_char` (out): `MiiCharInfo`
    ///
    /// **Note:** This creates a Mii and returns it, without saving it in the database.
    ///
    /// **Note:** Only available on [10.2.0+].
    pub fn miiLaCreateMii(special_key_code: MiiSpecialKeyCode, out_char: *mut MiiCharInfo) -> Result;

    /// Launches the applet for EditMii.
    ///
    /// # Arguments
    /// - `special_key_code` (in): `MiiSpecialKeyCode`
    /// - `in_char` (in): `MiiCharInfo`
    /// - `out_char` (out): `MiiCharInfo`
    ///
    /// **Note:** This edits the specified Mii and returns it, without saving it in the database.
    ///
    /// **Note:** Only available on [10.2.0+].
    pub fn miiLaEditMii(special_key_code: MiiSpecialKeyCode, in_char: *const MiiCharInfo, out_char: *mut MiiCharInfo) -> Result;
}
