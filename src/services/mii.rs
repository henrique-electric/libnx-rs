//! `switch/services/mii.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub type MiiServiceType = u32;
/// Initializes mii:e.
pub const MiiServiceType_System: MiiServiceType = 0;
/// Initializes mii:u.
pub const MiiServiceType_User: MiiServiceType = 1;

/// Mii age.
pub type MiiAge = u32;
/// Young
pub const MiiAge_Young: MiiAge = 0;
/// Normal
pub const MiiAge_Normal: MiiAge = 1;
/// Old
pub const MiiAge_Old: MiiAge = 2;
/// All of them
pub const MiiAge_All: MiiAge = 3;

/// Mii gender.
pub type MiiGender = u32;
/// Male
pub const MiiGender_Male: MiiGender = 0;
/// Female
pub const MiiGender_Female: MiiGender = 1;
/// Both of them
pub const MiiGender_All: MiiGender = 2;

/// Mii face color.
pub type MiiFaceColor = u32;
/// Black
pub const MiiFaceColor_Black: MiiFaceColor = 0;
/// White
pub const MiiFaceColor_White: MiiFaceColor = 1;
/// Asian
pub const MiiFaceColor_Asian: MiiFaceColor = 2;
/// All of them
pub const MiiFaceColor_All: MiiFaceColor = 3;

pub type MiiSourceFlag = u32;
/// Miis created by the user
pub const MiiSourceFlag_Database: MiiSourceFlag = BIT(0);
/// Default console miis
pub const MiiSourceFlag_Default: MiiSourceFlag = BIT(1);
/// All of them
pub const MiiSourceFlag_All: MiiSourceFlag = MiiSourceFlag_Database | MiiSourceFlag_Default;

pub type MiiSpecialKeyCode = u32;
/// Normal miis
pub const MiiSpecialKeyCode_Normal: MiiSpecialKeyCode = 0;
/// Special miis
pub const MiiSpecialKeyCode_Special: MiiSpecialKeyCode = 0xA523B78F;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MiiDatabase {
    pub s: Service,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MiiCreateId {
    pub uuid: Uuid,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MiiCharInfo {
    pub create_id: MiiCreateId,
    /// utf-16be, null-terminated
    pub mii_name: [u16; 11],
    pub unk_x26: u8,
    pub mii_color: u8,
    pub mii_sex: u8,
    pub mii_height: u8,
    pub mii_width: u8,
    pub unk_x2b: [u8; 2],
    pub mii_face_shape: u8,
    pub mii_face_color: u8,
    pub mii_wrinkles_style: u8,
    pub mii_makeup_style: u8,
    pub mii_hair_style: u8,
    pub mii_hair_color: u8,
    pub mii_has_hair_flipped: u8,
    pub mii_eye_style: u8,
    pub mii_eye_color: u8,
    pub mii_eye_size: u8,
    pub mii_eye_thickness: u8,
    pub mii_eye_angle: u8,
    pub mii_eye_pos_x: u8,
    pub mii_eye_pos_y: u8,
    pub mii_eyebrow_style: u8,
    pub mii_eyebrow_color: u8,
    pub mii_eyebrow_size: u8,
    pub mii_eyebrow_thickness: u8,
    pub mii_eyebrow_angle: u8,
    pub mii_eyebrow_pos_x: u8,
    pub mii_eyebrow_pos_y: u8,
    pub mii_nose_style: u8,
    pub mii_nose_size: u8,
    pub mii_nose_pos: u8,
    pub mii_mouth_style: u8,
    pub mii_mouth_color: u8,
    pub mii_mouth_size: u8,
    pub mii_mouth_thickness: u8,
    pub mii_mouth_pos: u8,
    pub mii_facial_hair_color: u8,
    pub mii_beard_style: u8,
    pub mii_mustache_style: u8,
    pub mii_mustache_size: u8,
    pub mii_mustache_pos: u8,
    pub mii_glasses_style: u8,
    pub mii_glasses_color: u8,
    pub mii_glasses_size: u8,
    pub mii_glasses_pos: u8,
    pub mii_has_mole: u8,
    pub mii_mole_size: u8,
    pub mii_mole_pos_x: u8,
    pub mii_mole_pos_y: u8,
    pub unk_x57: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MiiStoreData {
    pub data: [u8; 68],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MiiVer3StoreData {
    pub data: [u8; 92],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MiiNfpStoreDataExtension {
    pub faceline_color: u8,
    pub hair_color: u8,
    pub eye_color: u8,
    pub eyebrow_color: u8,
    pub mouth_color: u8,
    pub beard_color: u8,
    pub glass_color: u8,
    pub glass_type: u8,
}

impl_zeroed_default!(MiiDatabase, MiiCreateId, MiiCharInfo, MiiStoreData, MiiVer3StoreData, MiiNfpStoreDataExtension);

unsafe extern "C" {
    /// Initialize mii.
    pub fn miiInitialize(service_type: MiiServiceType) -> Result;

    /// Exit mii.
    pub fn miiExit();

    /// Gets the Service object for the actual mii service session.
    pub fn miiGetServiceSession() -> *mut Service;

    /// Opens a mii database.
    ///
    /// # Arguments
    /// - `key_code` (in): Mii key code filter.
    /// - `out` (out): Database.
    pub fn miiOpenDatabase(out: *mut MiiDatabase, key_code: MiiSpecialKeyCode) -> Result;

    /// Returns whether the mii database is updated.
    ///
    /// # Arguments
    /// - `db` (in): Database.
    /// - `flag` (in): Source flag.
    /// - `out_updated` (out): Whether the mii database is updated.
    pub fn miiDatabaseIsUpdated(db: *mut MiiDatabase, out_updated: *mut bool, flag: MiiSourceFlag) -> Result;

    /// Returns whether the mii database is full.
    ///
    /// # Arguments
    /// - `db` (in): Database.
    /// - `out_full` (out): Whether the mii database is full.
    pub fn miiDatabaseIsFull(db: *mut MiiDatabase, out_full: *mut bool) -> Result;

    /// Returns number of miis in the database with the specified source flag.
    ///
    /// # Arguments
    /// - `db` (in): Database.
    /// - `flag` (in): Source flag.
    /// - `out_count` (out): Out mii count.
    pub fn miiDatabaseGetCount(db: *mut MiiDatabase, out_count: *mut s32, flag: MiiSourceFlag) -> Result;

    /// Reads mii charinfo data from the specified source flag.
    ///
    /// # Arguments
    /// - `db` (in): Database.
    /// - `flag` (in): Source flag.
    /// - `out_infos` (out): Output mii charinfo array.
    /// - `count` (in): Number of mii chainfos to read.
    /// - `total_out` (out): Number of mii charinfos which were actually read.
    pub fn miiDatabaseGet1(db: *mut MiiDatabase, flag: MiiSourceFlag, out_infos: *mut MiiCharInfo, count: s32, total_out: *mut s32) -> Result;

    /// Generates a random mii charinfo (doesn't register it in the console database).
    ///
    /// # Arguments
    /// - `db` (in): Database.
    /// - `age` (in): Mii's age.
    /// - `gender` (in): Mii's gender.
    /// - `face_color` (in): Mii's face color.
    /// - `out_info` (out): Out mii charinfo data.
    pub fn miiDatabaseBuildRandom(db: *mut MiiDatabase, age: MiiAge, gender: MiiGender, face_color: MiiFaceColor, out_info: *mut MiiCharInfo) -> Result;

    /// Closes a mii database.
    pub fn miiDatabaseClose(db: *mut MiiDatabase);
}
