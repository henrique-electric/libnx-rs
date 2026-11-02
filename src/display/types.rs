//! `switch/display/types.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const PIXEL_FORMAT_RGBA_8888: u32 = 1;
pub const PIXEL_FORMAT_RGBX_8888: u32 = 2;
pub const PIXEL_FORMAT_RGB_888: u32 = 3;
pub const PIXEL_FORMAT_RGB_565: u32 = 4;
pub const PIXEL_FORMAT_BGRA_8888: u32 = 5;
pub const PIXEL_FORMAT_RGBA_5551: u32 = 6;
pub const PIXEL_FORMAT_RGBA_4444: u32 = 7;
pub const PIXEL_FORMAT_YCRCB_420_SP: u32 = 17;
pub const PIXEL_FORMAT_RAW16: u32 = 32;
pub const PIXEL_FORMAT_BLOB: u32 = 33;
pub const PIXEL_FORMAT_IMPLEMENTATION_DEFINED: u32 = 34;
pub const PIXEL_FORMAT_YCBCR_420_888: u32 = 35;
pub const PIXEL_FORMAT_Y8: u32 = 0x20203859;
pub const PIXEL_FORMAT_Y16: u32 = 0x20363159;
pub const PIXEL_FORMAT_YV12: u32 = 0x32315659;

pub const GRALLOC_USAGE_SW_READ_NEVER: u32 = 0x00000000;
pub const GRALLOC_USAGE_SW_READ_RARELY: u32 = 0x00000002;
pub const GRALLOC_USAGE_SW_READ_OFTEN: u32 = 0x00000003;
pub const GRALLOC_USAGE_SW_READ_MASK: u32 = 0x0000000F;
pub const GRALLOC_USAGE_SW_WRITE_NEVER: u32 = 0x00000000;
pub const GRALLOC_USAGE_SW_WRITE_RARELY: u32 = 0x00000020;
pub const GRALLOC_USAGE_SW_WRITE_OFTEN: u32 = 0x00000030;
pub const GRALLOC_USAGE_SW_WRITE_MASK: u32 = 0x000000F0;
pub const GRALLOC_USAGE_HW_TEXTURE: u32 = 0x00000100;
pub const GRALLOC_USAGE_HW_RENDER: u32 = 0x00000200;
pub const GRALLOC_USAGE_HW_2D: u32 = 0x00000400;
pub const GRALLOC_USAGE_HW_COMPOSER: u32 = 0x00000800;
pub const GRALLOC_USAGE_HW_FB: u32 = 0x00001000;
pub const GRALLOC_USAGE_EXTERNAL_DISP: u32 = 0x00002000;
pub const GRALLOC_USAGE_PROTECTED: u32 = 0x00004000;
pub const GRALLOC_USAGE_CURSOR: u32 = 0x00008000;
pub const GRALLOC_USAGE_HW_VIDEO_ENCODER: u32 = 0x00010000;
pub const GRALLOC_USAGE_HW_CAMERA_WRITE: u32 = 0x00020000;
pub const GRALLOC_USAGE_HW_CAMERA_READ: u32 = 0x00040000;
pub const GRALLOC_USAGE_HW_CAMERA_ZSL: u32 = 0x00060000;
pub const GRALLOC_USAGE_HW_CAMERA_MASK: u32 = 0x00060000;
pub const GRALLOC_USAGE_HW_MASK: u32 = 0x00071F00;
pub const GRALLOC_USAGE_RENDERSCRIPT: u32 = 0x00100000;

pub const NATIVE_WINDOW_WIDTH: u32 = 0;
pub const NATIVE_WINDOW_HEIGHT: u32 = 1;
pub const NATIVE_WINDOW_FORMAT: u32 = 2;

pub const NATIVE_WINDOW_API_CPU: u32 = 2;

pub const HAL_TRANSFORM_FLIP_H: u32 = 0x01;
pub const HAL_TRANSFORM_FLIP_V: u32 = 0x02;
pub const HAL_TRANSFORM_ROT_90: u32 = 0x04;
pub const HAL_TRANSFORM_ROT_180: u32 = 0x03;
pub const HAL_TRANSFORM_ROT_270: u32 = 0x07;

pub const NATIVE_WINDOW_TRANSFORM_FLIP_H: u32 = 1;
pub const NATIVE_WINDOW_TRANSFORM_FLIP_V: u32 = 2;
pub const NATIVE_WINDOW_TRANSFORM_ROT_90: u32 = 4;
pub const NATIVE_WINDOW_TRANSFORM_ROT_180: u32 = 3;
pub const NATIVE_WINDOW_TRANSFORM_ROT_270: u32 = 7;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NativeHandle {
    pub version: c_int,
    pub num_fds: c_int,
    pub num_ints: c_int,
}

impl_zeroed_default!(NativeHandle);
