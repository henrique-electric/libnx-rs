//! `net/if_arp.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const ARPHRD_ETHER: u32 = 1;
pub const ARPHRD_IEEE802: u32 = 6;
pub const ARPHRD_ARCNET: u32 = 7;
pub const ARPHRD_FRELAY: u32 = 15;
pub const ARPHRD_IEEE1394: u32 = 24;
pub const ARPHRD_INFINIBAND: u32 = 32;
pub const ARPOP_REQUEST: u32 = 1;
pub const ARPOP_REPLY: u32 = 2;
pub const ARPOP_REVREQUEST: u32 = 3;
pub const ARPOP_REVREPLY: u32 = 4;
pub const ARPOP_INVREQUEST: u32 = 8;
pub const ARPOP_INVREPLY: u32 = 9;
pub const ATF_INUSE: u32 = 0x1;
pub const ATF_COM: u32 = 0x2;
pub const ATF_PERM: u32 = 0x4;
pub const ATF_PUBL: u32 = 0x8;
pub const ATF_USETRAILERS: u32 = 0x10;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct arphdr {
    pub ar_hrd: u_short,
    pub ar_pro: u_short,
    pub ar_hln: u_char,
    pub ar_pln: u_char,
    pub ar_op: u_short,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct arpreq {
    pub arp_pa: sockaddr,
    pub arp_ha: sockaddr,
    pub arp_flags: c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct arpstat {
    pub txrequests: u64,
    pub txreplies: u64,
    pub rxrequests: u64,
    pub rxreplies: u64,
    pub received: u64,
    pub arp_spares: [u64; 4],
    pub dropped: u64,
    pub timeouts: u64,
    pub dupips: u64,
}

impl_zeroed_default!(arphdr, arpreq, arpstat);

/// Sender hardware address following an `arphdr`.
#[inline(always)]
pub fn ar_sha(ap: *mut arphdr) -> caddr_t {
    ap.wrapping_add(1).cast::<c_char>()
}

/// Sender protocol address following an `arphdr`.
///
/// # Safety
/// `ap` must point to a valid `arphdr`.
#[inline(always)]
pub unsafe fn ar_spa(ap: *mut arphdr) -> caddr_t {
    unsafe { ar_sha(ap).wrapping_add((*ap).ar_hln as usize) }
}

/// Target hardware address following an `arphdr`.
///
/// # Safety
/// `ap` must point to a valid `arphdr`.
#[inline(always)]
pub unsafe fn ar_tha(ap: *mut arphdr) -> caddr_t {
    unsafe { ar_sha(ap).wrapping_add((*ap).ar_hln as usize + (*ap).ar_pln as usize) }
}

/// Target protocol address following an `arphdr`.
///
/// # Safety
/// `ap` must point to a valid `arphdr`.
#[inline(always)]
pub unsafe fn ar_tpa(ap: *mut arphdr) -> caddr_t {
    unsafe { ar_sha(ap).wrapping_add(2 * (*ap).ar_hln as usize + (*ap).ar_pln as usize) }
}

#[inline(always)]
pub const fn arphdr_len2(ar_hln: usize, ar_pln: usize) -> usize {
    size_of::<arphdr>() + 2 * ar_hln + 2 * ar_pln
}

#[inline(always)]
pub const fn arphdr_len(ap: &arphdr) -> usize {
    arphdr_len2(ap.ar_hln as usize, ap.ar_pln as usize)
}
