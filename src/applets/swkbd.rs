//! `switch/applets/swkbd.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Output result returned by `SwkbdTextCheckCb`.
pub type SwkbdTextCheckResult = u32;
/// Success, valid string.
pub const SwkbdTextCheckResult_OK: SwkbdTextCheckResult = 0;
/// Failure, invalid string. Error message is displayed in a message-box, pressing OK will return to swkbd again.
pub const SwkbdTextCheckResult_Bad: SwkbdTextCheckResult = 1;
/// Failure, invalid string. Error message is displayed in a message-box, pressing Cancel will return to swkbd again, while pressing OK will continue as if the text was valid.
pub const SwkbdTextCheckResult_Prompt: SwkbdTextCheckResult = 2;
/// Failure, invalid string. With value 3 and above, swkbd will silently not accept the string, without displaying any error.
pub const SwkbdTextCheckResult_Silent: SwkbdTextCheckResult = 3;

/// Type of keyboard.
pub type SwkbdType = u32;
/// Normal keyboard.
pub const SwkbdType_Normal: SwkbdType = 0;
/// Number pad. The buttons at the bottom left/right are only available when they're set by `swkbdConfigSetLeftOptionalSymbolKey` / `swkbdConfigSetRightOptionalSymbolKey`.
pub const SwkbdType_NumPad: SwkbdType = 1;
/// QWERTY (and variants) keyboard only.
pub const SwkbdType_QWERTY: SwkbdType = 2;
/// The same as SwkbdType_Normal keyboard.
pub const SwkbdType_Unknown3: SwkbdType = 3;
/// All Latin like languages keyboard only (without CJK keyboard).
pub const SwkbdType_Latin: SwkbdType = 4;
/// Chinese Simplified keyboard only.
pub const SwkbdType_ZhHans: SwkbdType = 5;
/// Chinese Traditional keyboard only.
pub const SwkbdType_ZhHant: SwkbdType = 6;
/// Korean keyboard only.
pub const SwkbdType_Korean: SwkbdType = 7;
/// All language keyboards.
pub const SwkbdType_All: SwkbdType = 8;
/// Unknown
pub const SwkbdType_Unknown9: SwkbdType = 9;

/// Bitmask for SwkbdArgCommon::keySetDisableBitmask. This disables keys on the keyboard when the corresponding bit(s) are set.
/// Disable space-bar.
pub const SwkbdKeyDisableBitmask_Space: u32 = BIT(1);
/// Disable '@'.
pub const SwkbdKeyDisableBitmask_At: u32 = BIT(2);
/// Disable '%'.
pub const SwkbdKeyDisableBitmask_Percent: u32 = BIT(3);
/// Disable '/'.
pub const SwkbdKeyDisableBitmask_ForwardSlash: u32 = BIT(4);
/// Disable '\'.
pub const SwkbdKeyDisableBitmask_Backslash: u32 = BIT(5);
/// Disable numbers.
pub const SwkbdKeyDisableBitmask_Numbers: u32 = BIT(6);
/// Used for `swkbdConfigMakePresetDownloadCode`.
pub const SwkbdKeyDisableBitmask_DownloadCode: u32 = BIT(7);
/// Used for `swkbdConfigMakePresetUserName`. Disables '@', '%', and '\'.
pub const SwkbdKeyDisableBitmask_UserName: u32 = BIT(8);

/// Value for SwkbdArgCommon::textDrawType. Only applies when stringLenMax is 1..32, otherwise swkbd will only use SwkbdTextDrawType_Box.
pub type SwkbdTextDrawType = u32;
/// The text will be displayed on a line. Also enables displaying the Header and Sub text.
pub const SwkbdTextDrawType_Line: SwkbdTextDrawType = 0;
/// The text will be displayed in a box.
pub const SwkbdTextDrawType_Box: SwkbdTextDrawType = 1;
/// Used by `swkbdConfigMakePresetDownloadCode` on [5.0.0+]. Enables using `SwkbdArgV7` unk_x3e0.
pub const SwkbdTextDrawType_DownloadCode: SwkbdTextDrawType = 2;

/// SwkbdInline Interactive input storage request ID.
pub type SwkbdRequestCommand = u32;
pub const SwkbdRequestCommand_Finalize: SwkbdRequestCommand = 0x4;
pub const SwkbdRequestCommand_SetUserWordInfo: SwkbdRequestCommand = 0x6;
pub const SwkbdRequestCommand_SetCustomizeDic: SwkbdRequestCommand = 0x7;
pub const SwkbdRequestCommand_Calc: SwkbdRequestCommand = 0xA;
pub const SwkbdRequestCommand_SetCustomizedDictionaries: SwkbdRequestCommand = 0xB;
pub const SwkbdRequestCommand_UnsetCustomizedDictionaries: SwkbdRequestCommand = 0xC;
pub const SwkbdRequestCommand_SetChangedStringV2Flag: SwkbdRequestCommand = 0xD;
pub const SwkbdRequestCommand_SetMovedCursorV2Flag: SwkbdRequestCommand = 0xE;

/// SwkbdInline Interactive output storage reply ID.
pub type SwkbdReplyType = u32;
pub const SwkbdReplyType_FinishedInitialize: SwkbdReplyType = 0x0;
pub const SwkbdReplyType_ChangedString: SwkbdReplyType = 0x2;
pub const SwkbdReplyType_MovedCursor: SwkbdReplyType = 0x3;
pub const SwkbdReplyType_MovedTab: SwkbdReplyType = 0x4;
pub const SwkbdReplyType_DecidedEnter: SwkbdReplyType = 0x5;
pub const SwkbdReplyType_DecidedCancel: SwkbdReplyType = 0x6;
pub const SwkbdReplyType_ChangedStringUtf8: SwkbdReplyType = 0x7;
pub const SwkbdReplyType_MovedCursorUtf8: SwkbdReplyType = 0x8;
pub const SwkbdReplyType_DecidedEnterUtf8: SwkbdReplyType = 0x9;
pub const SwkbdReplyType_UnsetCustomizeDic: SwkbdReplyType = 0xA;
pub const SwkbdReplyType_ReleasedUserWordInfo: SwkbdReplyType = 0xB;
pub const SwkbdReplyType_UnsetCustomizedDictionaries: SwkbdReplyType = 0xC;
pub const SwkbdReplyType_ChangedStringV2: SwkbdReplyType = 0xD;
pub const SwkbdReplyType_MovedCursorV2: SwkbdReplyType = 0xE;
pub const SwkbdReplyType_ChangedStringUtf8V2: SwkbdReplyType = 0xF;
pub const SwkbdReplyType_MovedCursorUtf8V2: SwkbdReplyType = 0x10;

