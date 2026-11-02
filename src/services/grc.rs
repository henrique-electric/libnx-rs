//! `switch/services/grc.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Default size for `grcCreateMovieMaker`, this is the size used by official sw.
pub const GRC_MOVIEMAKER_WORKMEMORY_SIZE_DEFAULT: u32 = 0x6000000;

/// Stream type values for `grcdTransfer`.
pub type GrcStream = u32;
/// Video stream with H.264 NAL units. Official sw uses buffer size 0x32000.
pub const GrcStream_Video: GrcStream = 0;
/// Audio stream with PcmFormat_Int16, 2 channels, and samplerate = 48000Hz. Official sw uses buffer size 0x1000.
pub const GrcStream_Audio: GrcStream = 1;

/// GameMovieTrimmer
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GrcGameMovieTrimmer {
    /// IGameMovieTrimmer
    pub s: Service,
    /// TransferMemory
    pub tmem: TransferMemory,
}

/// IMovieMaker
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GrcMovieMaker {
    /// applet IMovieMaker
    pub a: Service,
    /// grc IMovieMaker
    pub s: Service,
    /// IHOSBinderDriver VideoProxy
    pub video_proxy: Service,
    /// Output Event from GetOffscreenLayerRecordingFinishReadyEvent with autoclear=false.
    pub recording_event: Event,
    /// Output Event from GetOffscreenLayerAudioEncodeReadyEvent with autoclear=false.
    pub audio_event: Event,
    /// TransferMemory
    pub tmem: TransferMemory,
    /// **Ref:** NWindow
    pub win: NWindow,
    /// LayerHandle
    pub layer_handle: u64,
    /// Whether OpenOffscreenLayer was used successfully, indicating that CloseOffscreenLayer should be used during `grcMovieMakerClose`.
    pub layer_open: bool,
    /// Whether `grcMovieMakerStart` was used successfully. This is also used by `grcMovieMakerAbort`.
    pub started_flag: bool,
}

/// GameMovieId
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GrcGameMovieId {
    /// **Ref:** CapsAlbumFileId
    pub file_id: CapsAlbumFileId,
    /// Unused, always zero.
    pub reserved: [u8; 40],
}

/// OffscreenRecordingParameter
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GrcOffscreenRecordingParameter {
    /// Unknown. Default value is 0.
    pub unk_x0: [u8; 16],
    /// Unknown. Must match value 0x103, which is the default value.
    pub unk_x10: u32,
    /// VideoBitRate, 0 is invalid. Default value is 8000000.
    pub video_bitrate: s32,
    /// VideoWidth, must match 1280 or 1920. Default value is 1280.
    pub video_width: s32,
    /// VideoHeight, must match 720 or 1080. Default value is 720.
    pub video_height: s32,
    /// VideoFrameRate, must match 30 or 60. Default value is 30.
    pub video_framerate: s32,
    /// VideoKeyFrameInterval, 0 is invalid. Default value is 30.
    pub video_keyFrameInterval: s32,
    /// AudioBitRate. Default value is 128000 ([5.0.0-5.1.0] 1536000).
    pub audio_bitrate: s32,
    /// AudioSampleRate, 0 is invalid. Default value is 48000.
    pub audio_samplerate: s32,
    /// AudioChannelCount. Must match 2, which is the default value.
    pub audio_channel_count: s32,
    /// **Ref:** PcmFormat AudioSampleFormat. Must match PcmFormat_Int16, which is the default value.
    pub audio_sample_format: s32,
    /// **Ref:** AlbumImageOrientation VideoImageOrientation. Default value is ::AlbumImageOrientation_Unknown0.
    pub video_imageOrientation: s32,
    /// Unknown. Default value is 0.
    pub unk_x3c: [u8; 68],
}

/// Gets the `NWindow` for the specified MovieMaker.
///
/// # Arguments
/// - `m`: `GrcMovieMaker`
#[inline(always)]
pub fn grcMovieMakerGetNWindow(m: &mut GrcMovieMaker) -> &mut NWindow {
    &mut m.win
}

impl_zeroed_default!(GrcGameMovieTrimmer, GrcMovieMaker, GrcGameMovieId, GrcOffscreenRecordingParameter);

