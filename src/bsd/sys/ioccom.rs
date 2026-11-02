//! `sys/ioccom.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const IOCPARM_SHIFT: u32 = 13;
pub const IOCPARM_MASK: u32 = 0x1fff;
pub const IOCPARM_MAX: u32 = 0x2000;
pub const IOC_VOID: u32 = 0x20000000;
pub const IOC_OUT: u32 = 0x40000000;
pub const IOC_IN: u32 = 0x80000000;
pub const IOC_INOUT: u32 = 0xc0000000;
pub const IOC_DIRMASK: u32 = 0xe0000000;

#[inline(always)]
pub const fn IOCPARM_LEN(x: u32) -> u32 { (x >> 16) & IOCPARM_MASK }

#[inline(always)]
pub const fn IOCBASECMD(x: u32) -> u32 { x & !(IOCPARM_MASK << 16) }

#[inline(always)]
pub const fn IOCGROUP(x: u32) -> u32 { (x >> 8) & 0xff }

/// Builds an ioctl number (an `int`, as in libnx's `ioccom.h`).
#[inline(always)]
pub const fn _IOC(inout: u32, group: u32, num: u32, len: usize) -> c_int {
    (inout | (((len as u32) & IOCPARM_MASK) << 16) | (group << 8) | num) as c_int
}

#[inline(always)]
pub const fn _IO(g: u32, n: u32) -> c_int { _IOC(IOC_VOID, g, n, 0) }

#[inline(always)]
pub const fn _IOWINT(g: u32, n: u32) -> c_int { _IOC(IOC_VOID, g, n, size_of::<c_int>()) }

/// `len` is the C macro's `sizeof(t)` argument.
#[inline(always)]
pub const fn _IOR(g: u32, n: u32, len: usize) -> c_int { _IOC(IOC_OUT, g, n, len) }

/// `len` is the C macro's `sizeof(t)` argument.
#[inline(always)]
pub const fn _IOW(g: u32, n: u32, len: usize) -> c_int { _IOC(IOC_IN, g, n, len) }

/// `len` is the C macro's `sizeof(t)` argument.
#[inline(always)]
pub const fn _IOWR(g: u32, n: u32, len: usize) -> c_int { _IOC(IOC_INOUT, g, n, len) }

unsafe extern "C" {
    pub fn ioctl(fd: c_int, request: c_int, ...) -> c_int;
}
