//! `switch/display/framebuffer.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Framebuffer structure.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Framebuffer {
    pub win: *mut NWindow,
    pub map: NvMap,
    pub buf: *mut c_void,
    pub buf_linear: *mut c_void,
    pub stride: u32,
    pub width_aligned: u32,
    pub height_aligned: u32,
    pub num_fbs: u32,
    pub fb_size: u32,
    pub has_init: bool,
}

impl_zeroed_default!(Framebuffer);

/// Converts red, green, blue, and alpha components to packed RGBA8.
#[inline(always)]
pub const fn RGBA8(r: u32, g: u32, b: u32, a: u32) -> u32 {
    (r & 0xff) | ((g & 0xff) << 8) | ((b & 0xff) << 16) | ((a & 0xff) << 24)
}

/// Same as [`RGBA8`] except with alpha=0xff.
#[inline(always)]
pub const fn RGBA8_MAXALPHA(r: u32, g: u32, b: u32) -> u32 {
    RGBA8(r, g, b, 0xff)
}

/// Same as [`RGBA8`] with alpha=0 (for RGBX8).
#[inline(always)]
pub const fn RGBX8(r: u32, g: u32, b: u32) -> u32 {
    RGBA8(r, g, b, 0)
}

/// Converts red (5 bits), green (6 bits), and blue (5 bits) components to packed RGB565.
#[inline(always)]
pub const fn RGB565(r: u32, g: u32, b: u32) -> u32 {
    (b & 0x1f) | ((g & 0x3f) << 5) | ((r & 0x1f) << 11)
}

/// Converts 8-bit red, green, and blue components to packed RGB565.
#[inline(always)]
pub const fn RGB565_FROM_RGB8(r: u32, g: u32, b: u32) -> u32 {
    RGB565(r >> 3, g >> 2, b >> 3)
}

/// Same as [`RGBA8`] except with the red and blue components swapped.
#[inline(always)]
pub const fn BGRA8(r: u32, g: u32, b: u32, a: u32) -> u32 {
    RGBA8(b, g, r, a)
}

/// Same as [`BGRA8`] except with alpha=0xff.
#[inline(always)]
pub const fn BGRA8_MAXALPHA(r: u32, g: u32, b: u32) -> u32 {
    RGBA8(b, g, r, 0xff)
}

/// Converts red, green, blue, and alpha components to packed RGBA4.
#[inline(always)]
pub const fn RGBA4(r: u32, g: u32, b: u32, a: u32) -> u32 {
    (r & 0xf) | ((g & 0xf) << 4) | ((b & 0xf) << 8) | ((a & 0xf) << 12)
}

/// Same as [`RGBA4`] except with alpha=0xf.
#[inline(always)]
pub const fn RGBA4_MAXALPHA(r: u32, g: u32, b: u32) -> u32 {
    RGBA4(r, g, b, 0xf)
}

/// Converts red, green, blue, and alpha components to packed RGBA4.
#[inline(always)]
pub const fn RGBA4_FROM_RGBA8(r: u32, g: u32, b: u32, a: u32) -> u32 {
    RGBA4(r >> 4, g >> 4, b >> 4, a >> 4)
}

/// Same as [`RGBA4_FROM_RGBA8`] except with alpha=0xf.
#[inline(always)]
pub const fn RGBA4_FROM_RGBA8_MAXALPHA(r: u32, g: u32, b: u32) -> u32 {
    RGBA4_MAXALPHA(r >> 4, g >> 4, b >> 4)
}

unsafe extern "C" {
    /// Creates a `Framebuffer` object.
    ///
    /// # Arguments
    /// - `fb` (out): Output `Framebuffer` structure.
    /// - `win` (in): Pointer to the `NWindow` to which the `Framebuffer` will be registered.
    /// - `width` (in): Desired width of the framebuffer (usually 1280).
    /// - `height` (in): Desired height of the framebuffer (usually 720).
    /// - `format` (in): Desired pixel format (see PIXEL_FORMAT_* enum).
    /// - `num_fbs` (in): Number of buffers to create. Pass 1 for single-buffering, 2 for double-buffering or 3 for triple-buffering.
    ///
    /// **Note:** Framebuffer images are stored in Tegra block linear format with 16Bx2 sector ordering, read TRM chapter 20.1 for more details.
    /// In order to use regular linear layout, consider calling `framebufferMakeLinear` after the `Framebuffer` object is created.
    ///
    /// **Note:** Presently, only the following pixel formats are supported:
    ///
    /// **Ref:** PIXEL_FORMAT_RGBA_8888
    ///
    /// **Ref:** PIXEL_FORMAT_RGBX_8888
    ///
    /// **Ref:** PIXEL_FORMAT_RGB_565
    ///
    /// **Ref:** PIXEL_FORMAT_BGRA_8888
    ///
    /// **Ref:** PIXEL_FORMAT_RGBA_4444
    pub fn framebufferCreate(fb: *mut Framebuffer, win: *mut NWindow, width: u32, height: u32, format: u32, num_fbs: u32) -> Result;

    /// Enables linear framebuffer mode in a `Framebuffer`, allocating a shadow buffer in the process.
    pub fn framebufferMakeLinear(fb: *mut Framebuffer) -> Result;

    /// Closes a `Framebuffer` object, freeing all resources associated with it.
    pub fn framebufferClose(fb: *mut Framebuffer);

    /// Begins rendering a frame in a `Framebuffer`.
    ///
    /// # Arguments
    /// - `fb` (in): Pointer to `Framebuffer` structure.
    /// - `out_stride` (out): Output variable containing the distance in bytes between rows of pixels in memory.
    ///
    /// **Returns:** Pointer to buffer to which new graphics data should be written to.
    ///
    /// **Note:** When this function is called, a buffer will be dequeued from the corresponding `NWindow`.
    ///
    /// **Note:** This function will return pointers to different buffers, depending on the number of buffers it was initialized with.
    ///
    /// **Note:** If `framebufferMakeLinear` was used, this function will instead return a pointer to the shadow linear buffer.
    /// In this case, the offset of a pixel is `y` * `out_stride` + `x` * `bytes_per_pixel`.
    ///
    /// **Note:** Each call to `framebufferBegin` must be paired with a `framebufferEnd` call.
    pub fn framebufferBegin(fb: *mut Framebuffer, out_stride: *mut u32) -> *mut c_void;

    /// Finishes rendering a frame in a `Framebuffer`.
    ///
    /// # Arguments
    /// - `fb` (in): Pointer to `Framebuffer` structure.
    ///
    /// **Note:** When this function is called, the written image data will be flushed and queued (presented) in the corresponding `NWindow`.
    ///
    /// **Note:** If `framebufferMakeLinear` was used, this function will copy the image from the shadow linear buffer to the actual framebuffer,
    /// converting it in the process to the layout expected by the compositor.
    ///
    /// **Note:** Each call to `framebufferBegin` must be paired with a `framebufferEnd` call.
    pub fn framebufferEnd(fb: *mut Framebuffer);
}
