//! `switch/services/pl.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub type PlServiceType = u32;
/// Initializes pl:u.
pub const PlServiceType_User: PlServiceType = 0;
/// Initializes pl:s. With [16.0.0+] SharedFont is no longer available with this, pl:u must be used for that.
pub const PlServiceType_System: PlServiceType = 1;

/// SharedFontType
pub type PlSharedFontType = u32;
/// Japan, US and Europe
pub const PlSharedFontType_Standard: PlSharedFontType = 0;
/// Chinese Simplified
pub const PlSharedFontType_ChineseSimplified: PlSharedFontType = 1;
/// Extended Chinese Simplified
pub const PlSharedFontType_ExtChineseSimplified: PlSharedFontType = 2;
/// Chinese Traditional
pub const PlSharedFontType_ChineseTraditional: PlSharedFontType = 3;
/// Korean (Hangul)
pub const PlSharedFontType_KO: PlSharedFontType = 4;
/// Nintendo Extended. This font only has the special Nintendo-specific characters, which aren't available with the other fonts.
pub const PlSharedFontType_NintendoExt: PlSharedFontType = 5;
/// Total fonts supported by this enum.
pub const PlSharedFontType_Total: PlSharedFontType = 6;

/// FontData
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PlFontData {
    /// **Ref:** PlSharedFontType
    pub type_: u32,
    /// Offset of the font in sharedmem.
    pub offset: u32,
    /// Size of the font.
    pub size: u32,
    /// Address of the actual font.
    pub address: *mut c_void,
}

impl_zeroed_default!(PlFontData);

unsafe extern "C" {
    /// Initialize pl.
    pub fn plInitialize(service_type: PlServiceType) -> Result;

    /// Exit pl.
    pub fn plExit();

    /// Gets the Service object for the actual pl service session.
    pub fn plGetServiceSession() -> *mut Service;

    /// Gets the address of the SharedMemory.
    pub fn plGetSharedmemAddr() -> *mut c_void;

    pub fn plGetSharedFontByType(font: *mut PlFontData, SharedFontType: PlSharedFontType) -> Result;

    pub fn plGetSharedFont(LanguageCode: u64, fonts: *mut PlFontData, max_fonts: s32, total_fonts: *mut s32) -> Result;
}
