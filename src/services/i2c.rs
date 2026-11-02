//! `switch/services/i2c.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub type I2cDevice = u32;
pub const I2cDevice_DebugPad: I2cDevice = 0;
pub const I2cDevice_TouchPanel: I2cDevice = 1;
pub const I2cDevice_Tmp451: I2cDevice = 2;
pub const I2cDevice_Nct72: I2cDevice = 3;
pub const I2cDevice_Alc5639: I2cDevice = 4;
pub const I2cDevice_Max77620Rtc: I2cDevice = 5;
pub const I2cDevice_Max77620Pmic: I2cDevice = 6;
pub const I2cDevice_Max77621Cpu: I2cDevice = 7;
pub const I2cDevice_Max77621Gpu: I2cDevice = 8;
pub const I2cDevice_Bq24193: I2cDevice = 9;
pub const I2cDevice_Max17050: I2cDevice = 10;
pub const I2cDevice_Bm92t30mwv: I2cDevice = 11;
pub const I2cDevice_Ina226Vdd15v0Hb: I2cDevice = 12;
pub const I2cDevice_Ina226VsysCpuDs: I2cDevice = 13;
pub const I2cDevice_Ina226VsysGpuDs: I2cDevice = 14;
pub const I2cDevice_Ina226VsysDdrDs: I2cDevice = 15;
pub const I2cDevice_Ina226VsysAp: I2cDevice = 16;
pub const I2cDevice_Ina226VsysBlDs: I2cDevice = 17;
pub const I2cDevice_Bh1730: I2cDevice = 18;
pub const I2cDevice_Ina226VsysCore: I2cDevice = 19;
pub const I2cDevice_Ina226Soc1V8: I2cDevice = 20;
pub const I2cDevice_Ina226Lpddr1V8: I2cDevice = 21;
pub const I2cDevice_Ina226Reg1V32: I2cDevice = 22;
pub const I2cDevice_Ina226Vdd3V3Sys: I2cDevice = 23;
pub const I2cDevice_HdmiDdc: I2cDevice = 24;
pub const I2cDevice_HdmiScdc: I2cDevice = 25;
pub const I2cDevice_HdmiHdcp: I2cDevice = 26;
pub const I2cDevice_Fan53528: I2cDevice = 27;
pub const I2cDevice_Max77812_3: I2cDevice = 28;
pub const I2cDevice_Max77812_2: I2cDevice = 29;
pub const I2cDevice_Ina226VddDdr0V6: I2cDevice = 30;
pub const I2cDevice_Count: I2cDevice = 31;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct I2cSession {
    pub s: Service,
}

pub type I2cTransactionOption = u32;
pub const I2cTransactionOption_Start: I2cTransactionOption = (1 << 0);
pub const I2cTransactionOption_Stop: I2cTransactionOption = (1 << 1);
pub const I2cTransactionOption_All: I2cTransactionOption = I2cTransactionOption_Start | I2cTransactionOption_Stop;

impl_zeroed_default!(I2cSession);

unsafe extern "C" {
    /// Initialize i2c.
    pub fn i2cInitialize() -> Result;

    /// Exit i2c.
    pub fn i2cExit();

    /// Gets the Service object for the actual i2c service session.
    pub fn i2cGetServiceSession() -> *mut Service;

    pub fn i2cOpenSession(out: *mut I2cSession, dev: I2cDevice) -> Result;

    pub fn i2csessionSendAuto(s: *mut I2cSession, buf: *const c_void, size: usize, option: I2cTransactionOption) -> Result;

    pub fn i2csessionReceiveAuto(s: *mut I2cSession, buf: *mut c_void, size: usize, option: I2cTransactionOption) -> Result;

    pub fn i2csessionExecuteCommandList(s: *mut I2cSession, dst: *mut c_void, dst_size: usize, cmd_list: *const c_void, cmd_list_size: usize) -> Result;

    pub fn i2csessionClose(s: *mut I2cSession);
}