/// SwkbdInline State
pub type SwkbdState = u32;
/// Default state from `swkbdInlineCreate`, before a state is set by `swkbdInlineUpdate` when a reply is received. Also indicates that the applet is no longer running.
pub const SwkbdState_Inactive: SwkbdState = 0x0;
/// Applet is initialized but hidden.
pub const SwkbdState_Initialized: SwkbdState = 0x1;
/// Applet is appearing.
pub const SwkbdState_Appearing: SwkbdState = 0x2;
/// Applet is fully shown and ready to accept text input.
pub const SwkbdState_Shown: SwkbdState = 0x3;
/// The user pressed the ok or cancel button, causing the applet to disappear.
pub const SwkbdState_Disappearing: SwkbdState = 0x4;
pub const SwkbdState_Unknown5: SwkbdState = 0x5;
pub const SwkbdState_Unknown6: SwkbdState = 0x6;

/// Value for `SwkbdInitializeArg` mode. Controls the LibAppletMode when launching the applet.
pub type SwkbdInlineMode = u32;
/// LibAppletMode_BackgroundIndirect. This is the default. The user-process must handle displaying the swkbd gfx on the screen, by loading the image with `swkbdInlineGetImage`.
pub const SwkbdInlineMode_UserDisplay: SwkbdInlineMode = 0;
/// LibAppletMode_Background. The applet will handle displaying gfx on the screen.
pub const SwkbdInlineMode_AppletDisplay: SwkbdInlineMode = 1;

/// TextCheck callback set by `swkbdConfigSetTextCheckCallback`, for validating the input string when the swkbd ok-button is pressed. This buffer contains an UTF-8 string. This callback should validate the input string, then return a `SwkbdTextCheckResult` indicating success/failure. On failure, this function must write an error message to the tmp_string buffer, which will then be displayed by swkbd.
pub type SwkbdTextCheckCb = Option<unsafe extern "C" fn(*mut c_char, usize) -> SwkbdTextCheckResult>;

/// User dictionary word.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SwkbdDictWord {
    pub unk_x0: [u8; 100],
}

/// Input data for SwkbdInline request SetCustomizeDic.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SwkbdCustomizeDicInfo {
    pub unk_x0: [u8; 112],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SwkbdCustomizedDictionarySet {
    /// 0x1000-byte aligned buffer.
    pub buffer: *mut c_void,
    /// 0x1000-byte aligned buffer size.
    pub buffer_size: u32,
    pub entries: [u64; 24],
    pub total_entries: u16,
}

/// Base swkbd arg struct.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SwkbdArgCommon {
    /// See `SwkbdType`.
    pub type_: SwkbdType,
    pub okButtonText: [u16; 9],
    pub leftButtonText: u16,
    pub rightButtonText: u16,
    /// Enables dictionary usage when non-zero (including the system dictionary).
    pub dicFlag: u8,
    pub pad_x1b: u8,
    /// See SwkbdKeyDisableBitmask_*.
    pub keySetDisableBitmask: u32,
    /// Initial cursor position in the string: 0 = start, 1 = end.
    pub initialCursorPos: u32,
    pub headerText: [u16; 65],
    pub subText: [u16; 129],
    pub guideText: [u16; 257],
    pub pad_x3aa: u16,
    /// When non-zero, specifies the max string length. When the input is too long, swkbd will stop accepting more input until text is deleted via the B button (Backspace). See also `SwkbdTextDrawType`.
    pub stringLenMax: u32,
    /// When non-zero, specifies the min string length. When the input is too short, swkbd will display an icon and disable the ok-button.
    pub stringLenMin: u32,
    /// Use password: 0 = disable, 1 = enable.
    pub passwordFlag: u32,
    /// See `SwkbdTextDrawType`.
    pub textDrawType: SwkbdTextDrawType,
    /// Controls whether the Return button is enabled, for newlines input. 0 = disabled, non-zero = enabled.
    pub returnButtonFlag: u16,
    /// When enabled with value 1, the background is blurred.
    pub blurBackground: u8,
    pub pad_x3bf: u8,
    pub initialStringOffset: u32,
    pub initialStringSize: u32,
    pub userDicOffset: u32,
    pub userDicEntries: s32,
    pub textCheckFlag: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SwkbdArgV0 {
    pub arg: SwkbdArgCommon,
    pub pad_x3d1: [u8; 7],
    /// This really doesn't belong in a struct sent to another process, but official sw does this.
    pub textCheckCb: SwkbdTextCheckCb,
}

/// Arg struct for version 0x30007+.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SwkbdArgV7 {
    pub arg: SwkbdArgV0,
    /// When set and enabled via `SwkbdTextDrawType`, controls displayed text grouping (inserts spaces, without affecting output string).
    pub textGrouping: [u32; 8],
}

