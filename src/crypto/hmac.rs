//! `switch/crypto/hmac.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const HMAC_SHA1_KEY_MAX: u64 = 64;
pub const HMAC_SHA256_KEY_MAX: u64 = 64;

/// Context for HMAC-SHA1 operations.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HmacSha1Context {
    pub sha_ctx: Sha1Context,
    pub key: [u32; 16],
    pub mac: [u32; 5],
    pub finalized: bool,
}

/// Context for HMAC-SHA256 operations.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HmacSha256Context {
    pub sha_ctx: Sha256Context,
    pub key: [u32; 16],
    pub mac: [u32; 8],
    pub finalized: bool,
}

impl_zeroed_default!(HmacSha1Context, HmacSha256Context);

unsafe extern "C" {
    /// Initialize a HMAC-SHA256 context.
    pub fn hmacSha256ContextCreate(out: *mut HmacSha256Context, key: *const c_void, key_size: usize);

    /// Updates HMAC-SHA256 context with data to hash
    pub fn hmacSha256ContextUpdate(ctx: *mut HmacSha256Context, src: *const c_void, size: usize);

    /// Gets the context's output mac, finalizes the context.
    pub fn hmacSha256ContextGetMac(ctx: *mut HmacSha256Context, dst: *mut c_void);

    /// Simple all-in-one HMAC-SHA256 calculator.
    pub fn hmacSha256CalculateMac(dst: *mut c_void, key: *const c_void, key_size: usize, src: *const c_void, size: usize);

    /// Initialize a HMAC-SHA1 context.
    pub fn hmacSha1ContextCreate(out: *mut HmacSha1Context, key: *const c_void, key_size: usize);

    /// Updates HMAC-SHA1 context with data to hash
    pub fn hmacSha1ContextUpdate(ctx: *mut HmacSha1Context, src: *const c_void, size: usize);

    /// Gets the context's output mac, finalizes the context.
    pub fn hmacSha1ContextGetMac(ctx: *mut HmacSha1Context, dst: *mut c_void);

    /// Simple all-in-one HMAC-SHA1 calculator.
    pub fn hmacSha1CalculateMac(dst: *mut c_void, key: *const c_void, key_size: usize, src: *const c_void, size: usize);
}
