//! `switch/crypto/sha1.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const SHA1_HASH_SIZE: u32 = 0x14;
pub const SHA1_BLOCK_SIZE: u32 = 0x40;

/// Context for SHA1 operations.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Sha1Context {
    pub intermediate_hash: [u32; 5],
    pub buffer: [u8; 64],
    pub bits_consumed: u64,
    pub num_buffered: usize,
    pub finalized: bool,
}

impl_zeroed_default!(Sha1Context);

unsafe extern "C" {
    /// Initialize a SHA1 context.
    pub fn sha1ContextCreate(out: *mut Sha1Context);

    /// Updates SHA1 context with data to hash
    pub fn sha1ContextUpdate(ctx: *mut Sha1Context, src: *const c_void, size: usize);

    /// Gets the context's output hash, finalizes the context.
    pub fn sha1ContextGetHash(ctx: *mut Sha1Context, dst: *mut c_void);

    /// Simple all-in-one SHA1 calculator.
    pub fn sha1CalculateHash(dst: *mut c_void, src: *const c_void, size: usize);
}
