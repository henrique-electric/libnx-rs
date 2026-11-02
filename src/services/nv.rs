//! `switch/services/nv.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// NvServiceType, for __nx_nv_service_type.
pub type NvServiceType = i32;
/// This is the default. Automatically select the type using `appletGetAppletType`.
pub const NvServiceType_Auto: NvServiceType = -1;
/// Initializes nvdrv.
pub const NvServiceType_Application: NvServiceType = 0;
/// Initializes nvdrv:a.
pub const NvServiceType_Applet: NvServiceType = 1;
/// Initializes nvdrv:s.
pub const NvServiceType_System: NvServiceType = 2;
/// Initializes nvdrv:t.
pub const NvServiceType_Factory: NvServiceType = 3;

pub type NvEventId = u32;
pub const NvEventId_Gpu_SmException_BptIntReport: NvEventId = 1;
pub const NvEventId_Gpu_SmException_BptPauseReport: NvEventId = 2;
pub const NvEventId_Gpu_ErrorNotifier: NvEventId = 3;
pub const NvEventId_CtrlGpu_ErrorEventHandle: NvEventId = 1;
pub const NvEventId_CtrlGpu_Unknown: NvEventId = 2;

#[inline(always)]
pub const fn NV_EVENT_ID_CTRL__SYNCPT(slot: u32, syncpt: u32) -> u32 {
    (1u32 << 28) | (syncpt << 16) | slot
}

unsafe extern "C" {
    /// Initialize nvdrv*.
    pub fn nvInitialize() -> Result;

    /// Exit nvdrv*.
    pub fn nvExit();

    /// Gets the Service object for the actual nvdrv* service session.
    pub fn nvGetServiceSession() -> *mut Service;

    pub fn nvOpen(fd: *mut u32, devicepath: *const c_char) -> Result;

    pub fn nvIoctl(fd: u32, request: u32, argp: *mut c_void) -> Result;

    /// [3.0.0+]
    pub fn nvIoctl2(fd: u32, request: u32, argp: *mut c_void, inbuf: *const c_void, inbuf_size: usize) -> Result;

    /// [3.0.0+]
    pub fn nvIoctl3(fd: u32, request: u32, argp: *mut c_void, outbuf: *mut c_void, outbuf_size: usize) -> Result;

    pub fn nvClose(fd: u32) -> Result;

    pub fn nvQueryEvent(fd: u32, event_id: u32, event_out: *mut Event) -> Result;

    pub fn nvConvertError(rc: c_int) -> Result;
}
