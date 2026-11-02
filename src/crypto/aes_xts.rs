//! `switch/crypto/aes_xts.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Context for AES-128 XTS.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Aes128XtsContext {
    pub aes_ctx: Aes128Context,
    pub tweak_ctx: Aes128Context,
    pub tweak: [u8; 16],
    pub buffer: [u8; 16],
    pub num_buffered: usize,
}

/// Context for AES-192 XTS.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Aes192XtsContext {
    pub aes_ctx: Aes192Context,
    pub tweak_ctx: Aes192Context,
    pub tweak: [u8; 16],
    pub buffer: [u8; 16],
    pub num_buffered: usize,
}

/// Context for AES-256 XTS.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Aes256XtsContext {
    pub aes_ctx: Aes256Context,
    pub tweak_ctx: Aes256Context,
    pub tweak: [u8; 16],
    pub buffer: [u8; 16],
    pub num_buffered: usize,
}

impl_zeroed_default!(Aes128XtsContext, Aes192XtsContext, Aes256XtsContext);

unsafe extern "C" {
    /// 128-bit XTS API.
    pub fn aes128XtsContextCreate(out: *mut Aes128XtsContext, key0: *const c_void, key1: *const c_void, is_encryptor: bool);

    pub fn aes128XtsContextResetTweak(ctx: *mut Aes128XtsContext, tweak: *const c_void);

    pub fn aes128XtsContextResetSector(ctx: *mut Aes128XtsContext, sector: u64, is_nintendo: bool);

    pub fn aes128XtsEncrypt(ctx: *mut Aes128XtsContext, dst: *mut c_void, src: *const c_void, size: usize) -> usize;

    pub fn aes128XtsDecrypt(ctx: *mut Aes128XtsContext, dst: *mut c_void, src: *const c_void, size: usize) -> usize;

    /// 192-bit XTS API.
    pub fn aes192XtsContextCreate(out: *mut Aes192XtsContext, key0: *const c_void, key1: *const c_void, is_encryptor: bool);

    pub fn aes192XtsContextResetTweak(ctx: *mut Aes192XtsContext, tweak: *const c_void);

    pub fn aes192XtsContextResetSector(ctx: *mut Aes192XtsContext, sector: u64, is_nintendo: bool);

    pub fn aes192XtsEncrypt(ctx: *mut Aes192XtsContext, dst: *mut c_void, src: *const c_void, size: usize) -> usize;

    pub fn aes192XtsDecrypt(ctx: *mut Aes192XtsContext, dst: *mut c_void, src: *const c_void, size: usize) -> usize;

    /// 256-bit XTS API.
    pub fn aes256XtsContextCreate(out: *mut Aes256XtsContext, key0: *const c_void, key1: *const c_void, is_encryptor: bool);

    pub fn aes256XtsContextResetTweak(ctx: *mut Aes256XtsContext, tweak: *const c_void);

    pub fn aes256XtsContextResetSector(ctx: *mut Aes256XtsContext, sector: u64, is_nintendo: bool);

    pub fn aes256XtsEncrypt(ctx: *mut Aes256XtsContext, dst: *mut c_void, src: *const c_void, size: usize) -> usize;

    pub fn aes256XtsDecrypt(ctx: *mut Aes256XtsContext, dst: *mut c_void, src: *const c_void, size: usize) -> usize;
}