/// Arg struct for version 0x6000B+.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SwkbdArgVB {
    pub arg: SwkbdArgCommon,
    pub pad_x3d1: [u8; 3],
    /// Same as SwkbdArgV7::textGrouping.
    pub textGrouping: [u32; 8],
    /// This is SwkbdCustomizedDictionarySet::entries.
    pub entries: [u64; 24],
    /// This is SwkbdCustomizedDictionarySet::total_entries.
    pub total_entries: u8,
    /// [8.0.0+]
    pub unkFlag: u8,
    pub pad_x4b6: [u8; 13],
    /// [8.0.0+]
    pub trigger: u8,
    pub pad_x4c4: [u8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SwkbdConfig {
    pub arg: SwkbdArgV7,
    pub workbuf: *mut u8,
    pub workbuf_size: usize,
    pub max_dictwords: s32,
    pub customizedDictionarySet: SwkbdCustomizedDictionarySet,
    pub unkFlag: u8,
    pub trigger: u8,
    pub version: u32,
}

/// Rect
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SwkbdRect {
    /// X
    pub x: s16,
    /// Y
    pub y: s16,
    /// Width
    pub width: s16,
    /// Height
    pub height: s16,
}

/// InitializeArg for SwkbdInline.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SwkbdInitializeArg {
    pub unk_x0: u32,
    /// See `SwkbdInlineMode`. (u8 bool)
    pub mode: u8,
    /// Only set on [5.0.0+].
    pub unk_x5: u8,
    pub pad: [u8; 2],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SwkbdAppearArg {
    /// See `SwkbdType`.
    pub type_: SwkbdType,
    pub okButtonText: [u16; 9],
    pub leftButtonText: u16,
    pub rightButtonText: u16,
    /// Enables dictionary usage when non-zero (including the system dictionary).
    pub dicFlag: u8,
    pub unk_x1b: u8,
    /// See SwkbdKeyDisableBitmask_*.
    pub keySetDisableBitmask: u32,
    /// When non-negative and non-zero, specifies the max string length. When the input is too long, swkbd will stop accepting more input until text is deleted via the B button (Backspace).
    pub stringLenMax: s32,
    /// When non-negative and non-zero, specifies the min string length. When the input is too short, swkbd will display an icon and disable the ok-button.
    pub stringLenMin: s32,
    /// Controls whether the Return button is enabled, for newlines input. 0 = disabled, non-zero = enabled.
    pub returnButtonFlag: u8,
    /// [10.0.0+] When value 1-2, `swkbdInlineAppear` / `swkbdInlineAppearEx` will set keytopAsFloating=0 and footerScalable=1.
    pub unk_x29: u8,
    pub unk_x2a: u8,
    pub unk_x2b: u8,
    /// Bitmask 0x4: unknown.
    pub flags: u32,
    pub unk_x30: u8,
    pub unk_x31: [u8; 23],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SwkbdInlineCalcArg {
    pub unk_x0: u32,
    /// Size of this struct.
    pub size: u16,
    pub unk_x6: u8,
    pub unk_x7: u8,
    pub flags: u64,
    /// Flags bitmask 0x1.
    pub initArg: SwkbdInitializeArg,
    /// Flags bitmask 0x2.
    pub volume: f32,
    /// Flags bitmask 0x10.
    pub cursorPos: s32,
    pub appearArg: SwkbdAppearArg,
    /// Flags bitmask 0x8.
    pub inputText: [u16; 506],
    /// Flags bitmask 0x20.
    pub utf8Mode: u8,
    pub unk_x45d: u8,
    /// Flags bitmask 0x8000. Only available with [5.0.0+].
    pub enableBackspace: u8,
    pub unk_x45f: [u8; 3],
    /// Flags bitmask 0x200.
    pub keytopAsFloating: u8,
    /// Flags bitmask 0x100.
    pub footerScalable: u8,
    /// Flags bitmask 0x100.
    pub alphaEnabledInInputMode: u8,
    /// Flags bitmask 0x100.
    pub inputModeFadeType: u8,
    /// Flags bitmask 0x200.
    pub disableTouch: u8,
    /// Flags bitmask 0x800.
    pub disableHardwareKeyboard: u8,
    pub unk_x468: [u8; 5],
    pub unk_x46d: u8,
    pub unk_x46e: u8,
    pub unk_x46f: u8,
    /// Flags bitmask 0x200.
    pub keytopScaleX: f32,
    /// Flags bitmask 0x200.
    pub keytopScaleY: f32,
    /// Flags bitmask 0x200.
    pub keytopTranslateX: f32,
    /// Flags bitmask 0x200.
    pub keytopTranslateY: f32,
    /// Flags bitmask 0x100.
    pub keytopBgAlpha: f32,
    /// Flags bitmask 0x100.
    pub footerBgAlpha: f32,
    /// Flags bitmask 0x200.
    pub balloonScale: f32,
    pub unk_x48c: f32,
    pub unk_x490: [u8; 12],
    /// Flags bitmask: enable=0x2000, disable=0x4000. Only available with [5.0.0+].
    pub seGroup: u8,
    /// [6.0.0+] Enables using the trigger field when set.
    pub triggerFlag: u8,
    /// [6.0.0+] Trigger
    pub trigger: u8,
    pub pad_x49f: u8,
}

/// Struct data for SwkbdInline Interactive reply storage ChangedString*, at the end following the string.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SwkbdChangedStringArg {
    /// String length in characters, without NUL-terminator.
    pub stringLen: u32,
    /// Starting cursorPos for the current dictionary word in the current text string. -1 for none.
    pub dicStartCursorPos: s32,
    /// Ending cursorPos for the current dictionary word in the current text string. -1 for none.
    pub dicEndCursorPos: s32,
    /// Cursor position.
    pub cursorPos: s32,
}

/// Struct data for SwkbdInline Interactive reply storage MovedCursor*, at the end following the string.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SwkbdMovedCursorArg {
    /// String length in characters, without NUL-terminator.
    pub stringLen: u32,
    /// Cursor position.
    pub cursorPos: s32,
}

/// Struct data for SwkbdInline Interactive reply storage MovedTab*, at the end following the string.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SwkbdMovedTabArg {
    pub unk_x0: u32,
    pub unk_x4: u32,
}

/// Struct data for SwkbdInline Interactive reply storage DecidedEnter*, at the end following the string.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SwkbdDecidedEnterArg {
    /// String length in characters, without NUL-terminator.
    pub stringLen: u32,
}

/// This callback is used by `swkbdInlineUpdate` when handling ChangedString* replies (text changed by the user or by `swkbdInlineSetInputText`).
/// str is the UTF-8 string for the current text.
pub type SwkbdChangedStringCb = Option<unsafe extern "C" fn(*const c_char, *mut SwkbdChangedStringArg)>;

/// This callback is used by `swkbdInlineUpdate` when handling ChangedString*V2 replies (text changed by the user or by `swkbdInlineSetInputText`).
/// str is the UTF-8 string for the current text.
pub type SwkbdChangedStringV2Cb = Option<unsafe extern "C" fn(*const c_char, *mut SwkbdChangedStringArg, bool)>;

/// This callback is used by `swkbdInlineUpdate` when handling MovedCursor* replies.
/// str is the UTF-8 string for the current text.
pub type SwkbdMovedCursorCb = Option<unsafe extern "C" fn(*const c_char, *mut SwkbdMovedCursorArg)>;

/// This callback is used by `swkbdInlineUpdate` when handling MovedCursor*V2 replies.
/// str is the UTF-8 string for the current text.
pub type SwkbdMovedCursorV2Cb = Option<unsafe extern "C" fn(*const c_char, *mut SwkbdMovedCursorArg, bool)>;

/// This callback is used by `swkbdInlineUpdate` when handling MovedTab* replies.
/// str is the UTF-8 string for the current text.
pub type SwkbdMovedTabCb = Option<unsafe extern "C" fn(*const c_char, *mut SwkbdMovedTabArg)>;

/// This callback is used by `swkbdInlineUpdate` when handling DecidedEnter* replies (when the final text was submitted via the button).
/// str is the UTF-8 string for the current text.
pub type SwkbdDecidedEnterCb = Option<unsafe extern "C" fn(*const c_char, *mut SwkbdDecidedEnterArg)>;

/// InlineKeyboard
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SwkbdInline {
    pub version: u32,
    pub holder: AppletHolder,
    pub calcArg: SwkbdInlineCalcArg,
    pub directionalButtonAssignFlag: bool,
    pub state: SwkbdState,
    pub dicCustomInitialized: bool,
    pub customizedDictionariesInitialized: bool,
    pub dicStorage: AppletStorage,
    pub wordInfoInitialized: bool,
    pub wordInfoStorage: AppletStorage,
    pub interactive_tmpbuf: *mut u8,
    pub interactive_tmpbuf_size: usize,
    pub interactive_strbuf: *mut c_char,
    pub interactive_strbuf_size: usize,
    pub finishedInitializeCb: VoidFn,
    pub decidedCancelCb: VoidFn,
    pub changedStringCb: SwkbdChangedStringCb,
    pub changedStringV2Cb: SwkbdChangedStringV2Cb,
    pub movedCursorCb: SwkbdMovedCursorCb,
    pub movedCursorV2Cb: SwkbdMovedCursorV2Cb,
    pub movedTabCb: SwkbdMovedTabCb,
    pub decidedEnterCb: SwkbdDecidedEnterCb,
    pub releasedUserWordInfoCb: VoidFn,
}

