//! `switch/nvidia/channel.h`

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
pub struct NvChannel {
    pub fd: u32,
    pub has_init: bool,
}

impl_zeroed_default!(NvChannel);

unsafe extern "C" {
    pub fn nvChannelCreate(c: *mut NvChannel, dev: *const c_char) -> Result;

    pub fn nvChannelClose(c: *mut NvChannel);

    pub fn nvChannelSetPriority(c: *mut NvChannel, prio: NvChannelPriority) -> Result;

    pub fn nvChannelSetTimeout(c: *mut NvChannel, timeout: u32) -> Result;
}
