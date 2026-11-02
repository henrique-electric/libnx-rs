// inline: padInitialize
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
// inline: padInitializeAny
/// # Safety
/// HID must be initialized.
#[inline(always)]
pub unsafe fn padInitializeAny(pad: &mut PadState) {
    unsafe { padInitializeWithMask(pad, PAD_ANY_ID_MASK) }
}
// inline: padInitializeDefault
/// # Safety
/// HID must be initialized.
#[inline(always)]
pub unsafe fn padInitializeDefault(pad: &mut PadState) {
    unsafe { padInitialize(pad, &[HidNpadIdType_No1, HidNpadIdType_Handheld]) }
}
// inline: padIsHandheld
#[inline(always)]
pub fn padIsHandheld(pad: &PadState) -> bool {
    pad.active_handheld
}
// inline: padIsNpadActive
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
// inline: padGetStyleSet
#[inline(always)]
pub fn padGetStyleSet(pad: &PadState) -> u32 {
    pad.style_set
}
// inline: padGetAttributes
#[inline(always)]
pub fn padGetAttributes(pad: &PadState) -> u32 {
    pad.attributes
}
// inline: padIsConnected
#[inline(always)]
pub fn padIsConnected(pad: &PadState) -> bool {
    (pad.attributes & HidNpadAttribute_IsConnected) != 0
}
// inline: padGetButtons
#[inline(always)]
pub fn padGetButtons(pad: &PadState) -> u64 {
    pad.buttons_cur
}
// inline: padGetButtonsDown
#[inline(always)]
pub fn padGetButtonsDown(pad: &PadState) -> u64 {
    !pad.buttons_old & pad.buttons_cur
}
// inline: padGetButtonsUp
#[inline(always)]
pub fn padGetButtonsUp(pad: &PadState) -> u64 {
    pad.buttons_old & !pad.buttons_cur
}
// inline: padGetStickPos
#[inline(always)]
pub fn padGetStickPos(pad: &PadState, i: c_uint) -> HidAnalogStickState {
    pad.sticks[i as usize]
}
// inline: padGetGcTriggerPos
#[inline(always)]
pub fn padGetGcTriggerPos(pad: &PadState, i: c_uint) -> u32 {
    pad.gc_triggers[i as usize]
}
// inline: padRepeaterInitialize
#[inline(always)]
pub fn padRepeaterInitialize(r: &mut PadRepeater, delay: u16, repeat: u16) {
    r.button_mask = 0;
    r.counter = 0;
    r.delay = delay;
    r.repeat = repeat;
}
// inline: padRepeaterGetButtons
#[inline(always)]
pub fn padRepeaterGetButtons(r: &PadRepeater) -> u64 {
    if r.counter == 0 { r.button_mask } else { 0 }
}
