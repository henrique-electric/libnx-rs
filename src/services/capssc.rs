//! `switch/services/capssc.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const CAPSSC_JPEG_BUFFER_SIZE: u32 = 0x80000;

unsafe extern "C" {
    /// Initialize caps:sc. Only available on [2.0.0+].
    pub fn capsscInitialize() -> Result;

    /// Exit caps:sc.
    pub fn capsscExit();

    /// Gets the Service for caps:sc.
    pub fn capsscGetServiceSession() -> *mut Service;

    /// This takes a screenshot, with the screenshot being written into the output buffer.
    ///
    /// # Arguments
    /// - `buf`: Output buffer containing the RGBA8 image.
    /// - `size`: Size of buf, should be 0x384000(1280*720*4) * buffer_count.
    /// - `layer_stack`: `ViLayerStack`
    /// - `width`: Image width, must be 1280.
    /// - `height`: Image height, must be 720.
    /// - `buffer_count`: Total number of output image buffers.
    /// - `buffer_index`: Starting image buffer index. Must be &lt; buffer_count.
    /// - `timeout`: Timeout in nanoseconds. A default value of 100000000 can be used.
    ///
    /// **Note:** Not available with [5.0.0+] (stubbed).
    ///
    /// **Note:** buffer_index and buffer_count correspond to buffers with size 0x384000(1280*720*4). These must not be negative.
    pub fn capsscCaptureRawImageWithTimeout(buf: *mut c_void, size: usize, layer_stack: ViLayerStack, width: u64, height: u64, buffer_count: s64, buffer_index: s64, timeout: s64) -> Result;

    /// This takes a raw screenshot, with the screenshot being held until `capsscCloseRawScreenShotReadStream` is called.
    ///
    /// # Arguments
    /// - `out_size`: Pointer to write the size of the captured raw image to. Always 0x384000(1280*720*4).
    /// - `out_width`: Pointer to write the width of the captured raw image to. Always 1280.
    /// - `out_height`: Pointer to write the height of the captured raw image to. Always 720.
    /// - `layer_stack`: `ViLayerStack`
    /// - `timeout`: Timeout in nanoseconds.
    ///
    /// **Note:** Only available on [3.0.0+]. Requires debug mode.
    pub fn capsscOpenRawScreenShotReadStream(out_size: *mut u64, out_width: *mut u64, out_height: *mut u64, layer_stack: ViLayerStack, timeout: s64) -> Result;

    /// Discards a stream opened by `capsscOpenRawScreenShotReadStream`.
    ///
    /// **Note:** Only available on [3.0.0+]. Requires debug mode.
    pub fn capsscCloseRawScreenShotReadStream() -> Result;

    /// Reads from a stream opened by `capsscOpenRawScreenShotReadStream`.
    ///
    /// # Arguments
    /// - `bytes_read`: Pointer to write the amounts of bytes written to buffer.
    /// - `buf`: Output buffer containing the RGBA8 image.
    /// - `size`: Size of buf.
    /// - `offset`: Offset in image where read should start.
    ///
    /// **Note:** Only available on [3.0.0+]. Requires debug mode.
    pub fn capsscReadRawScreenShotReadStream(bytes_read: *mut u64, buf: *mut c_void, size: usize, offset: u64) -> Result;

    /// This takes a screenshot, with the screenshot being written as jpeg into the output buffer.
    ///
    /// # Arguments
    /// - `out_jpeg_size`: Pointer to write the size of the captured jpeg to.
    /// - `jpeg_buf`: Output buffer containing the JPEG image.
    /// - `jpeg_buf_size`: Size of jpeg_buf, official software uses 0x80000.
    /// - `layer_stack`: `ViLayerStack`
    /// - `timeout`: Timeout in nanoseconds.
    ///
    /// **Note:** Only available on [9.0.0+]. Requires debug mode before [10.0.0].
    pub fn capsscCaptureJpegScreenShot(out_jpeg_size: *mut u64, jpeg_buf: *mut c_void, jpeg_buf_size: usize, layer_stack: ViLayerStack, timeout: s64) -> Result;
}
