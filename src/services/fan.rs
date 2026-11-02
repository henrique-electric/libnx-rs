//! `switch/services/fan.h`

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
pub struct FanController {
    pub s: Service,
}

impl_zeroed_default!(FanController);

unsafe extern "C" {
    /// Initialize fan.
    pub fn fanInitialize() -> Result;

    /// Exit fan.
    pub fn fanExit();

    /// Gets the Service object for the actual fan service session.
    pub fn fanGetServiceSession() -> *mut Service;

    /// Opens IController session.
    pub fn fanOpenController(out: *mut FanController, device_code: u32) -> Result;

    /// Close IController session.
    pub fn fanControllerClose(controller: *mut FanController);

    /// **Warning:** Disabling your fan can damage your system.
    pub fn fanControllerSetRotationSpeedLevel(controller: *mut FanController, level: f32) -> Result;

    pub fn fanControllerGetRotationSpeedLevel(controller: *mut FanController, level: *mut f32) -> Result;
}
