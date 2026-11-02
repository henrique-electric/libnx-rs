//! `switch/services/hidsys.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Selects what button to map to.
pub type HidcfgDigitalButtonAssignment = u32;
/// A
pub const HidcfgDigitalButtonAssignment_A: HidcfgDigitalButtonAssignment = 0;
/// B
pub const HidcfgDigitalButtonAssignment_B: HidcfgDigitalButtonAssignment = 1;
/// X
pub const HidcfgDigitalButtonAssignment_X: HidcfgDigitalButtonAssignment = 2;
/// Y
pub const HidcfgDigitalButtonAssignment_Y: HidcfgDigitalButtonAssignment = 3;
/// Left Stick Button
pub const HidcfgDigitalButtonAssignment_StickL: HidcfgDigitalButtonAssignment = 4;
/// Right Stick Button
pub const HidcfgDigitalButtonAssignment_StickR: HidcfgDigitalButtonAssignment = 5;
/// L
pub const HidcfgDigitalButtonAssignment_L: HidcfgDigitalButtonAssignment = 6;
/// R
pub const HidcfgDigitalButtonAssignment_R: HidcfgDigitalButtonAssignment = 7;
/// ZL
pub const HidcfgDigitalButtonAssignment_ZL: HidcfgDigitalButtonAssignment = 8;
/// ZR
pub const HidcfgDigitalButtonAssignment_ZR: HidcfgDigitalButtonAssignment = 9;
/// Select / Minus
pub const HidcfgDigitalButtonAssignment_Select: HidcfgDigitalButtonAssignment = 10;
/// Start / Plus
pub const HidcfgDigitalButtonAssignment_Start: HidcfgDigitalButtonAssignment = 11;
/// Left
pub const HidcfgDigitalButtonAssignment_Left: HidcfgDigitalButtonAssignment = 12;
/// Up
pub const HidcfgDigitalButtonAssignment_Up: HidcfgDigitalButtonAssignment = 13;
/// Right
pub const HidcfgDigitalButtonAssignment_Right: HidcfgDigitalButtonAssignment = 14;
/// Down
pub const HidcfgDigitalButtonAssignment_Down: HidcfgDigitalButtonAssignment = 15;
/// SL on Left controller.
pub const HidcfgDigitalButtonAssignment_LeftSL: HidcfgDigitalButtonAssignment = 16;
/// SR on Left controller.
pub const HidcfgDigitalButtonAssignment_LeftSR: HidcfgDigitalButtonAssignment = 17;
/// SL on Right controller.
pub const HidcfgDigitalButtonAssignment_RightSL: HidcfgDigitalButtonAssignment = 18;
/// SR on Right controller.
pub const HidcfgDigitalButtonAssignment_RightSR: HidcfgDigitalButtonAssignment = 19;
/// HomeButton
pub const HidcfgDigitalButtonAssignment_HomeButton: HidcfgDigitalButtonAssignment = 20;
/// CaptureButton
pub const HidcfgDigitalButtonAssignment_CaptureButton: HidcfgDigitalButtonAssignment = 21;
/// Invalid / Disabled
pub const HidcfgDigitalButtonAssignment_Invalid: HidcfgDigitalButtonAssignment = 22;

/// AnalogStickRotation
pub type HidcfgAnalogStickRotation = u32;
/// None
pub const HidcfgAnalogStickRotation_None: HidcfgAnalogStickRotation = 0;
/// Clockwise90
pub const HidcfgAnalogStickRotation_Clockwise90: HidcfgAnalogStickRotation = 1;
/// Anticlockwise90
pub const HidcfgAnalogStickRotation_Anticlockwise90: HidcfgAnalogStickRotation = 2;

/// UniquePadType
pub type HidsysUniquePadType = u32;
/// Embedded
pub const HidsysUniquePadType_Embedded: HidsysUniquePadType = 0;
/// FullKeyController
pub const HidsysUniquePadType_FullKeyController: HidsysUniquePadType = 1;
/// RightController
pub const HidsysUniquePadType_RightController: HidsysUniquePadType = 2;
/// LeftController
pub const HidsysUniquePadType_LeftController: HidsysUniquePadType = 3;
/// DebugPadController
pub const HidsysUniquePadType_DebugPadController: HidsysUniquePadType = 4;

/// UniquePadId for a controller.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidsysUniquePadId {
    /// UniquePadId
    pub id: u64,
}

/// UniquePadSerialNumber
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidsysUniquePadSerialNumber {
    /// SerialNumber
    pub serial_number: [c_char; 16],
}

/// Mini Cycle struct for `HidsysNotificationLedPattern`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidsysNotificationLedPatternCycle {
    /// Mini Cycle X LED Intensity.
    pub ledIntensity: u8,
    /// Fading Transition Steps to Mini Cycle X (Uses PWM). Value 0x0: Instant. Each step duration is based on HidsysNotificationLedPattern::baseMiniCycleDuration.
    pub transitionSteps: u8,
    /// Final Step Duration Multiplier of Mini Cycle X. Value 0x0: 12.5ms, 0x1 - xF: 1x - 15x. Value is a Multiplier of HidsysNotificationLedPattern::baseMiniCycleDuration.
    pub finalStepDuration: u8,
    pub pad: u8,
}

/// Structure for `hidsysSetNotificationLedPattern`.
/// See also: <https://switchbrew.org/wiki/HID_services#NotificationLedPattern>
/// Only the low 4bits of each used byte in this struct is used.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidsysNotificationLedPattern {
    /// Mini Cycle Base Duration. Value 0x1-0xF: 12.5ms - 187.5ms. Value 0x0 = 0ms/OFF.
    pub baseMiniCycleDuration: u8,
    /// Number of Mini Cycles + 1. Value 0x0-0xF: 1 - 16 mini cycles.
    pub totalMiniCycles: u8,
    /// Number of Full Cycles. Value 0x1-0xF: 1 - 15 full cycles. Value 0x0 is repeat forever, but if baseMiniCycleDuration is set to 0x0, it does the 1st Mini Cycle with a 12.5ms step duration and then the LED stays on with startIntensity.
    pub totalFullCycles: u8,
    /// LED Start Intensity. Value 0x0=0% - 0xF=100%.
    pub startIntensity: u8,
    /// Mini Cycles
    pub miniCycles: [HidsysNotificationLedPatternCycle; 16],
    /// Unknown
    pub unk_x44: [u8; 2],
    /// Padding
    pub pad_x46: [u8; 2],
}

