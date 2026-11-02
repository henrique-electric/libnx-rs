//! `net/if_dl.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct sockaddr_dl {
    pub sdl_len: u_char,
    pub sdl_family: u_char,
    pub sdl_index: u_short,
    pub sdl_type: u_char,
    pub sdl_nlen: u_char,
    pub sdl_alen: u_char,
    pub sdl_slen: u_char,
    pub sdl_data: [c_char; 46],
}

#[repr(C)]
#[derive(Debug)]
pub struct ifnet {
    _opaque: [u8; 0],
}

impl_zeroed_default!(sockaddr_dl);

/// Pointer to the link-level address inside a `sockaddr_dl`.
///
/// # Safety
/// `s` must point to a valid `sockaddr_dl`.
#[inline(always)]
pub unsafe fn LLADDR(s: *mut sockaddr_dl) -> caddr_t {
    unsafe { (&raw mut (*s).sdl_data).cast::<c_char>().wrapping_add((*s).sdl_nlen as usize) }
}

/// Const pointer to the link-level address inside a `sockaddr_dl`.
///
/// # Safety
/// `s` must point to a valid `sockaddr_dl`.
#[inline(always)]
pub unsafe fn CLLADDR(s: *const sockaddr_dl) -> *const c_char {
    unsafe { (&raw const (*s).sdl_data).cast::<c_char>().wrapping_add((*s).sdl_nlen as usize) }
}

#[inline(always)]
pub const fn LLINDEX(s: &sockaddr_dl) -> u_short {
    s.sdl_index
}
