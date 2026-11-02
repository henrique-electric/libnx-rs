//! `switch/services/capmtp.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

unsafe extern "C" {
    pub fn capmtpInitialize(mem: *mut c_void, size: usize, app_count: u32, max_img: u32, max_vid: u32, other_name: *const c_char) -> Result;

    pub fn capmtpExit();

    pub fn capmtpGetRootServiceSession() -> *mut Service;

    pub fn capmtpGetServiceSession() -> *mut Service;

    pub fn capmtpStartCommandHandler() -> Result;

    pub fn capmtpStopCommandHandler() -> Result;

    pub fn capmtpIsRunning() -> bool;

    pub fn capmtpGetConnectionEvent() -> *mut Event;

    pub fn capmtpIsConnected() -> bool;

    pub fn capmtpGetScanErrorEvent() -> *mut Event;

    pub fn capmtpGetScanError() -> Result;
}
