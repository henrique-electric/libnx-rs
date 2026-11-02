//! `switch/services/lbl.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub type LblBacklightSwitchStatus = u32;
pub const LblBacklightSwitchStatus_Disabled: LblBacklightSwitchStatus = 0;
pub const LblBacklightSwitchStatus_Enabled: LblBacklightSwitchStatus = 1;
pub const LblBacklightSwitchStatus_Enabling: LblBacklightSwitchStatus = 2;
pub const LblBacklightSwitchStatus_Disabling: LblBacklightSwitchStatus = 3;

unsafe extern "C" {
    /// Initialize lbl.
    pub fn lblInitialize() -> Result;

    /// Exit lbl.
    pub fn lblExit();

    /// Gets the Service object for the actual lbl service session.
    pub fn lblGetServiceSession() -> *mut Service;

    pub fn lblSaveCurrentSetting() -> Result;

    pub fn lblLoadCurrentSetting() -> Result;

    /// **Note:** The brightness goes from 0 to 1.0.
    pub fn lblSetCurrentBrightnessSetting(brightness: f32) -> Result;

    pub fn lblGetCurrentBrightnessSetting(out_value: *mut f32) -> Result;

    pub fn lblApplyCurrentBrightnessSettingToBacklight() -> Result;

    pub fn lblGetBrightnessSettingAppliedToBacklight(out_value: *mut f32) -> Result;

    pub fn lblSwitchBacklightOn(fade_time: u64) -> Result;

    pub fn lblSwitchBacklightOff(fade_time: u64) -> Result;

    pub fn lblGetBacklightSwitchStatus(out_value: *mut LblBacklightSwitchStatus) -> Result;

    pub fn lblEnableDimming() -> Result;

    pub fn lblDisableDimming() -> Result;

    pub fn lblIsDimmingEnabled(out_value: *mut bool) -> Result;

    pub fn lblEnableAutoBrightnessControl() -> Result;

    pub fn lblDisableAutoBrightnessControl() -> Result;

    pub fn lblIsAutoBrightnessControlEnabled(out_value: *mut bool) -> Result;

    pub fn lblSetAmbientLightSensorValue(value: f32) -> Result;

    /// **Note:** Used internally by `appletGetAmbientLightSensorValue` and `appletGetCurrentIlluminanceEx`.
    pub fn lblGetAmbientLightSensorValue(over_limit: *mut bool, lux: *mut f32) -> Result;

    /// **Note:** Only available on [3.0.0+].
    ///
    /// **Note:** Used internally by `appletIsIlluminanceAvailable`.
    pub fn lblIsAmbientLightSensorAvailable(out_value: *mut bool) -> Result;

    /// **Note:** Only available on [3.0.0+].
    pub fn lblSetCurrentBrightnessSettingForVrMode(brightness: f32) -> Result;

    /// **Note:** Only available on [3.0.0+].
    pub fn lblGetCurrentBrightnessSettingForVrMode(out_value: *mut f32) -> Result;

    /// **Note:** Only available on [3.0.0+].
    ///
    /// **Note:** Used internally by `appletSetVrModeEnabled`.
    pub fn lblEnableVrMode() -> Result;

    /// **Note:** Only available on [3.0.0+].
    ///
    /// **Note:** Used internally by `appletSetVrModeEnabled`.
    pub fn lblDisableVrMode() -> Result;

    /// **Note:** Only available on [3.0.0+].
    ///
    /// **Note:** Used internally by `appletIsVrModeEnabled`.
    pub fn lblIsVrModeEnabled(out_value: *mut bool) -> Result;
}