/// Sets SwkbdArgCommon::SwkbdType.
///
/// # Arguments
/// - `c`: SwkbdConfig struct.
/// - `type`: `SwkbdType`
#[inline(always)]
pub fn swkbdConfigSetType(c: &mut SwkbdConfig, type_: SwkbdType) {
    c.arg.arg.arg.type_ = type_;
}

/// Sets SwkbdArgCommon::dicFlag.
///
/// # Arguments
/// - `c`: SwkbdConfig struct.
/// - `flag`: Flag
#[inline(always)]
pub fn swkbdConfigSetDicFlag(c: &mut SwkbdConfig, flag: u8) {
    c.arg.arg.arg.dicFlag = flag;
}

/// Sets SwkbdArgCommon::keySetDisableBitmask.
///
/// # Arguments
/// - `c`: SwkbdConfig struct.
/// - `keySetDisableBitmask`: keySetDisableBitmask
#[inline(always)]
pub fn swkbdConfigSetKeySetDisableBitmask(c: &mut SwkbdConfig, keySetDisableBitmask: u32) {
    c.arg.arg.arg.keySetDisableBitmask = keySetDisableBitmask;
}

/// Sets SwkbdArgCommon::initialCursorPos.
///
/// # Arguments
/// - `c`: SwkbdConfig struct.
/// - `initialCursorPos`: initialCursorPos
#[inline(always)]
pub fn swkbdConfigSetInitialCursorPos(c: &mut SwkbdConfig, initialCursorPos: u32) {
    c.arg.arg.arg.initialCursorPos = initialCursorPos;
}

/// Sets SwkbdArgCommon::stringLenMax.
///
/// # Arguments
/// - `c`: SwkbdConfig struct.
/// - `stringLenMax`: stringLenMax
#[inline(always)]
pub fn swkbdConfigSetStringLenMax(c: &mut SwkbdConfig, stringLenMax: u32) {
    c.arg.arg.arg.stringLenMax = stringLenMax;
}

/// Sets SwkbdArgCommon::stringLenMin.
///
/// # Arguments
/// - `c`: SwkbdConfig struct.
/// - `stringLenMin`: stringLenMin
#[inline(always)]
pub fn swkbdConfigSetStringLenMin(c: &mut SwkbdConfig, stringLenMin: u32) {
    c.arg.arg.arg.stringLenMin = stringLenMin;
}

/// Sets SwkbdArgCommon::passwordFlag.
///
/// # Arguments
/// - `c`: SwkbdConfig struct.
/// - `flag`: Flag
#[inline(always)]
pub fn swkbdConfigSetPasswordFlag(c: &mut SwkbdConfig, flag: u32) {
    c.arg.arg.arg.passwordFlag = flag;
}

/// Sets SwkbdArgCommon::textDrawType.
///
/// # Arguments
/// - `c`: SwkbdConfig struct.
/// - `textDrawType`: `SwkbdTextDrawType`
#[inline(always)]
pub fn swkbdConfigSetTextDrawType(c: &mut SwkbdConfig, textDrawType: SwkbdTextDrawType) {
    c.arg.arg.arg.textDrawType = textDrawType;
}

/// Sets SwkbdArgCommon::returnButtonFlag.
///
/// # Arguments
/// - `c`: SwkbdConfig struct.
/// - `flag`: Flag
#[inline(always)]
pub fn swkbdConfigSetReturnButtonFlag(c: &mut SwkbdConfig, flag: u16) {
    c.arg.arg.arg.returnButtonFlag = flag;
}

/// Sets SwkbdArgCommon::blurBackground.
///
/// # Arguments
/// - `c`: SwkbdConfig struct.
/// - `blurBackground`: blurBackground
#[inline(always)]
pub fn swkbdConfigSetBlurBackground(c: &mut SwkbdConfig, blurBackground: u8) {
    c.arg.arg.arg.blurBackground = blurBackground;
}

/// Sets SwkbdArgV7::textGrouping.
///
/// # Arguments
/// - `index`: Array index.
/// - `value`: Value to write.
#[inline(always)]
pub fn swkbdConfigSetTextGrouping(c: &mut SwkbdConfig, index: u32, value: u32) {
    if let Some(slot) = c.arg.textGrouping.get_mut(index as usize) {
        *slot = value;
    }
}

/// Sets SwkbdConfig::unkFlag, default is 0. Copied to SwkbdArgVB::unkFlag with [8.0.0+].
///
/// # Arguments
/// - `flag`: Flag
#[inline(always)]
pub fn swkbdConfigSetUnkFlag(c: &mut SwkbdConfig, flag: u8) {
    c.unkFlag = flag;
}

/// Sets SwkbdConfig::trigger, default is 0. Copied to SwkbdArgVB::trigger with [8.0.0+].
///
/// # Arguments
/// - `trigger`: Trigger
#[inline(always)]
pub fn swkbdConfigSetTrigger(c: &mut SwkbdConfig, trigger: u8) {
    c.trigger = trigger;
}

/// GetWindowSize
///
/// # Arguments
/// - `width` (out): Output width.
/// - `height` (out): Output height.
#[inline(always)]
pub fn swkbdInlineGetWindowSize(width: &mut s32, height: &mut s32) {
    *width = 1280;
    *height = 720;
}

/// Sets the stringLenMax for the specified SwkbdAppearArg, which was previously initialized with `swkbdInlineMakeAppearArg`.
///
/// # Arguments
/// - `arg`: `SwkbdAppearArg`
/// - `stringLenMax`: Max string length
#[inline(always)]
pub fn swkbdInlineAppearArgSetStringLenMax(arg: &mut SwkbdAppearArg, stringLenMax: s32) {
    arg.stringLenMax = stringLenMax;
}

/// Sets the stringLenMin for the specified SwkbdAppearArg, which was previously initialized with `swkbdInlineMakeAppearArg`.
///
/// # Arguments
/// - `arg`: `SwkbdAppearArg`
/// - `stringLenMin`: Min string length
#[inline(always)]
pub fn swkbdInlineAppearArgSetStringLenMin(arg: &mut SwkbdAppearArg, stringLenMin: s32) {
    arg.stringLenMin = stringLenMin;
}

impl_zeroed_default!(SwkbdDictWord, SwkbdCustomizeDicInfo, SwkbdCustomizedDictionarySet, SwkbdArgCommon, SwkbdArgV0, SwkbdArgV7, SwkbdArgVB, SwkbdConfig, SwkbdRect, SwkbdInitializeArg, SwkbdAppearArg, SwkbdInlineCalcArg, SwkbdChangedStringArg, SwkbdMovedCursorArg, SwkbdMovedTabArg, SwkbdDecidedEnterArg, SwkbdInline);

