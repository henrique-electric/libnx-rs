//! `switch/nro.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const NROHEADER_MAGIC: u32 = 0x304f524e;
pub const NROASSETHEADER_MAGIC: u32 = 0x54455341;
pub const NROASSETHEADER_VERSION: u32 = 0;

/// Entry for each segment in the codebin.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NroSegment {
    pub file_off: u32,
    pub size: u32,
}

/// Offset 0x0 in the NRO.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NroStart {
    pub unused: u32,
    pub mod_offset: u32,
    pub padding: [u8; 8],
}

/// This follows NroStart, the actual nro-header.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NroHeader {
    pub magic: u32,
    pub unk1: u32,
    pub size: u32,
    pub unk2: u32,
    pub segments: [NroSegment; 3],
    pub bss_size: u32,
    pub unk3: u32,
    pub build_id: [u8; 32],
    pub padding: [u8; 32],
}

/// Custom asset section.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NroAssetSection {
    pub offset: u64,
    pub size: u64,
}

/// Custom asset header.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NroAssetHeader {
    pub magic: u32,
    pub version: u32,
    pub icon: NroAssetSection,
    pub nacp: NroAssetSection,
    pub romfs: NroAssetSection,
}

impl_zeroed_default!(NroSegment, NroStart, NroHeader, NroAssetSection, NroAssetHeader);
