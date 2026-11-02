//! `switch/services/hiddbg.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// HiddbgNpadButton. For the remaining buttons, see `HidNpadButton`.
pub type HiddbgNpadButton = u32;
/// HOME button
pub const HiddbgNpadButton_Home: HiddbgNpadButton = BIT(18);
/// Capture button
pub const HiddbgNpadButton_Capture: HiddbgNpadButton = BIT(19);

/// HdlsAttribute
pub type HiddbgHdlsAttribute = u32;
/// HasVirtualSixAxisSensorAcceleration
pub const HiddbgHdlsAttribute_HasVirtualSixAxisSensorAcceleration: HiddbgHdlsAttribute = BIT(0);
/// HasVirtualSixAxisSensorAngle
pub const HiddbgHdlsAttribute_HasVirtualSixAxisSensorAngle: HiddbgHdlsAttribute = BIT(1);

/// State for overriding `HidDebugPadState`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HiddbgDebugPadAutoPilotState {
    /// Bitfield of `HidDebugPadAttribute`.
    pub attributes: u32,
    /// Bitfield of `HidDebugPadButton`.
    pub buttons: u32,
    /// AnalogStickL
    pub analog_stick_l: HidAnalogStickState,
    /// AnalogStickR
    pub analog_stick_r: HidAnalogStickState,
}

/// State for overriding `HidMouseState`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HiddbgMouseAutoPilotState {
    /// X
    pub x: s32,
    /// Y
    pub y: s32,
    /// DeltaX
    pub delta_x: s32,
    /// DeltaY
    pub delta_y: s32,
    /// WheelDelta
    pub wheel_delta: s32,
    /// Bitfield of `HidMouseButton`.
    pub buttons: u32,
    /// Bitfield of `HidMouseAttribute`.
    pub attributes: u32,
}

/// State for overriding `HidKeyboardState`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HiddbgKeyboardAutoPilotState {
    /// Bitfield of `HidKeyboardModifier`.
    pub modifiers: u64,
    pub keys: [u64; 4],
}

/// State for overriding SleepButtonState.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HiddbgSleepButtonAutoPilotState {
    /// Bitfield of buttons, only bit0 is used.
    pub buttons: u64,
}

/// HdlsHandle
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HiddbgHdlsHandle {
    /// Handle
    pub handle: u64,
}

/// HdlsSessionId, returned by `hiddbgAttachHdlsWorkBuffer`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HiddbgHdlsSessionId {
    /// Id
    pub id: u64,
}

/// HdlsDeviceInfo, for [7.0.0-8.1.0].
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HiddbgHdlsDeviceInfoV7 {
    /// Only one bit can be set. BIT(N*4+0) = Pro-Controller, BIT(N*4+1) = Joy-Con Left, BIT(N*4+2) = Joy-Con Right, BIT(N*4+3) = invalid. Where N is 0-1. BIT(8-10) = Pro-Controller, BIT(11) = Famicom-Controller, BIT(12) = Famicom-Controller II with microphone, BIT(13) = NES-Controller(DeviceType=0x200), BIT(14) = NES-Controller(DeviceType=0x400), BIT(15-16) = invalid, BIT(17) = unknown(DeviceType=0x8000), BIT(18-20) = invalid, BIT(21-23) = unknown(DeviceType=0x80000000).
    pub deviceTypeInternal: u32,
    /// RGBA Single Body Color.
    pub singleColorBody: u32,
    /// RGBA Single Buttons Color.
    pub singleColorButtons: u32,
    /// **Ref:** HidNpadInterfaceType. Additional type field used with the above type field (only applies to type bit0-bit2 and bit21), if the value doesn't match one of the following a default is used. Type Pro-Controller: value 0x3 indicates that the controller is connected via USB. Type BIT(21): value 0x3 = unknown. When value is 0x2, state is merged with an existing controller (when the type value is compatible with this). Otherwise, it's a dedicated controller.
    pub npadInterfaceType: u8,
    /// Padding.
    pub pad: [u8; 3],
}

