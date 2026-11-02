//! `switch/crypto/aes.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const AES_BLOCK_SIZE: u32 = 0x10;
pub const AES_128_KEY_SIZE: u32 = 0x10;
pub const AES_128_U32_PER_KEY: u64 = 4;
pub const AES_128_NUM_ROUNDS: u32 = 10;
pub const AES_192_KEY_SIZE: u32 = 0x18;
pub const AES_192_U32_PER_KEY: u64 = 6;
pub const AES_192_NUM_ROUNDS: u32 = 12;
pub const AES_256_KEY_SIZE: u32 = 0x20;
pub const AES_256_U32_PER_KEY: u64 = 8;
pub const AES_256_NUM_ROUNDS: u32 = 14;

/// Context for AES-128 operations.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Aes128Context {
    pub round_keys: [[u8; 16]; 11],
}

/// Context for AES-192 operations.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Aes192Context {
    pub round_keys: [[u8; 16]; 13],
}

/// Context for AES-256 operations.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Aes256Context {
    pub round_keys: [[u8; 16]; 15],
}

impl_zeroed_default!(Aes128Context, Aes192Context, Aes256Context);

unsafe extern "C" {
    /// Initialize a 128-bit AES context.
    pub fn aes128ContextCreate(out: *mut Aes128Context, key: *const c_void, is_encryptor: bool);

    /// Encrypt using an AES context (Requires is_encryptor when initializing)
    pub fn aes128EncryptBlock(ctx: *const Aes128Context, dst: *mut c_void, src: *const c_void);

    /// Decrypt using an AES context (Requires !is_encryptor when initializing)
    pub fn aes128DecryptBlock(ctx: *const Aes128Context, dst: *mut c_void, src: *const c_void);

    /// Initialize a 192-bit AES context.
    pub fn aes192ContextCreate(out: *mut Aes192Context, key: *const c_void, is_encryptor: bool);

    /// Encrypt using an AES context (Requires is_encryptor when initializing)
    pub fn aes192EncryptBlock(ctx: *const Aes192Context, dst: *mut c_void, src: *const c_void);

    /// Decrypt using an AES context (Requires !is_encryptor when initializing)
    pub fn aes192DecryptBlock(ctx: *const Aes192Context, dst: *mut c_void, src: *const c_void);

    /// Initialize a 256-bit AES context.
    pub fn aes256ContextCreate(out: *mut Aes256Context, key: *const c_void, is_encryptor: bool);

    /// Encrypt using an AES context (Requires is_encryptor when initializing)
    pub fn aes256EncryptBlock(ctx: *const Aes256Context, dst: *mut c_void, src: *const c_void);

    /// Decrypt using an AES context (Requires !is_encryptor when initializing)
    pub fn aes256DecryptBlock(ctx: *const Aes256Context, dst: *mut c_void, src: *const c_void);
}
