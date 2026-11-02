//! `switch/nvidia/graphic_buffer.h`

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
pub struct NvSurface {
    pub width: u32,
    pub height: u32,
    pub color_format: NvColorFormat,
    pub layout: NvLayout,
    pub pitch: u32,
    pub unused: u32,
    pub offset: u32,
    pub kind: NvKind,
    pub block_height_log2: u32,
    pub scan: NvDisplayScanFormat,
    pub second_field_offset: u32,
    pub flags: u64,
    pub size: u64,
    pub unk: [u32; 6],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NvGraphicBuffer {
    pub header: NativeHandle,
    pub unk0: s32,
    pub nvmap_id: s32,
    pub unk2: u32,
    pub magic: u32,
    pub pid: u32,
    pub type_: u32,
    pub usage: u32,
    pub format: u32,
    pub ext_format: u32,
    pub stride: u32,
    pub total_size: u32,
    pub num_planes: u32,
    pub unk12: u32,
    pub planes: [NvSurface; 3],
    pub unused: u64,
}

impl_zeroed_default!(NvSurface, NvGraphicBuffer);
