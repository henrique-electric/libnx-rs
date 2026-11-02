//! `switch/runtime/pad.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Mask including all existing controller IDs.
pub const PAD_ANY_ID_MASK: u64 = 0x1000100ff;

/// Pad state object.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PadState {
    pub id_mask: u8,
    pub active_id_mask: u8,
    pub read_handheld: bool,
    pub active_handheld: bool,
    pub style_set: u32,
    pub attributes: u32,
    pub buttons_cur: u64,
    pub buttons_old: u64,
    pub sticks: [HidAnalogStickState; 2],
    pub gc_triggers: [u32; 2],
}

/// Pad button repeater state object.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PadRepeater {
    pub button_mask: u64,
    pub counter: s32,
    pub delay: u16,
    pub repeat: u16,
}

/// Same as `padInitialize`, but including every single controller input source.
///
/// # Arguments
/// - `pad` (in): Pointer to `PadState`.
///
/// **Remark:** Use this function if you want to accept input from any controller.
/// # Safety
/// HID must be initialized.
#[inline(always)]
pub unsafe fn padInitializeAny(pad: &mut PadState) {
    unsafe { padInitializeWithMask(pad, PAD_ANY_ID_MASK) }
}

/// Same as `padInitialize`, but including `HidNpadIdType_No1` and `HidNpadIdType_Handheld`.
///
/// # Arguments
/// - `pad` (in): Pointer to `PadState`.
///
/// **Remark:** Use this function if you just want to accept input for a single-player application.
/// # Safety
/// HID must be initialized.
#[inline(always)]
pub unsafe fn padInitializeDefault(pad: &mut PadState) {
    unsafe { padInitialize(pad, &[HidNpadIdType_No1, HidNpadIdType_Handheld]) }
}

/// Retrieves whether `HidNpadIdType_Handheld` is an active input source (i.e. it was possible to read from it).
///
/// # Arguments
/// - `pad` (in): Pointer to `PadState`.
///
/// **Returns:** Boolean value.
///
/// **Remark:** `padUpdate` must have been previously called.
#[inline(always)]
pub fn padIsHandheld(pad: &PadState) -> bool {
    pad.active_handheld
}

/// Retrieves whether the specified controller is an active input source (i.e. it was possible to read from it).
///
/// # Arguments
/// - `pad` (in): Pointer to `PadState`.
/// - `id` (in): ID of the controller input source (see `HidNpadIdType`)
///
/// **Returns:** Boolean value.
///
/// **Remark:** `padUpdate` must have been previously called.
#[inline(always)]
pub fn padIsNpadActive(pad: &PadState, id: HidNpadIdType) -> bool {
    if id <= HidNpadIdType_No8 {
        (pad.active_id_mask as u32 & BIT(id)) != 0
    } else if id == HidNpadIdType_Handheld {
        pad.active_handheld
    } else {
        false
    }
}

/// Retrieves the set of input styles supported by the selected controller input sources.
///
/// # Arguments
/// - `pad` (in): Pointer to `PadState`.
///
/// **Returns:** Bitfield of `HidNpadStyleTag`.
///
/// **Remark:** `padUpdate` must have been previously called.
#[inline(always)]
pub fn padGetStyleSet(pad: &PadState) -> u32 {
    pad.style_set
}

/// Retrieves the set of attributes reported by the system for the selected controller input sources.
///
/// # Arguments
/// - `pad` (in): Pointer to `PadState`.
///
/// **Returns:** Bitfield of `HidNpadAttribute`.
///
/// **Remark:** `padUpdate` must have been previously called.
#[inline(always)]
pub fn padGetAttributes(pad: &PadState) -> u32 {
    pad.attributes
}

/// Retrieves whether any of the selected controller input sources is connected.
///
/// # Arguments
/// - `pad` (in): Pointer to `PadState`.
///
/// **Returns:** Boolean value.
///
/// **Remark:** `padUpdate` must have been previously called.
#[inline(always)]
pub fn padIsConnected(pad: &PadState) -> bool {
    (pad.attributes & HidNpadAttribute_IsConnected) != 0
}

/// Retrieves the current set of pressed buttons across all selected controller input sources.
///
/// # Arguments
/// - `pad` (in): Pointer to `PadState`.
///
/// **Returns:** Bitfield of `HidNpadButton`.
///
/// **Remark:** `padUpdate` must have been previously called.
#[inline(always)]
pub fn padGetButtons(pad: &PadState) -> u64 {
    pad.buttons_cur
}

/// Retrieves the set of buttons that are newly pressed.
///
/// # Arguments
/// - `pad` (in): Pointer to `PadState`.
///
/// **Returns:** Bitfield of `HidNpadButton`.
///
/// **Remark:** `padUpdate` must have been previously called.
#[inline(always)]
pub fn padGetButtonsDown(pad: &PadState) -> u64 {
    !pad.buttons_old & pad.buttons_cur
}

