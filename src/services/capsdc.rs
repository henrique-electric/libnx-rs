//! `switch/services/capsdc.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

unsafe extern "C" {
    /// Initialize caps:dc
    pub fn capsdcInitialize() -> Result;

    /// Exit caps:dc.
    pub fn capsdcExit();

    /// Gets the Service for caps:dc.
    pub fn capsdcGetServiceSession() -> *mut Service;

    /// Decodes a jpeg buffer into RGBX.
    ///
    /// # Arguments
    /// - `width` (in): Image width.
    /// - `height` (in): Image height.
    /// - `opts` (in): `CapsScreenShotDecodeOption`.
    /// - `jpeg` (in): Jpeg image input buffer.
    /// - `jpeg_size` (in): Input image buffer size.
    /// - `out_image` (out): RGBA8 image output buffer.
    /// - `out_image_size` (in): Output image buffer size, should be at least large enough for RGBA8 width x height.
    pub fn capsdcDecodeJpeg(width: u32, height: u32, opts: *const CapsScreenShotDecodeOption, jpeg: *const c_void, jpeg_size: usize, out_image: *mut c_void, out_image_size: usize) -> Result;

    /// Shrinks a jpeg's dimensions by 2.
    ///
    /// # Arguments
    /// - `width` (in): Input image width.
    /// - `height` (in): Input image width.
    /// - `opts` (in): `CapsScreenShotDecodeOption`.
    /// - `jpeg` (in): Jpeg image input buffer.
    /// - `jpeg_size` (in): Input image buffer size.
    /// - `out_jpeg` (out): Jpeg image output buffer
    /// - `out_jpeg_size` (in): Output image buffer size.
    /// - `out_result_size` (out): size of the resulting JPEG.
    ///
    /// **Note:** Tries to compress with jpeg quality in this order: 98, 95, 90, 80, 70, 60, 50, 40, 30, 20, 10, 0.
    ///
    /// **Note:** Only available on [17.0.0+].
    pub fn capsdcShrinkJpeg(width: u32, height: u32, opts: *const CapsScreenShotDecodeOption, jpeg: *const c_void, jpeg_size: usize, out_jpeg: *mut c_void, out_jpeg_size: usize, out_result_size: *mut u64) -> Result;

    /// Shrinks a jpeg.
    ///
    /// # Arguments
    /// - `scaled_width` (in): Wanted image width.
    /// - `scaled_height` (in): Wanted image width.
    /// - `jpeg_quality` (in): has to be in range 0-100.
    /// - `opts` (in): `CapsScreenShotDecodeOption`.
    /// - `jpeg` (in): Jpeg image input buffer.
    /// - `jpeg_size` (in): Input image buffer size.
    /// - `out_jpeg` (out): Jpeg image output buffer
    /// - `out_jpeg_size` (in): Output image buffer size.
    /// - `out_result_size` (out): size of the resulting jpeg.
    ///
    /// **Note:** Fails if the scaled size is larger than the original or the output buffer isn't large enough.
    ///
    /// **Note:** Only available on [19.0.0+].
    pub fn capsdcShrinkJpegEx(scaled_width: u32, scaled_height: u32, jpeg_quality: u32, opts: *const CapsScreenShotDecodeOption, jpeg: *const c_void, jpeg_size: usize, out_jpeg: *mut c_void, out_jpeg_size: usize, out_result_size: *mut u64) -> Result;
}