/// HdlsDeviceInfo, for [9.0.0+]. Converted to/from `HiddbgHdlsDeviceInfoV7` on prior sysvers.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HiddbgHdlsDeviceInfo {
    /// **Ref:** HidDeviceType
    pub deviceType: u8,
    /// **Ref:** HidNpadInterfaceType. Additional type field used with the above type field (only applies to ::HidDeviceType_JoyRight1, ::HidDeviceType_JoyLeft2, ::HidDeviceType_FullKey3, and ::HidDeviceType_System19), if the value doesn't match one of the following a default is used. ::HidDeviceType_FullKey3: ::HidNpadInterfaceType_USB indicates that the controller is connected via USB. :::HidDeviceType_System19: ::HidNpadInterfaceType_USB = unknown. When value is ::HidNpadInterfaceType_Rail, state is merged with an existing controller (with ::HidDeviceType_JoyRight1 / ::HidDeviceType_JoyLeft2). Otherwise, it's a dedicated controller.
    pub npadInterfaceType: u8,
    /// Padding.
    pub pad: [u8; 2],
    /// RGBA Single Body Color.
    pub singleColorBody: u32,
    /// RGBA Single Buttons Color.
    pub singleColorButtons: u32,
    /// [9.0.0+] RGBA Left Grip Color.
    pub colorLeftGrip: u32,
    /// [9.0.0+] RGBA Right Grip Color.
    pub colorRightGrip: u32,
}

/// HdlsState, for [7.0.0-8.1.0].
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HiddbgHdlsStateV7 {
    /// IsPowered for the main PowerInfo, see `HidNpadSystemProperties`.
    pub is_powered: u8,
    /// ORRed with IsPowered to set the value of the first byte for `HidNpadSystemProperties`. For example, value 1 here will set IsCharging for the main PowerInfo.
    pub flags: u8,
    /// Unknown
    pub unk_x2: [u8; 6],
    /// BatteryLevel for the main PowerInfo, see `HidPowerInfo`.
    pub battery_level: u32,
    /// See `HiddbgNpadButton`.
    pub buttons: u32,
    /// AnalogStickL
    pub analog_stick_l: HidAnalogStickState,
    /// AnalogStickR
    pub analog_stick_r: HidAnalogStickState,
    /// Indicator. Unused for input. Set with output from `hiddbgDumpHdlsStates`. Not set by `hiddbgGetAbstractedPadsState`.
    pub indicator: u8,
    /// Padding
    pub padding: [u8; 3],
}

/// HdlsState, for [9.0.0-11.0.1].
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HiddbgHdlsStateV9 {
    /// BatteryLevel for the main PowerInfo, see `HidPowerInfo`.
    pub battery_level: u32,
    /// Used to set the main PowerInfo for `HidNpadSystemProperties`. BIT(0) -> IsPowered, BIT(1) -> IsCharging.
    pub flags: u32,
    /// See `HiddbgNpadButton`. [9.0.0+] Masked with 0xfffffffff00fffff.
    pub buttons: u64,
    /// AnalogStickL
    pub analog_stick_l: HidAnalogStickState,
    /// AnalogStickR
    pub analog_stick_r: HidAnalogStickState,
    /// Indicator. Unused for input. Set with output from `hiddbgDumpHdlsStates`.
    pub indicator: u8,
    /// Padding
    pub padding: [u8; 3],
}

/// HdlsState, for [12.0.0+].
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HiddbgHdlsState {
    /// BatteryLevel for the main PowerInfo, see `HidPowerInfo`.
    pub battery_level: u32,
    /// Used to set the main PowerInfo for `HidNpadSystemProperties`. BIT(0) -> IsPowered, BIT(1) -> IsCharging.
    pub flags: u32,
    /// See `HiddbgNpadButton`. [9.0.0+] Masked with 0xfffffffff00fffff.
    pub buttons: u64,
    /// AnalogStickL
    pub analog_stick_l: HidAnalogStickState,
    /// AnalogStickR
    pub analog_stick_r: HidAnalogStickState,
    /// VirtualSixAxisSensorAcceleration
    pub six_axis_sensor_acceleration: HidVector,
    /// VirtualSixAxisSensorAngle
    pub six_axis_sensor_angle: HidVector,
    /// Bitfield of `HiddbgHdlsAttribute`.
    pub attribute: u32,
    /// Indicator. Unused for input.
    pub indicator: u8,
    /// Padding
    pub padding: [u8; 3],
}

