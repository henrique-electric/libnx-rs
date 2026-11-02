// inline: swkbdConfigSetType
#[inline(always)]
pub fn swkbdConfigSetType(c: &mut SwkbdConfig, type_: SwkbdType) {
    c.arg.arg.arg.type_ = type_;
}
// inline: swkbdConfigSetDicFlag
#[inline(always)]
pub fn swkbdConfigSetDicFlag(c: &mut SwkbdConfig, flag: u8) {
    c.arg.arg.arg.dicFlag = flag;
}
// inline: swkbdConfigSetKeySetDisableBitmask
#[inline(always)]
pub fn swkbdConfigSetKeySetDisableBitmask(c: &mut SwkbdConfig, keySetDisableBitmask: u32) {
    c.arg.arg.arg.keySetDisableBitmask = keySetDisableBitmask;
}
// inline: swkbdConfigSetInitialCursorPos
#[inline(always)]
pub fn swkbdConfigSetInitialCursorPos(c: &mut SwkbdConfig, initialCursorPos: u32) {
    c.arg.arg.arg.initialCursorPos = initialCursorPos;
}
// inline: swkbdConfigSetStringLenMax
#[inline(always)]
pub fn swkbdConfigSetStringLenMax(c: &mut SwkbdConfig, stringLenMax: u32) {
    c.arg.arg.arg.stringLenMax = stringLenMax;
}
// inline: swkbdConfigSetStringLenMin
#[inline(always)]
pub fn swkbdConfigSetStringLenMin(c: &mut SwkbdConfig, stringLenMin: u32) {
    c.arg.arg.arg.stringLenMin = stringLenMin;
}
// inline: swkbdConfigSetPasswordFlag
#[inline(always)]
pub fn swkbdConfigSetPasswordFlag(c: &mut SwkbdConfig, flag: u32) {
    c.arg.arg.arg.passwordFlag = flag;
}
// inline: swkbdConfigSetTextDrawType
#[inline(always)]
pub fn swkbdConfigSetTextDrawType(c: &mut SwkbdConfig, textDrawType: SwkbdTextDrawType) {
    c.arg.arg.arg.textDrawType = textDrawType;
}
// inline: swkbdConfigSetReturnButtonFlag
#[inline(always)]
pub fn swkbdConfigSetReturnButtonFlag(c: &mut SwkbdConfig, flag: u16) {
    c.arg.arg.arg.returnButtonFlag = flag;
}
// inline: swkbdConfigSetBlurBackground
#[inline(always)]
pub fn swkbdConfigSetBlurBackground(c: &mut SwkbdConfig, blurBackground: u8) {
    c.arg.arg.arg.blurBackground = blurBackground;
}
// inline: swkbdConfigSetTextGrouping
#[inline(always)]
pub fn swkbdConfigSetTextGrouping(c: &mut SwkbdConfig, index: u32, value: u32) {
    if let Some(slot) = c.arg.textGrouping.get_mut(index as usize) {
        *slot = value;
    }
}
// inline: swkbdConfigSetUnkFlag
#[inline(always)]
pub fn swkbdConfigSetUnkFlag(c: &mut SwkbdConfig, flag: u8) {
    c.unkFlag = flag;
}
// inline: swkbdConfigSetTrigger
#[inline(always)]
pub fn swkbdConfigSetTrigger(c: &mut SwkbdConfig, trigger: u8) {
    c.trigger = trigger;
}
// inline: swkbdInlineGetWindowSize
#[inline(always)]
pub fn swkbdInlineGetWindowSize(width: &mut s32, height: &mut s32) {
    *width = 1280;
    *height = 720;
}
// inline: swkbdInlineAppearArgSetStringLenMax
#[inline(always)]
pub fn swkbdInlineAppearArgSetStringLenMax(arg: &mut SwkbdAppearArg, stringLenMax: s32) {
    arg.stringLenMax = stringLenMax;
}
// inline: swkbdInlineAppearArgSetStringLenMin
#[inline(always)]
pub fn swkbdInlineAppearArgSetStringLenMin(arg: &mut SwkbdAppearArg, stringLenMin: s32) {
    arg.stringLenMin = stringLenMin;
}
