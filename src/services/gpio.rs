//! `switch/services/gpio.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub type GpioPadName = u32;
pub const GpioPadName_AudioCodec: GpioPadName = 1;
pub const GpioPadName_ButtonVolUp: GpioPadName = 25;
pub const GpioPadName_ButtonVolDown: GpioPadName = 26;
pub const GpioPadName_SdCd: GpioPadName = 56;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GpioPadSession {
    pub s: Service,
}

pub type GpioDirection = u32;
pub const GpioDirection_Input: GpioDirection = 0;
pub const GpioDirection_Output: GpioDirection = 1;

pub type GpioValue = u32;
pub const GpioValue_Low: GpioValue = 0;
pub const GpioValue_High: GpioValue = 1;

pub type GpioInterruptMode = u32;
pub const GpioInterruptMode_LowLevel: GpioInterruptMode = 0;
pub const GpioInterruptMode_HighLevel: GpioInterruptMode = 1;
pub const GpioInterruptMode_RisingEdge: GpioInterruptMode = 2;
pub const GpioInterruptMode_FallingEdge: GpioInterruptMode = 3;
pub const GpioInterruptMode_AnyEdge: GpioInterruptMode = 4;

pub type GpioInterruptStatus = u32;
pub const GpioInterruptStatus_Inactive: GpioInterruptStatus = 0;
pub const GpioInterruptStatus_Active: GpioInterruptStatus = 1;

impl_zeroed_default!(GpioPadSession);

unsafe extern "C" {
    /// Initialize gpio.
    pub fn gpioInitialize() -> Result;

    /// Exit gpio.
    pub fn gpioExit();

    /// Gets the Service object for the actual gpio service session.
    pub fn gpioGetServiceSession() -> *mut Service;

    pub fn gpioOpenSession(out: *mut GpioPadSession, name: GpioPadName) -> Result;

    pub fn gpioOpenSession2(out: *mut GpioPadSession, device_code: u32, access_mode: u32) -> Result;

    pub fn gpioIsWakeEventActive(out: *mut bool, name: GpioPadName) -> Result;

    pub fn gpioIsWakeEventActive2(out: *mut bool, device_code: u32) -> Result;

    pub fn gpioPadSetDirection(p: *mut GpioPadSession, dir: GpioDirection) -> Result;

    pub fn gpioPadGetDirection(p: *mut GpioPadSession, out: *mut GpioDirection) -> Result;

    pub fn gpioPadSetInterruptMode(p: *mut GpioPadSession, mode: GpioInterruptMode) -> Result;

    pub fn gpioPadGetInterruptMode(p: *mut GpioPadSession, out: *mut GpioInterruptMode) -> Result;

    pub fn gpioPadSetInterruptEnable(p: *mut GpioPadSession, en: bool) -> Result;

    pub fn gpioPadGetInterruptEnable(p: *mut GpioPadSession, out: *mut bool) -> Result;

    /// [1.0.0-16.1.0]
    pub fn gpioPadGetInterruptStatus(p: *mut GpioPadSession, out: *mut GpioInterruptStatus) -> Result;

    /// [1.0.0-16.1.0]
    pub fn gpioPadClearInterruptStatus(p: *mut GpioPadSession) -> Result;

    pub fn gpioPadSetValue(p: *mut GpioPadSession, val: GpioValue) -> Result;

    pub fn gpioPadGetValue(p: *mut GpioPadSession, out: *mut GpioValue) -> Result;

    pub fn gpioPadBindInterrupt(p: *mut GpioPadSession, out: *mut Event) -> Result;

    pub fn gpioPadUnbindInterrupt(p: *mut GpioPadSession) -> Result;

    pub fn gpioPadSetDebounceEnabled(p: *mut GpioPadSession, en: bool) -> Result;

    pub fn gpioPadGetDebounceEnabled(p: *mut GpioPadSession, out: *mut bool) -> Result;

    pub fn gpioPadSetDebounceTime(p: *mut GpioPadSession, ms: s32) -> Result;

    pub fn gpioPadGetDebounceTime(p: *mut GpioPadSession, out: *mut s32) -> Result;

    pub fn gpioPadClose(p: *mut GpioPadSession);
}
