//! `switch/display/binder.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const BINDER_FIRST_CALL_TRANSACTION: u32 = 0x1;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Binder {
    pub created: bool,
    pub initialized: bool,
    pub id: s32,
    pub dummy: usize,
    pub relay: *mut Service,
}

/// # Safety
/// `b` must be an initialized binder.
#[inline(always)]
pub unsafe fn binderIncreaseWeakRef(b: &mut Binder) -> Result {
    unsafe { binderAdjustRefcount(b, 1, 0) }
}

/// # Safety
/// `b` must be an initialized binder.
#[inline(always)]
pub unsafe fn binderDecreaseWeakRef(b: &mut Binder) -> Result {
    unsafe { binderAdjustRefcount(b, -1, 0) }
}

/// # Safety
/// `b` must be an initialized binder.
#[inline(always)]
pub unsafe fn binderIncreaseStrongRef(b: &mut Binder) -> Result {
    unsafe { binderAdjustRefcount(b, 1, 1) }
}

/// # Safety
/// `b` must be an initialized binder.
#[inline(always)]
pub unsafe fn binderDecreaseStrongRef(b: &mut Binder) -> Result {
    unsafe { binderAdjustRefcount(b, -1, 1) }
}

impl_zeroed_default!(Binder);

unsafe extern "C" {
    pub fn binderCreate(b: *mut Binder, id: s32);

    pub fn binderClose(b: *mut Binder);

    pub fn binderInitSession(b: *mut Binder, relay: *mut Service) -> Result;

    pub fn binderTransactParcel(b: *mut Binder, code: u32, parcel_data: *mut c_void, parcel_data_size: usize, parcel_reply: *mut c_void, parcel_reply_size: usize, flags: u32) -> Result;

    pub fn binderConvertErrorCode(code: s32) -> Result;

    pub fn binderAdjustRefcount(b: *mut Binder, addval: s32, type_: s32) -> Result;

    pub fn binderGetNativeHandle(b: *mut Binder, unk0: u32, event_out: *mut Event) -> Result;
}
