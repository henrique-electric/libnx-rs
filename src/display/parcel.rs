//! `switch/display/parcel.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const PARCEL_MAX_PAYLOAD: u32 = 0x400;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ParcelHeader {
    pub payload_size: u32,
    pub payload_off: u32,
    pub objects_size: u32,
    pub objects_off: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Parcel {
    pub payload: [u8; 1024],
    pub payload_size: u32,
    pub objects: *mut u8,
    pub objects_size: u32,
    pub capacity: u32,
    pub pos: u32,
}

impl_zeroed_default!(ParcelHeader, Parcel);

unsafe extern "C" {
    pub fn parcelCreate(ctx: *mut Parcel);

    pub fn parcelTransact(session: *mut Binder, code: u32, in_parcel: *mut Parcel, reply_parcel: *mut Parcel) -> Result;

    pub fn parcelWriteData(ctx: *mut Parcel, data: *const c_void, data_size: usize) -> *mut c_void;

    pub fn parcelReadData(ctx: *mut Parcel, data: *mut c_void, data_size: usize) -> *mut c_void;

    pub fn parcelWriteInt32(ctx: *mut Parcel, val: s32);

    pub fn parcelWriteUInt32(ctx: *mut Parcel, val: u32);

    pub fn parcelWriteString16(ctx: *mut Parcel, str: *const c_char);

    pub fn parcelReadInt32(ctx: *mut Parcel) -> s32;

    pub fn parcelReadUInt32(ctx: *mut Parcel) -> u32;

    pub fn parcelWriteInterfaceToken(ctx: *mut Parcel, str: *const c_char);

    pub fn parcelReadFlattenedObject(ctx: *mut Parcel, size: *mut usize) -> *mut c_void;

    pub fn parcelWriteFlattenedObject(ctx: *mut Parcel, data: *const c_void, size: usize) -> *mut c_void;
}
