//! `netinet/udp.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const UDP_ENCAP: u32 = 1;
pub const UDP_VENDOR: u32 = 0x80000000;
pub const UDP_ENCAP_ESPINUDP_NON_IKE: u32 = 1;
pub const UDP_ENCAP_ESPINUDP: u32 = 2;
pub const UDP_ENCAP_ESPINUDP_PORT: u32 = 500;
pub const UDP_ENCAP_ESPINUDP_MAXFRAGLEN: u32 = 552;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct udphdr {
    pub uh_sport: u_short,
    pub uh_dport: u_short,
    pub uh_ulen: u_short,
    pub uh_sum: u_short,
}

impl_zeroed_default!(udphdr);
