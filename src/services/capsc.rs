//! `switch/services/capsc.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CapsApplicationId {
    pub application_id: u64,
    pub unknown_08: u8,
    pub unknown_09: u8,
    pub reserved: [u8; 6],
}

impl_zeroed_default!(CapsApplicationId);

unsafe extern "C" {
    /// Initialize caps:c
    pub fn capscInitialize() -> Result;

    /// Exit caps:c.
    pub fn capscExit();

    /// Gets the Service for caps:c.
    pub fn capscGetServiceSession() -> *mut Service;

    /// Notify the service that a storage is now available.
    ///
    /// # Arguments
    /// - `storage` (in): `CapsAlbumStorage`
    ///
    /// **Note:** This will result in capsrv mounting the image directory on that storage medium.
    pub fn capscNotifyAlbumStorageIsAvailable(storage: CapsAlbumStorage) -> Result;

    /// Notify the service that a storage is now unavailable.
    ///
    /// # Arguments
    /// - `storage` (in): `CapsAlbumStorage`
    ///
    /// **Note:** This will result in capsrv unmounting the image directory on that storage medium.
    pub fn capscNotifyAlbumStorageIsUnAvailable(storage: CapsAlbumStorage) -> Result;

    /// Register an applet for later usage.
    ///
    /// # Arguments
    /// - `appletResourceUserId` (in): AppletResourceUserId.
    /// - `application_id` (in): ApplicationId.
    ///
    /// **Note:** Called at application launch by the system.
    ///
    /// **Note:** Will generate a random AES-256 key for this application for use on Shim-Version 0.
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn capscRegisterAppletResourceUserId(appletResourceUserId: u64, application_id: *const CapsApplicationId) -> Result;

    /// Unregister an applet.
    ///
    /// # Arguments
    /// - `appletResourceUserId` (in): AppletResourceUserId.
    /// - `application_id` (in): ApplicationId.
    ///
    /// **Note:** Called at application exit by the system.
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn capscUnregisterAppletResourceUserId(appletResourceUserId: u64, application_id: *const CapsApplicationId) -> Result;

    /// Get an ApplicationId that corresponds to an AppletResourceUserId.
    ///
    /// # Arguments
    /// - `application_id` (out): ApplicationId.
    /// - `appletResourceUserId` (in): AppletResourceUserId.
    ///
    /// **Note:** Returns value set by `capscRegisterAppletResourceUserId`.
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn capscGetApplicationIdFromAruid(application_id: *mut CapsApplicationId, aruid: u64) -> Result;

    /// Checks whether an ApplicationId is registered.
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId.
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn capscCheckApplicationIdRegistered(application_id: u64) -> Result;

    /// Generate an AlbumFileId based on parameters and current time.
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId.
    /// - `contents` (in): `CapsAlbumFileContents`
    /// - `file_id` (out): `CapsAlbumFileId`
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn capscGenerateCurrentAlbumFileId(application_id: *const CapsApplicationId, contents: CapsAlbumFileContents, file_id: *mut CapsAlbumFileId) -> Result;

    /// Generate an ApplicationAlbumEntry based on parameters.
    ///
    /// # Arguments
    /// - `appEntry` (out): `CapsApplicationAlbumEntry`
    /// - `entry` (in): `CapsAlbumEntry`
    /// - `application_id` (in): ApplicationId.
    ///
    /// **Note:** Output will be different between Shim Version 0 and 1.
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn capscGenerateApplicationAlbumEntry(appEntry: *mut CapsApplicationAlbumEntry, entry: *const CapsAlbumEntry, application_id: u64) -> Result;

    /// Save a jpeg image.
    ///
    /// # Arguments
    /// - `file_id` (in): `CapsAlbumFileId`
    /// - `buffer` (in): JPEG image buffer.
    /// - `buffer_size` (in): Size of the JPEG image.
    ///
    /// **Note:** Only available on [2.0.0-3.0.2].
    pub fn capscSaveAlbumScreenShotFile(file_id: *const CapsAlbumFileId, buffer: *const c_void, buffer_size: u64) -> Result;

    /// Save a jpeg image.
    ///
    /// # Arguments
    /// - `file_id` (in): `CapsAlbumFileId`
    /// - `version` (in): Revision number.
    /// - `makernote_offset` (in): Offset to makernote in JPEG buffer.
    /// - `makernote_size` (in): Size of the makernote in JPEG buffer.
    /// - `buffer` (in): JPEG image buffer.
    /// - `buffer_size` (in): Size of the JPEG image.
    ///
    /// **Note:** Only available on [4.0.0+].
    ///
    /// **Note:** Version 3 as of [9.1.0].
    pub fn capscSaveAlbumScreenShotFileEx(file_id: *const CapsAlbumFileId, version: u64, makernote_offset: u64, makernote_size: u64, buffer: *const c_void, buffer_size: u64) -> Result;

    /// Sets thumbnail data for the last taken screenshot.
    ///
    /// # Arguments
    /// - `file_id` (in): `CapsAlbumFileId`
    /// - `image` (in): RGBA8 image buffer.
    /// - `image_size` (in): size of the RGBA8 image buffer.
    ///
    /// **Note:** 96×54 Image will get saved.
    pub fn capscSetOverlayScreenShotThumbnailData(file_id: *const CapsAlbumFileId, image: *const c_void, image_size: u64) -> Result;

    /// Sets thumbnail data for the last recorded movie.
    ///
    /// # Arguments
    /// - `file_id` (in): `CapsAlbumFileId`
    /// - `image` (in): RGBA8 image buffer.
    /// - `image_size` (in): size of the RGBA8 image buffer.
    ///
    /// **Note:** Only availabe on [4.0.0+].
    ///
    /// **Note:** 96×54 Image will get saved.
    pub fn capscSetOverlayMovieThumbnailData(file_id: *const CapsAlbumFileId, image: *const c_void, image_size: u64) -> Result;

    /// Opens an AlbumMovieReadStream.
    ///
    /// # Arguments
    /// - `stream` (out): Stream handle.
    /// - `entry` (in): `CapsAlbumFileId`
    ///
    /// **Note:** This opens IAlbumControlSession if not previously opened, it's closed during `capscExit`.
    ///
    /// **Note:** Up to 4 streams can be open at the same time. Multiple streams can be open at the same time for the same `CapsAlbumFileId`.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capscOpenAlbumMovieReadStream(stream: *mut u64, file_id: *const CapsAlbumFileId) -> Result;

    /// Closes an AlbumMovieReadStream.
    ///
    /// # Arguments
    /// - `stream` (in): Stream handle.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capscCloseAlbumMovieStream(stream: u64) -> Result;

    /// Gets the data size of an AlbumMovieReadStream.
    ///
    /// # Arguments
    /// - `stream` (in): Stream handle.
    /// - `size` (out): Size of the actual MP4, without the JPEG at the end.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capscGetAlbumMovieStreamSize(stream: u64, size: *mut u64) -> Result;

    /// Reads data from an AlbumMovieReadStream.
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
    /// **Note:** When offset(+size) goes beyond the size from `capscGetAlbumMovieStreamSize`, the regions of the buffer which goes beyond that are cleared to 0, and actual_size is still set to the input size.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capscReadMovieDataFromAlbumMovieReadStream(stream: u64, offset: u64, buffer: *mut c_void, size: usize, actual_size: *mut u64) -> Result;

    /// Gets the BrokenReason for an AlbumMovieReadStream.
    ///
    /// # Arguments
    /// - `stream` (in): Stream handle.
    ///
    /// **Note:** Official sw doesn't use this.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capscGetAlbumMovieReadStreamBrokenReason(stream: u64) -> Result;

    /// Gets the data size of an Image taken from an AlbumMovieReadStream.
    ///
    /// # Arguments
    /// - `stream` (in): Stream handle.
    /// - `size` (out): Expected size of an Image.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capscGetAlbumMovieReadStreamImageDataSize(stream: u64, size: *mut u64) -> Result;

    /// Reads data of an Image taken from an AlbumMovieReadStream.
    ///
    /// # Arguments
    /// - `stream` (in): Stream handle.
    /// - `offset` (in): Offset.
    /// - `buffer` (out): Output data buffer.
    /// - `size` (in): Data buffer size.
    /// - `actual_size` (out): Actual read size.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capscReadImageDataFromAlbumMovieReadStream(stream: u64, offset: u64, buffer: *mut c_void, size: usize, actual_size: *mut u64) -> Result;

    /// Gets the file attribute of an AlbumMovieReadStream.
    ///
    /// # Arguments
    /// - `stream` (in): Stream handle.
    /// - `attr` (out): `CapsScreenShotAttribute`
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capscReadFileAttributeFromAlbumMovieReadStream(stream: u64, attribute: *mut CapsScreenShotAttribute) -> Result;

    /// Opens an AlbumMovieWriteStream.
    ///
    /// # Arguments
    /// - `stream` (out): Stream handle.
    /// - `entry` (in): `CapsAlbumFileId`
    ///
    /// **Note:** This opens IAlbumControlSession if not previously opened, it's closed during `capsaExit`.
    ///
    /// **Note:** Up to 2 streams can be open at the same time.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capscOpenAlbumMovieWriteStream(stream: *mut u64, file_id: *const CapsAlbumFileId) -> Result;

    /// Finish write to AlbumMovieWriteStream.
    ///
    /// # Arguments
    /// - `stream` (in): Stream handle.
    ///
    /// **Note:** Copies file from save to destination storage and deletes the temporary file.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capscFinishAlbumMovieWriteStream(stream: u64) -> Result;

    /// Closes a finished AlbumMovieWriteStream.
    ///
    /// # Arguments
    /// - `stream` (in): Stream handle.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capscCommitAlbumMovieWriteStream(stream: u64) -> Result;

    /// Closes an AlbumMovieWriteStream in any state.
    ///
    /// # Arguments
    /// - `stream` (in): Stream handle.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capscDiscardAlbumMovieWriteStream(stream: u64) -> Result;

    /// Closes an AlbumMovieWriteStream in any state without deleting the temporary file.
    ///
    /// # Arguments
    /// - `stream` (in): Stream handle.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capscDiscardAlbumMovieWriteStreamNoDelete(stream: u64) -> Result;

    /// Closes a finished AlbumMovieWriteStream.
    ///
    /// # Arguments
    /// - `stream` (in): Stream handle.
    /// - `entry` (out): `CapsAlbumEntry`
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capscCommitAlbumMovieWriteStreamEx(stream: u64, entry: *mut CapsAlbumEntry) -> Result;

    /// Start AlbumMovieWriteStream data section.
    ///
    /// # Arguments
    /// - `stream` (in): Stream handle.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capscStartAlbumMovieWriteStreamDataSection(stream: u64) -> Result;

    /// End AlbumMovieWriteStream data section.
    ///
    /// # Arguments
    /// - `stream` (in): Stream handle.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capscEndAlbumMovieWriteStreamDataSection(stream: u64) -> Result;

    /// Start AlbumMovieWriteStream meta section.
    ///
    /// # Arguments
    /// - `stream` (in): Stream handle.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capscStartAlbumMovieWriteStreamMetaSection(stream: u64) -> Result;

    /// End AlbumMovieWriteStream meta section.
    ///
    /// # Arguments
    /// - `stream` (in): Stream handle.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capscEndAlbumMovieWriteStreamMetaSection(stream: u64) -> Result;

    /// Reads data from an AlbumMovieWriteStream.
    ///
    /// # Arguments
    /// - `stream` (in): Stream handle.
    /// - `offset` (in): Offset.
    /// - `buffer` (out): Output data buffer.
    /// - `size` (in): Data buffer size.
    /// - `actual_size` (out): Actual read size.
    ///
    /// **Note:** offset(+size) must not be negative. offset and size must be aligned to 0x40000-bytes.
    ///
    /// **Note:** When offset(+size) goes beyond the size from `capscGetAlbumMovieStreamSize`, the regions of the buffer which goes beyond that are cleared to 0, and actual_size is still set to the input size.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capscReadDataFromAlbumMovieWriteStream(stream: u64, offset: u64, buffer: *mut c_void, size: u64, actual_size: *mut u64) -> Result;

    /// Write data to an AlbumMovieWriteStream.
    ///
    /// # Arguments
    /// - `stream` (in): Stream handle.
    /// - `offset` (in): Offset.
    /// - `buffer` (in): Input data buffer.
    /// - `size` (in): Data buffer size.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capscWriteDataToAlbumMovieWriteStream(stream: u64, offset: u64, buffer: *const c_void, size: u64) -> Result;

    /// Write meta data to an AlbumMovieWriteStream.
    ///
    /// # Arguments
    /// - `stream` (in): Stream handle.
    /// - `offset` (in): Offset.
    /// - `buffer` (in): Input data buffer.
    /// - `size` (in): Data buffer size.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capscWriteMetaToAlbumMovieWriteStream(stream: u64, offset: u64, buffer: *const c_void, size: u64) -> Result;

    /// Gets the BrokenReason for an AlbumMovieWriteStream.
    ///
    /// # Arguments
    /// - `stream` (in): Stream handle.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capscGetAlbumMovieWriteStreamBrokenReason(stream: u64) -> Result;

    /// Gets the data size of an AlbumMovieWriteStream.
    ///
    /// # Arguments
    /// - `stream` (in): Stream handle.
    /// - `size` (out): Size of the data section.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capscGetAlbumMovieWriteStreamDataSize(stream: u64, size: *mut u64) -> Result;

    /// Sets the data size of an AlbumMovieWriteStream.
    ///
    /// # Arguments
    /// - `stream` (in): Stream handle.
    /// - `size` (out): Size of the data section.
    ///
    /// **Note:** Must not be bigger than 2GiB.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capscSetAlbumMovieWriteStreamDataSize(stream: u64, size: u64) -> Result;
}
