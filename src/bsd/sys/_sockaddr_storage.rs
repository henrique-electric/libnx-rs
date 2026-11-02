//! `sys/_sockaddr_storage.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const _SS_MAXSIZE: u32 = 128;
pub const _SS_ALIGNSIZE: u64 = 8;
pub const _SS_PAD1SIZE: u64 = 6;
pub const _SS_PAD2SIZE: u64 = 112;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct sockaddr_storage {
    pub ss_len: c_uchar,
    pub ss_family: sa_family_t,
    pub __ss_pad1: [c_char; 6],
    pub __ss_align: i64,
    pub __ss_pad2: [c_char; 112],
}

impl_zeroed_default!(sockaddr_storage);
