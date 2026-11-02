//! `switch/crypto/aes_ctr.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Context for AES-128 CTR.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Aes128CtrContext {
    pub aes_ctx: Aes128Context,
    pub ctr: [u8; 16],
    pub enc_ctr_buffer: [u8; 16],
    pub buffer_offset: usize,
}

/// Context for AES-192 CTR.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Aes192CtrContext {
    pub aes_ctx: Aes192Context,
    pub ctr: [u8; 16],
    pub enc_ctr_buffer: [u8; 16],
    pub buffer_offset: usize,
}

/// Context for AES-256 CTR.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Aes256CtrContext {
    pub aes_ctx: Aes256Context,
    pub ctr: [u8; 16],
    pub enc_ctr_buffer: [u8; 16],
    pub buffer_offset: usize,
}

impl_zeroed_default!(Aes128CtrContext, Aes192CtrContext, Aes256CtrContext);

unsafe extern "C" {
    /// 128-bit CTR API.
    pub fn aes128CtrContextCreate(out: *mut Aes128CtrContext, key: *const c_void, ctr: *const c_void);

    pub fn aes128CtrContextResetCtr(ctx: *mut Aes128CtrContext, ctr: *const c_void);

    pub fn aes128CtrCrypt(ctx: *mut Aes128CtrContext, dst: *mut c_void, src: *const c_void, size: usize);

    /// 192-bit CTR API.
    pub fn aes192CtrContextCreate(out: *mut Aes192CtrContext, key: *const c_void, ctr: *const c_void);

    pub fn aes192CtrContextResetCtr(ctx: *mut Aes192CtrContext, ctr: *const c_void);

    pub fn aes192CtrCrypt(ctx: *mut Aes192CtrContext, dst: *mut c_void, src: *const c_void, size: usize);

    /// 256-bit CTR API.
    pub fn aes256CtrContextCreate(out: *mut Aes256CtrContext, key: *const c_void, ctr: *const c_void);

    pub fn aes256CtrContextResetCtr(ctx: *mut Aes256CtrContext, ctr: *const c_void);

    pub fn aes256CtrCrypt(ctx: *mut Aes256CtrContext, dst: *mut c_void, src: *const c_void, size: usize);
}
