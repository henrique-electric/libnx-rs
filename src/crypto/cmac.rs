//! `switch/crypto/cmac.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Context for AES-128 CMAC.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Aes128CmacContext {
    pub ctx: Aes128Context,
    pub subkey: [u8; 16],
    pub mac: [u8; 16],
    pub buffer: [u8; 16],
    pub num_buffered: usize,
    pub finalized: bool,
}

/// Context for AES-192 CMAC.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Aes192CmacContext {
    pub ctx: Aes192Context,
    pub subkey: [u8; 16],
    pub mac: [u8; 16],
    pub buffer: [u8; 16],
    pub num_buffered: usize,
    pub finalized: bool,
}

/// Context for AES-256 CMAC.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Aes256CmacContext {
    pub ctx: Aes256Context,
    pub subkey: [u8; 16],
    pub mac: [u8; 16],
    pub buffer: [u8; 16],
    pub num_buffered: usize,
    pub finalized: bool,
}

impl_zeroed_default!(Aes128CmacContext, Aes192CmacContext, Aes256CmacContext);

unsafe extern "C" {
    /// Initialize an AES-128-CMAC context.
    pub fn cmacAes128ContextCreate(out: *mut Aes128CmacContext, key: *const c_void);

    /// Updates AES-128-CMAC context with data to hash
    pub fn cmacAes128ContextUpdate(ctx: *mut Aes128CmacContext, src: *const c_void, size: usize);

    /// Gets the context's output mac, finalizes the context.
    pub fn cmacAes128ContextGetMac(ctx: *mut Aes128CmacContext, dst: *mut c_void);

    /// Simple all-in-one AES-128-CMAC calculator.
    pub fn cmacAes128CalculateMac(dst: *mut c_void, key: *const c_void, src: *const c_void, size: usize);

    /// Initialize an AES-192-CMAC context.
    pub fn cmacAes192ContextCreate(out: *mut Aes192CmacContext, key: *const c_void);

    /// Updates AES-192-CMAC context with data to hash
    pub fn cmacAes192ContextUpdate(ctx: *mut Aes192CmacContext, src: *const c_void, size: usize);

    /// Gets the context's output mac, finalizes the context.
    pub fn cmacAes192ContextGetMac(ctx: *mut Aes192CmacContext, dst: *mut c_void);

    /// Simple all-in-one AES-192-CMAC calculator.
    pub fn cmacAes192CalculateMac(dst: *mut c_void, key: *const c_void, src: *const c_void, size: usize);

    /// Initialize an AES-256-CMAC context.
    pub fn cmacAes256ContextCreate(out: *mut Aes256CmacContext, key: *const c_void);

    /// Updates AES-256-CMAC context with data to hash
    pub fn cmacAes256ContextUpdate(ctx: *mut Aes256CmacContext, src: *const c_void, size: usize);

    /// Gets the context's output mac, finalizes the context.
    pub fn cmacAes256ContextGetMac(ctx: *mut Aes256CmacContext, dst: *mut c_void);

    /// Simple all-in-one AES-256-CMAC calculator.
    pub fn cmacAes256CalculateMac(dst: *mut c_void, key: *const c_void, src: *const c_void, size: usize);
}
