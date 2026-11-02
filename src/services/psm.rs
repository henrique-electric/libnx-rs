//! `switch/services/psm.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// ChargerType
pub type PsmChargerType = u32;
/// No charger
pub const PsmChargerType_Unconnected: PsmChargerType = 0;
/// Full supported power
pub const PsmChargerType_EnoughPower: PsmChargerType = 1;
/// Lower power supported USB-PD mode
pub const PsmChargerType_LowPower: PsmChargerType = 2;
/// No common supported USB-PD modes
pub const PsmChargerType_NotSupported: PsmChargerType = 3;

/// Vdd50State
pub type PsmVdd50State = u32;
pub const PsmVdd50State_Unknown: PsmVdd50State = 0;
pub const PsmVdd50State_Vdd50AOffVdd50BOff: PsmVdd50State = 1;
pub const PsmVdd50State_Vdd50AOnVdd50BOff: PsmVdd50State = 2;
pub const PsmVdd50State_Vdd50AOffVdd50BOn: PsmVdd50State = 3;

/// BatteryVoltageState
pub type PsmBatteryVoltageState = u32;
/// Power state should transition to shutdown
pub const PsmBatteryVoltageState_NeedsShutdown: PsmBatteryVoltageState = 0;
/// Power state should transition to sleep
pub const PsmBatteryVoltageState_NeedsSleep: PsmBatteryVoltageState = 1;
/// Performance boost modes cannot be entered
pub const PsmBatteryVoltageState_NoPerformanceBoost: PsmBatteryVoltageState = 2;
/// Everything is normal
pub const PsmBatteryVoltageState_Normal: PsmBatteryVoltageState = 3;

/// BatteryChargeInfoFieldsOld [1.0.0-16.1.0]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PsmBatteryChargeInfoFieldsOld {
    pub input_current_limit: u32,
    pub boost_mode_current_limit: u32,
    pub fast_charge_current_limit: u32,
    pub charge_voltage_limit: u32,
    pub charger_type: PsmChargerType,
    pub hi_z_mode: u8,
    pub battery_charging: bool,
    pub pad: [u8; 2],
    pub vdd50_state: PsmVdd50State,
    pub temperature_celcius: u32,
    pub battery_charge_percentage: u32,
    pub battery_charge_milli_voltage: u32,
    pub battery_age_percentage: u32,
    pub usb_power_role: u32,
    pub usb_charger_type: u32,
    pub charger_input_voltage_limit: u32,
    pub charger_input_current_limit: u32,
    pub fast_battery_charging: bool,
    pub controller_power_supply: bool,
    pub otg_request: bool,
    pub reserved: u8,
}

/// BatteryChargeInfoFields [17.0.0+]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PsmBatteryChargeInfoFields {
    /// Input (Sink) current limit in mA
    pub input_current_limit: u32,
    /// Output (Source/VBUS/OTG) current limit in mA
    pub boost_mode_current_limit: u32,
    /// Battery charging current limit in mA
    pub fast_charge_current_limit: u32,
    /// Battery charging voltage limit in mV
    pub charge_voltage_limit: u32,
    pub charger_type: PsmChargerType,
    pub hi_z_mode: u8,
    pub battery_charging: bool,
    pub pad: [u8; 2],
    /// Power Delivery Controller State
    pub vdd50_state: PsmVdd50State,
    /// Battery temperature in milli C
    pub temperature_celcius: u32,
    /// Raw battery charged capacity per cent-mille
    pub battery_charge_percentage: u32,
    /// Voltage average in mV
    pub battery_charge_milli_voltage: u32,
    /// Battery age per cent-mille
    pub battery_age_percentage: u32,
    pub usb_power_role: u32,
    pub usb_charger_type: u32,
    /// Charger and external device voltage limit in mV
    pub charger_input_voltage_limit: u32,
    /// Charger and external device current limit in mA
    pub charger_input_current_limit: u32,
    pub fast_battery_charging: bool,
    pub controller_power_supply: bool,
    pub otg_request: bool,
    pub reserved: u8,
    pub unk_x40: [u8; 20],
}

/// IPsmSession
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PsmSession {
    pub s: Service,
    /// autoclear=false
    pub StateChangeEvent: Event,
}

impl_zeroed_default!(PsmBatteryChargeInfoFieldsOld, PsmBatteryChargeInfoFields, PsmSession);

