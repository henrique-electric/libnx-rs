//! `switch/audio/audio.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// PcmFormat
pub type PcmFormat = u32;
pub const PcmFormat_Invalid: PcmFormat = 0;
pub const PcmFormat_Int8: PcmFormat = 1;
pub const PcmFormat_Int16: PcmFormat = 2;
pub const PcmFormat_Int24: PcmFormat = 3;
pub const PcmFormat_Int32: PcmFormat = 4;
pub const PcmFormat_Float: PcmFormat = 5;
pub const PcmFormat_Adpcm: PcmFormat = 6;

/// AudioDeviceName
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioDeviceName {
    pub name: [c_char; 256],
}

impl_zeroed_default!(AudioDeviceName);
