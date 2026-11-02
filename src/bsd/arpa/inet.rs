//! `arpa/inet.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

#[inline(always)]
pub const fn htonl(hostlong: u32) -> u32 {
    hostlong.swap_bytes()
}

#[inline(always)]
pub const fn htons(hostshort: u16) -> u16 {
    hostshort.swap_bytes()
}

#[inline(always)]
pub const fn ntohl(netlong: u32) -> u32 {
    netlong.swap_bytes()
}

#[inline(always)]
pub const fn ntohs(netshort: u16) -> u16 {
    netshort.swap_bytes()
}

unsafe extern "C" {
    pub fn inet_addr(cp: *const c_char) -> in_addr_t;

    pub fn inet_aton(cp: *const c_char, inp: *mut in_addr) -> c_int;

    pub fn inet_ntoa(in_: in_addr) -> *mut c_char;

    pub fn inet_ntop(af: c_int, src: *const c_void, dst: *mut c_char, size: socklen_t) -> *const c_char;

    pub fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
}