/// HdlsNpadAssignmentEntry
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HiddbgHdlsNpadAssignmentEntry {
    /// **Ref:** HiddbgHdlsHandle
    pub handle: HiddbgHdlsHandle,
    /// Unknown
    pub unk_x8: u32,
    /// Unknown
    pub unk_xc: u32,
    /// Unknown
    pub unk_x10: u64,
    /// Unknown
    pub unk_x18: u8,
    /// Padding
    pub pad: [u8; 7],
}

/// HdlsNpadAssignment. Same controllers as `HiddbgHdlsStateList`, with different entry data.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HiddbgHdlsNpadAssignment {
    /// Total entries for the below entries.
    pub total_entries: s32,
    /// Padding
    pub pad: u32,
    /// **Ref:** HiddbgHdlsNpadAssignmentEntry
    pub entries: [HiddbgHdlsNpadAssignmentEntry; 16],
}

/// HdlsStateListEntryV7, for [7.0.0-8.1.0].
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HiddbgHdlsStateListEntryV7 {
    /// **Ref:** HiddbgHdlsHandle
    pub handle: HiddbgHdlsHandle,
    /// **Ref:** HiddbgHdlsDeviceInfoV7. With `hiddbgApplyHdlsStateList` this is only used when creating new devices.
    pub device: HiddbgHdlsDeviceInfoV7,
    /// **Ref:** HiddbgHdlsStateV7
    pub state: HiddbgHdlsStateV7,
}

/// HdlsStateListV7, for [7.0.0-8.1.0]. This contains a list of all controllers, including non-virtual controllers.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HiddbgHdlsStateListV7 {
    /// Total entries for the below entries.
    pub total_entries: s32,
    /// Padding
    pub pad: u32,
    /// **Ref:** HiddbgHdlsStateListEntryV7
    pub entries: [HiddbgHdlsStateListEntryV7; 16],
}

/// HdlsStateListEntry, for [9.0.0-11.0.1].
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HiddbgHdlsStateListEntryV9 {
    /// **Ref:** HiddbgHdlsHandle
    pub handle: HiddbgHdlsHandle,
    /// **Ref:** HiddbgHdlsDeviceInfo. With `hiddbgApplyHdlsStateList` this is only used when creating new devices.
    pub device: HiddbgHdlsDeviceInfo,
    /// **Ref:** HiddbgHdlsStateV9
    pub state: HiddbgHdlsStateV9,
}

/// HdlsStateList, for [9.0.0-11.0.1].
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HiddbgHdlsStateListV9 {
    /// Total entries for the below entries.
    pub total_entries: s32,
    /// Padding
    pub pad: u32,
    /// **Ref:** HiddbgHdlsStateListEntryV9
    pub entries: [HiddbgHdlsStateListEntryV9; 16],
}

/// HdlsStateListEntry, for [12.0.0+].
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HiddbgHdlsStateListEntry {
    /// **Ref:** HiddbgHdlsHandle
    pub handle: HiddbgHdlsHandle,
    /// **Ref:** HiddbgHdlsDeviceInfo. With `hiddbgApplyHdlsStateList` this is only used when creating new devices.
    pub device: HiddbgHdlsDeviceInfo,
    /// **Ref:** HiddbgHdlsState
    pub state: HiddbgHdlsState,
}

/// HdlsStateList, for [12.0.0+].
/// This contains a list of all controllers, including non-virtual controllers.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HiddbgHdlsStateList {
    /// Total entries for the below entries.
    pub total_entries: s32,
    /// Padding
    pub pad: u32,
    /// **Ref:** HiddbgHdlsStateListEntry
    pub entries: [HiddbgHdlsStateListEntry; 16],
}

/// AbstractedPadHandle
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HiddbgAbstractedPadHandle {
    /// Handle
    pub handle: u64,
}

