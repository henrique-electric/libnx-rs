//! `switch/services/capsa.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

unsafe extern "C" {
    /// Initialize caps:a.
    pub fn capsaInitialize() -> Result;

    /// Exit caps:a.
    pub fn capsaExit();

    /// Gets the Service for caps:a.
    pub fn capsaGetServiceSession() -> *mut Service;

    /// Gets the Service for IAlbumAccessorSession, only initialized after `capsaOpenAlbumMovieStream` was used (unaffected by using `capsaCloseAlbumMovieStream`).
    pub fn capsaGetServiceSession_Accessor() -> *mut Service;

    /// Gets the amount of files at a AlbumStorage.
    ///
    /// # Arguments
    /// - `storage` (in): `CapsAlbumStorage`
    /// - `count` (out): Amount of files.
    pub fn capsaGetAlbumFileCount(storage: CapsAlbumStorage, count: *mut u64) -> Result;

    /// Gets a listing of `CapsAlbumEntry`, where the AlbumFile's storage matches the input one.
    ///
    /// # Arguments
    /// - `storage` (in): `CapsAlbumStorage`
    /// - `out` (out): Total output entries.
    /// - `entries` (out): Output array of `CapsAlbumEntry`.
    /// - `count` (in): Reserved entry count.
    pub fn capsaGetAlbumFileList(storage: CapsAlbumStorage, out: *mut u64, entries: *mut CapsAlbumEntry, count: u64) -> Result;

    /// Loads a file into the specified buffer.
    ///
    /// # Arguments
    /// - `file_id` (in): `CapsAlbumFileId`
    /// - `out_size` (out): Size of the AlbumFile.
    /// - `filebuf` (out): File output buffer.
    /// - `filebuf_size` (in): Size of the filebuf.
    pub fn capsaLoadAlbumFile(file_id: *const CapsAlbumFileId, out_size: *mut u64, filebuf: *mut c_void, filebuf_size: u64) -> Result;

    /// Deletes an AlbumFile corresponding to the specified `CapsAlbumFileId`.
    ///
    /// # Arguments
    /// - `file_id` (in): `CapsAlbumFileId`
    pub fn capsaDeleteAlbumFile(file_id: *const CapsAlbumFileId) -> Result;

    /// Copies an AlbumFile to the specified `CapsAlbumStorage`.
    ///
    /// # Arguments
    /// - `file_id` (in): `CapsAlbumFileId`
    /// - `dst_storage` (in): `CapsAlbumStorage`
    pub fn capsaStorageCopyAlbumFile(file_id: *const CapsAlbumFileId, dst_storage: CapsAlbumStorage) -> Result;

    /// Gets the mount status of the specified `CapsAlbumStorage`.
    ///
    /// # Arguments
    /// - `storage` (in): `CapsAlbumStorage`
    /// - `is_mounted` (out): Boolean over whether the storage is mounted or not.
    pub fn capsaIsAlbumMounted(storage: CapsAlbumStorage, is_mounted: *mut bool) -> Result;

    /// Returns the AlbumUsage for a specified `CapsAlbumStorage`.
    ///
    /// # Arguments
    /// - `storage` (in): `CapsAlbumStorage`
    /// - `out` (out): `CapsAlbumUsage2`
    pub fn capsaGetAlbumUsage(storage: CapsAlbumStorage, out: *mut CapsAlbumUsage2) -> Result;

    /// Gets the size for the specified AlbumFile.
    ///
    /// # Arguments
    /// - `file_id` (in): `CapsAlbumFileId`
    /// - `size` (out): Size of the file.
    pub fn capsaGetAlbumFileSize(file_id: *const CapsAlbumFileId, size: *mut u64) -> Result;

    /// Load the Thumbnail for the specified AlbumFile.
    ///
    /// # Arguments
    /// - `file_id` (in): `CapsAlbumFileId`
    /// - `out_size` (out): Size of the Thumbnail.
    /// - `image` (out): JPEG image output buffer.
    /// - `image_size` (in): Image buffer size.
    ///
    /// **Note:** Will always be 320x180.
    pub fn capsaLoadAlbumFileThumbnail(file_id: *const CapsAlbumFileId, out_size: *mut u64, image: *mut c_void, image_size: u64) -> Result;

    /// Load the ScreenShotImage for the specified AlbumFile.
    ///
    /// # Arguments
    /// - `width` (out): Output image width. Optional, can be NULL.
    /// - `height` (out): Output image height. Optional, can be NULL.
    /// - `file_id` (in): `CapsAlbumFileId`
    /// - `image` (out): RGBA8 image output buffer.
    /// - `image_size` (in): Image buffer size, should be at least large enough for RGBA8 1280x720.
    /// - `workbuf` (out): Work buffer, cleared to 0 by the cmd before it returns.
    /// - `workbuf_size` (in): Work buffer size, must be at least the size of the JPEG within the AlbumFile.
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn capsaLoadAlbumScreenShotImage(width: *mut u64, height: *mut u64, file_id: *const CapsAlbumFileId, image: *mut c_void, image_size: u64, workbuf: *mut c_void, workbuf_size: u64) -> Result;

    /// Load the ScreenShotThumbnailImage for the specified AlbumFile.
    ///
    /// # Arguments
    /// - `width` (out): Output image width. Optional, can be NULL.
    /// - `height` (out): Output image height. Optional, can be NULL.
    /// - `file_id` (in): `CapsAlbumFileId`
    /// - `image` (out): RGBA8 image output buffer.
    /// - `image_size` (in): Image buffer size, should be at least large enough for RGBA8 320x180.
    /// - `workbuf` (out): Work buffer, cleared to 0 by the cmd before it returns.
    /// - `workbuf_size` (in): Work buffer size, must be at least the size of the JPEG within the AlbumFile.
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn capsaLoadAlbumScreenShotThumbnailImage(width: *mut u64, height: *mut u64, file_id: *const CapsAlbumFileId, image: *mut c_void, image_size: u64, workbuf: *mut c_void, workbuf_size: u64) -> Result;

    /// Load an `CapsAlbumEntry` from a `CapsApplicationAlbumEntry` and an ApplicationId.
    ///
    /// # Arguments
    /// - `entry` (out): `CapsAlbumEntry`
    /// - `application_entry` (in): `CapsApplicationAlbumEntry`
    /// - `application_id` (in): ApplicationId
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn capsaGetAlbumEntryFromApplicationAlbumEntry(entry: *mut CapsAlbumEntry, application_entry: *const CapsApplicationAlbumEntry, application_id: u64) -> Result;

    /// Load the ScreenShotImage for the specified AlbumFile.
    ///
    /// # Arguments
    /// - `width` (out): Output image width. Optional, can be NULL.
    /// - `height` (out): Output image height. Optional, can be NULL.
    /// - `file_id` (in): `CapsAlbumFileId`
    /// - `opts` (in): `CapsScreenShotDecodeOption`
    /// - `image` (out): RGBA8 image output buffer.
    /// - `image_size` (in): Image buffer size, should be at least large enough for RGBA8 1280x720.
    /// - `workbuf` (out): Work buffer, cleared to 0 by the cmd before it returns.
    /// - `workbuf_size` (in): Work buffer size, must be at least the size of the JPEG within the AlbumFile.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn capsaLoadAlbumScreenShotImageEx(width: *mut u64, height: *mut u64, file_id: *const CapsAlbumFileId, opts: *const CapsScreenShotDecodeOption, image: *mut c_void, image_size: u64, workbuf: *mut c_void, workbuf_size: u64) -> Result;

    /// Load the ScreenShotThumbnailImage for the specified AlbumFile.
    ///
    /// # Arguments
    /// - `width` (out): Output image width. Optional, can be NULL.
    /// - `height` (out): Output image height. Optional, can be NULL.
    /// - `file_id` (in): `CapsAlbumFileId`
    /// - `opts` (in): `CapsScreenShotDecodeOption`
    /// - `image` (out): RGBA8 image output buffer.
    /// - `image_size` (in): Image buffer size, should be at least large enough for RGBA8 320x180.
    /// - `workbuf` (out): Work buffer, cleared to 0 by the cmd before it returns.
    /// - `workbuf_size` (in): Work buffer size, must be at least the size of the JPEG within the AlbumFile.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn capsaLoadAlbumScreenShotThumbnailImageEx(width: *mut u64, height: *mut u64, file_id: *const CapsAlbumFileId, opts: *const CapsScreenShotDecodeOption, image: *mut c_void, image_size: u64, workbuf: *mut c_void, workbuf_size: u64) -> Result;

    /// Load the ScreenShotImage for the specified AlbumFile.
    ///
    /// # Arguments
    /// - `width` (out): Output image width. Optional, can be NULL.
    /// - `height` (out): Output image height. Optional, can be NULL.
    /// - `attr` (out): `CapsScreenShotAttribute`
    /// - `file_id` (in): `CapsAlbumFileId`
    /// - `opts` (in): `CapsScreenShotDecodeOption`
    /// - `image` (out): RGBA8 image output buffer.
    /// - `image_size` (in): Image buffer size, should be at least large enough for RGBA8 1280x720.
    /// - `workbuf` (out): Work buffer, cleared to 0 by the cmd before it returns.
    /// - `workbuf_size` (in): Work buffer size, must be at least the size of the JPEG within the AlbumFile.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn capsaLoadAlbumScreenShotImageEx0(width: *mut u64, height: *mut u64, attr: *mut CapsScreenShotAttribute, file_id: *const CapsAlbumFileId, opts: *const CapsScreenShotDecodeOption, image: *mut c_void, image_size: u64, workbuf: *mut c_void, workbuf_size: u64) -> Result;

    /// Returns the AlbumUsage for a specified `CapsAlbumStorage`.
    ///
    /// # Arguments
    /// - `storage` (in): `CapsAlbumStorage`
    /// - `out` (out): `CapsAlbumUsage3`
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capsaGetAlbumUsage3(storage: CapsAlbumStorage, out: *mut CapsAlbumUsage3) -> Result;

    /// Returns the result for a AlbumStorage mount.
    ///
    /// # Arguments
    /// - `storage` (in): `CapsAlbumStorage`
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capsaGetAlbumMountResult(storage: CapsAlbumStorage) -> Result;

    /// Returns the AlbumUsage for a specified `CapsAlbumStorage`.
    ///
    /// # Arguments
    /// - `storage` (in): `CapsAlbumStorage`
    /// - `flags` (in): `CapsAlbumFileContentsFlag`
    /// - `out` (out): `CapsAlbumUsage16`
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capsaGetAlbumUsage16(storage: CapsAlbumStorage, flags: u8, out: *mut CapsAlbumUsage16) -> Result;

    /// Returns the start and end of the Applet Id range.
    ///
    /// # Arguments
    /// - `success` (out): Returns bool over whether the call was handled or not.
    /// - `min` (out): Mimimum applet id. Always 0x0100000000001000
    /// - `max` (out): Maximum applet id. Always 0x0100000000001FFF
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn capsaGetMinMaxAppletId(success: *mut bool, min: *mut u64, max: *mut u64) -> Result;

    /// Gets the amount of files of the specified type at a AlbumStorage.
    ///
    /// # Arguments
    /// - `storage` (in): `CapsAlbumStorage`
    /// - `flags` (in): `CapsAlbumFileContentsFlag`
    /// - `count` (out): Amount of files.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn capsaGetAlbumFileCountEx0(storage: CapsAlbumStorage, flags: u8, count: *mut u64) -> Result;

    /// Gets a listing of `CapsAlbumEntry`, where the AlbumFile's storage and type matches the input one.
    ///
    /// # Arguments
    /// - `storage` (in): `CapsAlbumStorage`
    /// - `flags` (in): `CapsAlbumFileContentsFlag`
    /// - `out` (out): Total output entries.
    /// - `entries` (out): Output array of `CapsAlbumEntry`.
    /// - `count` (in): Reserved entry count.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn capsaGetAlbumFileListEx0(storage: CapsAlbumStorage, flags: u8, out: *mut u64, entries: *mut CapsAlbumEntry, count: u64) -> Result;

    /// Returns the image from the last shown ScreenShot Overlay.
    ///
    /// # Arguments
    /// - `file_id` (out): `CapsAlbumFileId`
    /// - `out_size` (out): Size of the thumbnail image. Always 0x5100.
    /// - `image` (out): RGBA8 image output buffer.
    /// - `image_size` (in): Image buffer size, should be at least large enough for RGBA8 96×54.
    pub fn capsaGetLastOverlayScreenShotThumbnail(file_id: *mut CapsAlbumFileId, out_size: *mut u64, image: *mut c_void, image_size: u64) -> Result;

    /// Returns the image from the last shown Movie Overlay.
    ///
    /// # Arguments
    /// - `file_id` (out): `CapsAlbumFileId`
    /// - `out_size` (out): Size of the thumbnail image. Always 0x5100.
    /// - `image` (out): RGBA8 image output buffer.
    /// - `image_size` (in): Image buffer size, should be at least large enough for RGBA8 96×54.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capsaGetLastOverlayMovieThumbnail(file_id: *mut CapsAlbumFileId, out_size: *mut u64, image: *mut c_void, image_size: u64) -> Result;

    /// Gets the currently set autosaving storage.
    ///
    /// # Arguments
    /// - `storage` (out): `CapsAlbumStorage`
    ///
    /// **Note:** Wrapper around setsysGetPrimaryAlbumStorage but defaults to NAND if SD isn't available.
    pub fn capsaGetAutoSavingStorage(storage: *mut CapsAlbumStorage) -> Result;

    /// Gets required size to copy all files from one Storage to another.
    ///
    /// # Arguments
    /// - `dst_storage` (in): `CapsAlbumStorage`
    /// - `src_storage` (in): `CapsAlbumStorage`
    /// - `out` (out): Required storage space size.
    pub fn capsaGetRequiredStorageSpaceSizeToCopyAll(dst_storage: CapsAlbumStorage, src_storage: CapsAlbumStorage, out: *mut u64) -> Result;

    /// Load the ScreenShotThumbnailImage for the specified AlbumFile.
    ///
    /// # Arguments
    /// - `width` (out): Output image width. Optional, can be NULL.
    /// - `height` (out): Output image height. Optional, can be NULL.
    /// - `attr` (out): `CapsScreenShotAttribute`
    /// - `file_id` (in): `CapsAlbumFileId`
    /// - `opts` (in): `CapsScreenShotDecodeOption`
    /// - `image` (out): RGBA8 image output buffer.
    /// - `image_size` (in): Image buffer size, should be at least large enough for RGBA8 320x180.
    /// - `workbuf` (out): Work buffer, cleared to 0 by the cmd before it returns.
    /// - `workbuf_size` (in): Work buffer size, must be at least the size of the JPEG within the AlbumFile.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn capsLoadAlbumScreenShotThumbnailImageEx0(width: *mut u64, height: *mut u64, attr: *mut CapsScreenShotAttribute, file_id: *const CapsAlbumFileId, opts: *const CapsScreenShotDecodeOption, image: *mut c_void, image_size: u64, workbuf: *mut c_void, workbuf_size: u64) -> Result;

    /// Load the ScreenShotImage for the specified AlbumFile.
    ///
    /// # Arguments
    /// - `file_id` (in): `CapsAlbumFileId`
    /// - `opts` (in): `CapsScreenShotDecodeOption`
    /// - `out` (out): `CapsLoadAlbumScreenShotImageOutput`
    /// - `image` (out): RGBA8 image output buffer.
    /// - `image_size` (in): Image buffer size, should be at least large enough for RGBA8 1280x720.
    /// - `workbuf` (out): Work buffer, cleared to 0 by the cmd before it returns.
    /// - `workbuf_size` (in): Work buffer size, must be at least the size of the JPEG within the AlbumFile.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capsaLoadAlbumScreenShotImageEx1(file_id: *const CapsAlbumFileId, opts: *const CapsScreenShotDecodeOption, out: *mut CapsLoadAlbumScreenShotImageOutput, image: *mut c_void, image_size: u64, workbuf: *mut c_void, workbuf_size: u64) -> Result;

    /// Load the ScreenShotThumbnailImage for the specified AlbumFile.
    ///
    /// # Arguments
    /// - `file_id` (in): `CapsAlbumFileId`
    /// - `opts` (in): `CapsScreenShotDecodeOption`
    /// - `out` (out): `CapsLoadAlbumScreenShotImageOutput`
    /// - `image` (out): RGBA8 image output buffer.
    /// - `image_size` (in): Image buffer size, should be at least large enough for RGBA8 320x180.
    /// - `workbuf` (out): Work buffer, cleared to 0 by the cmd before it returns.
    /// - `workbuf_size` (in): Work buffer size, must be at least the size of the JPEG within the AlbumFile.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capsaLoadAlbumScreenShotThumbnailImageEx1(file_id: *const CapsAlbumFileId, opts: *const CapsScreenShotDecodeOption, out: *mut CapsLoadAlbumScreenShotImageOutput, image: *mut c_void, image_size: u64, workbuf: *mut c_void, workbuf_size: u64) -> Result;

    /// Unmounts the specified AlbumStorage.
    ///
    /// # Arguments
    /// - `storage` (in): `CapsAlbumStorage`
    pub fn capsaForceAlbumUnmounted(storage: CapsAlbumStorage) -> Result;

    /// Resets mount status for the specified AlbumStorage.
    ///
    /// # Arguments
    /// - `storage` (in): `CapsAlbumStorage`
    ///
    /// **Note:** Mounts the Storage if available.
    pub fn capsaResetAlbumMountStatus(storage: CapsAlbumStorage) -> Result;

    /// Refreshs Album Cache for the specified AlbumStorage.
    ///
    /// # Arguments
    /// - `storage` (in): `CapsAlbumStorage`
    pub fn capsaRefreshAlbumCache(storage: CapsAlbumStorage) -> Result;

    /// Gets the AlbumCache of the specified AlbumStorage.
    ///
    /// # Arguments
    /// - `storage` (in): `CapsAlbumStorage`
    /// - `cache` (out): `CapsAlbumCache`
    ///
    /// **Note:** Stubbed on [4.0.0+].
    ///
    /// **Note:** use `capsaGetAlbumCacheEx` instead.
    pub fn capsaGetAlbumCache(storage: CapsAlbumStorage, cache: *mut CapsAlbumCache) -> Result;

    /// Gets the AlbumCache for the specified type of the specified AlbumStorage.
    ///
    /// # Arguments
    /// - `storage` (in): `CapsAlbumStorage`
    /// - `contents` (in): `CapsAlbumFileContents`
    /// - `cache` (out): `CapsAlbumCache`
    pub fn capsaGetAlbumCacheEx(storage: CapsAlbumStorage, contents: CapsAlbumFileContents, cache: *mut CapsAlbumCache) -> Result;

    /// Load an `CapsAlbumEntry` from a `CapsApplicationAlbumEntry` and an AppletResourceUserId.
    ///
    /// # Arguments
    /// - `entry` (out): `CapsAlbumEntry`
    /// - `application_entry` (in): `CapsApplicationAlbumEntry`
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn capsaGetAlbumEntryFromApplicationAlbumEntryAruid(entry: *mut CapsAlbumEntry, application_entry: *const CapsApplicationAlbumEntry) -> Result;

    /// Opens an AlbumMovieStream.
    ///
    /// # Arguments
    /// - `stream` (out): Stream handle.
    /// - `entry` (in): `CapsAlbumFileId`
    ///
    /// **Note:** This opens IAlbumAccessorSession if not previously opened, it's closed during `capsaExit`.
    ///
    /// **Note:** Up to 4 streams can be open at the same time. Multiple streams can be open at the same time for the same `CapsAlbumFileId`.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capsaOpenAlbumMovieStream(stream: *mut u64, file_id: *const CapsAlbumFileId) -> Result;

    /// Closes an AlbumMovieStream.
    ///
    /// # Arguments
    /// - `stream` (in): Stream handle.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capsaCloseAlbumMovieStream(stream: u64) -> Result;

    /// Gets the data size of an AlbumMovieStream.
    ///
    /// # Arguments
    /// - `stream` (in): Stream handle.
    /// - `size` (out): Size of the actual MP4, without the JPEG at the end.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capsaGetAlbumMovieStreamSize(stream: u64, size: *mut u64) -> Result;

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
    /// **Note:** When offset(+size) goes beyond the size from `capsaGetAlbumMovieStreamSize`, the regions of the buffer which goes beyond that are cleared to 0, and actual_size is still set to the input size.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capsaReadMovieDataFromAlbumMovieReadStream(stream: u64, offset: s64, buffer: *mut c_void, size: usize, actual_size: *mut u64) -> Result;

    /// Gets the BrokenReason for an AlbumMovieStream.
    ///
    /// # Arguments
    /// - `stream` (in): Stream handle.
    ///
    /// **Note:** Official sw doesn't use this.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capsaGetAlbumMovieReadStreamBrokenReason(stream: u64) -> Result;

    /// Gets the data size of an Image taken from an AlbumMovieStream.
    ///
    /// # Arguments
    /// - `stream` (in): Stream handle.
    /// - `size` (out): Expected size of an Image.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capsaGetAlbumMovieReadStreamImageDataSize(stream: u64, size: *mut u64) -> Result;

    /// Reads data of an Image taken from an AlbumMovieStream.
    ///
    /// # Arguments
    /// - `stream` (in): Stream handle.
    /// - `offset` (in): Offset.
    /// - `Output` (out): data buffer.
    /// - `size` (in): Data buffer size.
    /// - `actual_size` (out): Actual read size.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capsaReadImageDataFromAlbumMovieReadStream(stream: u64, offset: s64, buffer: *mut c_void, size: usize, actual_size: *mut u64) -> Result;

    /// Gets the file attribute of an AlbumMovieStream.
    ///
    /// # Arguments
    /// - `stream` (in): Stream handle.
    /// - `attr` (out): `CapsScreenShotAttribute`
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn capsaReadFileAttributeFromAlbumMovieReadStream(stream: u64, attr: *mut CapsScreenShotAttribute) -> Result;
}
