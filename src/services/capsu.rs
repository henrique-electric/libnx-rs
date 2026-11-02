//! `switch/services/capsu.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

unsafe extern "C" {
    /// Initialize caps:u. Only available on [5.0.0+].
    pub fn capsuInitialize() -> Result;

    /// Exit caps:u.
    pub fn capsuExit();

    /// Gets the Service for caps:u.
    pub fn capsuGetServiceSession() -> *mut Service;

    /// Gets the Service for IAlbumAccessorApplicationSession, only initialized after `capsuOpenAlbumMovieStream` was used (unaffected by using `capsuCloseAlbumMovieStream`).
    pub fn capsuGetServiceSession_Accessor() -> *mut Service;

    /// Gets a listing of `CapsApplicationAlbumFileEntry`.
    ///
    /// # Arguments
    /// - `entries` (out): Output array of `CapsApplicationAlbumFileEntry`.
    /// - `count` (in): Max size of the output array in entries.
    /// - `type` (in): `CapsContentType`
    /// - `start_datetime` (in): Start `CapsAlbumFileDateTime`, when NULL the default is used.
    /// - `end_datetime` (in): End `CapsAlbumFileDateTime`, when NULL the default is used.
    /// - `total_entries` (out): Total output entries.
    ///
    /// **Note:** On [6.0.0+] this uses GetAlbumFileList1AafeAruidDeprecated, otherwise this uses GetAlbumFileList0AafeAruidDeprecated.
    ///
    /// **Note:** This is an old version of `capsuGetAlbumFileList3`.
    pub fn capsuGetAlbumFileListDeprecated1(entries: *mut CapsApplicationAlbumFileEntry, count: s32, type_: CapsContentType, start_datetime: *const CapsAlbumFileDateTime, end_datetime: *const CapsAlbumFileDateTime, total_entries: *mut s32) -> Result;

    /// Gets a listing of `CapsApplicationAlbumFileEntry`, where the AlbumFile has an UserId which matches the input one. See also `capssuSaveScreenShotWithUserIds`.
    ///
    /// # Arguments
    /// - `entries` (out): Output array of `CapsApplicationAlbumFileEntry`.
    /// - `count` (in): Max size of the output array in entries.
    /// - `type` (in): `CapsContentType`
    /// - `start_datetime` (in): Start `CapsAlbumFileDateTime`, when NULL the default is used.
    /// - `end_datetime` (in): End `CapsAlbumFileDateTime`, when NULL the default is used.
    /// - `uid` (in): `AccountUid`
    /// - `total_entries` (out): Total output entries.
    ///
    /// **Note:** Only available on [6.0.0+].
    ///
    /// **Note:** This is an old version of `capsuGetAlbumFileList4`.
    pub fn capsuGetAlbumFileListDeprecated2(entries: *mut CapsApplicationAlbumFileEntry, count: s32, type_: CapsContentType, start_datetime: *const CapsAlbumFileDateTime, end_datetime: *const CapsAlbumFileDateTime, uid: AccountUid, total_entries: *mut s32) -> Result;

    /// Gets a listing of `CapsApplicationAlbumEntry`.
    ///
    /// # Arguments
    /// - `entries` (out): Output array of `CapsApplicationAlbumEntry`.
    /// - `count` (in): Max size of the output array in entries.
    /// - `type` (in): `CapsContentType`
    /// - `start_datetime` (in): Start `CapsAlbumFileDateTime`, when NULL the default is used.
    /// - `end_datetime` (in): End `CapsAlbumFileDateTime`, when NULL the default is used.
    /// - `total_entries` (out): Total output entries.
    ///
    /// **Note:** Only available on [7.0.0+], on prior sysvers use `capsuGetAlbumFileListDeprecated1` instead.
    pub fn capsuGetAlbumFileList3(entries: *mut CapsApplicationAlbumEntry, count: s32, type_: CapsContentType, start_datetime: *const CapsAlbumFileDateTime, end_datetime: *const CapsAlbumFileDateTime, total_entries: *mut s32) -> Result;

    /// Gets a listing of `CapsApplicationAlbumEntry`, where the AlbumFile has an UserId which matches the input one. See also `capssuSaveScreenShotWithUserIds`.
    ///
    /// # Arguments
    /// - `entries` (out): Output array of `CapsApplicationAlbumEntry`.
    /// - `count` (in): Max size of the output array in entries.
    /// - `type` (in): `CapsContentType`
    /// - `start_datetime` (in): Start `CapsAlbumFileDateTime`, when NULL the default is used.
    /// - `end_datetime` (in): End `CapsAlbumFileDateTime`, when NULL the default is used.
    /// - `uid` (in): `AccountUid`
    /// - `total_entries` (out): Total output entries.
    ///
    /// **Note:** Only available on [7.0.0+], on prior sysvers use `capsuGetAlbumFileListDeprecated2` instead.
    pub fn capsuGetAlbumFileList4(entries: *mut CapsApplicationAlbumEntry, count: s32, type_: CapsContentType, start_datetime: *const CapsAlbumFileDateTime, end_datetime: *const CapsAlbumFileDateTime, uid: AccountUid, total_entries: *mut s32) -> Result;

    /// Deletes the specified AlbumFile.
    ///
    /// # Arguments
    /// - `type` (in): `CapsContentType`, must match ::CapsContentType_ExtraMovie.
    /// - `entry` (in): `CapsApplicationAlbumFileEntry`
    pub fn capsuDeleteAlbumFile(type_: CapsContentType, entry: *const CapsApplicationAlbumFileEntry) -> Result;

    /// Gets the filesize for the entire specified AlbumFile.
    ///
    /// # Arguments
    /// - `entry` (in): `CapsApplicationAlbumFileEntry`
    /// - `size` (out): Output filesize.
    pub fn capsuGetAlbumFileSize(entry: *const CapsApplicationAlbumFileEntry, size: *mut u64) -> Result;

    /// Load the ScreenShotImage for the specified AlbumFile.
    ///
    /// # Arguments
    /// - `width` (out): Output image width. Optional, can be NULL.
    /// - `height` (out): Output image height. Optional, can be NULL.
    /// - `attr` (out): `CapsScreenShotAttributeForApplication`
    /// - `userdata` (out): Output buffer containing the UserData. Optional, can be NULL. This buffer is cleared to 0 using userdata_maxsize, prior to doing the memcpy.
    /// - `userdata_maxsize` (in): Max size of the userdata buffer. Optional, can be 0.
    /// - `userdata_size` (out): Userdata size field, clamped to max size sizeof(CapsApplicationData::userdata) when needed.
    /// - `image` (out): RGBA8 image output buffer.
    /// - `image_size` (in): Image buffer size, should be at least large enough for RGBA8 1280x720.
    /// - `workbuf` (out): Work buffer, cleared to 0 by the cmd before it returns.
    /// - `workbuf_size` (in): Work buffer size, must be at least the size of the JPEG within the AlbumFile.
    /// - `entry` (in): `CapsApplicationAlbumFileEntry`
    /// - `option` (in): `CapsScreenShotDecodeOption`
    pub fn capsuLoadAlbumScreenShotImage(width: *mut s32, height: *mut s32, attr: *mut CapsScreenShotAttributeForApplication, userdata: *mut c_void, userdata_maxsize: usize, userdata_size: *mut u32, image: *mut c_void, image_size: usize, workbuf: *mut c_void, workbuf_size: usize, entry: *const CapsApplicationAlbumFileEntry, option: *const CapsScreenShotDecodeOption) -> Result;

    /// Load the ScreenShotThumbnailImage for the specified AlbumFile.
    ///
    /// # Arguments
    /// - `width` (out): Output image width. Optional, can be NULL.
    /// - `height` (out): Output image height. Optional, can be NULL.
    /// - `attr` (out): `CapsScreenShotAttributeForApplication`
    /// - `userdata` (out): Output buffer containing the UserData. Optional, can be NULL. This buffer is cleared to 0 using userdata_maxsize, prior to doing the memcpy.
    /// - `userdata_maxsize` (in): Max size of the userdata buffer. Optional, can be 0.
    /// - `userdata_size` (out): Userdata size field, clamped to max size sizeof(CapsApplicationData::userdata) when needed.
    /// - `image` (out): RGBA8 image output buffer.
    /// - `image_size` (in): Image buffer size, should be at least large enough for RGBA8 320x180.
    /// - `workbuf` (out): Work buffer, cleared to 0 by the cmd before it returns.
    /// - `workbuf_size` (in): Work buffer size, must be at least the size of the JPEG within the AlbumFile.
    /// - `entry` (in): `CapsApplicationAlbumFileEntry`
    /// - `option` (in): `CapsScreenShotDecodeOption`
    pub fn capsuLoadAlbumScreenShotThumbnailImage(width: *mut s32, height: *mut s32, attr: *mut CapsScreenShotAttributeForApplication, userdata: *mut c_void, userdata_maxsize: usize, userdata_size: *mut u32, image: *mut c_void, image_size: usize, workbuf: *mut c_void, workbuf_size: usize, entry: *const CapsApplicationAlbumFileEntry, option: *const CapsScreenShotDecodeOption) -> Result;

    /// PrecheckToCreateContents. Official sw only uses this with ::CapsContentType_ExtraMovie.
    ///
    /// # Arguments
    /// - `type` (in): `CapsContentType`
    /// - `unk` (in): Unknown.
    pub fn capsuPrecheckToCreateContents(type_: CapsContentType, unk: u64) -> Result;

    /// Opens an AlbumMovieStream.
    ///
    /// # Arguments
    /// - `stream` (out): Stream handle.
    /// - `entry` (in): `CapsApplicationAlbumFileEntry`
    ///
    /// **Note:** This opens IAlbumAccessorApplicationSession if not previously opened, it's closed during `capsuExit`.
    ///
    /// **Note:** Up to 4 streams can be open at the same time. Multiple streams can be open at the same time for the same `CapsApplicationAlbumFileEntry`.
    pub fn capsuOpenAlbumMovieStream(stream: *mut u64, entry: *const CapsApplicationAlbumFileEntry) -> Result;

    /// Closes an AlbumMovieStream.
    ///
    /// # Arguments
    /// - `stream` (in): Stream handle.
    pub fn capsuCloseAlbumMovieStream(stream: u64) -> Result;

    /// Gets the data size of an AlbumMovieStream.
    ///
    /// # Arguments
    /// - `stream` (in): Stream handle.
    /// - `size` (out): Size of the actual MP4, without the JPEG at the end.
    pub fn capsuGetAlbumMovieStreamSize(stream: u64, size: *mut u64) -> Result;

    /// Reads data from an AlbumMovieStream.
    ///
    /// # Arguments
    /// - `stream` (in): Stream handle.
    /// - `offset` (in): Offset.
    /// - `Output` (out): data buffer.
    /// - `size` (in): Data buffer size.
    /// - `actual_size` (out): Actual read size.
    ///
    /// **Note:** offset(+size) must not be negative. offset and size must be aligned to 0x40000-bytes.
    ///
    /// **Note:** When offset(+size) goes beyond the size from `capsuGetAlbumMovieStreamSize`, the regions of the buffer which goes beyond that are cleared to 0, and actual_size is still set to the input size.
    pub fn capsuReadAlbumMovieStream(stream: u64, offset: s64, buffer: *mut c_void, size: usize, actual_size: *mut u64) -> Result;

    /// Gets the BrokenReason for an AlbumMovieStream.
    ///
    /// # Arguments
    /// - `stream` (in): Stream handle.
    ///
    /// **Note:** Official sw doesn't use this.
    pub fn capsuGetAlbumMovieStreamBrokenReason(stream: u64) -> Result;
}