/// ButtonConfigEmbedded
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidsysButtonConfigEmbedded {
    pub unk_x0: [u8; 712],
}

/// ButtonConfigFull
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidsysButtonConfigFull {
    pub unk_x0: [u8; 712],
}

/// ButtonConfigLeft
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidsysButtonConfigLeft {
    pub unk_x0: [u8; 456],
}

/// ButtonConfigRight
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidsysButtonConfigRight {
    pub unk_x0: [u8; 416],
}

/// AnalogStickAssignment
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidcfgAnalogStickAssignment {
    /// **Ref:** HidcfgAnalogStickRotation
    pub rotation: u32,
    /// IsPairedStickAssigned
    pub is_paired_stick_assigned: u8,
    /// Reserved
    pub reserved: [u8; 3],
}

/// ButtonConfigEmbedded
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidcfgButtonConfigEmbedded {
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonLeft
    pub hardware_button_left: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonUp
    pub hardware_button_up: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonRight
    pub hardware_button_right: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonDown
    pub hardware_button_down: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonA
    pub hardware_button_a: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonB
    pub hardware_button_b: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonX
    pub hardware_button_x: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonY
    pub hardware_button_y: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonStickL
    pub hardware_button_stick_l: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonStickR
    pub hardware_button_stick_r: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonL
    pub hardware_button_l: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonR
    pub hardware_button_r: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonZL
    pub hardware_button_zl: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonZR
    pub hardware_button_zr: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonSelect
    pub hardware_button_select: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonStart
    pub hardware_button_start: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonCapture
    pub hardware_button_capture: u32,
    /// HardwareStickL
    pub hardware_stick_l: HidcfgAnalogStickAssignment,
    /// HardwareStickR
    pub hardware_stick_r: HidcfgAnalogStickAssignment,
}

/// ButtonConfigFull
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidcfgButtonConfigFull {
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonLeft
    pub hardware_button_left: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonUp
    pub hardware_button_up: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonRight
    pub hardware_button_right: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonDown
    pub hardware_button_down: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonA
    pub hardware_button_a: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonB
    pub hardware_button_b: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonX
    pub hardware_button_x: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonY
    pub hardware_button_y: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonStickL
    pub hardware_button_stick_l: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonStickR
    pub hardware_button_stick_r: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonL
    pub hardware_button_l: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonR
    pub hardware_button_r: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonZL
    pub hardware_button_zl: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonZR
    pub hardware_button_zr: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonSelect
    pub hardware_button_select: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonStart
    pub hardware_button_start: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonCapture
    pub hardware_button_capture: u32,
    /// HardwareStickL
    pub hardware_stick_l: HidcfgAnalogStickAssignment,
    /// HardwareStickR
    pub hardware_stick_r: HidcfgAnalogStickAssignment,
}

/// ButtonConfigLeft
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidcfgButtonConfigLeft {
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonLeft
    pub hardware_button_left: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonUp
    pub hardware_button_up: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonRight
    pub hardware_button_right: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonDown
    pub hardware_button_down: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonStickL
    pub hardware_button_stick_l: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonL
    pub hardware_button_l: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonZL
    pub hardware_button_zl: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonSelect
    pub hardware_button_select: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonLeftSL
    pub hardware_button_left_sl: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonLeftSR
    pub hardware_button_left_sr: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonCapture
    pub hardware_button_capture: u32,
    /// HardwareStickL
    pub hardware_stick_l: HidcfgAnalogStickAssignment,
}

/// ButtonConfigRight
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidcfgButtonConfigRight {
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonA
    pub hardware_button_a: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonB
    pub hardware_button_b: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonX
    pub hardware_button_x: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonY
    pub hardware_button_y: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonStickR
    pub hardware_button_stick_r: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonR
    pub hardware_button_r: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonZR
    pub hardware_button_zr: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonStart
    pub hardware_button_start: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonRightSL
    pub hardware_button_right_sl: u32,
    /// **Ref:** HidcfgDigitalButtonAssignment HardwareButtonRightSR
    pub hardware_button_right_sr: u32,
    /// HardwareStickR
    pub hardware_stick_r: HidcfgAnalogStickAssignment,
}

/// StorageName
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidcfgStorageName {
    /// UTF-8 NUL-terminated name string.
    pub name: [u8; 129],
}

impl_zeroed_default!(HidsysUniquePadId, HidsysUniquePadSerialNumber, HidsysNotificationLedPatternCycle, HidsysNotificationLedPattern, HidsysButtonConfigEmbedded, HidsysButtonConfigFull, HidsysButtonConfigLeft, HidsysButtonConfigRight, HidcfgAnalogStickAssignment, HidcfgButtonConfigEmbedded, HidcfgButtonConfigFull, HidcfgButtonConfigLeft, HidcfgButtonConfigRight, HidcfgStorageName);