/// AbstractedPadState
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HiddbgAbstractedPadState {
    /// Type. Converted to HiddbgHdlsDeviceInfoV7::type internally by `hiddbgSetAutoPilotVirtualPadState`. BIT(0) -> BIT(0), BIT(1) -> BIT(15), BIT(2-3) -> BIT(1-2), BIT(4-5) -> BIT(1-2), BIT(6) -> BIT(3). BIT(7-11) -> BIT(11-15), BIT(12-14) -> BIT(12-14), BIT(15) -> BIT(17), BIT(31) -> BIT(21).
    pub type_: u32,
    /// Flags. Only bit0 is used by `hiddbgSetAutoPilotVirtualPadState`, when clear it will skip using the rest of the input and run `hiddbgUnsetAutoPilotVirtualPadState` internally.
    pub flags: u8,
    /// Padding
    pub pad: [u8; 3],
    /// RGBA Single Body Color
    pub singleColorBody: u32,
    /// RGBA Single Buttons Color
    pub singleColorButtons: u32,
    /// See HiddbgHdlsDeviceInfo::npadInterfaceType.
    pub npadInterfaceType: u8,
    /// Padding
    pub pad2: [u8; 3],
    /// State
    pub state: HiddbgHdlsStateV7,
    /// Unused with `hiddbgSetAutoPilotVirtualPadState`. Not set by `hiddbgGetAbstractedPadsState`.
    pub unused: [u8; 96],
}

impl_zeroed_default!(HiddbgDebugPadAutoPilotState, HiddbgMouseAutoPilotState, HiddbgKeyboardAutoPilotState, HiddbgSleepButtonAutoPilotState, HiddbgHdlsHandle, HiddbgHdlsSessionId, HiddbgHdlsDeviceInfoV7, HiddbgHdlsDeviceInfo, HiddbgHdlsStateV7, HiddbgHdlsStateV9, HiddbgHdlsState, HiddbgHdlsNpadAssignmentEntry, HiddbgHdlsNpadAssignment, HiddbgHdlsStateListEntryV7, HiddbgHdlsStateListV7, HiddbgHdlsStateListEntryV9, HiddbgHdlsStateListV9, HiddbgHdlsStateListEntry, HiddbgHdlsStateList, HiddbgAbstractedPadHandle, HiddbgAbstractedPadState);