unsafe extern "C" {
    /// Creates a SwkbdConfig struct.
    ///
    /// # Arguments
    /// - `c`: SwkbdConfig struct.
    /// - `max_dictwords`: Max `SwkbdDictWord` entries, 0 for none.
    pub fn swkbdCreate(c: *mut SwkbdConfig, max_dictwords: s32) -> Result;

    /// Closes a SwkbdConfig struct.
    ///
    /// # Arguments
    /// - `c`: SwkbdConfig struct.
    pub fn swkbdClose(c: *mut SwkbdConfig);

    /// Clears the args in the SwkbdConfig struct and initializes it with the Default Preset.
    ///
    /// # Arguments
    /// - `c`: SwkbdConfig struct.
    ///
    /// **Note:** Do not use this before `swkbdCreate`.
    ///
    /// **Note:** Uses the following: swkbdConfigSetType() with `SwkbdType_QWERTY`, swkbdConfigSetInitialCursorPos() with value 1, swkbdConfigSetReturnButtonFlag() with value 1, and swkbdConfigSetBlurBackground() with value 1. Pre-5.0.0: swkbdConfigSetTextDrawType() with `SwkbdTextDrawType_Box`.
    pub fn swkbdConfigMakePresetDefault(c: *mut SwkbdConfig);

    /// Clears the args in the SwkbdConfig struct and initializes it with the Password Preset.
    ///
    /// # Arguments
    /// - `c`: SwkbdConfig struct.
    ///
    /// **Note:** Do not use this before `swkbdCreate`.
    ///
    /// **Note:** Uses the following: swkbdConfigSetType() with `SwkbdType_QWERTY`, swkbdConfigSetInitialCursorPos() with value 1, swkbdConfigSetPasswordFlag() with value 1, and swkbdConfigSetBlurBackground() with value 1.
    pub fn swkbdConfigMakePresetPassword(c: *mut SwkbdConfig);

    /// Clears the args in the SwkbdConfig struct and initializes it with the UserName Preset.
    ///
    /// # Arguments
    /// - `c`: SwkbdConfig struct.
    ///
    /// **Note:** Do not use this before `swkbdCreate`.
    ///
    /// **Note:** Uses the following: swkbdConfigSetType() with `SwkbdType_Normal`, swkbdConfigSetKeySetDisableBitmask() with SwkbdKeyDisableBitmask_UserName, swkbdConfigSetInitialCursorPos() with value 1, and swkbdConfigSetBlurBackground() with value 1.
    pub fn swkbdConfigMakePresetUserName(c: *mut SwkbdConfig);

    /// Clears the args in the SwkbdConfig struct and initializes it with the DownloadCode Preset.
    ///
    /// # Arguments
    /// - `c`: SwkbdConfig struct.
    ///
    /// **Note:** Do not use this before `swkbdCreate`.
    ///
    /// **Note:** Uses the following: swkbdConfigSetType() with `SwkbdType_Normal` (`SwkbdType_QWERTY` on [5.0.0+]), swkbdConfigSetKeySetDisableBitmask() with SwkbdKeyDisableBitmask_DownloadCode, swkbdConfigSetInitialCursorPos() with value 1, and swkbdConfigSetBlurBackground() with value 1. [5.0.0+]: swkbdConfigSetStringLenMax() with value 16, swkbdConfigSetStringLenMin() with value 1, and swkbdConfigSetTextDrawType() with SwkbdTextDrawType_DownloadCode. Uses swkbdConfigSetTextGrouping() for [0-2] with: 0x3, 0x7, and 0xb.
    pub fn swkbdConfigMakePresetDownloadCode(c: *mut SwkbdConfig);

    /// Sets the Ok button text. The default is "".
    ///
    /// # Arguments
    /// - `c`: SwkbdConfig struct.
    /// - `str`: UTF-8 input string.
    pub fn swkbdConfigSetOkButtonText(c: *mut SwkbdConfig, str: *const c_char);

    /// Sets the LeftOptionalSymbolKey, for `SwkbdType_NumPad`. The default is "".
    ///
    /// # Arguments
    /// - `c`: SwkbdConfig struct.
    /// - `str`: UTF-8 input string.
    pub fn swkbdConfigSetLeftOptionalSymbolKey(c: *mut SwkbdConfig, str: *const c_char);

    /// Sets the RightOptionalSymbolKey, for `SwkbdType_NumPad`. The default is "".
    ///
    /// # Arguments
    /// - `c`: SwkbdConfig struct.
    /// - `str`: UTF-8 input string.
    pub fn swkbdConfigSetRightOptionalSymbolKey(c: *mut SwkbdConfig, str: *const c_char);

    /// Sets the Header text. The default is "".
    ///
    /// # Arguments
    /// - `c`: SwkbdConfig struct.
    /// - `str`: UTF-8 input string.
    ///
    /// **Note:** See SwkbdArgCommon::stringLenMax.
    pub fn swkbdConfigSetHeaderText(c: *mut SwkbdConfig, str: *const c_char);

    /// Sets the Sub text. The default is "".
    ///
    /// # Arguments
    /// - `c`: SwkbdConfig struct.
    /// - `str`: UTF-8 input string.
    ///
    /// **Note:** See SwkbdArgCommon::stringLenMax.
    pub fn swkbdConfigSetSubText(c: *mut SwkbdConfig, str: *const c_char);

    /// Sets the Guide text. The default is "".
    ///
    /// # Arguments
    /// - `c`: SwkbdConfig struct.
    /// - `str`: UTF-8 input string.
    ///
    /// **Note:** The swkbd applet only displays this when the current displayed cursor position is 0.
    pub fn swkbdConfigSetGuideText(c: *mut SwkbdConfig, str: *const c_char);

    /// Sets the Initial text. The default is "".
    ///
    /// # Arguments
    /// - `c`: SwkbdConfig struct.
    /// - `str`: UTF-8 input string.
    pub fn swkbdConfigSetInitialText(c: *mut SwkbdConfig, str: *const c_char);

    /// Sets the user dictionary.
    ///
    /// # Arguments
    /// - `c`: SwkbdConfig struct.
    /// - `input`: Input data.
    /// - `entries`: Total entries in the buffer.
    pub fn swkbdConfigSetDictionary(c: *mut SwkbdConfig, input: *const SwkbdDictWord, entries: s32);

    /// Sets the CustomizedDictionaries.
    ///
    /// # Arguments
    /// - `c`: SwkbdConfig struct.
    /// - `dic`: Input `SwkbdCustomizedDictionarySet`
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn swkbdConfigSetCustomizedDictionaries(c: *mut SwkbdConfig, dic: *const SwkbdCustomizedDictionarySet) -> Result;

    /// Sets the TextCheck callback.
    ///
    /// # Arguments
    /// - `c`: SwkbdConfig struct.
    /// - `cb`: `SwkbdTextCheckCb` callback.
    pub fn swkbdConfigSetTextCheckCallback(c: *mut SwkbdConfig, cb: SwkbdTextCheckCb);

    /// Launch swkbd with the specified config. This will return once swkbd is finished running.
    ///
    /// # Arguments
    /// - `c`: SwkbdConfig struct.
    /// - `out_string`: UTF-8 Output string buffer.
    /// - `out_string_size`: UTF-8 Output string buffer size, including NUL-terminator.
    ///
    /// **Note:** The string buffer is also used for the buffer passed to the `SwkbdTextCheckCb`, when it's set. Hence, in that case this buffer should be large enough to handle TextCheck string input/output. The size passed to the callback is the same size passed here, -1.
    pub fn swkbdShow(c: *mut SwkbdConfig, out_string: *mut c_char, out_string_size: usize) -> Result;

    /// Creates a SwkbdInline object. Only available on [2.0.0+].
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    ///
    /// **Note:** This is essentially an asynchronous version of the regular swkbd.
    ///
    /// **Note:** This calls `swkbdInlineSetUtf8Mode` internally with flag=true.
    pub fn swkbdInlineCreate(s: *mut SwkbdInline) -> Result;

    /// Closes a SwkbdInline object. If the applet is running, this will tell the applet to exit, then wait for the applet to exit + applet exit handling.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    pub fn swkbdInlineClose(s: *mut SwkbdInline) -> Result;

    /// Does setup for `SwkbdInitializeArg` and launches the applet with the SwkbdInline object.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    ///
    /// **Note:** The initArg is cleared, and on [5.0.0+] unk_x5 is set to 1.
    pub fn swkbdInlineLaunch(s: *mut SwkbdInline) -> Result;

    /// Same as `swkbdInlineLaunch`, except mode and unk_x5 for `SwkbdInitializeArg` are set to the input params.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    /// - `mode`: Value for SwkbdInitializeArg::mode.
    /// - `unk_x5`: Value for SwkbdInitializeArg::unk_x5.
    pub fn swkbdInlineLaunchForLibraryApplet(s: *mut SwkbdInline, mode: u8, unk_x5: u8) -> Result;

    /// GetImageMemoryRequirement
    ///
    /// # Arguments
    /// - `out_size` (out): Output size.
    /// - `out_alignment` (out): Output alignment.
    ///
    /// **Note:** Wrapper for `viGetIndirectLayerImageRequiredMemoryInfo`.
    pub fn swkbdInlineGetImageMemoryRequirement(out_size: *mut u64, out_alignment: *mut u64) -> Result;

    /// GetImage
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    /// - `buffer` (out): Output RGBA8 image buffer, this must use the alignment from `swkbdInlineGetImageMemoryRequirement`.
    /// - `size` (in): Output buffer size, this must match the size from `swkbdInlineGetImageMemoryRequirement`.
    /// - `data_available` (out): Whether data is available.
    ///
    /// **Note:** Only available with ::SwkbdInlineMode_UserDisplay.
    ///
    /// **Note:** For width/height, see `swkbdInlineGetWindowSize`.
    pub fn swkbdInlineGetImage(s: *mut SwkbdInline, buffer: *mut c_void, size: u64, data_available: *mut bool) -> Result;

    /// Gets the image max height, relative to the bottom of the screen.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    pub fn swkbdInlineGetMaxHeight(s: *mut SwkbdInline) -> s32;

    /// Gets the MiniaturizedHeight, relative to the bottom of the screen.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    pub fn swkbdInlineGetMiniaturizedHeight(s: *mut SwkbdInline) -> s32;

    /// GetTouchRectangles. Returns number of valid Rects: 1 for only keytop, 2 for keytop/footer.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    /// - `keytop` (out): `SwkbdRect` for keytop. Optional, can be NULL.
    /// - `footer` (out): `SwkbdRect` for footer. Optional, can be NULL.
    pub fn swkbdInlineGetTouchRectangles(s: *mut SwkbdInline, keytop: *mut SwkbdRect, footer: *mut SwkbdRect) -> s32;

    /// Gets whether the input x/y are within the output from `swkbdInlineGetTouchRectangles`.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    /// - `x` (out): X
    /// - `y` (out): Y
    pub fn swkbdInlineIsUsedTouchPointByKeyboard(s: *mut SwkbdInline, x: s32, y: s32) -> bool;

    /// Handles updating SwkbdInline state, this should be called periodically.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    /// - `out_state`: Optional output `SwkbdState`.
    ///
    /// **Note:** Handles applet exit if needed, and also sends the `SwkbdInlineCalcArg` to the applet if needed. Hence, this should be called at some point after writing to `SwkbdInlineCalcArg`.
    ///
    /// **Note:** Handles applet Interactive storage output when needed.
    pub fn swkbdInlineUpdate(s: *mut SwkbdInline, out_state: *mut SwkbdState) -> Result;

    /// Sets the FinishedInitialize callback, used by `swkbdInlineUpdate`. The default is NULL for none.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    /// - `cb`: Callback
    pub fn swkbdInlineSetFinishedInitializeCallback(s: *mut SwkbdInline, cb: VoidFn);

    /// Sets the DecidedCancel callback, used by `swkbdInlineUpdate`. The default is NULL for none.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    /// - `cb`: Callback
    pub fn swkbdInlineSetDecidedCancelCallback(s: *mut SwkbdInline, cb: VoidFn);

    /// Sets the ChangedString callback, used by `swkbdInlineUpdate`. The default is NULL for none.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    /// - `cb`: `SwkbdChangedStringCb` Callback
    ///
    /// **Note:** This clears the callback set by `swkbdInlineSetChangedStringV2Callback`.
    ///
    /// **Note:** This should be called after `swkbdInlineLaunch` / `swkbdInlineLaunchForLibraryApplet`.
    pub fn swkbdInlineSetChangedStringCallback(s: *mut SwkbdInline, cb: SwkbdChangedStringCb);

    /// Sets the ChangedStringV2 callback, used by `swkbdInlineUpdate`. The default is NULL for none.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    /// - `cb`: `SwkbdChangedStringV2Cb` Callback
    ///
    /// **Note:** Only available with [8.0.0+].
    ///
    /// **Note:** This must be called after `swkbdInlineLaunch` / `swkbdInlineLaunchForLibraryApplet`.
    pub fn swkbdInlineSetChangedStringV2Callback(s: *mut SwkbdInline, cb: SwkbdChangedStringV2Cb);

    /// Sets the MovedCursor callback, used by `swkbdInlineUpdate`. The default is NULL for none.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    /// - `cb`: `SwkbdMovedCursorCb` Callback
    ///
    /// **Note:** This clears the callback set by `swkbdInlineSetMovedCursorV2Callback`.
    ///
    /// **Note:** This should be called after `swkbdInlineLaunch` / `swkbdInlineLaunchForLibraryApplet`.
    pub fn swkbdInlineSetMovedCursorCallback(s: *mut SwkbdInline, cb: SwkbdMovedCursorCb);

    /// Sets the MovedCursorV2 callback, used by `swkbdInlineUpdate`. The default is NULL for none.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    /// - `cb`: `SwkbdMovedCursorV2Cb` Callback
    ///
    /// **Note:** Only available with [8.0.0+].
    ///
    /// **Note:** This must be called after `swkbdInlineLaunch` / `swkbdInlineLaunchForLibraryApplet`.
    pub fn swkbdInlineSetMovedCursorV2Callback(s: *mut SwkbdInline, cb: SwkbdMovedCursorV2Cb);

    /// Sets the MovedTab callback, used by `swkbdInlineUpdate`. The default is NULL for none.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    /// - `cb`: `SwkbdMovedTabCb` Callback
    pub fn swkbdInlineSetMovedTabCallback(s: *mut SwkbdInline, cb: SwkbdMovedTabCb);

    /// Sets the DecidedEnter callback, used by `swkbdInlineUpdate`. The default is NULL for none.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    /// - `cb`: `SwkbdDecidedEnterCb` Callback
    pub fn swkbdInlineSetDecidedEnterCallback(s: *mut SwkbdInline, cb: SwkbdDecidedEnterCb);

    /// Sets the ReleasedUserWordInfo callback, used by `swkbdInlineUpdate`. The default is NULL for none.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    /// - `cb`: Callback
    pub fn swkbdInlineSetReleasedUserWordInfoCallback(s: *mut SwkbdInline, cb: VoidFn);

    /// Appear the kbd and set `SwkbdAppearArg`.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    /// - `arg`: Input SwkbdAppearArg.
    ///
    /// **Note:** `swkbdInlineUpdate` must be called at some point afterwards for this to take affect.
    ///
    /// **Note:** Wrapper for `swkbdInlineAppearEx`, with trigger=0.
    pub fn swkbdInlineAppear(s: *mut SwkbdInline, arg: *const SwkbdAppearArg);

    /// Appear the kbd and set `SwkbdAppearArg`.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    /// - `arg`: Input SwkbdAppearArg.
    /// - `trigger`: Trigger, default is 0. Requires [6.0.0+], on eariler versions this will always use value 0 internally.
    ///
    /// **Note:** `swkbdInlineUpdate` must be called at some point afterwards for this to take affect.
    pub fn swkbdInlineAppearEx(s: *mut SwkbdInline, arg: *const SwkbdAppearArg, trigger: u8);

    /// Disappear the kbd.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    ///
    /// **Note:** `swkbdInlineUpdate` must be called at some point afterwards for this to take affect.
    pub fn swkbdInlineDisappear(s: *mut SwkbdInline);

    /// Creates a `SwkbdAppearArg` which can then be passed to `swkbdInlineAppear`. arg is initialized with the defaults, with type being set to the input type.
    ///
    /// # Arguments
    /// - `arg`: Output `SwkbdAppearArg`.
    /// - `type`: `SwkbdType` type
    pub fn swkbdInlineMakeAppearArg(arg: *mut SwkbdAppearArg, type_: SwkbdType);

    /// Sets okButtonText for the specified SwkbdAppearArg, which was previously initialized with `swkbdInlineMakeAppearArg`.
    ///
    /// # Arguments
    /// - `arg`: `SwkbdAppearArg`
    /// - `str`: Input UTF-8 string for the Ok button text, this can be empty/NULL to use the default.
    pub fn swkbdInlineAppearArgSetOkButtonText(arg: *mut SwkbdAppearArg, str: *const c_char);

    /// Sets the LeftButtonText, for `SwkbdType_NumPad`. The default is "". Equivalent to `swkbdConfigSetLeftOptionalSymbolKey`.
    ///
    /// # Arguments
    /// - `arg`: `SwkbdAppearArg`, previously initialized by `swkbdInlineMakeAppearArg`.
    /// - `str`: UTF-8 input string.
    pub fn swkbdInlineAppearArgSetLeftButtonText(arg: *mut SwkbdAppearArg, str: *const c_char);

    /// Sets the RightButtonText, for `SwkbdType_NumPad`. The default is "". Equivalent to `swkbdConfigSetRightOptionalSymbolKey`.
    ///
    /// # Arguments
    /// - `arg`: `SwkbdAppearArg`, previously initialized by `swkbdInlineMakeAppearArg`.
    /// - `str`: UTF-8 input string.
    pub fn swkbdInlineAppearArgSetRightButtonText(arg: *mut SwkbdAppearArg, str: *const c_char);

    /// Sets the audio volume.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    /// - `volume`: Volume
    ///
    /// **Note:** `swkbdInlineUpdate` must be called at some point afterwards for this to take affect.
    pub fn swkbdInlineSetVolume(s: *mut SwkbdInline, volume: f32);

    /// Sets the current input text string. Overrides the entire user input string if the user previously entered any text.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    /// - `str`: UTF-8 input string.
    ///
    /// **Note:** `swkbdInlineUpdate` must be called at some point afterwards for this to take affect.
    ///
    /// **Note:** This will not affect the cursor position, see `swkbdInlineSetCursorPos` for that.
    pub fn swkbdInlineSetInputText(s: *mut SwkbdInline, str: *const c_char);

    /// Sets the cursor character position in the string.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    /// - `pos`: Position
    ///
    /// **Note:** `swkbdInlineUpdate` must be called at some point afterwards for this to take affect.
    pub fn swkbdInlineSetCursorPos(s: *mut SwkbdInline, pos: s32);

    /// Sets the UserWordInfo.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    /// - `input`: Input data.
    /// - `entries`: Total entries in the buffer.
    ///
    /// **Note:** Not available when `SwkbdState` is above ::SwkbdState_Initialized. Can't be used if this was already used previously.
    ///
    /// **Note:** The specified buffer must not be used after this, until `swkbdInlineClose` is used.
    ///
    /// **Note:** `swkbdInlineUpdate` must be called at some point afterwards.
    ///
    /// **Note:** If input==NULL or total_entries==0, this will just call `swkbdInlineUnsetUserWordInfo` internally.
    pub fn swkbdInlineSetUserWordInfo(s: *mut SwkbdInline, input: *const SwkbdDictWord, entries: s32) -> Result;

    /// Request UnsetUserWordInfo.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    ///
    /// **Note:** `swkbdInlineUpdate` must be called at some point afterwards for this to take affect.
    ///
    /// **Note:** Not available when `SwkbdState` is above ::SwkbdState_Initialized.
    pub fn swkbdInlineUnsetUserWordInfo(s: *mut SwkbdInline) -> Result;

    /// Sets the utf8Mode.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    /// - `flag`: Flag
    ///
    /// **Note:** `swkbdInlineUpdate` must be called at some point afterwards for this to take affect.
    ///
    /// **Note:** Automatically used internally by `swkbdInlineCreate`.
    pub fn swkbdInlineSetUtf8Mode(s: *mut SwkbdInline, flag: bool);

    /// Sets the CustomizeDic.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    /// - `buffer`: 0x1000-byte aligned buffer.
    /// - `size`: 0x1000-byte aligned buffer size.
    /// - `info`: Input `SwkbdCustomizeDicInfo`
    ///
    /// **Note:** Not available when `SwkbdState` is above ::SwkbdState_Initialized. Can't be used if this or `swkbdInlineSetCustomizedDictionaries` was already used previously.
    ///
    /// **Note:** The specified buffer must not be used after this, until `swkbdInlineClose` is used. However, it will also become available once `swkbdInlineUpdate` handles SwkbdReplyType_UnsetCustomizeDic internally.
    pub fn swkbdInlineSetCustomizeDic(s: *mut SwkbdInline, buffer: *mut c_void, size: usize, info: *mut SwkbdCustomizeDicInfo) -> Result;

    /// Request UnsetCustomizeDic.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    ///
    /// **Note:** `swkbdInlineUpdate` must be called at some point afterwards for this to take affect.
    ///
    /// **Note:** Not available when `SwkbdState` is above ::SwkbdState_Initialized.
    pub fn swkbdInlineUnsetCustomizeDic(s: *mut SwkbdInline);

    /// Sets the CustomizedDictionaries.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    /// - `dic`: Input `SwkbdCustomizedDictionarySet`
    ///
    /// **Note:** Not available when `SwkbdState` is above ::SwkbdState_Initialized. Can't be used if this or `swkbdInlineSetCustomizeDic` was already used previously.
    ///
    /// **Note:** The specified buffer in dic must not be used after this, until `swkbdInlineClose` is used. However, it will also become available once `swkbdInlineUpdate` handles SwkbdReplyType_UnsetCustomizedDictionaries internally.
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn swkbdInlineSetCustomizedDictionaries(s: *mut SwkbdInline, dic: *const SwkbdCustomizedDictionarySet) -> Result;

    /// Request UnsetCustomizedDictionaries.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    ///
    /// **Note:** Not available when `SwkbdState` is above ::SwkbdState_Initialized.
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn swkbdInlineUnsetCustomizedDictionaries(s: *mut SwkbdInline) -> Result;

    /// Sets InputModeFadeType.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    /// - `type`: Type
    ///
    /// **Note:** `swkbdInlineUpdate` must be called at some point afterwards for this to take affect.
    pub fn swkbdInlineSetInputModeFadeType(s: *mut SwkbdInline, type_: u8);

    /// Sets AlphaEnabledInInputMode.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    /// - `flag`: Flag
    ///
    /// **Note:** `swkbdInlineUpdate` must be called at some point afterwards for this to take affect.
    pub fn swkbdInlineSetAlphaEnabledInInputMode(s: *mut SwkbdInline, flag: bool);

    /// Sets KeytopBgAlpha.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    /// - `alpha`: Alpha, clamped to range 0.0f..1.0f.
    ///
    /// **Note:** `swkbdInlineUpdate` must be called at some point afterwards for this to take affect.
    pub fn swkbdInlineSetKeytopBgAlpha(s: *mut SwkbdInline, alpha: f32);

    /// Sets FooterBgAlpha.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    /// - `alpha`: Alpha, clamped to range 0.0f..1.0f.
    ///
    /// **Note:** `swkbdInlineUpdate` must be called at some point afterwards for this to take affect.
    pub fn swkbdInlineSetFooterBgAlpha(s: *mut SwkbdInline, alpha: f32);

    /// Sets gfx scaling. Configures KeytopScale* and BalloonScale based on the input value.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    /// - `scale`: Scale
    ///
    /// **Note:** `swkbdInlineUpdate` must be called at some point afterwards for this to take affect.
    ///
    /// **Note:** The BalloonScale is not updated when `SwkbdState` is above ::SwkbdState_Initialized.
    pub fn swkbdInlineSetKeytopScale(s: *mut SwkbdInline, scale: f32);

    /// Sets gfx translation for the displayed swkbd image position.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    /// - `x`: X
    /// - `y`: Y
    ///
    /// **Note:** `swkbdInlineUpdate` must be called at some point afterwards for this to take affect.
    pub fn swkbdInlineSetKeytopTranslate(s: *mut SwkbdInline, x: f32, y: f32);

    /// Sets KeytopAsFloating.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    /// - `flag`: Flag
    ///
    /// **Note:** `swkbdInlineUpdate` must be called at some point afterwards for this to take affect.
    ///
    /// **Note:** Not available when `SwkbdState` is above ::SwkbdState_Initialized.
    pub fn swkbdInlineSetKeytopAsFloating(s: *mut SwkbdInline, flag: bool);

    /// Sets FooterScalable.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    /// - `flag`: Flag
    ///
    /// **Note:** `swkbdInlineUpdate` must be called at some point afterwards for this to take affect.
    pub fn swkbdInlineSetFooterScalable(s: *mut SwkbdInline, flag: bool);

    /// Sets whether touch is enabled. The default is enabled.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    /// - `flag`: Flag
    ///
    /// **Note:** `swkbdInlineUpdate` must be called at some point afterwards for this to take affect.
    pub fn swkbdInlineSetTouchFlag(s: *mut SwkbdInline, flag: bool);

    /// Sets whether Hardware-keyboard is enabled. The default is enabled.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    /// - `flag`: Flag
    ///
    /// **Note:** `swkbdInlineUpdate` must be called at some point afterwards for this to take affect.
    pub fn swkbdInlineSetHardwareKeyboardFlag(s: *mut SwkbdInline, flag: bool);

    /// Sets whether DirectionalButtonAssign is enabled. The default is disabled.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    /// - `flag`: Flag
    ///
    /// **Note:** `swkbdInlineUpdate` must be called at some point afterwards for this to take affect.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn swkbdInlineSetDirectionalButtonAssignFlag(s: *mut SwkbdInline, flag: bool);

    /// Sets whether the specified SeGroup (sound effect) is enabled. The default is enabled.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    /// - `seGroup`: SeGroup
    /// - `flag`: Flag
    ///
    /// **Note:** `swkbdInlineUpdate` must be called at some point afterwards for this to take affect. If called again with a different seGroup, `swkbdInlineUpdate` must be called prior to calling this again.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn swkbdInlineSetSeGroup(s: *mut SwkbdInline, seGroup: u8, flag: bool);

    /// Sets whether the backspace button is enabled. The default is enabled.
    ///
    /// # Arguments
    /// - `s`: SwkbdInline object.
    /// - `flag`: Flag
    ///
    /// **Note:** `swkbdInlineUpdate` must be called at some point afterwards for this to take affect.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn swkbdInlineSetBackspaceFlag(s: *mut SwkbdInline, flag: bool);
}