unsafe extern "C" {
    /// Initialize hidsys.
    pub fn hidsysInitialize() -> Result;

    /// Exit hidsys.
    pub fn hidsysExit();

    /// Gets the Service object for the actual hidsys service session.
    pub fn hidsysGetServiceSession() -> *mut Service;

    /// SendKeyboardLockKeyEvent
    ///
    /// # Arguments
    /// - `events` (in): Bitfield of `HidKeyboardLockKeyEvent`.
    pub fn hidsysSendKeyboardLockKeyEvent(events: u32) -> Result;

    /// Gets an Event which is signaled when HidHomeButtonState is updated.
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event.
    /// - `Event` (in): autoclear.
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    ///
    /// **Note:** This generally shouldn't be used, since AM-sysmodule uses it internally.
    pub fn hidsysAcquireHomeButtonEventHandle(out_event: *mut Event, autoclear: bool) -> Result;

    /// Activates the HomeButton sharedmem.
    ///
    /// **Note:** This generally shouldn't be used, since AM-sysmodule uses it internally.
    pub fn hidsysActivateHomeButton() -> Result;

    /// Gets an Event which is signaled when HidSleepButtonState is updated.
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event.
    /// - `Event` (in): autoclear.
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    ///
    /// **Note:** This generally shouldn't be used, since AM-sysmodule uses it internally.
    pub fn hidsysAcquireSleepButtonEventHandle(out_event: *mut Event, autoclear: bool) -> Result;

    /// Activates the SleepButton sharedmem.
    ///
    /// **Note:** This generally shouldn't be used, since AM-sysmodule uses it internally.
    pub fn hidsysActivateSleepButton() -> Result;

    /// Gets an Event which is signaled when HidCaptureButtonState is updated.
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event.
    /// - `Event` (in): autoclear.
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    ///
    /// **Note:** This generally shouldn't be used, since AM-sysmodule uses it internally.
    pub fn hidsysAcquireCaptureButtonEventHandle(out_event: *mut Event, autoclear: bool) -> Result;

    /// Activates the CaptureButton sharedmem.
    ///
    /// **Note:** This generally shouldn't be used, since AM-sysmodule uses it internally.
    pub fn hidsysActivateCaptureButton() -> Result;

    /// Applies npad system common policy.
    pub fn hidsysApplyNpadSystemCommonPolicy() -> Result;

    /// Gets the npad id type of the last controller that was active.
    ///
    /// # Arguments
    /// - `out` (out): of `HidNpadIdType`.
    pub fn hidsysGetLastActiveNpad(out: *mut u32) -> Result;

    /// Gets the SupportedNpadStyleSet for the CallerApplet. applet must be initialized in order to use this (uses `appletGetAppletResourceUserIdOfCallerApplet`).
    ///
    /// # Arguments
    /// - `out` (out): Bitmask of `HidNpadStyleTag`.
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn hidsysGetSupportedNpadStyleSetOfCallerApplet(out: *mut u32) -> Result;

    /// Gets the `HidNpadInterfaceType` for the specified controller.
    ///
    /// # Arguments
    /// - `id` (in): `HidNpadIdType`
    /// - `out` (out): `HidNpadInterfaceType`
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn hidsysGetNpadInterfaceType(id: HidNpadIdType, out: *mut u8) -> Result;

    /// GetNpadLeftRightInterfaceType
    ///
    /// # Arguments
    /// - `id` (in): `HidNpadIdType`
    /// - `out0` (out): `HidNpadInterfaceType`
    /// - `out1` (out): `HidNpadInterfaceType`
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn hidsysGetNpadLeftRightInterfaceType(id: HidNpadIdType, out0: *mut u8, out1: *mut u8) -> Result;

    /// HasBattery
    ///
    /// # Arguments
    /// - `id` (in): `HidNpadIdType`
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn hidsysHasBattery(id: HidNpadIdType, out: *mut bool) -> Result;

    /// HasLeftRightBattery
    ///
    /// # Arguments
    /// - `id` (in): `HidNpadIdType`
    /// - `out0` (out): Output flag.
    /// - `out1` (out): Output flag.
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn hidsysHasLeftRightBattery(id: HidNpadIdType, out0: *mut bool, out1: *mut bool) -> Result;

    /// Gets the UniquePadIds for the specified controller.
    ///
    /// # Arguments
    /// - `id` (in): `HidNpadIdType`
    /// - `unique_pad_ids` (out): Output array of `HidsysUniquePadId`.
    /// - `count` (in): Max number of entries for the unique_pad_ids array.
    /// - `total_out` (out): Total output array entries. Optional, can be NULL.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn hidsysGetUniquePadsFromNpad(id: HidNpadIdType, unique_pad_ids: *mut HidsysUniquePadId, count: s32, total_out: *mut s32) -> Result;

    /// EnableAppletToGetInput
    ///
    /// # Arguments
    /// - `enable` (in): Input flag.
    pub fn hidsysEnableAppletToGetInput(enable: bool) -> Result;

    /// EnableHandheldHids
    pub fn hidsysEnableHandheldHids() -> Result;

    /// DisableHandheldHids
    pub fn hidsysDisableHandheldHids() -> Result;

    /// SetJoyConRailEnabled
    ///
    /// # Arguments
    /// - `enable` (in): Input flag.
    ///
    /// **Note:** Only available on [9.0.0+].
    pub fn hidsysSetJoyConRailEnabled(enable: bool) -> Result;

    /// IsJoyConRailEnabled
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [9.0.0+].
    pub fn hidsysIsJoyConRailEnabled(out: *mut bool) -> Result;

    /// IsHandheldHidsEnabled
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn hidsysIsHandheldHidsEnabled(out: *mut bool) -> Result;

    /// IsJoyConAttachedOnAllRail
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [11.0.0+].
    pub fn hidsysIsJoyConAttachedOnAllRail(out: *mut bool) -> Result;

    /// IsInvertedControllerConnectedOnRail
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [19.0.0+].
    pub fn hidsysIsInvertedControllerConnectedOnRail(out: *mut bool) -> Result;

    /// AcquireUniquePadConnectionEventHandle
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event.
    pub fn hidsysAcquireUniquePadConnectionEventHandle(out_event: *mut Event) -> Result;

    /// Gets a list of all UniquePadIds.
    ///
    /// # Arguments
    /// - `unique_pad_ids` (out): Output array of `HidsysUniquePadId`.
    /// - `count` (in): Max number of entries for the unique_pad_ids array.
    /// - `total_out` (out): Total output array entries. Optional, can be NULL.
    pub fn hidsysGetUniquePadIds(unique_pad_ids: *mut HidsysUniquePadId, count: s32, total_out: *mut s32) -> Result;

    /// AcquireJoyDetachOnBluetoothOffEventHandle
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event.
    /// - `Event` (in): autoclear.
    pub fn hidsysAcquireJoyDetachOnBluetoothOffEventHandle(out_event: *mut Event, autoclear: bool) -> Result;

    /// GetUniquePadBluetoothAddress
    ///
    /// # Arguments
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    /// - `address` (out): `BtdrvAddress`
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn hidsysGetUniquePadBluetoothAddress(unique_pad_id: HidsysUniquePadId, address: *mut BtdrvAddress) -> Result;

    /// DisconnectUniquePad
    ///
    /// # Arguments
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn hidsysDisconnectUniquePad(unique_pad_id: HidsysUniquePadId) -> Result;

    /// GetUniquePadType
    ///
    /// # Arguments
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    /// - `pad_type` (out): `HidsysUniquePadType`
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn hidsysGetUniquePadType(unique_pad_id: HidsysUniquePadId, pad_type: *mut HidsysUniquePadType) -> Result;

    /// GetUniquePadInterface
    ///
    /// # Arguments
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    /// - `interface` (out): `HidNpadInterfaceType`
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn hidsysGetUniquePadInterface(unique_pad_id: HidsysUniquePadId, interface: *mut HidNpadInterfaceType) -> Result;

    /// Gets the `HidsysUniquePadSerialNumber`.
    ///
    /// # Arguments
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    /// - `serial` (out): `HidsysUniquePadSerialNumber`
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn hidsysGetUniquePadSerialNumber(unique_pad_id: HidsysUniquePadId, serial: *mut HidsysUniquePadSerialNumber) -> Result;

    /// GetUniquePadControllerNumber
    ///
    /// # Arguments
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    /// - `number` (out): Controller number.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn hidsysGetUniquePadControllerNumber(unique_pad_id: HidsysUniquePadId, number: *mut u64) -> Result;

    /// Sets the HOME-button notification LED pattern, for the specified controller.
    ///
    /// # Arguments
    /// - `pattern` (in): `HidsysNotificationLedPattern`
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    ///
    /// **Note:** Generally this should only be used if `hidsysSetNotificationLedPatternWithTimeout` is not usable.
    ///
    /// **Note:** Only available on [7.0.0+].
    pub fn hidsysSetNotificationLedPattern(pattern: *const HidsysNotificationLedPattern, unique_pad_id: HidsysUniquePadId) -> Result;

    /// Sets the HOME-button notification LED pattern, for the specified controller. The LED will automatically be disabled once the specified timeout occurs.
    ///
    /// # Arguments
    /// - `pattern` (in): `HidsysNotificationLedPattern`
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    /// - `timeout` (in): Timeout in nanoseconds.
    ///
    /// **Note:** Only available on [9.0.0+], and with controllers which have the [9.0.0+] firmware installed.
    pub fn hidsysSetNotificationLedPatternWithTimeout(pattern: *const HidsysNotificationLedPattern, unique_pad_id: HidsysUniquePadId, timeout: u64) -> Result;

    /// IsUsbFullKeyControllerEnabled
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn hidsysIsUsbFullKeyControllerEnabled(out: *mut bool) -> Result;

    /// EnableUsbFullKeyController
    ///
    /// # Arguments
    /// - `flag` (in): Flag
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn hidsysEnableUsbFullKeyController(flag: bool) -> Result;

    /// IsUsbConnected
    ///
    /// # Arguments
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn hidsysIsUsbConnected(unique_pad_id: HidsysUniquePadId, out: *mut bool) -> Result;

    /// GetTouchScreenDefaultConfiguration
    ///
    /// # Arguments
    /// - `touch_screen_configuration` (out): `HidTouchScreenConfigurationForNx`.
    ///
    /// **Note:** Only available on [9.0.0+].
    pub fn hidsysGetTouchScreenDefaultConfiguration(touch_screen_configuration: *mut HidTouchScreenConfigurationForNx) -> Result;

    /// IsFirmwareUpdateNeededForNotification
    ///
    /// # Arguments
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [9.0.0+].
    pub fn hidsysIsFirmwareUpdateNeededForNotification(unique_pad_id: HidsysUniquePadId, out: *mut bool) -> Result;

    /// Legacy IsButtonConfigSupported.
    ///
    /// # Arguments
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    /// - `out` (out): Output bool flag.
    ///
    /// **Note:** Only available on [10.0.0-10.2.0]. On [11.0.0-17.0.1], use `hidsysIsButtonConfigSupported` instead.
    pub fn hidsysLegacyIsButtonConfigSupported(unique_pad_id: HidsysUniquePadId, out: *mut bool) -> Result;

    /// IsButtonConfigSupported
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    /// - `out` (out): Output bool flag.
    ///
    /// **Note:** Only available on [11.0.0-17.0.1]. On [10.0.0-10.2.0], use `hidsysLegacyIsButtonConfigSupported` instead.
    pub fn hidsysIsButtonConfigSupported(addr: BtdrvAddress, out: *mut bool) -> Result;

    /// IsButtonConfigEmbeddedSupported
    ///
    /// # Arguments
    /// - `out` (out): Output bool flag.
    ///
    /// **Note:** Only available on [11.0.0-17.0.1].
    pub fn hidsysIsButtonConfigEmbeddedSupported(out: *mut bool) -> Result;

    /// Legacy DeleteButtonConfig.
    ///
    /// # Arguments
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    ///
    /// **Note:** Only available on [10.0.0-10.2.0]. On [11.0.0-17.0.1], use `hidsysDeleteButtonConfig` instead.
    pub fn hidsysLegacyDeleteButtonConfig(unique_pad_id: HidsysUniquePadId) -> Result;

    /// DeleteButtonConfig
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    ///
    /// **Note:** Only available on [11.0.0-17.0.1]. On [10.0.0-10.2.0], use `hidsysLegacyDeleteButtonConfig` instead.
    pub fn hidsysDeleteButtonConfig(addr: BtdrvAddress) -> Result;

    /// DeleteButtonConfigEmbedded
    ///
    /// **Note:** Only available on [11.0.0-17.0.1].
    pub fn hidsysDeleteButtonConfigEmbedded() -> Result;

    /// Legacy SetButtonConfigEnabled.
    ///
    /// # Arguments
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    /// - `flag` (in): Input flag.
    ///
    /// **Note:** Only available on [10.0.0-10.2.0]. On [11.0.0-17.0.1], use `hidsysSetButtonConfigEnabled` instead.
    pub fn hidsysLegacySetButtonConfigEnabled(unique_pad_id: HidsysUniquePadId, flag: bool) -> Result;

    /// SetButtonConfigEnabled
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    /// - `flag` (in): Input flag.
    ///
    /// **Note:** Only available on [11.0.0-17.0.1]. On [10.0.0-10.2.0], use `hidsysLegacySetButtonConfigEnabled` instead.
    pub fn hidsysSetButtonConfigEnabled(addr: BtdrvAddress, flag: bool) -> Result;

    /// SetButtonConfigEmbeddedEnabled
    ///
    /// # Arguments
    /// - `flag` (in): Input flag.
    ///
    /// **Note:** Only available on [11.0.0-17.0.1].
    pub fn hidsysSetButtonConfigEmbeddedEnabled(flag: bool) -> Result;

    /// Legacy IsButtonConfigEnabled.
    ///
    /// # Arguments
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    /// - `out` (out): Output bool flag.
    ///
    /// **Note:** Only available on [10.0.0-10.2.0]. On [11.0.0-17.0.1], use `hidsysIsButtonConfigEnabled` instead.
    pub fn hidsysLegacyIsButtonConfigEnabled(unique_pad_id: HidsysUniquePadId, out: *mut bool) -> Result;

    /// IsButtonConfigEnabled
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    /// - `out` (in): Output bool flag.
    ///
    /// **Note:** Only available on [11.0.0-17.0.1]. On [10.0.0-10.2.0], use `hidsysLegacyIsButtonConfigEnabled` instead.
    pub fn hidsysIsButtonConfigEnabled(addr: BtdrvAddress, out: *mut bool) -> Result;

    /// IsButtonConfigEmbeddedEnabled
    ///
    /// # Arguments
    /// - `out` (out): Output bool flag.
    ///
    /// **Note:** Only available on [11.0.0-17.0.1].
    pub fn hidsysIsButtonConfigEmbeddedEnabled(out: *mut bool) -> Result;

    /// Legacy SetButtonConfigEmbedded.
    ///
    /// # Arguments
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    /// - `config` (in): `HidsysButtonConfigEmbedded`
    ///
    /// **Note:** Only available on [10.0.0-10.2.0]. On [11.0.0-17.0.1], use `hidsysSetButtonConfigEmbedded` instead.
    pub fn hidsysLegacySetButtonConfigEmbedded(unique_pad_id: HidsysUniquePadId, config: *const HidsysButtonConfigEmbedded) -> Result;

    /// SetButtonConfigEmbedded
    ///
    /// # Arguments
    /// - `config` (in): `HidsysButtonConfigEmbedded`
    ///
    /// **Note:** Only available on [11.0.0-17.0.1]. On [10.0.0-10.2.0], use `hidsysLegacySetButtonConfigEmbedded` instead.
    pub fn hidsysSetButtonConfigEmbedded(config: *const HidsysButtonConfigEmbedded) -> Result;

    /// Legacy SetButtonConfigFull.
    ///
    /// # Arguments
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    /// - `config` (in): `HidsysButtonConfigFull`
    ///
    /// **Note:** Only available on [10.0.0-10.2.0]. On [11.0.0-17.0.1], use `hidsysSetButtonConfigFull` instead.
    pub fn hidsysLegacySetButtonConfigFull(unique_pad_id: HidsysUniquePadId, config: *const HidsysButtonConfigFull) -> Result;

    /// SetButtonConfigFull
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    /// - `config` (in): `HidsysButtonConfigFull`
    ///
    /// **Note:** Only available on [11.0.0-17.0.1]. On [10.0.0-10.2.0], use `hidsysLegacySetButtonConfigFull` instead.
    pub fn hidsysSetButtonConfigFull(addr: BtdrvAddress, config: *const HidsysButtonConfigFull) -> Result;

    /// Legacy SetButtonConfigLeft.
    ///
    /// # Arguments
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    /// - `config` (in): `HidsysButtonConfigLeft`
    ///
    /// **Note:** Only available on [10.0.0-10.2.0]. On [11.0.0-17.0.1], use `hidsysSetButtonConfigLeft` instead.
    pub fn hidsysLegacySetButtonConfigLeft(unique_pad_id: HidsysUniquePadId, config: *const HidsysButtonConfigLeft) -> Result;

    /// SetButtonConfigLeft
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    /// - `config` (in): `HidsysButtonConfigLeft`
    ///
    /// **Note:** Only available on [11.0.0-17.0.1]. On [10.0.0-10.2.0], use `hidsysLegacySetButtonConfigLeft` instead.
    pub fn hidsysSetButtonConfigLeft(addr: BtdrvAddress, config: *const HidsysButtonConfigLeft) -> Result;

    /// Legacy SetButtonConfigRight.
    ///
    /// # Arguments
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    /// - `config` (in): `HidsysButtonConfigRight`
    ///
    /// **Note:** Only available on [10.0.0-10.2.0]. On [11.0.0-17.0.1], use `hidsysSetButtonConfigRight` instead.
    pub fn hidsysLegacySetButtonConfigRight(unique_pad_id: HidsysUniquePadId, config: *const HidsysButtonConfigRight) -> Result;

    /// SetButtonConfigRight
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    /// - `config` (in): `HidsysButtonConfigRight`
    ///
    /// **Note:** Only available on [11.0.0-17.0.1]. On [10.0.0-10.2.0], use `hidsysLegacySetButtonConfigRight` instead.
    pub fn hidsysSetButtonConfigRight(addr: BtdrvAddress, config: *const HidsysButtonConfigRight) -> Result;

    /// Legacy GetButtonConfigEmbedded.
    ///
    /// # Arguments
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    /// - `config` (out): `HidsysButtonConfigEmbedded`
    ///
    /// **Note:** Only available on [10.0.0-10.2.0]. On [11.0.0-17.0.1], use `hidsysGetButtonConfigEmbedded` instead.
    pub fn hidsysLegacyGetButtonConfigEmbedded(unique_pad_id: HidsysUniquePadId, config: *mut HidsysButtonConfigEmbedded) -> Result;

    /// GetButtonConfigEmbedded
    ///
    /// # Arguments
    /// - `config` (out): `HidsysButtonConfigEmbedded`
    ///
    /// **Note:** Only available on [11.0.0-17.0.1]. On [10.0.0-10.2.0], use `hidsysLegacyGetButtonConfigEmbedded` instead.
    pub fn hidsysGetButtonConfigEmbedded(config: *mut HidsysButtonConfigEmbedded) -> Result;

    /// Legacy GetButtonConfigFull.
    ///
    /// # Arguments
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    /// - `config` (out): `HidsysButtonConfigFull`
    ///
    /// **Note:** Only available on [10.0.0-10.2.0]. On [11.0.0-17.0.1], use `hidsysGetButtonConfigFull` instead.
    pub fn hidsysLegacyGetButtonConfigFull(unique_pad_id: HidsysUniquePadId, config: *mut HidsysButtonConfigFull) -> Result;

    /// GetButtonConfigFull
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    /// - `config` (out): `HidsysButtonConfigFull`
    ///
    /// **Note:** Only available on [11.0.0-17.0.1]. On [10.0.0-10.2.0], use `hidsysLegacyGetButtonConfigFull` instead.
    pub fn hidsysGetButtonConfigFull(addr: BtdrvAddress, config: *mut HidsysButtonConfigFull) -> Result;

    /// Legacy GetButtonConfigLeft.
    ///
    /// # Arguments
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    /// - `config` (out): `HidsysButtonConfigLeft`
    ///
    /// **Note:** Only available on [10.0.0-10.2.0]. On [11.0.0-17.0.1], use `hidsysGetButtonConfigLeft` instead.
    pub fn hidsysLegacyGetButtonConfigLeft(unique_pad_id: HidsysUniquePadId, config: *mut HidsysButtonConfigLeft) -> Result;

    /// GetButtonConfigLeft
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    /// - `config` (out): `HidsysButtonConfigLeft`
    ///
    /// **Note:** Only available on [11.0.0-17.0.1]. On [10.0.0-10.2.0], use `hidsysLegacyGetButtonConfigLeft` instead.
    pub fn hidsysGetButtonConfigLeft(addr: BtdrvAddress, config: *mut HidsysButtonConfigLeft) -> Result;

    /// Legacy GetButtonConfigRight.
    ///
    /// # Arguments
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    /// - `config` (out): `HidsysButtonConfigRight`
    ///
    /// **Note:** Only available on [10.0.0-10.2.0]. On [11.0.0-17.0.1], use `hidsysGetButtonConfigRight` instead.
    pub fn hidsysLegacyGetButtonConfigRight(unique_pad_id: HidsysUniquePadId, config: *mut HidsysButtonConfigRight) -> Result;

    /// GetButtonConfigRight
    ///
    /// # Arguments
    /// - `addr` (in): `BtdrvAddress`
    /// - `config` (out): `HidsysButtonConfigRight`
    ///
    /// **Note:** Only available on [11.0.0-17.0.1]. On [10.0.0-10.2.0], use `hidsysLegacyGetButtonConfigRight` instead.
    pub fn hidsysGetButtonConfigRight(addr: BtdrvAddress, config: *mut HidsysButtonConfigRight) -> Result;

    /// IsCustomButtonConfigSupported
    ///
    /// # Arguments
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    /// - `out` (out): Output bool flag.
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn hidsysIsCustomButtonConfigSupported(unique_pad_id: HidsysUniquePadId, out: *mut bool) -> Result;

    /// IsDefaultButtonConfigEmbedded
    ///
    /// # Arguments
    /// - `config` (in): `HidcfgButtonConfigEmbedded`
    /// - `out` (out): Output bool flag.
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn hidsysIsDefaultButtonConfigEmbedded(config: *const HidcfgButtonConfigEmbedded, out: *mut bool) -> Result;

    /// IsDefaultButtonConfigFull
    ///
    /// # Arguments
    /// - `config` (in): `HidcfgButtonConfigFull`
    /// - `out` (out): Output bool flag.
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn hidsysIsDefaultButtonConfigFull(config: *const HidcfgButtonConfigFull, out: *mut bool) -> Result;

    /// IsDefaultButtonConfigLeft
    ///
    /// # Arguments
    /// - `config` (in): `HidcfgButtonConfigLeft`
    /// - `out` (out): Output bool flag.
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn hidsysIsDefaultButtonConfigLeft(config: *const HidcfgButtonConfigLeft, out: *mut bool) -> Result;

    /// IsDefaultButtonConfigRight
    ///
    /// # Arguments
    /// - `config` (in): `HidcfgButtonConfigRight`
    /// - `out` (out): Output bool flag.
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn hidsysIsDefaultButtonConfigRight(config: *const HidcfgButtonConfigRight, out: *mut bool) -> Result;

    /// IsButtonConfigStorageEmbeddedEmpty
    ///
    /// # Arguments
    /// - `index` (in): Array index, should be 0-4.
    /// - `out` (out): Output bool flag.
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn hidsysIsButtonConfigStorageEmbeddedEmpty(index: s32, out: *mut bool) -> Result;

    /// IsButtonConfigStorageFullEmpty
    ///
    /// # Arguments
    /// - `index` (in): Array index, should be 0-4.
    /// - `out` (out): Output bool flag.
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn hidsysIsButtonConfigStorageFullEmpty(index: s32, out: *mut bool) -> Result;

    /// IsButtonConfigStorageLeftEmpty
    ///
    /// # Arguments
    /// - `index` (in): Array index, should be 0-4.
    /// - `out` (out): Output bool flag.
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn hidsysIsButtonConfigStorageLeftEmpty(index: s32, out: *mut bool) -> Result;

    /// IsButtonConfigStorageRightEmpty
    ///
    /// # Arguments
    /// - `index` (in): Array index, should be 0-4.
    /// - `out` (out): Output bool flag.
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn hidsysIsButtonConfigStorageRightEmpty(index: s32, out: *mut bool) -> Result;

    /// GetButtonConfigStorageEmbeddedDeprecated
    ///
    /// # Arguments
    /// - `index` (in): Array index, should be 0-4.
    /// - `config` (out): `HidcfgButtonConfigEmbedded`
    ///
    /// **Note:** Only available on [10.0.0-12.1.0].
    pub fn hidsysGetButtonConfigStorageEmbeddedDeprecated(index: s32, config: *mut HidcfgButtonConfigEmbedded) -> Result;

    /// GetButtonConfigStorageFullDeprecated
    ///
    /// # Arguments
    /// - `index` (in): Array index, should be 0-4.
    /// - `config` (out): `HidcfgButtonConfigFull`
    ///
    /// **Note:** Only available on [10.0.0-12.1.0].
    pub fn hidsysGetButtonConfigStorageFullDeprecated(index: s32, config: *mut HidcfgButtonConfigFull) -> Result;

    /// GetButtonConfigStorageLeftDeprecated
    ///
    /// # Arguments
    /// - `index` (in): Array index, should be 0-4.
    /// - `config` (out): `HidcfgButtonConfigLeft`
    ///
    /// **Note:** Only available on [10.0.0-12.1.0].
    pub fn hidsysGetButtonConfigStorageLeftDeprecated(index: s32, config: *mut HidcfgButtonConfigLeft) -> Result;

    /// GetButtonConfigStorageRightDeprecated
    ///
    /// # Arguments
    /// - `index` (in): Array index, should be 0-4.
    /// - `config` (out): `HidcfgButtonConfigRight`
    ///
    /// **Note:** Only available on [10.0.0-12.1.0].
    pub fn hidsysGetButtonConfigStorageRightDeprecated(index: s32, config: *mut HidcfgButtonConfigRight) -> Result;

    /// SetButtonConfigStorageEmbeddedDeprecated
    ///
    /// # Arguments
    /// - `index` (in): Array index, should be 0-4.
    /// - `config` (in): `HidcfgButtonConfigEmbedded`
    ///
    /// **Note:** Only available on [10.0.0-12.1.0].
    pub fn hidsysSetButtonConfigStorageEmbeddedDeprecated(index: s32, config: *const HidcfgButtonConfigEmbedded) -> Result;

    /// SetButtonConfigStorageFullDeprecated
    ///
    /// # Arguments
    /// - `index` (in): Array index, should be 0-4.
    /// - `config` (in): `HidcfgButtonConfigFull`
    ///
    /// **Note:** Only available on [10.0.0-12.1.0].
    pub fn hidsysSetButtonConfigStorageFullDeprecated(index: s32, config: *const HidcfgButtonConfigFull) -> Result;

    /// SetButtonConfigStorageLeftDeprecated
    ///
    /// # Arguments
    /// - `index` (in): Array index, should be 0-4.
    /// - `config` (in): `HidcfgButtonConfigLeft`
    ///
    /// **Note:** Only available on [10.0.0-12.1.0].
    pub fn hidsysSetButtonConfigStorageLeftDeprecated(index: s32, config: *const HidcfgButtonConfigLeft) -> Result;

    /// SetButtonConfigStorageRightDeprecated
    ///
    /// # Arguments
    /// - `index` (in): Array index, should be 0-4.
    /// - `config` (in): `HidcfgButtonConfigRight`
    ///
    /// **Note:** Only available on [10.0.0-12.1.0].
    pub fn hidsysSetButtonConfigStorageRightDeprecated(index: s32, config: *const HidcfgButtonConfigRight) -> Result;

    /// DeleteButtonConfigStorageEmbedded
    ///
    /// # Arguments
    /// - `index` (in): Array index, should be 0-4.
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn hidsysDeleteButtonConfigStorageEmbedded(index: s32) -> Result;

    /// DeleteButtonConfigStorageFull
    ///
    /// # Arguments
    /// - `index` (in): Array index, should be 0-4.
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn hidsysDeleteButtonConfigStorageFull(index: s32) -> Result;

    /// DeleteButtonConfigStorageLeft
    ///
    /// # Arguments
    /// - `index` (in): Array index, should be 0-4.
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn hidsysDeleteButtonConfigStorageLeft(index: s32) -> Result;

    /// DeleteButtonConfigStorageRight
    ///
    /// # Arguments
    /// - `index` (in): Array index, should be 0-4.
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn hidsysDeleteButtonConfigStorageRight(index: s32) -> Result;

    /// IsUsingCustomButtonConfig
    ///
    /// # Arguments
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    /// - `out` (out): Output bool flag.
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn hidsysIsUsingCustomButtonConfig(unique_pad_id: HidsysUniquePadId, out: *mut bool) -> Result;

    /// IsAnyCustomButtonConfigEnabled
    ///
    /// # Arguments
    /// - `out` (out): Output bool flag.
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn hidsysIsAnyCustomButtonConfigEnabled(out: *mut bool) -> Result;

    /// SetAllCustomButtonConfigEnabled
    ///
    /// # Arguments
    /// - `AppletResourceUserId` (in): AppletResourceUserId
    /// - `flag` (in): Input bool flag.
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn hidsysSetAllCustomButtonConfigEnabled(AppletResourceUserId: u64, flag: bool) -> Result;

    /// SetAllDefaultButtonConfig
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn hidsysSetAllDefaultButtonConfig() -> Result;

    /// SetHidButtonConfigEmbedded
    ///
    /// # Arguments
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    /// - `config` (in): `HidcfgButtonConfigEmbedded`
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn hidsysSetHidButtonConfigEmbedded(unique_pad_id: HidsysUniquePadId, config: *const HidcfgButtonConfigEmbedded) -> Result;

    /// SetHidButtonConfigFull
    ///
    /// # Arguments
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    /// - `config` (in): `HidcfgButtonConfigFull`
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn hidsysSetHidButtonConfigFull(unique_pad_id: HidsysUniquePadId, config: *const HidcfgButtonConfigFull) -> Result;

    /// SetHidButtonConfigLeft
    ///
    /// # Arguments
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    /// - `config` (in): `HidcfgButtonConfigLeft`
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn hidsysSetHidButtonConfigLeft(unique_pad_id: HidsysUniquePadId, config: *const HidcfgButtonConfigLeft) -> Result;

    /// SetHidButtonConfigRight
    ///
    /// # Arguments
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    /// - `config` (in): `HidcfgButtonConfigRight`
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn hidsysSetHidButtonConfigRight(unique_pad_id: HidsysUniquePadId, config: *const HidcfgButtonConfigRight) -> Result;

    /// GetHidButtonConfigEmbedded
    ///
    /// # Arguments
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    /// - `config` (out): `HidcfgButtonConfigEmbedded`
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn hidsysGetHidButtonConfigEmbedded(unique_pad_id: HidsysUniquePadId, config: *mut HidcfgButtonConfigEmbedded) -> Result;

    /// GetHidButtonConfigFull
    ///
    /// # Arguments
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    /// - `config` (out): `HidcfgButtonConfigFull`
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn hidsysGetHidButtonConfigFull(unique_pad_id: HidsysUniquePadId, config: *mut HidcfgButtonConfigFull) -> Result;

    /// GetHidButtonConfigLeft
    ///
    /// # Arguments
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    /// - `config` (out): `HidcfgButtonConfigLeft`
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn hidsysGetHidButtonConfigLeft(unique_pad_id: HidsysUniquePadId, config: *mut HidcfgButtonConfigLeft) -> Result;

    /// GetHidButtonConfigRight
    ///
    /// # Arguments
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    /// - `config` (out): `HidcfgButtonConfigRight`
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn hidsysGetHidButtonConfigRight(unique_pad_id: HidsysUniquePadId, config: *mut HidcfgButtonConfigRight) -> Result;

    /// GetButtonConfigStorageEmbedded
    ///
    /// # Arguments
    /// - `index` (in): Array index, should be 0-4.
    /// - `config` (out): `HidcfgButtonConfigEmbedded`
    /// - `name` (out): `HidcfgStorageName`
    ///
    /// **Note:** Only available on [11.0.0+].
    pub fn hidsysGetButtonConfigStorageEmbedded(index: s32, config: *mut HidcfgButtonConfigEmbedded, name: *mut HidcfgStorageName) -> Result;

    /// GetButtonConfigStorageFull
    ///
    /// # Arguments
    /// - `index` (in): Array index, should be 0-4.
    /// - `config` (out): `HidcfgButtonConfigFull`
    /// - `name` (out): `HidcfgStorageName`
    ///
    /// **Note:** Only available on [11.0.0+].
    pub fn hidsysGetButtonConfigStorageFull(index: s32, config: *mut HidcfgButtonConfigFull, name: *mut HidcfgStorageName) -> Result;

    /// GetButtonConfigStorageLeft
    ///
    /// # Arguments
    /// - `index` (in): Array index, should be 0-4.
    /// - `config` (out): `HidcfgButtonConfigLeft`
    /// - `name` (out): `HidcfgStorageName`
    ///
    /// **Note:** Only available on [11.0.0+].
    pub fn hidsysGetButtonConfigStorageLeft(index: s32, config: *mut HidcfgButtonConfigLeft, name: *mut HidcfgStorageName) -> Result;

    /// GetButtonConfigStorageRight
    ///
    /// # Arguments
    /// - `index` (in): Array index, should be 0-4.
    /// - `config` (out): `HidcfgButtonConfigRight`
    /// - `name` (out): `HidcfgStorageName`
    ///
    /// **Note:** Only available on [11.0.0+].
    pub fn hidsysGetButtonConfigStorageRight(index: s32, config: *mut HidcfgButtonConfigRight, name: *mut HidcfgStorageName) -> Result;

    /// SetButtonConfigStorageEmbedded
    ///
    /// # Arguments
    /// - `index` (in): Array index, should be 0-4.
    /// - `config` (in): `HidcfgButtonConfigEmbedded`
    /// - `name` (in): `HidcfgStorageName`
    ///
    /// **Note:** Only available on [11.0.0+].
    pub fn hidsysSetButtonConfigStorageEmbedded(index: s32, config: *const HidcfgButtonConfigEmbedded, name: *const HidcfgStorageName) -> Result;

    /// SetButtonConfigStorageFull
    ///
    /// # Arguments
    /// - `index` (in): Array index, should be 0-4.
    /// - `config` (in): `HidcfgButtonConfigFull`
    /// - `name` (in): `HidcfgStorageName`
    ///
    /// **Note:** Only available on [11.0.0+].
    pub fn hidsysSetButtonConfigStorageFull(index: s32, config: *const HidcfgButtonConfigFull, name: *const HidcfgStorageName) -> Result;

    /// SetButtonConfigStorageLeft
    ///
    /// # Arguments
    /// - `index` (in): Array index, should be 0-4.
    /// - `config` (in): `HidcfgButtonConfigLeft`
    /// - `name` (in): `HidcfgStorageName`
    ///
    /// **Note:** Only available on [11.0.0+].
    pub fn hidsysSetButtonConfigStorageLeft(index: s32, config: *const HidcfgButtonConfigLeft, name: *const HidcfgStorageName) -> Result;

    /// SetButtonConfigStorageRight
    ///
    /// # Arguments
    /// - `index` (in): Array index, should be 0-4.
    /// - `config` (in): `HidcfgButtonConfigRight`
    /// - `name` (in): `HidcfgStorageName`
    ///
    /// **Note:** Only available on [11.0.0+].
    pub fn hidsysSetButtonConfigStorageRight(index: s32, config: *const HidcfgButtonConfigRight, name: *const HidcfgStorageName) -> Result;
}