/// Retrieves the set of buttons that are newly released.
///
/// # Arguments
/// - `pad` (in): Pointer to `PadState`.
///
/// **Returns:** Bitfield of `HidNpadButton`.
///
/// **Remark:** `padUpdate` must have been previously called.
#[inline(always)]
pub fn padGetButtonsUp(pad: &PadState) -> u64 {
    pad.buttons_old & !pad.buttons_cur
}

/// Retrieves the position of an analog stick in a controller.
///
/// # Arguments
/// - `pad` (in): Pointer to `PadState`.
/// - `i` (in): ID of the analog stick to read (0=left, 1=right).
///
/// **Returns:** `HidAnalogStickState`.
///
/// **Remark:** `padUpdate` must have been previously called.
#[inline(always)]
pub fn padGetStickPos(pad: &PadState, i: c_uint) -> HidAnalogStickState {
    pad.sticks[i as usize]
}

/// Retrieves the position of an analog trigger in a GameCube controller.
///
/// # Arguments
/// - `pad` (in): Pointer to `PadState`.
/// - `i` (in): ID of the analog trigger to read (0=left, 1=right).
///
/// **Returns:** Analog trigger position (range is 0 to 0x7fff).
///
/// **Remark:** `padUpdate` must have been previously called.
///
/// **Remark:** `HidNpadStyleTag_NpadGc` must have been previously configured as a supported style in `padConfigureInput` for GC trigger data to be readable.
#[inline(always)]
pub fn padGetGcTriggerPos(pad: &PadState, i: c_uint) -> u32 {
    pad.gc_triggers[i as usize]
}

/// Initializes a `PadRepeater` object with the specified settings.
///
/// # Arguments
/// - `r` (in): Pointer to `PadRepeater`.
/// - `delay` (in): Number of input updates between button presses being first detected and them being considered for repeat.
/// - `repeat` (in): Number of input updates between autogenerated repeat button presses.
#[inline(always)]
pub fn padRepeaterInitialize(r: &mut PadRepeater, delay: u16, repeat: u16) {
    r.button_mask = 0;
    r.counter = 0;
    r.delay = delay;
    r.repeat = repeat;
}

/// Retrieves the set of buttons that are being repeated according to the parameters specified in `padRepeaterInitialize`.
///
/// # Arguments
/// - `r` (in): Pointer to `PadRepeater`.
///
/// **Returns:** Bitfield of `HidNpadButton`.
///
/// **Remark:** It is suggested to bitwise-OR the return value of this function with that of `padGetButtonsDown`.
#[inline(always)]
pub fn padRepeaterGetButtons(r: &PadRepeater) -> u64 {
    if r.counter == 0 { r.button_mask } else { 0 }
}

impl_zeroed_default!(PadState, PadRepeater);

/// Initializes a [`PadState`] object to read input from one or more controller input sources.
///
/// Replaces the variadic C macro: pass the `HidNpadIdType` value of each controller to add to the set.
///
/// # Safety
/// HID must be initialized.
#[inline(always)]
pub unsafe fn padInitialize(pad: &mut PadState, ids: &[HidNpadIdType]) {
    let mut mask: u64 = 0;
    for &id in ids {
        mask |= 1u64 << id;
    }
    unsafe { padInitializeWithMask(pad, mask) }
}

unsafe extern "C" {
    /// Configures the input layout supported by the application.
    ///
    /// # Arguments
    /// - `max_players` (in): The maximum supported number of players (1 to 8).
    /// - `style_set` (in): Bitfield of supported controller styles (see `HidNpadStyleTag`).
    pub fn padConfigureInput(max_players: u32, style_set: u32);

    /// Same as `padInitialize`, but taking a bitfield of controller IDs directly.
    ///
    /// # Arguments
    /// - `pad` (in): Pointer to `PadState`.
    /// - `mask` (in): Bitfield of controller IDs (each bit's position indicates a different `HidNpadIdType` value).
    pub fn padInitializeWithMask(pad: *mut PadState, mask: u64);

    /// Updates pad state by reading from the controller input sources specified during initialization.
    ///
    /// # Arguments
    /// - `pad` (in): Pointer to `PadState`.
    pub fn padUpdate(pad: *mut PadState);

    /// Updates pad repeat state.
    ///
    /// # Arguments
    /// - `r` (in): Pointer to `PadRepeater`.
    /// - `button_mask` (in): Bitfield of currently pressed `HidNpadButton` that will be considered for repeat.
    pub fn padRepeaterUpdate(r: *mut PadRepeater, button_mask: u64);
}
