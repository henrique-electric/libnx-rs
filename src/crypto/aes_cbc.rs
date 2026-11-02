//! `switch/crypto/aes_cbc.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Context for AES-128 CBC.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Aes128CbcContext {
    pub aes_ctx: Aes128Context,
    pub iv: [u8; 16],
    pub buffer: [u8; 16],
    pub num_buffered: usize,
}

/// Context for AES-192 CBC.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Aes192CbcContext {
    pub aes_ctx: Aes192Context,
    pub iv: [u8; 16],
    pub buffer: [u8; 16],
    pub num_buffered: usize,
}

/// Context for AES-256 CBC.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Aes256CbcContext {
    pub aes_ctx: Aes256Context,
    pub iv: [u8; 16],
    pub buffer: [u8; 16],
    pub num_buffered: usize,
}

impl_zeroed_default!(Aes128CbcContext, Aes192CbcContext, Aes256CbcContext);

unsafe extern "C" {
    /// 128-bit CBC API.
    pub fn aes128CbcContextCreate(out: *mut Aes128CbcContext, key: *const c_void, iv: *const c_void, is_encryptor: bool);

    pub fn aes128CbcContextResetIv(ctx: *mut Aes128CbcContext, iv: *const c_void);

    pub fn aes128CbcEncrypt(ctx: *mut Aes128CbcContext, dst: *mut c_void, src: *const c_void, size: usize) -> usize;

    pub fn aes128CbcDecrypt(ctx: *mut Aes128CbcContext, dst: *mut c_void, src: *const c_void, size: usize) -> usize;

    /// 192-bit CBC API.
    pub fn aes192CbcContextCreate(out: *mut Aes192CbcContext, key: *const c_void, iv: *const c_void, is_encryptor: bool);

    pub fn aes192CbcContextResetIv(ctx: *mut Aes192CbcContext, iv: *const c_void);

    pub fn aes192CbcEncrypt(ctx: *mut Aes192CbcContext, dst: *mut c_void, src: *const c_void, size: usize) -> usize;

    pub fn aes192CbcDecrypt(ctx: *mut Aes192CbcContext, dst: *mut c_void, src: *const c_void, size: usize) -> usize;

    /// 256-bit CBC API.
    pub fn aes256CbcContextCreate(out: *mut Aes256CbcContext, key: *const c_void, iv: *const c_void, is_encryptor: bool);

    pub fn aes256CbcContextResetIv(ctx: *mut Aes256CbcContext, iv: *const c_void);

    pub fn aes256CbcEncrypt(ctx: *mut Aes256CbcContext, dst: *mut c_void, src: *const c_void, size: usize) -> usize;

    pub fn aes256CbcDecrypt(ctx: *mut Aes256CbcContext, dst: *mut c_void, src: *const c_void, size: usize) -> usize;
}