unsafe extern "C" {
    /// Initialize psm.
    pub fn psmInitialize() -> Result;

    /// Exit psm.
    pub fn psmExit();

    /// Gets the Service object for the actual psm service session.
    pub fn psmGetServiceSession() -> *mut Service;

    /// GetBatteryChargePercentage
    ///
    /// # Arguments
    /// - `out` (out): Battery charge percentage.
    pub fn psmGetBatteryChargePercentage(out: *mut u32) -> Result;

    /// GetChargerType
    ///
    /// # Arguments
    /// - `out` (out): `PsmChargerType`
    pub fn psmGetChargerType(out: *mut PsmChargerType) -> Result;

    /// EnableBatteryCharging
    pub fn psmEnableBatteryCharging() -> Result;

    /// DisableBatteryCharging
    pub fn psmDisableBatteryCharging() -> Result;

    /// IsBatteryChargingEnabled
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    pub fn psmIsBatteryChargingEnabled(out: *mut bool) -> Result;

    /// AcquireControllerPowerSupply
    pub fn psmAcquireControllerPowerSupply() -> Result;

    /// ReleaseControllerPowerSupply
    pub fn psmReleaseControllerPowerSupply() -> Result;

    /// EnableEnoughPowerChargeEmulation
    pub fn psmEnableEnoughPowerChargeEmulation() -> Result;

    /// DisableEnoughPowerChargeEmulation
    pub fn psmDisableEnoughPowerChargeEmulation() -> Result;

    /// EnableFastBatteryCharging
    pub fn psmEnableFastBatteryCharging() -> Result;

    /// DisableFastBatteryCharging
    pub fn psmDisableFastBatteryCharging() -> Result;

    /// GetBatteryVoltageState
    ///
    /// # Arguments
    /// - `out` (out): `PsmBatteryVoltageState`
    pub fn psmGetBatteryVoltageState(out: *mut PsmBatteryVoltageState) -> Result;

    /// GetRawBatteryChargePercentage
    ///
    /// # Arguments
    /// - `out` (out): Raw battery charge percentage.
    pub fn psmGetRawBatteryChargePercentage(out: *mut f64) -> Result;

    /// IsEnoughPowerSupplied
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    pub fn psmIsEnoughPowerSupplied(out: *mut bool) -> Result;

    /// GetBatteryAgePercentage
    ///
    /// # Arguments
    /// - `out` (out): Battery age percentage.
    pub fn psmGetBatteryAgePercentage(out: *mut f64) -> Result;

    /// GetBatteryChargeInfoEvent
    ///
    /// # Arguments
    /// - `out_event` (out): Event object.
    /// - `autoclear` (in): Event autoclear.
    pub fn psmGetBatteryChargeInfoEvent(out_event: *mut Event, autoclear: bool) -> Result;

    /// GetBatteryChargeInfoFields
    ///
    /// # Arguments
    /// - `out_fields` (out): `PsmBatteryChargeInfoFields`
    pub fn psmGetBatteryChargeInfoFields(out_fields: *mut PsmBatteryChargeInfoFields) -> Result;

    /// GetBatteryChargeCalibratedEvent
    ///
    /// # Arguments
    /// - `out_event` (out): Event object.
    /// - `autoclear` (in): Event autoclear.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn psmGetBatteryChargeCalibratedEvent(out_event: *mut Event, autoclear: bool) -> Result;

    /// Wrapper func which opens a PsmSession and handles event setup.
    ///
    /// # Arguments
    /// - `s` (out): PsmSession object.
    /// - `ChargerType` (in): Passed to SetChargerTypeChangeEventEnabled.
    /// - `PowerSupply` (in): Passed to SetPowerSupplyChangeEventEnabled.
    /// - `BatteryVoltage` (in): Passed to SetBatteryVoltageStateChangeEventEnabled.
    ///
    /// **Note:** Uses the actual BindStateChangeEvent cmd internally.
    ///
    /// **Note:** The event is not signalled on BatteryChargePercentage changes.
    pub fn psmBindStateChangeEvent(s: *mut PsmSession, ChargerType: bool, PowerSupply: bool, BatteryVoltage: bool) -> Result;

    /// Wait on the Event setup by `psmBindStateChangeEvent`.
    pub fn psmWaitStateChangeEvent(s: *mut PsmSession, timeout: u64) -> Result;

    /// Cleanup version of `psmBindStateChangeEvent`. Must be called by the user once the PsmSession is done being used.
    pub fn psmUnbindStateChangeEvent(s: *mut PsmSession) -> Result;
}