unsafe extern "C" {
    /// Initialize hiddbg.
    pub fn hiddbgInitialize() -> Result;

    /// Exit hiddbg.
    pub fn hiddbgExit();

    /// Gets the Service object for the actual hiddbg service session.
    pub fn hiddbgGetServiceSession() -> *mut Service;

    /// SetDebugPadAutoPilotState
    ///
    /// # Arguments
    /// - `state` (in): `HiddbgDebugPadAutoPilotState`
    pub fn hiddbgSetDebugPadAutoPilotState(state: *const HiddbgDebugPadAutoPilotState) -> Result;

    /// UnsetDebugPadAutoPilotState
    pub fn hiddbgUnsetDebugPadAutoPilotState() -> Result;

    /// SetTouchScreenAutoPilotState
    ///
    /// # Arguments
    /// - `states` (in): Input array of `HiddbgMouseAutoPilotState`.
    /// - `count` (in): Total entries in the states array. Max is 16.
    pub fn hiddbgSetTouchScreenAutoPilotState(states: *const HidTouchState, count: s32) -> Result;

    /// UnsetTouchScreenAutoPilotState
    pub fn hiddbgUnsetTouchScreenAutoPilotState() -> Result;

    /// SetMouseAutoPilotState
    ///
    /// # Arguments
    /// - `state` (in): `HiddbgMouseAutoPilotState`
    pub fn hiddbgSetMouseAutoPilotState(state: *const HiddbgMouseAutoPilotState) -> Result;

    /// UnsetMouseAutoPilotState
    pub fn hiddbgUnsetMouseAutoPilotState() -> Result;

    /// SetKeyboardAutoPilotState
    ///
    /// # Arguments
    /// - `state` (in): `HiddbgKeyboardAutoPilotState`
    pub fn hiddbgSetKeyboardAutoPilotState(state: *const HiddbgKeyboardAutoPilotState) -> Result;

    /// UnsetKeyboardAutoPilotState
    pub fn hiddbgUnsetKeyboardAutoPilotState() -> Result;

    /// Deactivates the HomeButton.
    pub fn hiddbgDeactivateHomeButton() -> Result;

    /// SetSleepButtonAutoPilotState
    ///
    /// # Arguments
    /// - `state` (in): `HiddbgSleepButtonAutoPilotState`
    pub fn hiddbgSetSleepButtonAutoPilotState(state: *const HiddbgSleepButtonAutoPilotState) -> Result;

    /// UnsetSleepButtonAutoPilotState
    pub fn hiddbgUnsetSleepButtonAutoPilotState() -> Result;

    /// Writes the input RGB colors to the spi-flash for the specified UniquePad (offset 0x6050 size 0x6).
    ///
    /// # Arguments
    /// - `colorBody` (in): RGB body color.
    /// - `colorButtons` (in): RGB buttons color.
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    ///
    /// **Note:** Only available with [3.0.0+].
    pub fn hiddbgUpdateControllerColor(colorBody: u32, colorButtons: u32, unique_pad_id: HidsysUniquePadId) -> Result;

    /// Writes the input RGB colors followed by inval to the spi-flash for the specified UniquePad (offset 0x6050 size 0xD).
    ///
    /// # Arguments
    /// - `colorBody` (in): RGB body color.
    /// - `colorButtons` (in): RGB buttons color.
    /// - `colorLeftGrip` (in): RGB left grip color.
    /// - `colorRightGrip` (in): RGB right grip color.
    /// - `inval` (in): Input value.
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    ///
    /// **Note:** Only available with [5.0.0+].
    pub fn hiddbgUpdateDesignInfo(colorBody: u32, colorButtons: u32, colorLeftGrip: u32, colorRightGrip: u32, inval: u8, unique_pad_id: HidsysUniquePadId) -> Result;

    /// Get the OperationEvent for the specified UniquePad.
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event.
    /// - `autoclear` (in): The autoclear for the Event.
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    ///
    /// **Note:** Only available with [6.0.0+].
    pub fn hiddbgAcquireOperationEventHandle(out_event: *mut Event, autoclear: bool, unique_pad_id: HidsysUniquePadId) -> Result;

    /// Reads spi-flash for the specified UniquePad.
    ///
    /// # Arguments
    /// - `offset` (in): Offset in spi-flash.
    /// - `buffer` (out): Output buffer.
    /// - `size` (in): Output buffer size.
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    ///
    /// **Note:** This also uses `hiddbgAcquireOperationEventHandle` to wait for the operation to finish, then `hiddbgGetOperationResult` is used.
    ///
    /// **Note:** Only available with [6.0.0+].
    pub fn hiddbgReadSerialFlash(offset: u32, buffer: *mut c_void, size: usize, unique_pad_id: HidsysUniquePadId) -> Result;

    /// Writes spi-flash for the specified UniquePad.
    ///
    /// # Arguments
    /// - `offset` (in): Offset in spi-flash.
    /// - `buffer` (in): Input buffer, must be 0x1000-byte aligned.
    /// - `tmem_size` (in): Size of the buffer, must be 0x1000-byte aligned.
    /// - `size` (in): Actual transfer size.
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    ///
    /// **Note:** This also uses `hiddbgAcquireOperationEventHandle` to wait for the operation to finish, then `hiddbgGetOperationResult` is used.
    ///
    /// **Note:** Only available with [6.0.0+].
    pub fn hiddbgWriteSerialFlash(offset: u32, buffer: *mut c_void, tmem_size: usize, size: usize, unique_pad_id: HidsysUniquePadId) -> Result;

    /// Get the Result for the Operation and handles cleanup, for the specified UniquePad.
    ///
    /// # Arguments
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    ///
    /// **Note:** Only available with [6.0.0+].
    pub fn hiddbgGetOperationResult(unique_pad_id: HidsysUniquePadId) -> Result;

    /// Gets the internal DeviceType for the specified controller.
    ///
    /// # Arguments
    /// - `unique_pad_id` (in): `HidsysUniquePadId`
    /// - `out` (out): Pre-9.0.0 this is an u32, with [9.0.0+] it's an u8.
    ///
    /// **Note:** Only available with [6.0.0+].
    pub fn hiddbgGetUniquePadDeviceTypeSetInternal(unique_pad_id: HidsysUniquePadId, out: *mut u32) -> Result;

    /// Gets a list of `HiddbgAbstractedPadHandle`.
    ///
    /// # Arguments
    /// - `handles` (out): Output array of `HiddbgAbstractedPadHandle`.
    /// - `count` (in): Max number of entries for the handles array.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** Only available with [5.0.0-8.1.0].
    pub fn hiddbgGetAbstractedPadHandles(handles: *mut HiddbgAbstractedPadHandle, count: s32, total_out: *mut s32) -> Result;

    /// Gets the state for the specified `HiddbgAbstractedPadHandle`.
    ///
    /// # Arguments
    /// - `handle` (in): `HiddbgAbstractedPadHandle`
    /// - `state` (out): `HiddbgAbstractedPadState`
    ///
    /// **Note:** Only available with [5.0.0-8.1.0].
    pub fn hiddbgGetAbstractedPadState(handle: HiddbgAbstractedPadHandle, state: *mut HiddbgAbstractedPadState) -> Result;

    /// Similar to `hiddbgGetAbstractedPadHandles` except this also returns the state for each pad in output array states.
    ///
    /// # Arguments
    /// - `handles` (out): Output array of `HiddbgAbstractedPadHandle`.
    /// - `states` (out): Output array of `HiddbgAbstractedPadState`.
    /// - `count` (in): Max number of entries for the handles/states arrays.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** Only available with [5.0.0-8.1.0].
    pub fn hiddbgGetAbstractedPadsState(handles: *mut HiddbgAbstractedPadHandle, states: *mut HiddbgAbstractedPadState, count: s32, total_out: *mut s32) -> Result;

    /// Sets AutoPilot state for the specified pad.
    ///
    /// # Arguments
    /// - `AbstractedVirtualPadId` (in): This can be any unique value as long as it's within bounds. For example, 0-7 is usable.
    /// - `state` (in): `HiddbgAbstractedPadState`
    ///
    /// **Note:** Only available with [5.0.0-8.1.0].
    pub fn hiddbgSetAutoPilotVirtualPadState(AbstractedVirtualPadId: s8, state: *const HiddbgAbstractedPadState) -> Result;

    /// Clears AutoPilot state for the specified pad set by `hiddbgSetAutoPilotVirtualPadState`.
    ///
    /// # Arguments
    /// - `AbstractedVirtualPadId` (in): Id from `hiddbgSetAutoPilotVirtualPadState`.
    ///
    /// **Note:** Only available with [5.0.0-8.1.0].
    pub fn hiddbgUnsetAutoPilotVirtualPadState(AbstractedVirtualPadId: s8) -> Result;

    /// Clears AutoPilot state for all pads set by `hiddbgSetAutoPilotVirtualPadState`.
    pub fn hiddbgUnsetAllAutoPilotVirtualPadState() -> Result;

    /// Initialize Hdls.
    ///
    /// # Arguments
    /// - `session_id` (out): [13.0.0+] `HiddbgHdlsSessionId`
    /// - `buffer` (in): An existing buffer to be used as transfer memory.
    /// - `size` (in): Size of the supplied buffer.
    ///
    /// **Note:** Only available with [7.0.0+].
    pub fn hiddbgAttachHdlsWorkBuffer(session_id: *mut HiddbgHdlsSessionId, buffer: *mut c_void, size: usize) -> Result;

    /// Exit Hdls, must be called at some point prior to `hiddbgExit`.
    ///
    /// # Arguments
    /// - `session_id` (in): [13.0.0+] `HiddbgHdlsSessionId`
    ///
    /// **Note:** Only available with [7.0.0+].
    pub fn hiddbgReleaseHdlsWorkBuffer(session_id: HiddbgHdlsSessionId) -> Result;

    /// Checks if the given device is still attached.
    ///
    /// # Arguments
    /// - `session_id` (in): [13.0.0+] `HiddbgHdlsSessionId`
    /// - `handle` (in): `HiddbgHdlsHandle`
    /// - `out` (out): Whether the device is attached.
    ///
    /// **Note:** Only available with [7.0.0+].
    pub fn hiddbgIsHdlsVirtualDeviceAttached(session_id: HiddbgHdlsSessionId, handle: HiddbgHdlsHandle, out: *mut bool) -> Result;

    /// Gets state for `HiddbgHdlsNpadAssignment`.
    ///
    /// # Arguments
    /// - `session_id` (in): [13.0.0+] `HiddbgHdlsSessionId`
    /// - `state` (out): `HiddbgHdlsNpadAssignment`
    ///
    /// **Note:** Only available with [7.0.0+].
    pub fn hiddbgDumpHdlsNpadAssignmentState(session_id: HiddbgHdlsSessionId, state: *mut HiddbgHdlsNpadAssignment) -> Result;

    /// Gets state for `HiddbgHdlsStateList`.
    ///
    /// # Arguments
    /// - `session_id` (in): [13.0.0+] `HiddbgHdlsSessionId`
    /// - `state` (out): `HiddbgHdlsStateList`
    ///
    /// **Note:** Only available with [7.0.0+].
    pub fn hiddbgDumpHdlsStates(session_id: HiddbgHdlsSessionId, state: *mut HiddbgHdlsStateList) -> Result;

    /// Sets state for `HiddbgHdlsNpadAssignment`.
    ///
    /// # Arguments
    /// - `session_id` (in): [13.0.0+] `HiddbgHdlsSessionId`
    /// - `state` (in): `HiddbgHdlsNpadAssignment`
    /// - `flag` (in): Flag
    ///
    /// **Note:** Only available with [7.0.0+].
    pub fn hiddbgApplyHdlsNpadAssignmentState(session_id: HiddbgHdlsSessionId, state: *const HiddbgHdlsNpadAssignment, flag: bool) -> Result;

    /// Sets state for `HiddbgHdlsStateList`.
    ///
    /// # Arguments
    /// - `session_id` (in): [13.0.0+] `HiddbgHdlsSessionId`
    /// - `state` (in): `HiddbgHdlsStateList`
    ///
    /// **Note:** The `HiddbgHdlsState` will be applied for each `HiddbgHdlsHandle`. If a `HiddbgHdlsHandle` is not found, code similar to `hiddbgAttachHdlsVirtualDevice` will run with the `HiddbgHdlsDeviceInfo`, then it will continue with applying state with the new device.
    ///
    /// **Note:** Only available with [7.0.0+].
    pub fn hiddbgApplyHdlsStateList(session_id: HiddbgHdlsSessionId, state: *const HiddbgHdlsStateList) -> Result;

    /// Attach a device with the input info.
    ///
    /// # Arguments
    /// - `handle` (out): `HiddbgHdlsHandle`
    /// - `info` (in): `HiddbgHdlsDeviceInfo`
    ///
    /// **Note:** Only available with [7.0.0+].
    pub fn hiddbgAttachHdlsVirtualDevice(handle: *mut HiddbgHdlsHandle, info: *const HiddbgHdlsDeviceInfo) -> Result;

    /// Detach the specified device.
    ///
    /// # Arguments
    /// - `handle` (in): `HiddbgHdlsHandle`
    ///
    /// **Note:** Only available with [7.0.0+].
    pub fn hiddbgDetachHdlsVirtualDevice(handle: HiddbgHdlsHandle) -> Result;

    /// Sets state for the specified device.
    ///
    /// # Arguments
    /// - `handle` (in): `HiddbgHdlsHandle`
    /// - `state` (in): `HiddbgHdlsState`
    ///
    /// **Note:** Only available with [7.0.0+].
    pub fn hiddbgSetHdlsState(handle: HiddbgHdlsHandle, state: *const HiddbgHdlsState) -> Result;
}
