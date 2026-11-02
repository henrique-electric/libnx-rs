//! `switch/services/capssu.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

unsafe extern "C" {
    /// Initialize caps:su. Only available on [4.0.0+].
    pub fn capssuInitialize() -> Result;

    /// Exit caps:su.
    pub fn capssuExit();

    /// Gets the Service for caps:su.
    pub fn capssuGetServiceSession() -> *mut Service;

    /// This is a wrapper for `capssuSaveScreenShotEx0`.
    ///
    /// # Arguments
    /// - `buffer` (in): RGBA8 1280x720 image buffer.
    /// - `size` (in): Size of the buffer.
    /// - `reportoption` (in): `AlbumReportOption`
    /// - `orientation` (in): `AlbumImageOrientation`
    /// - `out` (out): `CapsApplicationAlbumEntry`. Optional, can be NULL.
    ///
    /// **Note:** This uses an all-zero `CapsScreenShotAttribute` with orientation = input orientation, and unk_xc = 1.
    pub fn capssuSaveScreenShot(buffer: *const c_void, size: usize, reportoption: AlbumReportOption, orientation: AlbumImageOrientation, out: *mut CapsApplicationAlbumEntry) -> Result;

    /// Similar to `capssuSaveScreenShot`, except this is a wrapper for `capssuSaveScreenShotEx1`.
    ///
    /// # Arguments
    /// - `buffer` (in): RGBA8 1280x720 image data buffer.
    /// - `size` (in): Size of the buffer.
    /// - `reportoption` (in): `AlbumReportOption`
    /// - `orientation` (in): `AlbumImageOrientation`
    /// - `userdata` (in): Input UserData buffer. If NULL, the `CapsApplicationData` will be empty.
    /// - `userdata_size` (in): Input UserData size, must be within bounds for CapsApplicationData::userdata. If 0, the `CapsApplicationData` will be empty.
    /// - `out` (out): `CapsApplicationAlbumEntry`. Optional, can be NULL.
    ///
    /// **Note:** This uses an all-zero `CapsScreenShotAttribute` with orientation = input orientation, and unk_xc = 1.
    ///
    /// **Note:** Only available on [8.0.0+].
    pub fn capssuSaveScreenShotWithUserData(buffer: *const c_void, size: usize, reportoption: AlbumReportOption, orientation: AlbumImageOrientation, userdata: *const c_void, userdata_size: usize, out: *mut CapsApplicationAlbumEntry) -> Result;

    /// Similar to `capssuSaveScreenShot`, except this is a wrapper for `capssuSaveScreenShotEx2`.
    ///
    /// # Arguments
    /// - `buffer` (in): RGBA8 1280x720 image data buffer.
    /// - `size` (in): Size of the buffer.
    /// - `reportoption` (in): `AlbumReportOption`
    /// - `orientation` (in): `AlbumImageOrientation`
    /// - `uids` (in): Input array of `AccountUid`. If NULL, the `CapsUserIdList` will be empty.
    /// - `uid_count` (in): Size of the uids array in entries, must be within bounds for CapsUserIdList::uids. If 0, the `CapsUserIdList` will be empty.
    /// - `out` (out): `CapsApplicationAlbumEntry`. Optional, can be NULL.
    ///
    /// **Note:** This uses an all-zero `CapsScreenShotAttribute` with orientation = input orientation, and unk_xc = 1.
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn capssuSaveScreenShotWithUserIds(buffer: *const c_void, size: usize, reportoption: AlbumReportOption, orientation: AlbumImageOrientation, uids: *const AccountUid, uid_count: usize, out: *mut CapsApplicationAlbumEntry) -> Result;

    /// Saves an Album screenshot using the specified gfx data in the buffer, with the specified `CapsScreenShotAttribute`.
    ///
    /// # Arguments
    /// - `buffer` (in): RGBA8 1280x720 image data buffer.
    /// - `size` (in): Size of the buffer, must be at least 0x384000.
    /// - `attr` (in): `CapsScreenShotAttribute`
    /// - `reportoption` (in): `AlbumReportOption`
    /// - `out` (out): `CapsApplicationAlbumEntry`. Optional, can be NULL.
    pub fn capssuSaveScreenShotEx0(buffer: *const c_void, size: usize, attr: *const CapsScreenShotAttribute, reportoption: AlbumReportOption, out: *mut CapsApplicationAlbumEntry) -> Result;

    /// Same as `capssuSaveScreenShotEx0`, except this allows specifying the `CapsApplicationData`.
    ///
    /// # Arguments
    /// - `buffer` (in): RGBA8 1280x720 image data buffer.
    /// - `size` (in): Size of the buffer, must be at least 0x384000.
    /// - `attr` (in): `CapsScreenShotAttribute`
    /// - `reportoption` (in): `AlbumReportOption`
    /// - `appdata` (in): `CapsApplicationData`
    /// - `out` (out): `CapsApplicationAlbumEntry`. Optional, can be NULL.
    ///
    /// **Note:** Only available on [8.0.0+].
    pub fn capssuSaveScreenShotEx1(buffer: *const c_void, size: usize, attr: *const CapsScreenShotAttribute, reportoption: AlbumReportOption, appdata: *mut CapsApplicationData, out: *mut CapsApplicationAlbumEntry) -> Result;

    /// Same as `capssuSaveScreenShotEx0`, except this allows specifying the `CapsUserIdList`.
    ///
    /// # Arguments
    /// - `buffer` (in): RGBA8 1280x720 image data buffer.
    /// - `size` (in): Size of the buffer, must be at least 0x384000.
    /// - `attr` (in): `CapsScreenShotAttribute`
    /// - `reportoption` (in): `AlbumReportOption`
    /// - `list` (in): `CapsUserIdList`
    /// - `out` (out): `CapsApplicationAlbumEntry`. Optional, can be NULL.
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn capssuSaveScreenShotEx2(buffer: *const c_void, size: usize, attr: *const CapsScreenShotAttribute, reportoption: AlbumReportOption, list: *mut CapsUserIdList, out: *mut CapsApplicationAlbumEntry) -> Result;
}
