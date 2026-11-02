//! `switch/services/psc.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub type PscPmState = u32;
/// Everything is awake.
pub const PscPmState_Awake: PscPmState = 0;
/// Preparing to transition to awake.
pub const PscPmState_ReadyAwaken: PscPmState = 1;
/// Preparing to transition to sleep.
pub const PscPmState_ReadySleep: PscPmState = 2;
/// Critical services are ready to sleep.
pub const PscPmState_ReadySleepCritical: PscPmState = 3;
/// Critical services are ready to wake up.
pub const PscPmState_ReadyAwakenCritical: PscPmState = 4;
/// Preparing to transition to shutdown.
pub const PscPmState_ReadyShutdown: PscPmState = 5;

pub type PscPmModuleId = u32;
pub const PscPmModuleId_Usb: PscPmModuleId = 4;
pub const PscPmModuleId_Ethernet: PscPmModuleId = 5;
pub const PscPmModuleId_Fgm: PscPmModuleId = 6;
pub const PscPmModuleId_PcvClock: PscPmModuleId = 7;
pub const PscPmModuleId_PcvVoltage: PscPmModuleId = 8;
pub const PscPmModuleId_Gpio: PscPmModuleId = 9;
pub const PscPmModuleId_Pinmux: PscPmModuleId = 10;
pub const PscPmModuleId_Uart: PscPmModuleId = 11;
pub const PscPmModuleId_I2c: PscPmModuleId = 12;
pub const PscPmModuleId_I2cPcv: PscPmModuleId = 13;
pub const PscPmModuleId_Spi: PscPmModuleId = 14;
pub const PscPmModuleId_Pwm: PscPmModuleId = 15;
pub const PscPmModuleId_Psm: PscPmModuleId = 16;
pub const PscPmModuleId_Tc: PscPmModuleId = 17;
pub const PscPmModuleId_Omm: PscPmModuleId = 18;
pub const PscPmModuleId_Pcie: PscPmModuleId = 19;
pub const PscPmModuleId_Lbl: PscPmModuleId = 20;
pub const PscPmModuleId_Display: PscPmModuleId = 21;
pub const PscPmModuleId_Hid: PscPmModuleId = 24;
pub const PscPmModuleId_WlanSockets: PscPmModuleId = 25;
pub const PscPmModuleId_Fs: PscPmModuleId = 27;
pub const PscPmModuleId_Audio: PscPmModuleId = 28;
pub const PscPmModuleId_TmaHostIo: PscPmModuleId = 30;
pub const PscPmModuleId_Bluetooth: PscPmModuleId = 31;
pub const PscPmModuleId_Bpc: PscPmModuleId = 32;
pub const PscPmModuleId_Fan: PscPmModuleId = 33;
pub const PscPmModuleId_Pcm: PscPmModuleId = 34;
pub const PscPmModuleId_Nfc: PscPmModuleId = 35;
pub const PscPmModuleId_Apm: PscPmModuleId = 36;
pub const PscPmModuleId_Btm: PscPmModuleId = 37;
pub const PscPmModuleId_Nifm: PscPmModuleId = 38;
pub const PscPmModuleId_GpioLow: PscPmModuleId = 39;
pub const PscPmModuleId_Npns: PscPmModuleId = 40;
pub const PscPmModuleId_Lm: PscPmModuleId = 41;
pub const PscPmModuleId_Bcat: PscPmModuleId = 42;
pub const PscPmModuleId_Time: PscPmModuleId = 43;
pub const PscPmModuleId_Pctl: PscPmModuleId = 44;
pub const PscPmModuleId_Erpt: PscPmModuleId = 45;
pub const PscPmModuleId_Eupld: PscPmModuleId = 46;
pub const PscPmModuleId_Friends: PscPmModuleId = 47;
pub const PscPmModuleId_Bgtc: PscPmModuleId = 48;
pub const PscPmModuleId_Account: PscPmModuleId = 49;
pub const PscPmModuleId_Sasbus: PscPmModuleId = 50;
pub const PscPmModuleId_Ntc: PscPmModuleId = 51;
pub const PscPmModuleId_Idle: PscPmModuleId = 52;
pub const PscPmModuleId_Tcap: PscPmModuleId = 53;
pub const PscPmModuleId_PsmLow: PscPmModuleId = 54;
pub const PscPmModuleId_Ndd: PscPmModuleId = 55;
pub const PscPmModuleId_Olsc: PscPmModuleId = 56;
pub const PscPmModuleId_Ns: PscPmModuleId = 61;
pub const PscPmModuleId_Nvservices: PscPmModuleId = 101;
pub const PscPmModuleId_Spsm: PscPmModuleId = 127;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PscPmModule {
    pub event: Event,
    pub srv: Service,
    pub module_id: PscPmModuleId,
}

impl_zeroed_default!(PscPmModule);

unsafe extern "C" {
    /// Initialize psc:m.
    pub fn pscmInitialize() -> Result;

    /// Exit psc:m.
    pub fn pscmExit();

    /// Gets the Service object for the actual psc:m service session.
    pub fn pscmGetServiceSession() -> *mut Service;

    pub fn pscmGetPmModule(out: *mut PscPmModule, module_id: PscPmModuleId, dependencies: *const u32, dependency_count: usize, autoclear: bool) -> Result;

    pub fn pscPmModuleGetRequest(module: *mut PscPmModule, out_state: *mut PscPmState, out_flags: *mut u32) -> Result;

    pub fn pscPmModuleAcknowledge(module: *mut PscPmModule, state: PscPmState) -> Result;

    pub fn pscPmModuleFinalize(module: *mut PscPmModule) -> Result;

    pub fn pscPmModuleClose(module: *mut PscPmModule);
}
