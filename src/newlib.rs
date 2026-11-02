//! Types from devkitA64's newlib that the libnx headers refer to.

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;

/// `va_list` is a 32-byte struct on AArch64 that is passed by reference; Rust has no stable equivalent.
pub type va_list = *mut c_void;

pub type _LOCK_RECURSIVE_T = __lock_t;
pub type _LOCK_T = u32;
pub type __sa_family_t = u8;
pub type __socklen_t = u32;
pub type caddr_t = *mut c_char;
pub type in_addr_t = u32;
pub type in_port_t = u16;
pub type pid_t = __pid_t;
pub type time_t = c_long;
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: c_long,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct timeval {
    pub tv_sec: time_t,
    pub tv_usec: suseconds_t,
}

pub type u_char = c_uchar;
pub type u_int = c_uint;
pub type u_int32_t = u32;
pub type u_int64_t = u64;
pub type u_int8_t = u8;
pub type u_long = c_ulong;
pub type u_short = c_ushort;
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct __lock_t {
    pub lock: _LOCK_T,
    pub counter: u32,
}

pub type __pid_t = c_int;
pub type suseconds_t = __suseconds_t;
pub type __suseconds_t = c_long;

impl_zeroed_default!(timespec, timeval, __lock_t);
