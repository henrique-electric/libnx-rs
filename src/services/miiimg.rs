//! `switch/services/miiimg.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Image ID.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MiiimgImageId {
    pub uuid: Uuid,
}

/// Image attribute.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MiiimgImageAttribute {
    /// Image ID.
    pub image_id: MiiimgImageId,
    /// Mii's create ID.
    pub create_id: MiiCreateId,
    pub unk: u32,
    /// utf-16be, null-terminated
    pub mii_name: [u16; 11],
}

impl_zeroed_default!(MiiimgImageId, MiiimgImageAttribute);

unsafe extern "C" {
    /// Initialize miiimg.
    pub fn miiimgInitialize() -> Result;

    /// Exit miiimg.
    pub fn miiimgExit();

    /// Gets the Service object for the actual miiimg service session.
    pub fn miiimgGetServiceSession() -> *mut Service;

    /// Reloads the image database.
    pub fn miiimgReload() -> Result;

    /// Gets the number of mii images in the database.
    ///
    /// # Arguments
    /// - `out_count` (out): Mii image count.
    pub fn miiimgGetCount(out_count: *mut s32) -> Result;

    /// Gets whether the image database is empty.
    ///
    /// # Arguments
    /// - `out_empty` (out): Whether the database is empty.
    pub fn miiimgIsEmpty(out_empty: *mut bool) -> Result;

    /// Gets whether the image database is full.
    ///
    /// # Arguments
    /// - `out_empty` (out): Whether the database is full.
    pub fn miiimgIsFull(out_full: *mut bool) -> Result;

    /// Gets the image attribute for the specified image index.
    ///
    /// # Arguments
    /// - `index` (in): Image index.
    /// - `out_attr` (out): Out image attribute.
    pub fn miiimgGetAttribute(index: s32, out_attr: *mut MiiimgImageAttribute) -> Result;

    /// Loads the image data (raw RGBA8) for the specified image ID.
    ///
    /// # Arguments
    /// - `id` (in): Input image ID.
    /// - `out_image` (out): Out iamge buffer.
    /// - `out_image_size` (in): Out image buffer size.
    ///
    /// **Note:** Server doesn't seem to check the image buffer size, but 0x40000 is the optimal size.
    pub fn miiimgLoadImage(id: MiiimgImageId, out_image: *mut c_void, out_image_size: usize) -> Result;
}
