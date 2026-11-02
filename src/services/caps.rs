//! `switch/services/caps.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// ImageOrientation
pub type AlbumImageOrientation = u32;
/// Unknown. Default.
pub const AlbumImageOrientation_Unknown0: AlbumImageOrientation = 0;
/// Unknown.
pub const AlbumImageOrientation_Unknown1: AlbumImageOrientation = 1;
/// Unknown.
pub const AlbumImageOrientation_Unknown2: AlbumImageOrientation = 2;
/// Unknown.
pub const AlbumImageOrientation_Unknown3: AlbumImageOrientation = 3;

/// AlbumReportOption
pub type AlbumReportOption = u32;
/// Don't display the screenshot-taken Overlay-applet notification.
pub const AlbumReportOption_Disable: AlbumReportOption = 0;
/// Display the screenshot-taken Overlay notification.
pub const AlbumReportOption_Enable: AlbumReportOption = 1;

pub type CapsAlbumStorage = u32;
/// Nand
pub const CapsAlbumStorage_Nand: CapsAlbumStorage = 0;
/// Sd
pub const CapsAlbumStorage_Sd: CapsAlbumStorage = 1;

/// ContentType
pub type CapsContentType = u32;
/// Album screenshots.
pub const CapsContentType_Screenshot: CapsContentType = 0;
/// Album videos.
pub const CapsContentType_Movie: CapsContentType = 1;
/// Videos recorded by the current host Application via `grcCreateMovieMaker`.
pub const CapsContentType_ExtraMovie: CapsContentType = 3;

/// ScreenShotAttribute
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CapsScreenShotAttribute {
    /// Always set to 0 by official sw.
    pub unk_x0: u32,
    /// **Ref:** AlbumImageOrientation
    pub orientation: u32,
    /// Always set to 0 by official sw.
    pub unk_x8: u32,
    /// Always set to 1 by official sw.
    pub unk_xc: u32,
    /// Always set to 0 by official sw.
    pub unk_x10: [u8; 48],
}

/// ScreenShotAttributeForApplication. Only unk_x0 is used by official sw.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CapsScreenShotAttributeForApplication {
    /// Unknown.
    pub unk_x0: u32,
    /// Unknown.
    pub unk_x4: u8,
    /// Unknown.
    pub unk_x5: u8,
    /// Unknown.
    pub unk_x6: u8,
    /// Padding.
    pub pad: u8,
    /// Unknown.
    pub unk_x8: u32,
    /// Unknown.
    pub unk_xc: u32,
    /// Unknown.
    pub unk_x10: u32,
    /// Unknown.
    pub unk_x14: u32,
    /// Unknown.
    pub unk_x18: u32,
    /// Unknown.
    pub unk_x1c: u32,
    /// Unknown.
    pub unk_x20: u16,
    /// Unknown.
    pub unk_x22: u16,
    /// Unknown.
    pub unk_x24: u16,
    /// Unknown.
    pub unk_x26: u16,
    /// Always zero.
    pub reserved: [u8; 24],
}

/// ScreenShotDecoderFlag
pub type CapsScreenShotDecoderFlag = u32;
/// No special processing.
pub const CapsScreenShotDecoderFlag_None: CapsScreenShotDecoderFlag = 0;
/// See libjpeg-turbo do_fancy_upsampling.
pub const CapsScreenShotDecoderFlag_EnableFancyUpsampling: CapsScreenShotDecoderFlag = BIT(0);
/// See libjpeg-turbo do_block_smoothing.
pub const CapsScreenShotDecoderFlag_EnableBlockSmoothing: CapsScreenShotDecoderFlag = BIT(1);

/// ScreenShotDecodeOption
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CapsScreenShotDecodeOption {
    /// Bitflags, see `CapsScreenShotDecoderFlag`.
    pub flags: u64,
    /// Reserved. Unused by official sw.
    pub reserved: [u64; 3],
}

/// AlbumFileDateTime. This corresponds to each field in the Album entry filename, prior to the "-": "YYYYMMDDHHMMSSII".
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CapsAlbumFileDateTime {
    /// Year.
    pub year: u16,
    /// Month.
    pub month: u8,
    /// Day of the month.
    pub day: u8,
    /// Hour.
    pub hour: u8,
    /// Minute.
    pub minute: u8,
    /// Second.
    pub second: u8,
    /// Unique ID for when there's multiple Album files with the same timestamp.
    pub id: u8,
}