unsafe extern "C" {
    /// Creates a `GrcGameMovieTrimmer` using `appletCreateGameMovieTrimmer`, uses the cmds from it to trim the specified video, then closes it.
    ///
    /// # Arguments
    /// - `dst_movieid` (out): `GrcGameMovieId` for the output video.
    /// - `src_movieid` (in): `GrcGameMovieId` for the input video.
    /// - `tmem_size` (in): TransferMemory size. Official sw uses size 0x2000000.
    /// - `thumbnail` (in): Optional, can be NULL. RGBA8 1280x720 thumbnail image data.
    /// - `start` (in): Start timestamp in 0.5s units.
    /// - `end` (in): End timestamp in 0.5s units.
    ///
    /// **Note:** See `appletCreateGameMovieTrimmer` for the requirements for using this.
    ///
    /// **Note:** This will block until video trimming finishes.
    pub fn grcTrimGameMovie(dst_movieid: *mut GrcGameMovieId, src_movieid: *const GrcGameMovieId, tmem_size: usize, thumbnail: *const c_void, start: s32, end: s32) -> Result;

    /// Creates a `GrcOffscreenRecordingParameter` with the default values, see `GrcOffscreenRecordingParameter` for the default values.
    ///
    /// # Arguments
    /// - `param` (out): `GrcOffscreenRecordingParameter`
    pub fn grcCreateOffscreenRecordingParameter(param: *mut GrcOffscreenRecordingParameter);

    /// Creates a `GrcMovieMaker` using `appletCreateMovieMaker`, and does the required initialization.
    ///
    /// # Arguments
    /// - `m` (out): `GrcMovieMaker`
    /// - `size` (in): TransferMemory WorkMemory size. See `GRC_MOVIEMAKER_WORKMEMORY_SIZE_DEFAULT`.
    ///
    /// **Note:** See `appletCreateMovieMaker` for the requirements for using this.
    pub fn grcCreateMovieMaker(m: *mut GrcMovieMaker, size: usize) -> Result;

    /// Closes a `GrcMovieMaker`.
    ///
    /// # Arguments
    /// - `m`: `GrcMovieMaker`
    ///
    /// **Note:** This also uses `grcMovieMakerAbort`.
    pub fn grcMovieMakerClose(m: *mut GrcMovieMaker);

    /// Aborts recording with the specified MovieMaker.
    ///
    /// # Arguments
    /// - `m`: `GrcMovieMaker`
    ///
    /// **Note:** This is used automatically by `grcMovieMakerClose`.
    ///
    /// **Note:** This will throw an error if `grcMovieMakerStart` was not used previously, with the flag used for this being cleared afterwards on success.
    pub fn grcMovieMakerAbort(m: *mut GrcMovieMaker) -> Result;

    /// Starts recording with the specified MovieMaker and `GrcOffscreenRecordingParameter`.
    ///
    /// # Arguments
    /// - `m`: `GrcMovieMaker`
    /// - `param` (in): `GrcOffscreenRecordingParameter`
    pub fn grcMovieMakerStart(m: *mut GrcMovieMaker, param: *const GrcOffscreenRecordingParameter) -> Result;

    /// Finishes recording with the specified MovieMaker.
    ///
    /// # Arguments
    /// - `m`: `GrcMovieMaker`
    /// - `width`: Width for the thumbnail, must be 1280.
    /// - `height`: Height for the thumbnail, must be 720.
    /// - `userdata` (in): UserData input buffer for the JPEG thumbnail. Optional, can be NULL.
    /// - `userdata_size` (in): Size of the UserData input buffer. Optional, can be 0. Must be &lt;=0x400.
    /// - `thumbnail` (in): RGBA8 image buffer containing the thumbnail. Optional, can be NULL.
    /// - `thumbnail_size` (in): Size of the thumbnail buffer. Optional, can be 0.
    /// - `entry` (out): Output `CapsApplicationAlbumEntry` for the recorded video. Optional, can be NULL. Only available on [7.0.0+], if this is not NULL on pre-7.0.0 an error is thrown.
    ///
    /// **Note:** This automatically uses `grcMovieMakerAbort` on error.
    ///
    /// **Note:** The recorded video will not be accessible via the Album-applet since it's stored separately from other Album data.
    pub fn grcMovieMakerFinish(m: *mut GrcMovieMaker, width: s32, height: s32, userdata: *const c_void, userdata_size: usize, thumbnail: *const c_void, thumbnail_size: usize, entry: *mut CapsApplicationAlbumEntry) -> Result;

    /// Gets the recording error with the specified MovieMaker.
    ///
    /// # Arguments
    /// - `m`: `GrcMovieMaker`
    pub fn grcMovieMakerGetError(m: *mut GrcMovieMaker) -> Result;

    /// Encodes audio sample data with the specified MovieMaker.
    ///
    /// # Arguments
    /// - `m`: `GrcMovieMaker`
    /// - `buffer` (in): Audio buffer.
    /// - `size` (in): Size of the buffer.
    ///
    /// **Note:** This waits on the event and uses the cmd repeatedly until the entire input buffer is handled.
    ///
    /// **Note:** If you don't use this the recorded video will be missing audio.
    pub fn grcMovieMakerEncodeAudioSample(m: *mut GrcMovieMaker, buffer: *const c_void, size: usize) -> Result;

    /// Initialize grc:d.
    pub fn grcdInitialize() -> Result;

    /// Exit grc:d.
    pub fn grcdExit();

    /// Gets the Service for grc:d.
    pub fn grcdGetServiceSession() -> *mut Service;

    /// Begins streaming. This must not be called more than once, even from a different service session: otherwise the sysmodule will assert.
    pub fn grcdBegin() -> Result;

    /// Retrieves stream data from the continuous recorder in use (from the video recording of the currently running application).
    ///
    /// # Arguments
    /// - `stream` (in): `GrcStream`
    /// - `buffer` (out): Output buffer.
    /// - `size` (in): Max size of the output buffer.
    /// - `num_frames` (out): num_frames
    /// - `data_size` (out): Actual output data size.
    /// - `start_timestamp` (out): Start timestamp.
    ///
    /// **Note:** This will block until data is available. This will hang if there is no application running which has video capture enabled.
    pub fn grcdTransfer(stream: GrcStream, buffer: *mut c_void, size: usize, num_frames: *mut u32, data_size: *mut u32, start_timestamp: *mut u64) -> Result;
}
