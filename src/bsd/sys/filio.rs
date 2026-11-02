//! `sys/filio.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const FIOCLEX: u32 = 0x20006601;
pub const FIONCLEX: u32 = 0x20006602;
pub const FIONREAD: u32 = 0x4004667f;
pub const FIONBIO: i32 = -0x7ffb9982;
pub const FIOASYNC: i32 = -0x7ffb9983;
pub const FIOSETOWN: i32 = -0x7ffb9984;
pub const FIOGETOWN: u32 = 0x4004667b;
pub const FIODTYPE: u32 = 0x4004667a;
pub const FIOGETLBA: u32 = 0x40046679;
pub const FIODGNAME: i32 = -0x7fef9988;
pub const FIONWRITE: u32 = 0x40046677;
pub const FIONSPACE: u32 = 0x40046676;
pub const FIOSEEKDATA: i32 = -0x3ff7999f;
pub const FIOSEEKHOLE: i32 = -0x3ff7999e;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct fiodgname_arg {
    pub len: c_int,
    pub buf: *mut c_void,
}

impl_zeroed_default!(fiodgname_arg);