/// AlbumEntryId
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CapsAlbumFileId {
    /// ApplicationId
    pub application_id: u64,
    /// **Ref:** CapsAlbumFileDateTime
    pub datetime: CapsAlbumFileDateTime,
    /// **Ref:** CapsAlbumStorage
    pub storage: u8,
    /// **Ref:** CapsAlbumFileContents
    pub content: u8,
    /// [19.0.0+]
    pub unknown_12: u8,
    /// [19.0.0+]
    pub unknown_13: u8,
    /// padding
    pub pad_x14: [u8; 4],
}

/// AlbumEntry
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CapsAlbumEntry {
    /// Size.
    pub size: u64,
    /// **Ref:** CapsAlbumFileId
    pub file_id: CapsAlbumFileId,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CapsApplicationAlbumEntry_anon0_v0 {
    /// aes256 with random key over `AlbumEntry`.
    pub unk_x0: [u8; 32],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CapsApplicationAlbumEntry_anon0_v1 {
    /// size of the entry
    pub size: u64,
    /// aes256 with hardcoded key over `AlbumEntry`.
    pub hash: u64,
    /// **Ref:** CapsAlbumFileDateTime
    pub datetime: CapsAlbumFileDateTime,
    /// **Ref:** CapsAlbumStorage
    pub storage: u8,
    /// **Ref:** CapsAlbumFileContents
    pub content: u8,
    /// padding
    pub pad_x1a: [u8; 5],
    /// Set to 1 by official software
    pub unk_x1f: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union CapsApplicationAlbumEntry_anon0 {
    /// Data.
    pub data: [u8; 32],
    /// Pre-7.0.0
    pub v0: CapsApplicationAlbumEntry_anon0_v0,
    /// [7.0.0+]
    pub v1: CapsApplicationAlbumEntry_anon0_v1,
}

/// ApplicationAlbumEntry
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CapsApplicationAlbumEntry {
    pub anon0: CapsApplicationAlbumEntry_anon0,
}

/// ApplicationAlbumFileEntry
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CapsApplicationAlbumFileEntry {
    /// **Ref:** CapsApplicationAlbumEntry
    pub entry: CapsApplicationAlbumEntry,
    /// **Ref:** CapsAlbumFileDateTime
    pub datetime: CapsAlbumFileDateTime,
    /// Unknown.
    pub unk_x28: u64,
}

/// ApplicationData
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CapsApplicationData {
    /// UserData.
    pub userdata: [u8; 1024],
    /// UserData size.
    pub size: u32,
}

/// AlbumFileContents
pub type CapsAlbumFileContents = u32;
pub const CapsAlbumFileContents_ScreenShot: CapsAlbumFileContents = 0;
pub const CapsAlbumFileContents_Movie: CapsAlbumFileContents = 1;
pub const CapsAlbumFileContents_ExtraScreenShot: CapsAlbumFileContents = 2;
pub const CapsAlbumFileContents_ExtraMovie: CapsAlbumFileContents = 3;

pub type CapsAlbumContentsUsageFlag = u32;
/// Indicates that there are additional files not captured by the count/size fields of CapsAlbumContentsUsage
pub const CapsAlbumContentsUsageFlag_HasGreaterUsage: CapsAlbumContentsUsageFlag = BIT(0);
/// Indicates that the file is not a known content type
pub const CapsAlbumContentsUsageFlag_IsUnknownContents: CapsAlbumContentsUsageFlag = BIT(1);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CapsAlbumContentsUsage {
    /// Count.
    pub count: s64,
    /// Size. Used storage space.
    pub size: s64,
    /// **Ref:** CapsAlbumContentsUsageFlag
    pub flags: u32,
    /// **Ref:** CapsAlbumFileContents
    pub file_contents: u8,
    /// Unused
    pub pad_x15: [u8; 3],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CapsAlbumUsage2 {
    /// **Ref:** CapsAlbumContentsUsage
    pub usages: [CapsAlbumContentsUsage; 2],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CapsAlbumUsage3 {
    /// **Ref:** CapsAlbumContentsUsage
    pub usages: [CapsAlbumContentsUsage; 3],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CapsAlbumUsage16 {
    /// **Ref:** CapsAlbumContentsUsage
    pub usages: [CapsAlbumContentsUsage; 16],
}

/// UserIdList
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CapsUserIdList {
    /// **Ref:** AccountUid
    pub uids: [AccountUid; 8],
    /// Total userIDs.
    pub count: u8,
    /// Padding.
    pub pad: [u8; 7],
}

/// LoadAlbumScreenShotImageOutputForApplication
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CapsLoadAlbumScreenShotImageOutputForApplication {
    /// Width. Official sw copies this to a s32 output field.
    pub width: s64,
    /// Height. Official sw copies this to a s32 output field.
    pub height: s64,
    /// **Ref:** CapsScreenShotAttributeForApplication
    pub attr: CapsScreenShotAttributeForApplication,
    /// **Ref:** CapsApplicationData
    pub appdata: CapsApplicationData,
    /// Unused.
    pub reserved: [u8; 172],
}

/// LoadAlbumScreenShotImageOutput
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CapsLoadAlbumScreenShotImageOutput {
    /// Width. Official sw copies this to a s32 output field.
    pub width: s64,
    /// Height. Official sw copies this to a s32 output field.
    pub height: s64,
    /// **Ref:** CapsScreenShotAttribute
    pub attr: CapsScreenShotAttribute,
    /// Unused.
    pub unk_x50: [u8; 1024],
}

/// AlbumFileContentsFlag
pub type CapsAlbumFileContentsFlag = u32;
/// Query for ScreenShot files.
pub const CapsAlbumFileContentsFlag_ScreenShot: CapsAlbumFileContentsFlag = BIT(0);
/// Query for Movie files.
pub const CapsAlbumFileContentsFlag_Movie: CapsAlbumFileContentsFlag = BIT(1);

/// AlbumCache
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CapsAlbumCache {
    /// Count
    pub count: u64,
    /// Unknown
    pub unk_x8: u64,
}

/// Gets the default start_datetime.
#[inline(always)]
pub fn capsGetDefaultStartDateTime() -> CapsAlbumFileDateTime {
    CapsAlbumFileDateTime { year: 1970, month: 1, day: 1, ..Default::default() }
}

/// Gets the default end_datetime.
#[inline(always)]
pub fn capsGetDefaultEndDateTime() -> CapsAlbumFileDateTime {
    CapsAlbumFileDateTime { year: 3000, month: 1, day: 1, ..Default::default() }
}

/// Convert a `CapsApplicationAlbumFileEntry` to `CapsApplicationAlbumEntry`.
#[inline(always)]
pub fn capsConvertApplicationAlbumFileEntryToApplicationAlbumEntry(out: &mut CapsApplicationAlbumEntry, in_: &CapsApplicationAlbumFileEntry) {
    *out = in_.entry;
}

/// Convert a `CapsApplicationAlbumEntry` to `CapsApplicationAlbumFileEntry`. Should only be used on [7.0.0+].
#[inline(always)]
pub fn capsConvertApplicationAlbumEntryToApplicationAlbumFileEntry(out: &mut CapsApplicationAlbumFileEntry, in_: &CapsApplicationAlbumEntry) {
    out.entry = *in_;
    // SAFETY: every bit pattern of the entry union is a valid `v1`.
    out.datetime = unsafe { in_.anon0.v1.datetime };
    out.unk_x28 = 0;
}

impl_zeroed_default!(CapsScreenShotAttribute, CapsScreenShotAttributeForApplication, CapsScreenShotDecodeOption, CapsAlbumFileDateTime, CapsAlbumFileId, CapsAlbumEntry, CapsApplicationAlbumEntry_anon0_v0, CapsApplicationAlbumEntry_anon0_v1, CapsApplicationAlbumEntry_anon0, CapsApplicationAlbumEntry, CapsApplicationAlbumFileEntry, CapsApplicationData, CapsAlbumContentsUsage, CapsAlbumUsage2, CapsAlbumUsage3, CapsAlbumUsage16, CapsUserIdList, CapsLoadAlbumScreenShotImageOutputForApplication, CapsLoadAlbumScreenShotImageOutput, CapsAlbumCache);

unsafe extern "C" {
    /// Gets the ShimLibraryVersion.
    pub fn capsGetShimLibraryVersion() -> u64;
}
