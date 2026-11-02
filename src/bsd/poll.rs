//! `poll.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const POLLIN: u32 = 0x1;
pub const POLLPRI: u32 = 0x2;
pub const POLLOUT: u32 = 0x4;
pub const POLLRDNORM: u32 = 0x40;
pub const POLLWRNORM: u32 = 4;
pub const POLLRDBAND: u32 = 0x80;
pub const POLLWRBAND: u32 = 0x100;
pub const POLLINIGNEOF: u32 = 0x2000;
pub const POLLERR: u32 = 0x8;
pub const POLLHUP: u32 = 0x10;
pub const POLLNVAL: u32 = 0x20;
pub const POLLSTANDARD: u32 = 511;
pub const INFTIM: i32 = -1;

pub type nfds_t = c_uint;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct pollfd {
    pub fd: c_int,
    pub events: c_short,
    pub revents: c_short,
}

impl_zeroed_default!(pollfd);

unsafe extern "C" {
    pub fn poll(pfd: *mut pollfd, nfds: nfds_t, timeout: c_int) -> c_int;
}
