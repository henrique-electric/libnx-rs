//! `switch/crypto/sha256.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const SHA256_HASH_SIZE: u32 = 0x20;
pub const SHA256_BLOCK_SIZE: u32 = 0x40;

/// Context for SHA256 operations.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Sha256Context {
    pub intermediate_hash: [u32; 8],
    pub buffer: [u8; 64],
    pub bits_consumed: u64,
    pub num_buffered: usize,
    pub finalized: bool,
}

impl_zeroed_default!(Sha256Context);

unsafe extern "C" {
    /// Initialize a SHA256 context.
    pub fn sha256ContextCreate(out: *mut Sha256Context);

    /// Updates SHA256 context with data to hash
    pub fn sha256ContextUpdate(ctx: *mut Sha256Context, src: *const c_void, size: usize);

    /// Gets the context's output hash, finalizes the context.
    pub fn sha256ContextGetHash(ctx: *mut Sha256Context, dst: *mut c_void);

    /// Simple all-in-one SHA256 calculator.
    pub fn sha256CalculateHash(dst: *mut c_void, src: *const c_void, size: usize);
}
