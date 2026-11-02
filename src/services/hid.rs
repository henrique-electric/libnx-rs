//! `switch/services/hid.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const JOYSTICK_MAX: u32 = 0x7fff;
pub const JOYSTICK_MIN: i32 = -0x7fff;

/// HidDebugPadButton
pub type HidDebugPadButton = u32;
/// A button
pub const HidDebugPadButton_A: HidDebugPadButton = BIT(0);
/// B button
pub const HidDebugPadButton_B: HidDebugPadButton = BIT(1);
/// X button
pub const HidDebugPadButton_X: HidDebugPadButton = BIT(2);
/// Y button
pub const HidDebugPadButton_Y: HidDebugPadButton = BIT(3);
/// L button
pub const HidDebugPadButton_L: HidDebugPadButton = BIT(4);
/// R button
pub const HidDebugPadButton_R: HidDebugPadButton = BIT(5);
/// ZL button
pub const HidDebugPadButton_ZL: HidDebugPadButton = BIT(6);
/// ZR button
pub const HidDebugPadButton_ZR: HidDebugPadButton = BIT(7);
/// Start button
pub const HidDebugPadButton_Start: HidDebugPadButton = BIT(8);
/// Select button
pub const HidDebugPadButton_Select: HidDebugPadButton = BIT(9);
/// D-Pad Left button
pub const HidDebugPadButton_Left: HidDebugPadButton = BIT(10);
/// D-Pad Up button
pub const HidDebugPadButton_Up: HidDebugPadButton = BIT(11);
/// D-Pad Right button
pub const HidDebugPadButton_Right: HidDebugPadButton = BIT(12);
/// D-Pad Down button
pub const HidDebugPadButton_Down: HidDebugPadButton = BIT(13);

/// HidTouchScreenModeForNx
pub type HidTouchScreenModeForNx = u32;
/// UseSystemSetting
pub const HidTouchScreenModeForNx_UseSystemSetting: HidTouchScreenModeForNx = 0;
/// Finger
pub const HidTouchScreenModeForNx_Finger: HidTouchScreenModeForNx = 1;
/// Heat2
pub const HidTouchScreenModeForNx_Heat2: HidTouchScreenModeForNx = 2;

/// HidMouseButton
pub type HidMouseButton = u32;
pub const HidMouseButton_Left: HidMouseButton = BIT(0);
pub const HidMouseButton_Right: HidMouseButton = BIT(1);
pub const HidMouseButton_Middle: HidMouseButton = BIT(2);
pub const HidMouseButton_Forward: HidMouseButton = BIT(3);
pub const HidMouseButton_Back: HidMouseButton = BIT(4);

/// HidKeyboardKey
pub type HidKeyboardKey = u32;
pub const HidKeyboardKey_A: HidKeyboardKey = 4;
pub const HidKeyboardKey_B: HidKeyboardKey = 5;
pub const HidKeyboardKey_C: HidKeyboardKey = 6;
pub const HidKeyboardKey_D: HidKeyboardKey = 7;
pub const HidKeyboardKey_E: HidKeyboardKey = 8;
pub const HidKeyboardKey_F: HidKeyboardKey = 9;
pub const HidKeyboardKey_G: HidKeyboardKey = 10;
pub const HidKeyboardKey_H: HidKeyboardKey = 11;
pub const HidKeyboardKey_I: HidKeyboardKey = 12;
pub const HidKeyboardKey_J: HidKeyboardKey = 13;
pub const HidKeyboardKey_K: HidKeyboardKey = 14;
pub const HidKeyboardKey_L: HidKeyboardKey = 15;
pub const HidKeyboardKey_M: HidKeyboardKey = 16;
pub const HidKeyboardKey_N: HidKeyboardKey = 17;
pub const HidKeyboardKey_O: HidKeyboardKey = 18;
pub const HidKeyboardKey_P: HidKeyboardKey = 19;
pub const HidKeyboardKey_Q: HidKeyboardKey = 20;
pub const HidKeyboardKey_R: HidKeyboardKey = 21;
pub const HidKeyboardKey_S: HidKeyboardKey = 22;
pub const HidKeyboardKey_T: HidKeyboardKey = 23;
pub const HidKeyboardKey_U: HidKeyboardKey = 24;
pub const HidKeyboardKey_V: HidKeyboardKey = 25;
pub const HidKeyboardKey_W: HidKeyboardKey = 26;
pub const HidKeyboardKey_X: HidKeyboardKey = 27;
pub const HidKeyboardKey_Y: HidKeyboardKey = 28;
pub const HidKeyboardKey_Z: HidKeyboardKey = 29;
pub const HidKeyboardKey_D1: HidKeyboardKey = 30;
pub const HidKeyboardKey_D2: HidKeyboardKey = 31;
pub const HidKeyboardKey_D3: HidKeyboardKey = 32;
pub const HidKeyboardKey_D4: HidKeyboardKey = 33;
pub const HidKeyboardKey_D5: HidKeyboardKey = 34;
pub const HidKeyboardKey_D6: HidKeyboardKey = 35;
pub const HidKeyboardKey_D7: HidKeyboardKey = 36;
pub const HidKeyboardKey_D8: HidKeyboardKey = 37;
pub const HidKeyboardKey_D9: HidKeyboardKey = 38;
pub const HidKeyboardKey_D0: HidKeyboardKey = 39;
pub const HidKeyboardKey_Return: HidKeyboardKey = 40;
pub const HidKeyboardKey_Escape: HidKeyboardKey = 41;
pub const HidKeyboardKey_Backspace: HidKeyboardKey = 42;
pub const HidKeyboardKey_Tab: HidKeyboardKey = 43;
pub const HidKeyboardKey_Space: HidKeyboardKey = 44;
pub const HidKeyboardKey_Minus: HidKeyboardKey = 45;
pub const HidKeyboardKey_Plus: HidKeyboardKey = 46;
pub const HidKeyboardKey_OpenBracket: HidKeyboardKey = 47;
pub const HidKeyboardKey_CloseBracket: HidKeyboardKey = 48;
pub const HidKeyboardKey_Pipe: HidKeyboardKey = 49;
pub const HidKeyboardKey_Tilde: HidKeyboardKey = 50;
pub const HidKeyboardKey_Semicolon: HidKeyboardKey = 51;
pub const HidKeyboardKey_Quote: HidKeyboardKey = 52;
pub const HidKeyboardKey_Backquote: HidKeyboardKey = 53;
pub const HidKeyboardKey_Comma: HidKeyboardKey = 54;
pub const HidKeyboardKey_Period: HidKeyboardKey = 55;
pub const HidKeyboardKey_Slash: HidKeyboardKey = 56;
pub const HidKeyboardKey_CapsLock: HidKeyboardKey = 57;
pub const HidKeyboardKey_F1: HidKeyboardKey = 58;
pub const HidKeyboardKey_F2: HidKeyboardKey = 59;
pub const HidKeyboardKey_F3: HidKeyboardKey = 60;
pub const HidKeyboardKey_F4: HidKeyboardKey = 61;
pub const HidKeyboardKey_F5: HidKeyboardKey = 62;
pub const HidKeyboardKey_F6: HidKeyboardKey = 63;
pub const HidKeyboardKey_F7: HidKeyboardKey = 64;
pub const HidKeyboardKey_F8: HidKeyboardKey = 65;
pub const HidKeyboardKey_F9: HidKeyboardKey = 66;
pub const HidKeyboardKey_F10: HidKeyboardKey = 67;
pub const HidKeyboardKey_F11: HidKeyboardKey = 68;
pub const HidKeyboardKey_F12: HidKeyboardKey = 69;
pub const HidKeyboardKey_PrintScreen: HidKeyboardKey = 70;
pub const HidKeyboardKey_ScrollLock: HidKeyboardKey = 71;
pub const HidKeyboardKey_Pause: HidKeyboardKey = 72;
pub const HidKeyboardKey_Insert: HidKeyboardKey = 73;
pub const HidKeyboardKey_Home: HidKeyboardKey = 74;
pub const HidKeyboardKey_PageUp: HidKeyboardKey = 75;
pub const HidKeyboardKey_Delete: HidKeyboardKey = 76;
pub const HidKeyboardKey_End: HidKeyboardKey = 77;
pub const HidKeyboardKey_PageDown: HidKeyboardKey = 78;
pub const HidKeyboardKey_RightArrow: HidKeyboardKey = 79;
pub const HidKeyboardKey_LeftArrow: HidKeyboardKey = 80;
pub const HidKeyboardKey_DownArrow: HidKeyboardKey = 81;
pub const HidKeyboardKey_UpArrow: HidKeyboardKey = 82;
pub const HidKeyboardKey_NumLock: HidKeyboardKey = 83;
pub const HidKeyboardKey_NumPadDivide: HidKeyboardKey = 84;
pub const HidKeyboardKey_NumPadMultiply: HidKeyboardKey = 85;
pub const HidKeyboardKey_NumPadSubtract: HidKeyboardKey = 86;
pub const HidKeyboardKey_NumPadAdd: HidKeyboardKey = 87;
pub const HidKeyboardKey_NumPadEnter: HidKeyboardKey = 88;
pub const HidKeyboardKey_NumPad1: HidKeyboardKey = 89;
pub const HidKeyboardKey_NumPad2: HidKeyboardKey = 90;
pub const HidKeyboardKey_NumPad3: HidKeyboardKey = 91;
pub const HidKeyboardKey_NumPad4: HidKeyboardKey = 92;
pub const HidKeyboardKey_NumPad5: HidKeyboardKey = 93;
pub const HidKeyboardKey_NumPad6: HidKeyboardKey = 94;
pub const HidKeyboardKey_NumPad7: HidKeyboardKey = 95;
pub const HidKeyboardKey_NumPad8: HidKeyboardKey = 96;
pub const HidKeyboardKey_NumPad9: HidKeyboardKey = 97;
pub const HidKeyboardKey_NumPad0: HidKeyboardKey = 98;
pub const HidKeyboardKey_NumPadDot: HidKeyboardKey = 99;
pub const HidKeyboardKey_Backslash: HidKeyboardKey = 100;
pub const HidKeyboardKey_Application: HidKeyboardKey = 101;
pub const HidKeyboardKey_Power: HidKeyboardKey = 102;
pub const HidKeyboardKey_NumPadEquals: HidKeyboardKey = 103;
pub const HidKeyboardKey_F13: HidKeyboardKey = 104;
pub const HidKeyboardKey_F14: HidKeyboardKey = 105;
pub const HidKeyboardKey_F15: HidKeyboardKey = 106;
pub const HidKeyboardKey_F16: HidKeyboardKey = 107;
pub const HidKeyboardKey_F17: HidKeyboardKey = 108;
pub const HidKeyboardKey_F18: HidKeyboardKey = 109;
pub const HidKeyboardKey_F19: HidKeyboardKey = 110;
pub const HidKeyboardKey_F20: HidKeyboardKey = 111;
pub const HidKeyboardKey_F21: HidKeyboardKey = 112;
pub const HidKeyboardKey_F22: HidKeyboardKey = 113;
pub const HidKeyboardKey_F23: HidKeyboardKey = 114;
pub const HidKeyboardKey_F24: HidKeyboardKey = 115;
pub const HidKeyboardKey_NumPadComma: HidKeyboardKey = 133;
pub const HidKeyboardKey_Ro: HidKeyboardKey = 135;
pub const HidKeyboardKey_KatakanaHiragana: HidKeyboardKey = 136;
pub const HidKeyboardKey_Yen: HidKeyboardKey = 137;
pub const HidKeyboardKey_Henkan: HidKeyboardKey = 138;
pub const HidKeyboardKey_Muhenkan: HidKeyboardKey = 139;
pub const HidKeyboardKey_NumPadCommaPc98: HidKeyboardKey = 140;
pub const HidKeyboardKey_HangulEnglish: HidKeyboardKey = 144;
pub const HidKeyboardKey_Hanja: HidKeyboardKey = 145;
pub const HidKeyboardKey_Katakana: HidKeyboardKey = 146;
pub const HidKeyboardKey_Hiragana: HidKeyboardKey = 147;
pub const HidKeyboardKey_ZenkakuHankaku: HidKeyboardKey = 148;
pub const HidKeyboardKey_LeftControl: HidKeyboardKey = 224;
pub const HidKeyboardKey_LeftShift: HidKeyboardKey = 225;
pub const HidKeyboardKey_LeftAlt: HidKeyboardKey = 226;
pub const HidKeyboardKey_LeftGui: HidKeyboardKey = 227;
pub const HidKeyboardKey_RightControl: HidKeyboardKey = 228;
pub const HidKeyboardKey_RightShift: HidKeyboardKey = 229;
pub const HidKeyboardKey_RightAlt: HidKeyboardKey = 230;
pub const HidKeyboardKey_RightGui: HidKeyboardKey = 231;

/// HidKeyboardModifier
pub type HidKeyboardModifier = u32;
pub const HidKeyboardModifier_Control: HidKeyboardModifier = BIT(0);
pub const HidKeyboardModifier_Shift: HidKeyboardModifier = BIT(1);
pub const HidKeyboardModifier_LeftAlt: HidKeyboardModifier = BIT(2);
pub const HidKeyboardModifier_RightAlt: HidKeyboardModifier = BIT(3);
pub const HidKeyboardModifier_Gui: HidKeyboardModifier = BIT(4);
pub const HidKeyboardModifier_CapsLock: HidKeyboardModifier = BIT(8);
pub const HidKeyboardModifier_ScrollLock: HidKeyboardModifier = BIT(9);
pub const HidKeyboardModifier_NumLock: HidKeyboardModifier = BIT(10);
pub const HidKeyboardModifier_Katakana: HidKeyboardModifier = BIT(11);
pub const HidKeyboardModifier_Hiragana: HidKeyboardModifier = BIT(12);

/// KeyboardLockKeyEvent
pub type HidKeyboardLockKeyEvent = u32;
/// NumLockOn
pub const HidKeyboardLockKeyEvent_NumLockOn: HidKeyboardLockKeyEvent = BIT(0);
/// NumLockOff
pub const HidKeyboardLockKeyEvent_NumLockOff: HidKeyboardLockKeyEvent = BIT(1);
/// NumLockToggle
pub const HidKeyboardLockKeyEvent_NumLockToggle: HidKeyboardLockKeyEvent = BIT(2);
/// CapsLockOn
pub const HidKeyboardLockKeyEvent_CapsLockOn: HidKeyboardLockKeyEvent = BIT(3);
/// CapsLockOff
pub const HidKeyboardLockKeyEvent_CapsLockOff: HidKeyboardLockKeyEvent = BIT(4);
/// CapsLockToggle
pub const HidKeyboardLockKeyEvent_CapsLockToggle: HidKeyboardLockKeyEvent = BIT(5);
/// ScrollLockOn
pub const HidKeyboardLockKeyEvent_ScrollLockOn: HidKeyboardLockKeyEvent = BIT(6);
/// ScrollLockOff
pub const HidKeyboardLockKeyEvent_ScrollLockOff: HidKeyboardLockKeyEvent = BIT(7);
/// ScrollLockToggle
pub const HidKeyboardLockKeyEvent_ScrollLockToggle: HidKeyboardLockKeyEvent = BIT(8);

/// HID controller IDs
pub type HidNpadIdType = u32;
/// Player 1 controller
pub const HidNpadIdType_No1: HidNpadIdType = 0;
/// Player 2 controller
pub const HidNpadIdType_No2: HidNpadIdType = 1;
/// Player 3 controller
pub const HidNpadIdType_No3: HidNpadIdType = 2;
/// Player 4 controller
pub const HidNpadIdType_No4: HidNpadIdType = 3;
/// Player 5 controller
pub const HidNpadIdType_No5: HidNpadIdType = 4;
/// Player 6 controller
pub const HidNpadIdType_No6: HidNpadIdType = 5;
/// Player 7 controller
pub const HidNpadIdType_No7: HidNpadIdType = 6;
/// Player 8 controller
pub const HidNpadIdType_No8: HidNpadIdType = 7;
/// Other controller
pub const HidNpadIdType_Other: HidNpadIdType = 0x10;
/// Handheld mode controls
pub const HidNpadIdType_Handheld: HidNpadIdType = 0x20;

/// HID controller styles
pub type HidNpadStyleTag = u32;
/// Pro Controller
pub const HidNpadStyleTag_NpadFullKey: HidNpadStyleTag = BIT(0);
/// Joy-Con controller in handheld mode
pub const HidNpadStyleTag_NpadHandheld: HidNpadStyleTag = BIT(1);
/// Joy-Con controller in dual mode
pub const HidNpadStyleTag_NpadJoyDual: HidNpadStyleTag = BIT(2);
/// Joy-Con left controller in single mode
pub const HidNpadStyleTag_NpadJoyLeft: HidNpadStyleTag = BIT(3);
/// Joy-Con right controller in single mode
pub const HidNpadStyleTag_NpadJoyRight: HidNpadStyleTag = BIT(4);
/// GameCube controller
pub const HidNpadStyleTag_NpadGc: HidNpadStyleTag = BIT(5);
/// Poké Ball Plus controller
pub const HidNpadStyleTag_NpadPalma: HidNpadStyleTag = BIT(6);
/// NES/Famicom controller
pub const HidNpadStyleTag_NpadLark: HidNpadStyleTag = BIT(7);
/// NES/Famicom controller in handheld mode
pub const HidNpadStyleTag_NpadHandheldLark: HidNpadStyleTag = BIT(8);
/// SNES controller
pub const HidNpadStyleTag_NpadLucia: HidNpadStyleTag = BIT(9);
/// N64 controller
pub const HidNpadStyleTag_NpadLagon: HidNpadStyleTag = BIT(10);
/// Sega Genesis controller
pub const HidNpadStyleTag_NpadLager: HidNpadStyleTag = BIT(11);
/// Generic external controller
pub const HidNpadStyleTag_NpadSystemExt: HidNpadStyleTag = BIT(29);
/// Generic controller
pub const HidNpadStyleTag_NpadSystem: HidNpadStyleTag = BIT(30);
/// Style set comprising Npad styles containing the full set of controls {FullKey, Handheld, JoyDual}
pub const HidNpadStyleSet_NpadFullCtrl: HidNpadStyleTag = HidNpadStyleTag_NpadFullKey  | HidNpadStyleTag_NpadHandheld | HidNpadStyleTag_NpadJoyDual;
/// Style set comprising all standard Npad styles {FullKey, Handheld, JoyDual, JoyLeft, JoyRight}
pub const HidNpadStyleSet_NpadStandard: HidNpadStyleTag = HidNpadStyleSet_NpadFullCtrl | HidNpadStyleTag_NpadJoyLeft  | HidNpadStyleTag_NpadJoyRight;

/// HidColorAttribute
pub type HidColorAttribute = u32;
/// Ok
pub const HidColorAttribute_Ok: HidColorAttribute = 0;
/// ReadError
pub const HidColorAttribute_ReadError: HidColorAttribute = 1;
/// NoController
pub const HidColorAttribute_NoController: HidColorAttribute = 2;

/// HidNpadButton
pub type HidNpadButton = u64;
/// A button / Right face button
pub const HidNpadButton_A: HidNpadButton = BITL(0);
/// B button / Down face button
pub const HidNpadButton_B: HidNpadButton = BITL(1);
/// X button / Up face button
pub const HidNpadButton_X: HidNpadButton = BITL(2);
/// Y button / Left face button
pub const HidNpadButton_Y: HidNpadButton = BITL(3);
/// Left Stick button
pub const HidNpadButton_StickL: HidNpadButton = BITL(4);
/// Right Stick button
pub const HidNpadButton_StickR: HidNpadButton = BITL(5);
/// L button
pub const HidNpadButton_L: HidNpadButton = BITL(6);
/// R button
pub const HidNpadButton_R: HidNpadButton = BITL(7);
/// ZL button
pub const HidNpadButton_ZL: HidNpadButton = BITL(8);
/// ZR button
pub const HidNpadButton_ZR: HidNpadButton = BITL(9);
/// Plus button
pub const HidNpadButton_Plus: HidNpadButton = BITL(10);
/// Minus button
pub const HidNpadButton_Minus: HidNpadButton = BITL(11);
/// D-Pad Left button
pub const HidNpadButton_Left: HidNpadButton = BITL(12);
/// D-Pad Up button
pub const HidNpadButton_Up: HidNpadButton = BITL(13);
/// D-Pad Right button
pub const HidNpadButton_Right: HidNpadButton = BITL(14);
/// D-Pad Down button
pub const HidNpadButton_Down: HidNpadButton = BITL(15);
/// Left Stick pseudo-button when moved Left
pub const HidNpadButton_StickLLeft: HidNpadButton = BITL(16);
/// Left Stick pseudo-button when moved Up
pub const HidNpadButton_StickLUp: HidNpadButton = BITL(17);
/// Left Stick pseudo-button when moved Right
pub const HidNpadButton_StickLRight: HidNpadButton = BITL(18);
/// Left Stick pseudo-button when moved Down
pub const HidNpadButton_StickLDown: HidNpadButton = BITL(19);
/// Right Stick pseudo-button when moved Left
pub const HidNpadButton_StickRLeft: HidNpadButton = BITL(20);
/// Right Stick pseudo-button when moved Up
pub const HidNpadButton_StickRUp: HidNpadButton = BITL(21);
/// Right Stick pseudo-button when moved Right
pub const HidNpadButton_StickRRight: HidNpadButton = BITL(22);
/// Right Stick pseudo-button when moved Left
pub const HidNpadButton_StickRDown: HidNpadButton = BITL(23);
/// SL button on Left Joy-Con
pub const HidNpadButton_LeftSL: HidNpadButton = BITL(24);
/// SR button on Left Joy-Con
pub const HidNpadButton_LeftSR: HidNpadButton = BITL(25);
/// SL button on Right Joy-Con
pub const HidNpadButton_RightSL: HidNpadButton = BITL(26);
/// SR button on Right Joy-Con
pub const HidNpadButton_RightSR: HidNpadButton = BITL(27);
/// Top button on Poké Ball Plus (Palma) controller
pub const HidNpadButton_Palma: HidNpadButton = BITL(28);
/// Verification
pub const HidNpadButton_Verification: HidNpadButton = BITL(29);
/// B button on Left NES/HVC controller in Handheld mode
pub const HidNpadButton_HandheldLeftB: HidNpadButton = BITL(30);
/// Left C button in N64 controller
pub const HidNpadButton_LagonCLeft: HidNpadButton = BITL(31);
/// Up C button in N64 controller
pub const HidNpadButton_LagonCUp: HidNpadButton = BITL(32);
/// Right C button in N64 controller
pub const HidNpadButton_LagonCRight: HidNpadButton = BITL(33);
/// Down C button in N64 controller
pub const HidNpadButton_LagonCDown: HidNpadButton = BITL(34);
/// Bitmask containing all buttons that are considered Left (D-Pad, Sticks)
pub const HidNpadButton_AnyLeft: HidNpadButton = HidNpadButton_Left   | HidNpadButton_StickLLeft  | HidNpadButton_StickRLeft;
/// Bitmask containing all buttons that are considered Up (D-Pad, Sticks)
pub const HidNpadButton_AnyUp: HidNpadButton = HidNpadButton_Up     | HidNpadButton_StickLUp    | HidNpadButton_StickRUp;
/// Bitmask containing all buttons that are considered Right (D-Pad, Sticks)
pub const HidNpadButton_AnyRight: HidNpadButton = HidNpadButton_Right  | HidNpadButton_StickLRight | HidNpadButton_StickRRight;
/// Bitmask containing all buttons that are considered Down (D-Pad, Sticks)
pub const HidNpadButton_AnyDown: HidNpadButton = HidNpadButton_Down   | HidNpadButton_StickLDown  | HidNpadButton_StickRDown;
/// Bitmask containing SL buttons on both Joy-Cons (Left/Right)
pub const HidNpadButton_AnySL: HidNpadButton = HidNpadButton_LeftSL | HidNpadButton_RightSL;
/// Bitmask containing SR buttons on both Joy-Cons (Left/Right)
pub const HidNpadButton_AnySR: HidNpadButton = HidNpadButton_LeftSR | HidNpadButton_RightSR;

/// HidDebugPadAttribute
pub type HidDebugPadAttribute = u32;
/// IsConnected
pub const HidDebugPadAttribute_IsConnected: HidDebugPadAttribute = BIT(0);

/// HidTouchAttribute
pub type HidTouchAttribute = u32;
/// Start
pub const HidTouchAttribute_Start: HidTouchAttribute = BIT(0);
/// End
pub const HidTouchAttribute_End: HidTouchAttribute = BIT(1);

/// HidMouseAttribute
pub type HidMouseAttribute = u32;
/// Transferable
pub const HidMouseAttribute_Transferable: HidMouseAttribute = BIT(0);
/// IsConnected
pub const HidMouseAttribute_IsConnected: HidMouseAttribute = BIT(1);

/// HidNpadAttribute
pub type HidNpadAttribute = u32;
/// IsConnected
pub const HidNpadAttribute_IsConnected: HidNpadAttribute = BIT(0);
/// IsWired
pub const HidNpadAttribute_IsWired: HidNpadAttribute = BIT(1);
/// IsLeftConnected
pub const HidNpadAttribute_IsLeftConnected: HidNpadAttribute = BIT(2);
/// IsLeftWired
pub const HidNpadAttribute_IsLeftWired: HidNpadAttribute = BIT(3);
/// IsRightConnected
pub const HidNpadAttribute_IsRightConnected: HidNpadAttribute = BIT(4);
/// IsRightWired
pub const HidNpadAttribute_IsRightWired: HidNpadAttribute = BIT(5);

/// HidSixAxisSensorAttribute
pub type HidSixAxisSensorAttribute = u32;
/// IsConnected
pub const HidSixAxisSensorAttribute_IsConnected: HidSixAxisSensorAttribute = BIT(0);
/// IsInterpolated
pub const HidSixAxisSensorAttribute_IsInterpolated: HidSixAxisSensorAttribute = BIT(1);

/// HidGestureAttribute
pub type HidGestureAttribute = u32;
/// IsNewTouch
pub const HidGestureAttribute_IsNewTouch: HidGestureAttribute = BIT(4);
/// IsDoubleTap
pub const HidGestureAttribute_IsDoubleTap: HidGestureAttribute = BIT(8);

/// HidGestureDirection
pub type HidGestureDirection = u32;
/// None
pub const HidGestureDirection_None: HidGestureDirection = 0;
/// Left
pub const HidGestureDirection_Left: HidGestureDirection = 1;
/// Up
pub const HidGestureDirection_Up: HidGestureDirection = 2;
/// Right
pub const HidGestureDirection_Right: HidGestureDirection = 3;
/// Down
pub const HidGestureDirection_Down: HidGestureDirection = 4;

/// HidGestureType
pub type HidGestureType = u32;
/// Idle
pub const HidGestureType_Idle: HidGestureType = 0;
/// Complete
pub const HidGestureType_Complete: HidGestureType = 1;
/// Cancel
pub const HidGestureType_Cancel: HidGestureType = 2;
/// Touch
pub const HidGestureType_Touch: HidGestureType = 3;
/// Press
pub const HidGestureType_Press: HidGestureType = 4;
/// Tap
pub const HidGestureType_Tap: HidGestureType = 5;
/// Pan
pub const HidGestureType_Pan: HidGestureType = 6;
/// Swipe
pub const HidGestureType_Swipe: HidGestureType = 7;
/// Pinch
pub const HidGestureType_Pinch: HidGestureType = 8;
/// Rotate
pub const HidGestureType_Rotate: HidGestureType = 9;

/// GyroscopeZeroDriftMode
pub type HidGyroscopeZeroDriftMode = u32;
/// Loose
pub const HidGyroscopeZeroDriftMode_Loose: HidGyroscopeZeroDriftMode = 0;
/// Standard
pub const HidGyroscopeZeroDriftMode_Standard: HidGyroscopeZeroDriftMode = 1;
/// Tight
pub const HidGyroscopeZeroDriftMode_Tight: HidGyroscopeZeroDriftMode = 2;

/// NpadJoyHoldType
pub type HidNpadJoyHoldType = u32;
/// Default / Joy-Con held vertically.
pub const HidNpadJoyHoldType_Vertical: HidNpadJoyHoldType = 0;
/// Joy-Con held horizontally.
pub const HidNpadJoyHoldType_Horizontal: HidNpadJoyHoldType = 1;

/// NpadJoyDeviceType
pub type HidNpadJoyDeviceType = u32;
/// Left
pub const HidNpadJoyDeviceType_Left: HidNpadJoyDeviceType = 0;
/// Right
pub const HidNpadJoyDeviceType_Right: HidNpadJoyDeviceType = 1;

/// This controls how many Joy-Cons must be attached for handheld-mode to be activated.
pub type HidNpadHandheldActivationMode = u32;
/// Dual (2 Joy-Cons)
pub const HidNpadHandheldActivationMode_Dual: HidNpadHandheldActivationMode = 0;
/// Single (1 Joy-Con)
pub const HidNpadHandheldActivationMode_Single: HidNpadHandheldActivationMode = 1;
/// None (0 Joy-Cons)
pub const HidNpadHandheldActivationMode_None: HidNpadHandheldActivationMode = 2;

/// NpadJoyAssignmentMode
pub type HidNpadJoyAssignmentMode = u32;
/// Dual (Set by `hidSetNpadJoyAssignmentModeDual`)
pub const HidNpadJoyAssignmentMode_Dual: HidNpadJoyAssignmentMode = 0;
/// Single (Set by hidSetNpadJoyAssignmentModeSingle*())
pub const HidNpadJoyAssignmentMode_Single: HidNpadJoyAssignmentMode = 1;

/// NpadCommunicationMode
pub type HidNpadCommunicationMode = u32;
/// 5ms
pub const HidNpadCommunicationMode_5ms: HidNpadCommunicationMode = 0;
/// 10ms
pub const HidNpadCommunicationMode_10ms: HidNpadCommunicationMode = 1;
/// 15ms
pub const HidNpadCommunicationMode_15ms: HidNpadCommunicationMode = 2;
/// Default
pub const HidNpadCommunicationMode_Default: HidNpadCommunicationMode = 3;

/// DeviceType (system)
pub type HidDeviceTypeBits = u32;
/// Pro Controller and Gc controller.
pub const HidDeviceTypeBits_FullKey: HidDeviceTypeBits = BIT(0);
/// DebugPad
pub const HidDeviceTypeBits_DebugPad: HidDeviceTypeBits = BIT(1);
/// Joy-Con/Famicom/NES left controller in handheld mode.
pub const HidDeviceTypeBits_HandheldLeft: HidDeviceTypeBits = BIT(2);
/// Joy-Con/Famicom/NES right controller in handheld mode.
pub const HidDeviceTypeBits_HandheldRight: HidDeviceTypeBits = BIT(3);
/// Joy-Con left controller.
pub const HidDeviceTypeBits_JoyLeft: HidDeviceTypeBits = BIT(4);
/// Joy-Con right controller.
pub const HidDeviceTypeBits_JoyRight: HidDeviceTypeBits = BIT(5);
/// Poké Ball Plus controller.
pub const HidDeviceTypeBits_Palma: HidDeviceTypeBits = BIT(6);
/// Famicom left controller.
pub const HidDeviceTypeBits_LarkHvcLeft: HidDeviceTypeBits = BIT(7);
/// Famicom right controller (with microphone).
pub const HidDeviceTypeBits_LarkHvcRight: HidDeviceTypeBits = BIT(8);
/// NES left controller.
pub const HidDeviceTypeBits_LarkNesLeft: HidDeviceTypeBits = BIT(9);
/// NES right controller.
pub const HidDeviceTypeBits_LarkNesRight: HidDeviceTypeBits = BIT(10);
/// Famicom left controller in handheld mode.
pub const HidDeviceTypeBits_HandheldLarkHvcLeft: HidDeviceTypeBits = BIT(11);
/// Famicom right controller (with microphone) in handheld mode.
pub const HidDeviceTypeBits_HandheldLarkHvcRight: HidDeviceTypeBits = BIT(12);
/// NES left controller in handheld mode.
pub const HidDeviceTypeBits_HandheldLarkNesLeft: HidDeviceTypeBits = BIT(13);
/// NES right controller in handheld mode.
pub const HidDeviceTypeBits_HandheldLarkNesRight: HidDeviceTypeBits = BIT(14);
/// SNES controller
pub const HidDeviceTypeBits_Lucia: HidDeviceTypeBits = BIT(15);
/// N64 controller
pub const HidDeviceTypeBits_Lagon: HidDeviceTypeBits = BIT(16);
/// Sega Genesis controller
pub const HidDeviceTypeBits_Lager: HidDeviceTypeBits = BIT(17);
/// Generic controller.
pub const HidDeviceTypeBits_System: HidDeviceTypeBits = BIT(31);

/// Internal DeviceType for [9.0.0+]. Converted to/from the pre-9.0.0 version of this by the hiddbg funcs.
pub type HidDeviceType = u32;
/// ::HidDeviceTypeBits_JoyRight
pub const HidDeviceType_JoyRight1: HidDeviceType = 1;
/// ::HidDeviceTypeBits_JoyLeft
pub const HidDeviceType_JoyLeft2: HidDeviceType = 2;
/// ::HidDeviceTypeBits_FullKey
pub const HidDeviceType_FullKey3: HidDeviceType = 3;
/// ::HidDeviceTypeBits_JoyLeft
pub const HidDeviceType_JoyLeft4: HidDeviceType = 4;
/// ::HidDeviceTypeBits_JoyRight
pub const HidDeviceType_JoyRight5: HidDeviceType = 5;
/// ::HidDeviceTypeBits_FullKey
pub const HidDeviceType_FullKey6: HidDeviceType = 6;
/// ::HidDeviceTypeBits_LarkHvcLeft, ::HidDeviceTypeBits_HandheldLarkHvcLeft
pub const HidDeviceType_LarkHvcLeft: HidDeviceType = 7;
/// ::HidDeviceTypeBits_LarkHvcRight, ::HidDeviceTypeBits_HandheldLarkHvcRight
pub const HidDeviceType_LarkHvcRight: HidDeviceType = 8;
/// ::HidDeviceTypeBits_LarkNesLeft, ::HidDeviceTypeBits_HandheldLarkNesLeft
pub const HidDeviceType_LarkNesLeft: HidDeviceType = 9;
/// ::HidDeviceTypeBits_LarkNesRight, ::HidDeviceTypeBits_HandheldLarkNesRight
pub const HidDeviceType_LarkNesRight: HidDeviceType = 10;
/// ::HidDeviceTypeBits_Lucia
pub const HidDeviceType_Lucia: HidDeviceType = 11;
/// [9.0.0+] ::HidDeviceTypeBits_Palma
pub const HidDeviceType_Palma: HidDeviceType = 12;
/// ::HidDeviceTypeBits_FullKey
pub const HidDeviceType_FullKey13: HidDeviceType = 13;
/// ::HidDeviceTypeBits_FullKey
pub const HidDeviceType_FullKey15: HidDeviceType = 15;
/// ::HidDeviceTypeBits_DebugPad
pub const HidDeviceType_DebugPad: HidDeviceType = 17;
/// ::HidDeviceTypeBits_System with `HidNpadStyleTag` |= ::HidNpadStyleTag_NpadFullKey.
pub const HidDeviceType_System19: HidDeviceType = 19;
/// ::HidDeviceTypeBits_System with `HidNpadStyleTag` |= ::HidNpadStyleTag_NpadJoyDual.
pub const HidDeviceType_System20: HidDeviceType = 20;
/// ::HidDeviceTypeBits_System with `HidNpadStyleTag` |= ::HidNpadStyleTag_NpadJoyDual.
pub const HidDeviceType_System21: HidDeviceType = 21;
/// ::HidDeviceTypeBits_Lagon
pub const HidDeviceType_Lagon: HidDeviceType = 22;
/// ::HidDeviceTypeBits_Lager
pub const HidDeviceType_Lager: HidDeviceType = 28;

/// AppletFooterUiType (system)
pub type HidAppletFooterUiType = u32;
/// None
pub const HidAppletFooterUiType_None: HidAppletFooterUiType = 0;
/// HandheldNone
pub const HidAppletFooterUiType_HandheldNone: HidAppletFooterUiType = 1;
/// HandheldJoyConLeftOnly
pub const HidAppletFooterUiType_HandheldJoyConLeftOnly: HidAppletFooterUiType = 2;
/// HandheldJoyConRightOnly
pub const HidAppletFooterUiType_HandheldJoyConRightOnly: HidAppletFooterUiType = 3;
/// HandheldJoyConLeftJoyConRight
pub const HidAppletFooterUiType_HandheldJoyConLeftJoyConRight: HidAppletFooterUiType = 4;
/// JoyDual
pub const HidAppletFooterUiType_JoyDual: HidAppletFooterUiType = 5;
/// JoyDualLeftOnly
pub const HidAppletFooterUiType_JoyDualLeftOnly: HidAppletFooterUiType = 6;
/// JoyDualRightOnly
pub const HidAppletFooterUiType_JoyDualRightOnly: HidAppletFooterUiType = 7;
/// JoyLeftHorizontal
pub const HidAppletFooterUiType_JoyLeftHorizontal: HidAppletFooterUiType = 8;
/// JoyLeftVertical
pub const HidAppletFooterUiType_JoyLeftVertical: HidAppletFooterUiType = 9;
/// JoyRightHorizontal
pub const HidAppletFooterUiType_JoyRightHorizontal: HidAppletFooterUiType = 10;
/// JoyRightVertical
pub const HidAppletFooterUiType_JoyRightVertical: HidAppletFooterUiType = 11;
/// SwitchProController
pub const HidAppletFooterUiType_SwitchProController: HidAppletFooterUiType = 12;
/// CompatibleProController
pub const HidAppletFooterUiType_CompatibleProController: HidAppletFooterUiType = 13;
/// CompatibleJoyCon
pub const HidAppletFooterUiType_CompatibleJoyCon: HidAppletFooterUiType = 14;
/// LarkHvc1
pub const HidAppletFooterUiType_LarkHvc1: HidAppletFooterUiType = 15;
/// LarkHvc2
pub const HidAppletFooterUiType_LarkHvc2: HidAppletFooterUiType = 16;
/// LarkNesLeft
pub const HidAppletFooterUiType_LarkNesLeft: HidAppletFooterUiType = 17;
/// LarkNesRight
pub const HidAppletFooterUiType_LarkNesRight: HidAppletFooterUiType = 18;
/// Lucia
pub const HidAppletFooterUiType_Lucia: HidAppletFooterUiType = 19;
/// Verification
pub const HidAppletFooterUiType_Verification: HidAppletFooterUiType = 20;
/// [13.0.0+] Lagon
pub const HidAppletFooterUiType_Lagon: HidAppletFooterUiType = 21;

/// NpadInterfaceType (system)
pub type HidNpadInterfaceType = u32;
/// Bluetooth.
pub const HidNpadInterfaceType_Bluetooth: HidNpadInterfaceType = 1;
/// Rail.
pub const HidNpadInterfaceType_Rail: HidNpadInterfaceType = 2;
/// USB.
pub const HidNpadInterfaceType_USB: HidNpadInterfaceType = 3;
/// Unknown.
pub const HidNpadInterfaceType_Unknown4: HidNpadInterfaceType = 4;

/// XcdInterfaceType
pub type XcdInterfaceType = u32;
pub const XcdInterfaceType_Bluetooth: XcdInterfaceType = BIT(0);
pub const XcdInterfaceType_Uart: XcdInterfaceType = BIT(1);
pub const XcdInterfaceType_Usb: XcdInterfaceType = BIT(2);
pub const XcdInterfaceType_FieldSet: XcdInterfaceType = BIT(7);

/// NpadLarkType
pub type HidNpadLarkType = u32;
/// Invalid
pub const HidNpadLarkType_Invalid: HidNpadLarkType = 0;
/// H1
pub const HidNpadLarkType_H1: HidNpadLarkType = 1;
/// H2
pub const HidNpadLarkType_H2: HidNpadLarkType = 2;
/// NL
pub const HidNpadLarkType_NL: HidNpadLarkType = 3;
/// NR
pub const HidNpadLarkType_NR: HidNpadLarkType = 4;

/// NpadLuciaType
pub type HidNpadLuciaType = u32;
/// Invalid
pub const HidNpadLuciaType_Invalid: HidNpadLuciaType = 0;
/// J
pub const HidNpadLuciaType_J: HidNpadLuciaType = 1;
/// E
pub const HidNpadLuciaType_E: HidNpadLuciaType = 2;
/// U
pub const HidNpadLuciaType_U: HidNpadLuciaType = 3;

/// NpadLagerType
pub type HidNpadLagerType = u32;
/// Invalid
pub const HidNpadLagerType_Invalid: HidNpadLagerType = 0;
/// J
pub const HidNpadLagerType_J: HidNpadLagerType = 1;
/// E
pub const HidNpadLagerType_E: HidNpadLagerType = 2;
/// U
pub const HidNpadLagerType_U: HidNpadLagerType = 3;

/// Type values for HidVibrationDeviceInfo::type.
pub type HidVibrationDeviceType = u32;
/// Unknown
pub const HidVibrationDeviceType_Unknown: HidVibrationDeviceType = 0;
/// LinearResonantActuator
pub const HidVibrationDeviceType_LinearResonantActuator: HidVibrationDeviceType = 1;
/// GcErm (::HidNpadStyleTag_NpadGc)
pub const HidVibrationDeviceType_GcErm: HidVibrationDeviceType = 2;

/// VibrationDevicePosition
pub type HidVibrationDevicePosition = u32;
/// None
pub const HidVibrationDevicePosition_None: HidVibrationDevicePosition = 0;
/// Left
pub const HidVibrationDevicePosition_Left: HidVibrationDevicePosition = 1;
/// Right
pub const HidVibrationDevicePosition_Right: HidVibrationDevicePosition = 2;

/// VibrationGcErmCommand
pub type HidVibrationGcErmCommand = u32;
/// Stops the vibration with a decay phase.
pub const HidVibrationGcErmCommand_Stop: HidVibrationGcErmCommand = 0;
/// Starts the vibration.
pub const HidVibrationGcErmCommand_Start: HidVibrationGcErmCommand = 1;
/// Stops the vibration immediately, with no decay phase.
pub const HidVibrationGcErmCommand_StopHard: HidVibrationGcErmCommand = 2;

/// PalmaOperationType
pub type HidPalmaOperationType = u32;
/// PlayActivity
pub const HidPalmaOperationType_PlayActivity: HidPalmaOperationType = 0;
/// SetFrModeType
pub const HidPalmaOperationType_SetFrModeType: HidPalmaOperationType = 1;
/// ReadStep
pub const HidPalmaOperationType_ReadStep: HidPalmaOperationType = 2;
/// EnableStep
pub const HidPalmaOperationType_EnableStep: HidPalmaOperationType = 3;
/// ResetStep
pub const HidPalmaOperationType_ResetStep: HidPalmaOperationType = 4;
/// ReadApplicationSection
pub const HidPalmaOperationType_ReadApplicationSection: HidPalmaOperationType = 5;
/// WriteApplicationSection
pub const HidPalmaOperationType_WriteApplicationSection: HidPalmaOperationType = 6;
/// ReadUniqueCode
pub const HidPalmaOperationType_ReadUniqueCode: HidPalmaOperationType = 7;
/// SetUniqueCodeInvalid
pub const HidPalmaOperationType_SetUniqueCodeInvalid: HidPalmaOperationType = 8;
/// WriteActivityEntry
pub const HidPalmaOperationType_WriteActivityEntry: HidPalmaOperationType = 9;
/// WriteRgbLedPatternEntry
pub const HidPalmaOperationType_WriteRgbLedPatternEntry: HidPalmaOperationType = 10;
/// WriteWaveEntry
pub const HidPalmaOperationType_WriteWaveEntry: HidPalmaOperationType = 11;
/// ReadDataBaseIdentificationVersion
pub const HidPalmaOperationType_ReadDataBaseIdentificationVersion: HidPalmaOperationType = 12;
/// WriteDataBaseIdentificationVersion
pub const HidPalmaOperationType_WriteDataBaseIdentificationVersion: HidPalmaOperationType = 13;
/// SuspendFeature
pub const HidPalmaOperationType_SuspendFeature: HidPalmaOperationType = 14;
/// [5.1.0+] ReadPlayLog
pub const HidPalmaOperationType_ReadPlayLog: HidPalmaOperationType = 15;
/// [5.1.0+] ResetPlayLog
pub const HidPalmaOperationType_ResetPlayLog: HidPalmaOperationType = 16;

/// PalmaFrModeType
pub type HidPalmaFrModeType = u32;
/// Off
pub const HidPalmaFrModeType_Off: HidPalmaFrModeType = 0;
/// B01
pub const HidPalmaFrModeType_B01: HidPalmaFrModeType = 1;
/// B02
pub const HidPalmaFrModeType_B02: HidPalmaFrModeType = 2;
/// B03
pub const HidPalmaFrModeType_B03: HidPalmaFrModeType = 3;
/// Downloaded
pub const HidPalmaFrModeType_Downloaded: HidPalmaFrModeType = 4;

/// PalmaWaveSet
pub type HidPalmaWaveSet = u32;
/// Small
pub const HidPalmaWaveSet_Small: HidPalmaWaveSet = 0;
/// Medium
pub const HidPalmaWaveSet_Medium: HidPalmaWaveSet = 1;
/// Large
pub const HidPalmaWaveSet_Large: HidPalmaWaveSet = 2;

/// PalmaFeature
pub type HidPalmaFeature = u32;
/// FrMode
pub const HidPalmaFeature_FrMode: HidPalmaFeature = BIT(0);
/// RumbleFeedback
pub const HidPalmaFeature_RumbleFeedback: HidPalmaFeature = BIT(1);
/// Step
pub const HidPalmaFeature_Step: HidPalmaFeature = BIT(2);
/// MuteSwitch
pub const HidPalmaFeature_MuteSwitch: HidPalmaFeature = BIT(3);

/// HidAnalogStickState
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidAnalogStickState {
    /// X
    pub x: s32,
    /// Y
    pub y: s32,
}

/// HidVector
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidVector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// HidDirectionState
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidDirectionState {
    /// 3x3 matrix
    pub direction: [[f32; 3]; 3],
}

/// HidCommonLifoHeader
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidCommonLifoHeader {
    /// Unused
    pub unused: u64,
    /// BufferCount
    pub buffer_count: u64,
    /// Tail
    pub tail: u64,
    /// Count
    pub count: u64,
}

/// HidDebugPadState
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidDebugPadState {
    /// SamplingNumber
    pub sampling_number: u64,
    /// Bitfield of `HidDebugPadAttribute`.
    pub attributes: u32,
    /// Bitfield of `HidDebugPadButton`.
    pub buttons: u32,
    /// AnalogStickR
    pub analog_stick_r: HidAnalogStickState,
    /// AnalogStickL
    pub analog_stick_l: HidAnalogStickState,
}

/// HidDebugPadStateAtomicStorage
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidDebugPadStateAtomicStorage {
    /// SamplingNumber
    pub sampling_number: u64,
    /// **Ref:** HidDebugPadState
    pub state: HidDebugPadState,
}

/// HidDebugPadLifo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidDebugPadLifo {
    /// **Ref:** HidCommonLifoHeader
    pub header: HidCommonLifoHeader,
    /// **Ref:** HidDebugPadStateAtomicStorage
    pub storage: [HidDebugPadStateAtomicStorage; 17],
}

/// HidDebugPadSharedMemoryFormat
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidDebugPadSharedMemoryFormat {
    pub lifo: HidDebugPadLifo,
    pub padding: [u8; 312],
}

/// HidTouchState
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidTouchState {
    /// DeltaTime
    pub delta_time: u64,
    /// Bitfield of `HidTouchAttribute`.
    pub attributes: u32,
    /// FingerId
    pub finger_id: u32,
    /// X
    pub x: u32,
    /// Y
    pub y: u32,
    /// DiameterX
    pub diameter_x: u32,
    /// DiameterY
    pub diameter_y: u32,
    /// RotationAngle
    pub rotation_angle: u32,
    /// Reserved
    pub reserved: u32,
}

/// HidTouchScreenState
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidTouchScreenState {
    /// SamplingNumber
    pub sampling_number: u64,
    /// Number of entries in the touches array.
    pub count: s32,
    /// Reserved
    pub reserved: u32,
    /// Array of `HidTouchState`, with the above count.
    pub touches: [HidTouchState; 16],
}

/// HidTouchScreenStateAtomicStorage
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidTouchScreenStateAtomicStorage {
    /// SamplingNumber
    pub sampling_number: u64,
    /// **Ref:** HidTouchScreenState
    pub state: HidTouchScreenState,
}

/// HidTouchScreenLifo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidTouchScreenLifo {
    /// **Ref:** HidCommonLifoHeader
    pub header: HidCommonLifoHeader,
    /// **Ref:** HidTouchScreenStateAtomicStorage
    pub storage: [HidTouchScreenStateAtomicStorage; 17],
}

/// HidTouchScreenSharedMemoryFormat
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidTouchScreenSharedMemoryFormat {
    pub lifo: HidTouchScreenLifo,
    pub padding: [u8; 968],
}

/// HidTouchScreenConfigurationForNx
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidTouchScreenConfigurationForNx {
    /// **Ref:** HidTouchScreenModeForNx
    pub mode: u8,
    /// Reserved
    pub reserved: [u8; 15],
}

/// HidMouseState
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidMouseState {
    /// SamplingNumber
    pub sampling_number: u64,
    /// X
    pub x: s32,
    /// Y
    pub y: s32,
    /// DeltaX
    pub delta_x: s32,
    /// DeltaY
    pub delta_y: s32,
    /// WheelDeltaX
    pub wheel_delta_x: s32,
    /// WheelDeltaY
    pub wheel_delta_y: s32,
    /// Bitfield of `HidMouseButton`.
    pub buttons: u32,
    /// Bitfield of `HidMouseAttribute`.
    pub attributes: u32,
}

/// HidMouseStateAtomicStorage
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidMouseStateAtomicStorage {
    /// SamplingNumber
    pub sampling_number: u64,
    pub state: HidMouseState,
}

/// HidMouseLifo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidMouseLifo {
    pub header: HidCommonLifoHeader,
    pub storage: [HidMouseStateAtomicStorage; 17],
}

/// HidMouseSharedMemoryFormat
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidMouseSharedMemoryFormat {
    pub lifo: HidMouseLifo,
    pub padding: [u8; 176],
}

/// HidKeyboardState
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidKeyboardState {
    /// SamplingNumber
    pub sampling_number: u64,
    /// Bitfield of `HidKeyboardModifier`.
    pub modifiers: u64,
    pub keys: [u64; 4],
}

/// HidKeyboardStateAtomicStorage
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidKeyboardStateAtomicStorage {
    /// SamplingNumber
    pub sampling_number: u64,
    pub state: HidKeyboardState,
}

/// HidKeyboardLifo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidKeyboardLifo {
    pub header: HidCommonLifoHeader,
    pub storage: [HidKeyboardStateAtomicStorage; 17],
}

/// HidKeyboardSharedMemoryFormat
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidKeyboardSharedMemoryFormat {
    pub lifo: HidKeyboardLifo,
    pub padding: [u8; 40],
}

/// HidBasicXpadState
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidBasicXpadState {
    pub sampling_number: u64,
    pub attributes: u32,
    pub buttons: u32,
    pub analog_stick_left: u64,
    pub analog_stick_right: u64,
}

/// HidBasicXpadStateAtomicStorage
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidBasicXpadStateAtomicStorage {
    pub sampling_number: u64,
    pub state: HidBasicXpadState,
}

/// HidBasicXpadLifo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidBasicXpadLifo {
    pub header: HidCommonLifoHeader,
    pub storage: [HidBasicXpadStateAtomicStorage; 17],
}

/// HidBasicXpadSharedMemoryEntry
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidBasicXpadSharedMemoryEntry {
    pub lifo: HidBasicXpadLifo,
    pub padding: [u8; 312],
}

/// HidBasicXpadSharedMemoryFormat
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidBasicXpadSharedMemoryFormat {
    pub entries: [HidBasicXpadSharedMemoryEntry; 4],
}

/// HidDigitizerState
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidDigitizerState {
    pub sampling_number: u64,
    pub unk_0x8: u32,
    pub unk_0xC: u32,
    pub attributes: u32,
    pub buttons: u32,
    pub unk_0x18: u32,
    pub unk_0x1C: u32,
    pub unk_0x20: u32,
    pub unk_0x24: u32,
    pub unk_0x28: u32,
    pub unk_0x2C: u32,
    pub unk_0x30: u32,
    pub unk_0x34: u32,
    pub unk_0x38: u32,
    pub unk_0x3C: u32,
    pub unk_0x40: u32,
    pub unk_0x44: u32,
    pub unk_0x48: u32,
    pub unk_0x4C: u32,
    pub unk_0x50: u32,
    pub unk_0x54: u32,
}

/// HidDigitizerStateAtomicStorage
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidDigitizerStateAtomicStorage {
    pub sampling_number: u64,
    pub state: HidDigitizerState,
}

/// HidDigitizerLifo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidDigitizerLifo {
    pub header: HidCommonLifoHeader,
    pub storage: [HidDigitizerStateAtomicStorage; 17],
}

/// HidDigitizerSharedMemoryFormat
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidDigitizerSharedMemoryFormat {
    pub lifo: HidDigitizerLifo,
    pub padding: [u8; 2432],
}

/// HidHomeButtonState
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidHomeButtonState {
    pub sampling_number: u64,
    pub buttons: u64,
}

/// HidHomeButtonStateAtomicStorage
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidHomeButtonStateAtomicStorage {
    pub sampling_number: u64,
    pub state: HidHomeButtonState,
}

/// HidHomeButtonLifo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidHomeButtonLifo {
    pub header: HidCommonLifoHeader,
    pub storage: [HidHomeButtonStateAtomicStorage; 17],
}

/// HidHomeButtonSharedMemoryFormat
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidHomeButtonSharedMemoryFormat {
    pub lifo: HidHomeButtonLifo,
    pub padding: [u8; 72],
}

/// HidSleepButtonState
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidSleepButtonState {
    pub sampling_number: u64,
    pub buttons: u64,
}

/// HidSleepButtonStateAtomicStorage
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidSleepButtonStateAtomicStorage {
    pub sampling_number: u64,
    pub state: HidSleepButtonState,
}

/// HidSleepButtonLifo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidSleepButtonLifo {
    pub header: HidCommonLifoHeader,
    pub storage: [HidSleepButtonStateAtomicStorage; 17],
}

/// HidSleepButtonSharedMemoryFormat
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidSleepButtonSharedMemoryFormat {
    pub lifo: HidSleepButtonLifo,
    pub padding: [u8; 72],
}

/// HidCaptureButtonState
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidCaptureButtonState {
    pub sampling_number: u64,
    pub buttons: u64,
}

/// HidCaptureButtonStateAtomicStorage
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidCaptureButtonStateAtomicStorage {
    pub sampling_number: u64,
    pub state: HidCaptureButtonState,
}

/// HidCaptureButtonLifo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidCaptureButtonLifo {
    pub header: HidCommonLifoHeader,
    pub storage: [HidCaptureButtonStateAtomicStorage; 17],
}

/// HidCaptureButtonSharedMemoryFormat
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidCaptureButtonSharedMemoryFormat {
    pub lifo: HidCaptureButtonLifo,
    pub padding: [u8; 72],
}

/// HidInputDetectorState
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidInputDetectorState {
    pub input_source_state: u64,
    pub sampling_number: u64,
}

/// HidInputDetectorStateAtomicStorage
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidInputDetectorStateAtomicStorage {
    pub sampling_number: u64,
    pub state: HidInputDetectorState,
}

/// HidInputDetectorLifo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidInputDetectorLifo {
    pub header: HidCommonLifoHeader,
    pub storage: [HidInputDetectorStateAtomicStorage; 2],
}

/// HidInputDetectorSharedMemoryEntry
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidInputDetectorSharedMemoryEntry {
    pub lifo: HidInputDetectorLifo,
    pub padding: [u8; 48],
}

/// HidInputDetectorSharedMemoryFormat
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidInputDetectorSharedMemoryFormat {
    pub entries: [HidInputDetectorSharedMemoryEntry; 16],
}

/// HidUniquePadConfigMutex
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidUniquePadConfigMutex {
    pub unk_0x0: [u8; 32],
}

/// HidSixAxisSensorUserCalibrationState
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidSixAxisSensorUserCalibrationState {
    pub flags: u32,
    pub reserved: [u8; 4],
    pub stage: u64,
    pub sampling_number: u64,
}

/// HidSixAxisSensorUserCalibrationStateAtomicStorage
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidSixAxisSensorUserCalibrationStateAtomicStorage {
    pub sampling_number: u64,
    pub calib_state: HidSixAxisSensorUserCalibrationState,
}

/// HidSixAxisSensorUserCalibrationStateLifo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidSixAxisSensorUserCalibrationStateLifo {
    pub header: HidCommonLifoHeader,
    pub storage: [HidSixAxisSensorUserCalibrationStateAtomicStorage; 2],
}

/// HidAnalogStickCalibrationStateImpl
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidAnalogStickCalibrationStateImpl {
    pub state: u64,
    pub flags: u64,
    pub stage: u64,
    pub sampling_number: u64,
}

/// HidAnalogStickCalibrationStateImplAtomicStorage
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidAnalogStickCalibrationStateImplAtomicStorage {
    pub sampling_number: u64,
    pub calib_state: HidAnalogStickCalibrationStateImpl,
}

/// HidAnalogStickCalibrationStateImplLifo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidAnalogStickCalibrationStateImplLifo {
    pub header: HidCommonLifoHeader,
    pub storage: [HidAnalogStickCalibrationStateImplAtomicStorage; 2],
}

/// HidUniquePadConfig
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidUniquePadConfig {
    pub type_: u32,
    pub interface: u32,
    pub serial_number: [u8; 16],
    pub controller_number: u32,
    pub is_active: bool,
    pub reserved: [u8; 3],
    pub sampling_number: u64,
}

/// HidUniquePadConfigAtomicStorage
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidUniquePadConfigAtomicStorage {
    pub sampling_number: u64,
    pub config: HidUniquePadConfig,
}

/// HidUniquePadConfigLifo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidUniquePadConfigLifo {
    pub header: HidCommonLifoHeader,
    pub storage: [HidUniquePadConfigAtomicStorage; 2],
}

/// HidUniquePadLifo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidUniquePadLifo {
    pub config_lifo: HidUniquePadConfigLifo,
    pub analog_stick_calib_lifo: [HidAnalogStickCalibrationStateImplLifo; 2],
    pub sixaxis_calib_lifo: HidSixAxisSensorUserCalibrationStateLifo,
    pub mutex: HidUniquePadConfigMutex,
}

/// HidUniquePadSharedMemoryEntry
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidUniquePadSharedMemoryEntry {
    pub lifo: HidUniquePadLifo,
    pub padding: [u8; 544],
}

/// HidUniquePadSharedMemoryFormat
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidUniquePadSharedMemoryFormat {
    pub entries: [HidUniquePadSharedMemoryEntry; 16],
}

/// Npad colors.
/// Color fields are zero when not set.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidNpadControllerColor {
    /// RGBA Body Color
    pub main: u32,
    /// RGBA Buttons Color
    pub sub: u32,
}

/// HidNpadFullKeyColorState
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidNpadFullKeyColorState {
    /// **Ref:** HidColorAttribute
    pub attribute: u32,
    /// **Ref:** HidNpadControllerColor FullKey
    pub full_key: HidNpadControllerColor,
}

/// HidNpadJoyColorState
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidNpadJoyColorState {
    /// **Ref:** HidColorAttribute
    pub attribute: u32,
    /// **Ref:** HidNpadControllerColor Left
    pub left: HidNpadControllerColor,
    /// **Ref:** HidNpadControllerColor Right
    pub right: HidNpadControllerColor,
}

/// HidNpadCommonState
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidNpadCommonState {
    /// SamplingNumber
    pub sampling_number: u64,
    /// Bitfield of `HidNpadButton`.
    pub buttons: u64,
    /// AnalogStickL
    pub analog_stick_l: HidAnalogStickState,
    /// AnalogStickR
    pub analog_stick_r: HidAnalogStickState,
    /// Bitfield of `HidNpadAttribute`.
    pub attributes: u32,
    /// Reserved
    pub reserved: u32,
}

/// State for ::HidNpadStyleTag_NpadFullKey.
pub type HidNpadFullKeyState = HidNpadCommonState;

/// State for ::HidNpadStyleTag_NpadHandheld.
pub type HidNpadHandheldState = HidNpadCommonState;

/// State for ::HidNpadStyleTag_NpadJoyDual.
pub type HidNpadJoyDualState = HidNpadCommonState;

/// State for ::HidNpadStyleTag_NpadJoyLeft.
pub type HidNpadJoyLeftState = HidNpadCommonState;

/// State for ::HidNpadStyleTag_NpadJoyRight.
pub type HidNpadJoyRightState = HidNpadCommonState;

/// State for ::HidNpadStyleTag_NpadGc. Loaded from the same lifo as `HidNpadFullKeyState`, with the additional trigger_l/trigger_r loaded from elsewhere.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidNpadGcState {
    /// SamplingNumber
    pub sampling_number: u64,
    /// Bitfield of `HidNpadButton`.
    pub buttons: u64,
    /// AnalogStickL
    pub analog_stick_l: HidAnalogStickState,
    /// AnalogStickR
    pub analog_stick_r: HidAnalogStickState,
    /// Bitfield of `HidNpadAttribute`.
    pub attributes: u32,
    /// L analog trigger. Valid range: 0x0-0x7FFF.
    pub trigger_l: u32,
    /// R analog trigger. Valid range: 0x0-0x7FFF.
    pub trigger_r: u32,
    pub pad: u32,
}

/// State for ::HidNpadStyleTag_NpadPalma.
pub type HidNpadPalmaState = HidNpadCommonState;

/// State for ::HidNpadStyleTag_NpadLark. The base state is loaded from the same lifo as `HidNpadFullKeyState`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidNpadLarkState {
    /// SamplingNumber
    pub sampling_number: u64,
    /// Bitfield of `HidNpadButton`.
    pub buttons: u64,
    /// This is always zero.
    pub analog_stick_l: HidAnalogStickState,
    /// This is always zero.
    pub analog_stick_r: HidAnalogStickState,
    /// Bitfield of `HidNpadAttribute`.
    pub attributes: u32,
    /// **Ref:** HidNpadLarkType LarkTypeLAndMain
    pub lark_type_l_and_main: HidNpadLarkType,
}

/// State for ::HidNpadStyleTag_NpadHandheldLark. The base state is loaded from the same lifo as `HidNpadHandheldState`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidNpadHandheldLarkState {
    /// SamplingNumber
    pub sampling_number: u64,
    /// Bitfield of `HidNpadButton`.
    pub buttons: u64,
    /// AnalogStickL
    pub analog_stick_l: HidAnalogStickState,
    /// AnalogStickR
    pub analog_stick_r: HidAnalogStickState,
    /// Bitfield of `HidNpadAttribute`.
    pub attributes: u32,
    /// **Ref:** HidNpadLarkType LarkTypeLAndMain
    pub lark_type_l_and_main: HidNpadLarkType,
    /// **Ref:** HidNpadLarkType LarkTypeR
    pub lark_type_r: HidNpadLarkType,
    pub pad: u32,
}

/// State for ::HidNpadStyleTag_NpadLucia. The base state is loaded from the same lifo as `HidNpadFullKeyState`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidNpadLuciaState {
    /// SamplingNumber
    pub sampling_number: u64,
    /// Bitfield of `HidNpadButton`.
    pub buttons: u64,
    /// This is always zero.
    pub analog_stick_l: HidAnalogStickState,
    /// This is always zero.
    pub analog_stick_r: HidAnalogStickState,
    /// Bitfield of `HidNpadAttribute`.
    pub attributes: u32,
    /// **Ref:** HidNpadLuciaType
    pub lucia_type: HidNpadLuciaType,
}

/// State for ::HidNpadStyleTag_NpadLager. Analog-sticks state are always zero.
pub type HidNpadLagerState = HidNpadCommonState;

/// State for ::HidNpadStyleTag_NpadSystemExt.
pub type HidNpadSystemExtState = HidNpadCommonState;

/// State for ::HidNpadStyleTag_NpadSystem. Analog-sticks state are always zero. Only the following button bits are available: HidNpadButton_A, HidNpadButton_B, HidNpadButton_X, HidNpadButton_Y, HidNpadButton_Left, HidNpadButton_Up, HidNpadButton_Right, HidNpadButton_Down, HidNpadButton_L, HidNpadButton_R.
pub type HidNpadSystemState = HidNpadCommonState;

/// HidNpadCommonStateAtomicStorage
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidNpadCommonStateAtomicStorage {
    /// SamplingNumber
    pub sampling_number: u64,
    pub state: HidNpadCommonState,
}

/// HidNpadCommonLifo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidNpadCommonLifo {
    pub header: HidCommonLifoHeader,
    pub storage: [HidNpadCommonStateAtomicStorage; 17],
}

/// HidNpadGcTriggerState
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidNpadGcTriggerState {
    /// SamplingNumber
    pub sampling_number: u64,
    pub trigger_l: u32,
    pub trigger_r: u32,
}

/// HidNpadGcTriggerStateAtomicStorage
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidNpadGcTriggerStateAtomicStorage {
    /// SamplingNumber
    pub sampling_number: u64,
    pub state: HidNpadGcTriggerState,
}

/// HidNpadGcTriggerLifo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidNpadGcTriggerLifo {
    pub header: HidCommonLifoHeader,
    pub storage: [HidNpadGcTriggerStateAtomicStorage; 17],
}

/// HidSixAxisSensorState
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidSixAxisSensorState {
    /// DeltaTime
    pub delta_time: u64,
    /// SamplingNumber
    pub sampling_number: u64,
    /// Acceleration
    pub acceleration: HidVector,
    /// AngularVelocity
    pub angular_velocity: HidVector,
    /// Angle
    pub angle: HidVector,
    /// Direction
    pub direction: HidDirectionState,
    /// Bitfield of `HidSixAxisSensorAttribute`.
    pub attributes: u32,
    /// Reserved
    pub reserved: u32,
}

/// HidSixAxisSensorStateAtomicStorage
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidSixAxisSensorStateAtomicStorage {
    /// SamplingNumber
    pub sampling_number: u64,
    pub state: HidSixAxisSensorState,
}

/// HidNpadSixAxisSensorLifo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidNpadSixAxisSensorLifo {
    pub header: HidCommonLifoHeader,
    pub storage: [HidSixAxisSensorStateAtomicStorage; 17],
}

/// NpadSystemProperties
#[repr(C, align(8))]
#[derive(Debug, Clone, Copy)]
pub struct HidNpadSystemProperties {
    pub _bitfield_0: [u8; 8],
}

impl HidNpadSystemProperties {
    bitfields!(_bitfield_0 {
        /// Use `hidGetNpadPowerInfoSingle` / `hidGetNpadPowerInfoSplit` instead of accessing this directly.
        is_charging / set_is_charging: 0, 3 => u64;
        /// Use `hidGetNpadPowerInfoSingle` / `hidGetNpadPowerInfoSplit` instead of accessing this directly.
        is_powered / set_is_powered: 3, 3 => u64;
        /// Unused
        bit6 / set_bit6: 6, 1 => u64;
        /// Unused
        bit7 / set_bit7: 7, 1 => u64;
        /// Unused
        bit8 / set_bit8: 8, 1 => u64;
        /// IsUnsupportedButtonPressedOnNpadSystem
        is_unsupported_button_pressed_on_npad_system / set_is_unsupported_button_pressed_on_npad_system: 9, 1 => u64;
        /// IsUnsupportedButtonPressedOnNpadSystemExt
        is_unsupported_button_pressed_on_npad_system_ext / set_is_unsupported_button_pressed_on_npad_system_ext: 10, 1 => u64;
        /// IsAbxyButtonOriented
        is_abxy_button_oriented / set_is_abxy_button_oriented: 11, 1 => u64;
        /// IsSlSrButtonOriented
        is_sl_sr_button_oriented / set_is_sl_sr_button_oriented: 12, 1 => u64;
        /// [4.0.0+] IsPlusAvailable
        is_plus_available / set_is_plus_available: 13, 1 => u64;
        /// [4.0.0+] IsMinusAvailable
        is_minus_available / set_is_minus_available: 14, 1 => u64;
        /// [8.0.0+] IsDirectionalButtonsAvailable
        is_directional_buttons_available / set_is_directional_buttons_available: 15, 1 => u64;
        /// Unused
        unused / set_unused: 16, 48 => u64;
    });
}

/// NpadSystemButtonProperties
#[repr(C, align(4))]
#[derive(Debug, Clone, Copy)]
pub struct HidNpadSystemButtonProperties {
    pub _bitfield_0: [u8; 1],
}

impl HidNpadSystemButtonProperties {
    bitfields!(_bitfield_0 {
        /// IsUnintendedHomeButtonInputProtectionEnabled
        is_unintended_home_button_input_protection_enabled / set_is_unintended_home_button_input_protection_enabled: 0, 1 => u32;
    });
}

/// HidPowerInfo (system)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidPowerInfo {
    /// IsPowered
    pub is_powered: bool,
    /// IsCharging
    pub is_charging: bool,
    /// Reserved
    pub reserved: [u8; 6],
    /// BatteryLevel, always 0-4.
    pub battery_level: u32,
}

/// XcdDeviceHandle
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct XcdDeviceHandle {
    pub handle: u64,
}

/// HidNfcXcdDeviceHandleStateImpl
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidNfcXcdDeviceHandleStateImpl {
    pub handle: XcdDeviceHandle,
    pub is_available: u8,
    pub is_activated: u8,
    pub reserved: [u8; 6],
    /// SamplingNumber
    pub sampling_number: u64,
}

/// HidNfcXcdDeviceHandleStateImplAtomicStorage
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidNfcXcdDeviceHandleStateImplAtomicStorage {
    /// SamplingNumber
    pub sampling_number: u64,
    /// **Ref:** HidNfcXcdDeviceHandleStateImpl
    pub state: HidNfcXcdDeviceHandleStateImpl,
}

/// HidNfcXcdDeviceHandleState
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidNfcXcdDeviceHandleState {
    pub header: HidCommonLifoHeader,
    pub storage: [HidNfcXcdDeviceHandleStateImplAtomicStorage; 2],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidNpadInternalState_anon0_anon0 {
    pub nfc_xcd_device_handle: HidNfcXcdDeviceHandleState,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidNpadInternalState_anon0_anon1 {
    /// Bitfield of AppletFooterUiAttribute.
    pub applet_footer_ui_attribute: u32,
    /// **Ref:** HidAppletFooterUiType
    pub applet_footer_ui_type: u8,
    pub reserved_x41AD: [u8; 91],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union HidNpadInternalState_anon0 {
    pub anon0: HidNpadInternalState_anon0_anon0,
    pub anon1: HidNpadInternalState_anon0_anon1,
}

/// HidNpadInternalState
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HidNpadInternalState {
    /// Bitfield of `HidNpadStyleTag`.
    pub style_set: u32,
    /// **Ref:** HidNpadJoyAssignmentMode
    pub joy_assignment_mode: u32,
    /// **Ref:** HidNpadFullKeyColorState
    pub full_key_color: HidNpadFullKeyColorState,
    /// **Ref:** HidNpadJoyColorState
    pub joy_color: HidNpadJoyColorState,
    /// FullKeyLifo
    pub full_key_lifo: HidNpadCommonLifo,
    /// HandheldLifo
    pub handheld_lifo: HidNpadCommonLifo,
    /// JoyDualLifo
    pub joy_dual_lifo: HidNpadCommonLifo,
    /// JoyLeftLifo
    pub joy_left_lifo: HidNpadCommonLifo,
    /// JoyRightLifo
    pub joy_right_lifo: HidNpadCommonLifo,
    /// PalmaLifo
    pub palma_lifo: HidNpadCommonLifo,
    /// SystemExtLifo
    pub system_ext_lifo: HidNpadCommonLifo,
    /// FullKeySixAxisSensorLifo
    pub full_key_six_axis_sensor_lifo: HidNpadSixAxisSensorLifo,
    /// HandheldSixAxisSensorLifo
    pub handheld_six_axis_sensor_lifo: HidNpadSixAxisSensorLifo,
    /// JoyDualLeftSixAxisSensorLifo
    pub joy_dual_left_six_axis_sensor_lifo: HidNpadSixAxisSensorLifo,
    /// JoyDualRightSixAxisSensorLifo
    pub joy_dual_right_six_axis_sensor_lifo: HidNpadSixAxisSensorLifo,
    /// JoyLeftSixAxisSensorLifo
    pub joy_left_six_axis_sensor_lifo: HidNpadSixAxisSensorLifo,
    /// JoyRightSixAxisSensorLifo
    pub joy_right_six_axis_sensor_lifo: HidNpadSixAxisSensorLifo,
    /// Bitfield of `HidDeviceTypeBits`.
    pub device_type: u32,
    /// Reserved
    pub reserved: u32,
    pub system_properties: HidNpadSystemProperties,
    pub system_button_properties: HidNpadSystemButtonProperties,
    pub battery_level: [u32; 3],
    pub anon0: HidNpadInternalState_anon0,
    /// Mutex on pre-10.0.0.
    pub reserved_x4208: [u8; 32],
    pub gc_trigger_lifo: HidNpadGcTriggerLifo,
    /// **Ref:** HidNpadLarkType
    pub lark_type_l_and_main: u32,
    /// **Ref:** HidNpadLarkType
    pub lark_type_r: u32,
    /// **Ref:** HidNpadLuciaType
    pub lucia_type: u32,
    /// **Ref:** HidNpadLagerType
    pub lager_type: u32,
}

/// HidNpadSharedMemoryEntry
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HidNpadSharedMemoryEntry {
    pub internal_state: HidNpadInternalState,
    pub pad: [u8; 3088],
}

/// HidNpadSharedMemoryFormat
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HidNpadSharedMemoryFormat {
    pub entries: [HidNpadSharedMemoryEntry; 10],
}

/// HidGesturePoint
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidGesturePoint {
    /// X
    pub x: u32,
    /// Y
    pub y: u32,
}

/// HidGestureState
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidGestureState {
    /// SamplingNumber
    pub sampling_number: u64,
    /// ContextNumber
    pub context_number: u64,
    /// **Ref:** HidGestureType
    pub type_: u32,
    /// **Ref:** HidGestureDirection
    pub direction: u32,
    /// X
    pub x: u32,
    /// Y
    pub y: u32,
    /// DeltaX
    pub delta_x: s32,
    /// DeltaY
    pub delta_y: s32,
    /// VelocityX
    pub velocity_x: f32,
    /// VelocityY
    pub velocity_y: f32,
    /// Bitfield of `HidGestureAttribute`.
    pub attributes: u32,
    /// Scale
    pub scale: f32,
    /// RotationAngle
    pub rotation_angle: f32,
    /// Number of entries in the points array.
    pub point_count: s32,
    /// Array of `HidGesturePoint` with the above count.
    pub points: [HidGesturePoint; 4],
}

/// HidGestureDummyStateAtomicStorage
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidGestureDummyStateAtomicStorage {
    /// SamplingNumber
    pub sampling_number: u64,
    pub state: HidGestureState,
}

/// HidGestureLifo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidGestureLifo {
    pub header: HidCommonLifoHeader,
    pub storage: [HidGestureDummyStateAtomicStorage; 17],
}

/// HidGestureSharedMemoryFormat
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidGestureSharedMemoryFormat {
    pub lifo: HidGestureLifo,
    pub pad: [u8; 248],
}

/// HidConsoleSixAxisSensor
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidConsoleSixAxisSensor {
    /// SamplingNumber
    pub sampling_number: u64,
    /// IsSevenSixAxisSensorAtRest
    pub is_seven_six_axis_sensor_at_rest: u8,
    /// Padding
    pub pad: [u8; 3],
    /// VerticalizationError
    pub verticalization_error: f32,
    /// GyroBias
    pub gyro_bias: UtilFloat3,
    /// Padding
    pub pad2: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union HidSharedMemory_anon0 {
    /// [1.0.0-9.2.0] BasicXpad
    pub basic_xpad: HidBasicXpadSharedMemoryFormat,
    /// [10.0.0+] Digitizer
    pub digitizer: HidDigitizerSharedMemoryFormat,
}

/// HidSharedMemory
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HidSharedMemory {
    pub debug_pad: HidDebugPadSharedMemoryFormat,
    pub touchscreen: HidTouchScreenSharedMemoryFormat,
    pub mouse: HidMouseSharedMemoryFormat,
    pub keyboard: HidKeyboardSharedMemoryFormat,
    pub anon0: HidSharedMemory_anon0,
    pub home_button: HidHomeButtonSharedMemoryFormat,
    pub sleep_button: HidSleepButtonSharedMemoryFormat,
    pub capture_button: HidCaptureButtonSharedMemoryFormat,
    pub input_detector: HidInputDetectorSharedMemoryFormat,
    /// [1.0.0-4.1.0] UniquePad
    pub unique_pad: HidUniquePadSharedMemoryFormat,
    pub npad: HidNpadSharedMemoryFormat,
    pub gesture: HidGestureSharedMemoryFormat,
    /// [5.0.0+] ConsoleSixAxisSensor
    pub console_six_axis_sensor: HidConsoleSixAxisSensor,
    pub unk_x3C220: [u8; 15840],
}

/// HidSevenSixAxisSensorState
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidSevenSixAxisSensorState {
    pub timestamp0: u64,
    pub sampling_number: u64,
    pub unk_x10: u64,
    pub unk_x18: [f32; 10],
}

/// HidSevenSixAxisSensorStateEntry
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidSevenSixAxisSensorStateEntry {
    pub sampling_number: u64,
    pub unused: u64,
    pub state: HidSevenSixAxisSensorState,
}

/// HidSevenSixAxisSensorStates
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidSevenSixAxisSensorStates {
    pub header: HidCommonLifoHeader,
    pub storage: [HidSevenSixAxisSensorStateEntry; 33],
}

#[repr(C, align(4))]
#[derive(Debug, Clone, Copy)]
pub struct HidSixAxisSensorHandle_anon0 {
    pub _bitfield_0: [u8; 4],
}

impl HidSixAxisSensorHandle_anon0 {
    bitfields!(_bitfield_0 {
        /// NpadStyleIndex
        npad_style_index / set_npad_style_index: 0, 8 => u32;
        /// PlayerNumber
        player_number / set_player_number: 8, 8 => u32;
        /// DeviceIdx
        device_idx / set_device_idx: 16, 8 => u32;
        /// Padding
        pad / set_pad: 24, 8 => u32;
    });
}

/// HidSixAxisSensorHandle
#[repr(C)]
#[derive(Clone, Copy)]
pub union HidSixAxisSensorHandle {
    /// TypeValue
    pub type_value: u32,
    pub anon0: HidSixAxisSensorHandle_anon0,
}

#[repr(C, align(4))]
#[derive(Debug, Clone, Copy)]
pub struct HidVibrationDeviceHandle_anon0 {
    pub _bitfield_0: [u8; 4],
}

impl HidVibrationDeviceHandle_anon0 {
    bitfields!(_bitfield_0 {
        /// NpadStyleIndex
        npad_style_index / set_npad_style_index: 0, 8 => u32;
        /// PlayerNumber
        player_number / set_player_number: 8, 8 => u32;
        /// DeviceIdx
        device_idx / set_device_idx: 16, 8 => u32;
        /// Padding
        pad / set_pad: 24, 8 => u32;
    });
}

/// HidVibrationDeviceHandle
#[repr(C)]
#[derive(Clone, Copy)]
pub union HidVibrationDeviceHandle {
    /// TypeValue
    pub type_value: u32,
    pub anon0: HidVibrationDeviceHandle_anon0,
}

/// HidVibrationDeviceInfo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidVibrationDeviceInfo {
    /// **Ref:** HidVibrationDeviceType
    pub type_: u32,
    /// **Ref:** HidVibrationDevicePosition
    pub position: u32,
}

/// HidVibrationValue
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidVibrationValue {
    /// Low Band amplitude. 1.0f: Max amplitude.
    pub amp_low: f32,
    /// Low Band frequency in Hz.
    pub freq_low: f32,
    /// High Band amplitude. 1.0f: Max amplitude.
    pub amp_high: f32,
    /// High Band frequency in Hz.
    pub freq_high: f32,
}

/// PalmaConnectionHandle
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidPalmaConnectionHandle {
    /// Handle
    pub handle: u64,
}

/// PalmaOperationInfo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidPalmaOperationInfo {
    /// **Ref:** HidPalmaOperationType
    pub type_: u32,
    /// Result
    pub res: Result,
    /// Data
    pub data: [u8; 320],
}

/// PalmaApplicationSectionAccessBuffer
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidPalmaApplicationSectionAccessBuffer {
    /// Application data.
    pub data: [u8; 256],
}

/// PalmaActivityEntry
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidPalmaActivityEntry {
    /// RgbLedPatternIndex
    pub rgb_led_pattern_index: u16,
    /// Padding
    pub pad: u16,
    /// **Ref:** HidPalmaWaveSet
    pub wave_set: u32,
    /// WaveIndex
    pub wave_index: u16,
}

/// Gets the state of a key in a `HidKeyboardState`.
///
/// # Arguments
/// - `state` (in): `HidKeyboardState`.
/// - `key` (in): `HidKeyboardKey`.
///
/// **Returns:** true if the key is pressed, false if not.
#[inline(always)]
pub const fn hidKeyboardStateGetKey(state: &HidKeyboardState, key: HidKeyboardKey) -> bool {
    (state.keys[(key / 64) as usize] & (1u64 << (key & 63))) != 0
}

impl_zeroed_default!(HidAnalogStickState, HidVector, HidDirectionState, HidCommonLifoHeader, HidDebugPadState, HidDebugPadStateAtomicStorage, HidDebugPadLifo, HidDebugPadSharedMemoryFormat, HidTouchState, HidTouchScreenState, HidTouchScreenStateAtomicStorage, HidTouchScreenLifo, HidTouchScreenSharedMemoryFormat, HidTouchScreenConfigurationForNx, HidMouseState, HidMouseStateAtomicStorage, HidMouseLifo, HidMouseSharedMemoryFormat, HidKeyboardState, HidKeyboardStateAtomicStorage, HidKeyboardLifo, HidKeyboardSharedMemoryFormat, HidBasicXpadState, HidBasicXpadStateAtomicStorage, HidBasicXpadLifo, HidBasicXpadSharedMemoryEntry, HidBasicXpadSharedMemoryFormat, HidDigitizerState, HidDigitizerStateAtomicStorage, HidDigitizerLifo, HidDigitizerSharedMemoryFormat, HidHomeButtonState, HidHomeButtonStateAtomicStorage, HidHomeButtonLifo, HidHomeButtonSharedMemoryFormat, HidSleepButtonState, HidSleepButtonStateAtomicStorage, HidSleepButtonLifo, HidSleepButtonSharedMemoryFormat, HidCaptureButtonState, HidCaptureButtonStateAtomicStorage, HidCaptureButtonLifo, HidCaptureButtonSharedMemoryFormat, HidInputDetectorState, HidInputDetectorStateAtomicStorage, HidInputDetectorLifo, HidInputDetectorSharedMemoryEntry, HidInputDetectorSharedMemoryFormat, HidUniquePadConfigMutex, HidSixAxisSensorUserCalibrationState, HidSixAxisSensorUserCalibrationStateAtomicStorage, HidSixAxisSensorUserCalibrationStateLifo, HidAnalogStickCalibrationStateImpl, HidAnalogStickCalibrationStateImplAtomicStorage, HidAnalogStickCalibrationStateImplLifo, HidUniquePadConfig, HidUniquePadConfigAtomicStorage, HidUniquePadConfigLifo, HidUniquePadLifo, HidUniquePadSharedMemoryEntry, HidUniquePadSharedMemoryFormat, HidNpadControllerColor, HidNpadFullKeyColorState, HidNpadJoyColorState, HidNpadCommonState, HidNpadGcState, HidNpadLarkState, HidNpadHandheldLarkState, HidNpadLuciaState, HidNpadCommonStateAtomicStorage, HidNpadCommonLifo, HidNpadGcTriggerState, HidNpadGcTriggerStateAtomicStorage, HidNpadGcTriggerLifo, HidSixAxisSensorState, HidSixAxisSensorStateAtomicStorage, HidNpadSixAxisSensorLifo, HidNpadSystemProperties, HidNpadSystemButtonProperties, HidPowerInfo, XcdDeviceHandle, HidNfcXcdDeviceHandleStateImpl, HidNfcXcdDeviceHandleStateImplAtomicStorage, HidNfcXcdDeviceHandleState, HidNpadInternalState_anon0_anon0, HidNpadInternalState_anon0_anon1, HidNpadInternalState_anon0, HidNpadInternalState, HidNpadSharedMemoryEntry, HidNpadSharedMemoryFormat, HidGesturePoint, HidGestureState, HidGestureDummyStateAtomicStorage, HidGestureLifo, HidGestureSharedMemoryFormat, HidConsoleSixAxisSensor, HidSharedMemory_anon0, HidSharedMemory, HidSevenSixAxisSensorState, HidSevenSixAxisSensorStateEntry, HidSevenSixAxisSensorStates, HidSixAxisSensorHandle_anon0, HidSixAxisSensorHandle, HidVibrationDeviceHandle_anon0, HidVibrationDeviceHandle, HidVibrationDeviceInfo, HidVibrationValue, HidPalmaConnectionHandle, HidPalmaOperationInfo, HidPalmaApplicationSectionAccessBuffer, HidPalmaActivityEntry);

unsafe extern "C" {
    /// Initialize hid. Called automatically during app startup.
    pub fn hidInitialize() -> Result;

    /// Exit hid. Called automatically during app exit.
    pub fn hidExit();

    /// Gets the Service object for the actual hid service session.
    pub fn hidGetServiceSession() -> *mut Service;

    /// Gets the address of the SharedMemory.
    pub fn hidGetSharedmemAddr() -> *mut c_void;

    /// Initialize TouchScreen. Must be called when TouchScreen is being used. Used automatically by `hidScanInput` when required.
    pub fn hidInitializeTouchScreen();

    /// Gets `HidTouchScreenState`.
    ///
    /// # Arguments
    /// - `states` (out): Output array of `HidTouchScreenState`.
    /// - `count` (in): Size of the states array in entries.
    ///
    /// **Returns:** Total output entries.
    pub fn hidGetTouchScreenStates(states: *mut HidTouchScreenState, count: usize) -> usize;

    /// Initialize Mouse. Must be called when Mouse is being used. Used automatically by `hidScanInput` when required.
    pub fn hidInitializeMouse();

    /// Gets `HidMouseState`.
    ///
    /// # Arguments
    /// - `states` (out): Output array of `HidMouseState`.
    /// - `count` (in): Size of the states array in entries.
    ///
    /// **Returns:** Total output entries.
    pub fn hidGetMouseStates(states: *mut HidMouseState, count: usize) -> usize;

    /// Initialize Keyboard. Must be called when Keyboard is being used. Used automatically by `hidScanInput` when required.
    pub fn hidInitializeKeyboard();

    /// Gets `HidKeyboardState`.
    ///
    /// # Arguments
    /// - `states` (out): Output array of `HidKeyboardState`.
    /// - `count` (in): Size of the states array in entries.
    ///
    /// **Returns:** Total output entries.
    pub fn hidGetKeyboardStates(states: *mut HidKeyboardState, count: usize) -> usize;

    /// Gets `HidHomeButtonState`.
    ///
    /// # Arguments
    /// - `states` (out): Output array of `HidHomeButtonState`.
    /// - `count` (in): Size of the states array in entries.
    ///
    /// **Note:** Home button shmem must be activated with `hidsysActivateHomeButton`
    ///
    /// **Returns:** Total output entries.
    pub fn hidGetHomeButtonStates(states: *mut HidHomeButtonState, count: usize) -> usize;

    /// Gets `HidSleepButtonState`.
    ///
    /// # Arguments
    /// - `states` (out): Output array of `HidSleepButtonState`.
    /// - `count` (in): Size of the states array in entries.
    ///
    /// **Note:** Sleep button shmem must be activated with `hidsysActivateSleepButton`
    ///
    /// **Returns:** Total output entries.
    pub fn hidGetSleepButtonStates(states: *mut HidSleepButtonState, count: usize) -> usize;

    /// Gets `HidCaptureButtonState`.
    ///
    /// # Arguments
    /// - `states` (out): Output array of `HidCaptureButtonState`.
    /// - `count` (in): Size of the states array in entries.
    ///
    /// **Note:** Capture button shmem must be activated with `hidsysActivateCaptureButton`
    ///
    /// **Returns:** Total output entries.
    pub fn hidGetCaptureButtonStates(states: *mut HidCaptureButtonState, count: usize) -> usize;

    /// Initialize Npad. Must be called when Npad is being used. Used automatically by `hidScanInput` when required.
    pub fn hidInitializeNpad();

    /// Gets the StyleSet for the specified Npad.
    ///
    /// # Arguments
    /// - `id` (in): `HidNpadIdType`
    ///
    /// **Returns:** Bitfield of `HidNpadStyleTag`.
    pub fn hidGetNpadStyleSet(id: HidNpadIdType) -> u32;

    /// Gets the `HidNpadJoyAssignmentMode` for the specified Npad.
    ///
    /// # Arguments
    /// - `id` (in): `HidNpadIdType`
    ///
    /// **Returns:** `HidNpadJoyAssignmentMode`
    pub fn hidGetNpadJoyAssignment(id: HidNpadIdType) -> HidNpadJoyAssignmentMode;

    /// Gets the main `HidNpadControllerColor` for the specified Npad.
    ///
    /// # Arguments
    /// - `id` (in): `HidNpadIdType`
    /// - `color` (out): `HidNpadControllerColor`
    pub fn hidGetNpadControllerColorSingle(id: HidNpadIdType, color: *mut HidNpadControllerColor) -> Result;

    /// Gets the left/right `HidNpadControllerColor` for the specified Npad (Joy-Con pair in dual mode).
    ///
    /// # Arguments
    /// - `id` (in): `HidNpadIdType`
    /// - `color_left` (out): `HidNpadControllerColor`
    /// - `color_right` (out): `HidNpadControllerColor`
    pub fn hidGetNpadControllerColorSplit(id: HidNpadIdType, color_left: *mut HidNpadControllerColor, color_right: *mut HidNpadControllerColor) -> Result;

    /// Gets the DeviceType for the specified Npad.
    ///
    /// # Arguments
    /// - `id` (in): `HidNpadIdType`
    ///
    /// **Returns:** Bitfield of `HidDeviceTypeBits`.
    pub fn hidGetNpadDeviceType(id: HidNpadIdType) -> u32;

    /// Gets the `HidNpadSystemProperties` for the specified Npad.
    ///
    /// # Arguments
    /// - `id` (in): `HidNpadIdType`
    /// - `out` (out): `HidNpadSystemProperties`
    pub fn hidGetNpadSystemProperties(id: HidNpadIdType, out: *mut HidNpadSystemProperties);

    /// Gets the `HidNpadSystemButtonProperties` for the specified Npad.
    ///
    /// # Arguments
    /// - `id` (in): `HidNpadIdType`
    /// - `out` (out): `HidNpadSystemButtonProperties`
    pub fn hidGetNpadSystemButtonProperties(id: HidNpadIdType, out: *mut HidNpadSystemButtonProperties);

    /// Gets the main `HidPowerInfo` for the specified Npad.
    ///
    /// # Arguments
    /// - `id` (in): `HidNpadIdType`
    /// - `info` (out): `HidPowerInfo`
    pub fn hidGetNpadPowerInfoSingle(id: HidNpadIdType, info: *mut HidPowerInfo);

    /// Gets the left/right `HidPowerInfo` for the specified Npad (Joy-Con pair in dual mode).
    ///
    /// # Arguments
    /// - `id` (in): `HidNpadIdType`
    /// - `info_left` (out): `HidPowerInfo`
    /// - `info_right` (out): `HidPowerInfo`
    pub fn hidGetNpadPowerInfoSplit(id: HidNpadIdType, info_left: *mut HidPowerInfo, info_right: *mut HidPowerInfo);

    /// Gets the AppletFooterUiAttributesSet for the specified Npad.
    ///
    /// # Arguments
    /// - `id` (in): `HidNpadIdType`
    ///
    /// **Note:** Only available on [9.0.0+].
    ///
    /// **Returns:** Bitfield of AppletFooterUiAttribute (system).
    pub fn hidGetAppletFooterUiAttributesSet(id: HidNpadIdType) -> u32;

    /// Gets `HidAppletFooterUiType` for the specified Npad.
    ///
    /// # Arguments
    /// - `id` (in): `HidNpadIdType`
    ///
    /// **Note:** Only available on [9.0.0+].
    ///
    /// **Returns:** `HidAppletFooterUiType`
    pub fn hidGetAppletFooterUiTypes(id: HidNpadIdType) -> HidAppletFooterUiType;

    /// Gets `HidNpadLagerType` for the specified Npad.
    ///
    /// # Arguments
    /// - `id` (in): `HidNpadIdType`
    ///
    /// **Returns:** `HidNpadLagerType`
    pub fn hidGetNpadLagerType(id: HidNpadIdType) -> HidNpadLagerType;

    /// Gets `HidNpadFullKeyState`.
    ///
    /// # Arguments
    /// - `states` (out): Output array of `HidNpadFullKeyState`.
    /// - `count` (in): Size of the states array in entries.
    ///
    /// **Returns:** Total output entries.
    pub fn hidGetNpadStatesFullKey(id: HidNpadIdType, states: *mut HidNpadFullKeyState, count: usize) -> usize;

    /// Gets `HidNpadHandheldState`.
    ///
    /// # Arguments
    /// - `states` (out): Output array of `HidNpadHandheldState`.
    /// - `count` (in): Size of the states array in entries.
    ///
    /// **Returns:** Total output entries.
    pub fn hidGetNpadStatesHandheld(id: HidNpadIdType, states: *mut HidNpadHandheldState, count: usize) -> usize;

    /// Gets `HidNpadJoyDualState`.
    ///
    /// # Arguments
    /// - `states` (out): Output array of `HidNpadJoyDualState`.
    /// - `count` (in): Size of the states array in entries.
    ///
    /// **Returns:** Total output entries.
    pub fn hidGetNpadStatesJoyDual(id: HidNpadIdType, states: *mut HidNpadJoyDualState, count: usize) -> usize;

    /// Gets `HidNpadJoyLeftState`.
    ///
    /// # Arguments
    /// - `states` (out): Output array of `HidNpadJoyLeftState`.
    /// - `count` (in): Size of the states array in entries.
    ///
    /// **Returns:** Total output entries.
    pub fn hidGetNpadStatesJoyLeft(id: HidNpadIdType, states: *mut HidNpadJoyLeftState, count: usize) -> usize;

    /// Gets `HidNpadJoyRightState`.
    ///
    /// # Arguments
    /// - `states` (out): Output array of `HidNpadJoyRightState`.
    /// - `count` (in): Size of the states array in entries.
    ///
    /// **Returns:** Total output entries.
    pub fn hidGetNpadStatesJoyRight(id: HidNpadIdType, states: *mut HidNpadJoyRightState, count: usize) -> usize;

    /// Gets `HidNpadGcState`.
    ///
    /// # Arguments
    /// - `states` (out): Output array of `HidNpadGcState`.
    /// - `count` (in): Size of the states array in entries.
    ///
    /// **Returns:** Total output entries.
    pub fn hidGetNpadStatesGc(id: HidNpadIdType, states: *mut HidNpadGcState, count: usize) -> usize;

    /// Gets `HidNpadPalmaState`.
    ///
    /// # Arguments
    /// - `states` (out): Output array of `HidNpadPalmaState`.
    /// - `count` (in): Size of the states array in entries.
    ///
    /// **Returns:** Total output entries.
    pub fn hidGetNpadStatesPalma(id: HidNpadIdType, states: *mut HidNpadPalmaState, count: usize) -> usize;

    /// Gets `HidNpadLarkState`.
    ///
    /// # Arguments
    /// - `states` (out): Output array of `HidNpadLarkState`.
    /// - `count` (in): Size of the states array in entries.
    ///
    /// **Returns:** Total output entries.
    pub fn hidGetNpadStatesLark(id: HidNpadIdType, states: *mut HidNpadLarkState, count: usize) -> usize;

    /// Gets `HidNpadHandheldLarkState`.
    ///
    /// # Arguments
    /// - `states` (out): Output array of `HidNpadHandheldLarkState`.
    /// - `count` (in): Size of the states array in entries.
    ///
    /// **Returns:** Total output entries.
    pub fn hidGetNpadStatesHandheldLark(id: HidNpadIdType, states: *mut HidNpadHandheldLarkState, count: usize) -> usize;

    /// Gets `HidNpadLuciaState`.
    ///
    /// # Arguments
    /// - `states` (out): Output array of `HidNpadLuciaState`.
    /// - `count` (in): Size of the states array in entries.
    ///
    /// **Returns:** Total output entries.
    pub fn hidGetNpadStatesLucia(id: HidNpadIdType, states: *mut HidNpadLuciaState, count: usize) -> usize;

    /// Gets `HidNpadLagerState`.
    ///
    /// # Arguments
    /// - `states` (out): Output array of `HidNpadLagerState`.
    /// - `count` (in): Size of the states array in entries.
    ///
    /// **Returns:** Total output entries.
    pub fn hidGetNpadStatesLager(id: HidNpadIdType, states: *mut HidNpadLagerState, count: usize) -> usize;

    /// Gets `HidNpadSystemExtState`.
    ///
    /// # Arguments
    /// - `states` (out): Output array of `HidNpadSystemExtState`.
    /// - `count` (in): Size of the states array in entries.
    ///
    /// **Returns:** Total output entries.
    pub fn hidGetNpadStatesSystemExt(id: HidNpadIdType, states: *mut HidNpadSystemExtState, count: usize) -> usize;

    /// Gets `HidNpadSystemState`.
    ///
    /// # Arguments
    /// - `states` (out): Output array of `HidNpadSystemState`.
    /// - `count` (in): Size of the states array in entries.
    ///
    /// **Returns:** Total output entries.
    pub fn hidGetNpadStatesSystem(id: HidNpadIdType, states: *mut HidNpadSystemState, count: usize) -> usize;

    /// Gets `HidSixAxisSensorState` for the specified handle.
    ///
    /// # Arguments
    /// - `handle` (in): `HidSixAxisSensorHandle`
    /// - `states` (out): Output array of `HidSixAxisSensorState`.
    /// - `count` (in): Size of the states array in entries.
    ///
    /// **Returns:** Total output entries.
    pub fn hidGetSixAxisSensorStates(handle: HidSixAxisSensorHandle, states: *mut HidSixAxisSensorState, count: usize) -> usize;

    /// Initialize Gesture. Must be called when Gesture is being used.
    pub fn hidInitializeGesture();

    /// Gets `HidGestureState`.
    ///
    /// # Arguments
    /// - `states` (out): Output array of `HidGestureState`.
    /// - `count` (in): Size of the states array in entries.
    ///
    /// **Returns:** Total output entries.
    pub fn hidGetGestureStates(states: *mut HidGestureState, count: usize) -> usize;

    /// SendKeyboardLockKeyEvent
    ///
    /// # Arguments
    /// - `events` (in): Bitfield of `HidKeyboardLockKeyEvent`.
    ///
    /// **Note:** Same as `hidsysSendKeyboardLockKeyEvent`.
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn hidSendKeyboardLockKeyEvent(events: u32) -> Result;

    /// Gets SixAxisSensorHandles.
    ///
    /// # Arguments
    /// - `handles` (out): Output array of `HidSixAxisSensorHandle`.
    /// - `total_handles` (in): Total handles for the handles array. Must be 1 or 2, if 2 handles aren't supported by the specified style an error is thrown.
    /// - `id` (in): `HidNpadIdType`
    /// - `style` (in): `HidNpadStyleTag`
    ///
    /// **Note:** Only ::HidNpadStyleTag_NpadJoyDual supports total_handles==2.
    pub fn hidGetSixAxisSensorHandles(handles: *mut HidSixAxisSensorHandle, total_handles: s32, id: HidNpadIdType, style: HidNpadStyleTag) -> Result;

    /// Starts the SixAxisSensor for the specified handle.
    ///
    /// # Arguments
    /// - `handle` (in): `HidSixAxisSensorHandle`
    pub fn hidStartSixAxisSensor(handle: HidSixAxisSensorHandle) -> Result;

    /// Stops the SixAxisSensor for the specified handle.
    ///
    /// # Arguments
    /// - `handle` (in): `HidSixAxisSensorHandle`
    pub fn hidStopSixAxisSensor(handle: HidSixAxisSensorHandle) -> Result;

    /// IsSixAxisSensorFusionEnabled
    ///
    /// # Arguments
    /// - `handle` (in): `HidSixAxisSensorHandle`
    /// - `out` (out): Output flag.
    pub fn hidIsSixAxisSensorFusionEnabled(handle: HidSixAxisSensorHandle, out: *mut bool) -> Result;

    /// EnableSixAxisSensorFusion
    ///
    /// # Arguments
    /// - `handle` (in): `HidSixAxisSensorHandle`
    /// - `flag` (in): Flag
    pub fn hidEnableSixAxisSensorFusion(handle: HidSixAxisSensorHandle, flag: bool) -> Result;

    /// SetSixAxisSensorFusionParameters
    ///
    /// # Arguments
    /// - `handle` (in): `HidSixAxisSensorHandle`
    /// - `unk0` (in): Must be 0.0f-1.0f.
    /// - `unk1` (in): Unknown
    pub fn hidSetSixAxisSensorFusionParameters(handle: HidSixAxisSensorHandle, unk0: f32, unk1: f32) -> Result;

    /// GetSixAxisSensorFusionParameters
    ///
    /// # Arguments
    /// - `handle` (in): `HidSixAxisSensorHandle`
    /// - `unk0` (out): Unknown
    /// - `unk1` (out): Unknown
    pub fn hidGetSixAxisSensorFusionParameters(handle: HidSixAxisSensorHandle, unk0: *mut f32, unk1: *mut f32) -> Result;

    /// ResetSixAxisSensorFusionParameters
    ///
    /// # Arguments
    /// - `handle` (in): `HidSixAxisSensorHandle`
    pub fn hidResetSixAxisSensorFusionParameters(handle: HidSixAxisSensorHandle) -> Result;

    /// Sets the ::HidGyroscopeZeroDriftMode for the specified SixAxisSensorHandle.
    ///
    /// # Arguments
    /// - `handle` (in): `HidSixAxisSensorHandle`
    /// - `mode` (in): `HidGyroscopeZeroDriftMode`
    pub fn hidSetGyroscopeZeroDriftMode(handle: HidSixAxisSensorHandle, mode: HidGyroscopeZeroDriftMode) -> Result;

    /// Gets the ::HidGyroscopeZeroDriftMode for the specified SixAxisSensorHandle.
    ///
    /// # Arguments
    /// - `handle` (in): `HidSixAxisSensorHandle`
    /// - `mode` (out): `HidGyroscopeZeroDriftMode`
    pub fn hidGetGyroscopeZeroDriftMode(handle: HidSixAxisSensorHandle, mode: *mut HidGyroscopeZeroDriftMode) -> Result;

    /// Resets the ::HidGyroscopeZeroDriftMode for the specified SixAxisSensorHandle to ::HidGyroscopeZeroDriftMode_Standard.
    ///
    /// # Arguments
    /// - `handle` (in): `HidSixAxisSensorHandle`
    pub fn hidResetGyroscopeZeroDriftMode(handle: HidSixAxisSensorHandle) -> Result;

    /// IsSixAxisSensorAtRest
    ///
    /// # Arguments
    /// - `handle` (in): `HidSixAxisSensorHandle`
    /// - `out` (out): Output flag.
    pub fn hidIsSixAxisSensorAtRest(handle: HidSixAxisSensorHandle, out: *mut bool) -> Result;

    /// IsFirmwareUpdateAvailableForSixAxisSensor
    ///
    /// # Arguments
    /// - `handle` (in): `HidSixAxisSensorHandle`
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn hidIsFirmwareUpdateAvailableForSixAxisSensor(handle: HidSixAxisSensorHandle, out: *mut bool) -> Result;

    /// Sets which controller styles are supported.
    ///
    /// # Arguments
    /// - `style_set` (in): Bitfield of `HidNpadStyleTag`.
    ///
    /// **Note:** This is automatically called with the needed styles in `hidScanInput` when required.
    pub fn hidSetSupportedNpadStyleSet(style_set: u32) -> Result;

    /// Gets which controller styles are supported.
    ///
    /// # Arguments
    /// - `style_set` (out): Bitfield of `HidNpadStyleTag`.
    pub fn hidGetSupportedNpadStyleSet(style_set: *mut u32) -> Result;

    /// Sets which `HidNpadIdType` are supported.
    ///
    /// # Arguments
    /// - `ids` (in): Input array of `HidNpadIdType`.
    /// - `count` (in): Total entries in the ids array. Must be &lt;=10.
    ///
    /// **Note:** This is automatically called with HidNpadIdType_No{1-8} and HidNpadIdType_Handheld when required in `hidScanInput`.
    pub fn hidSetSupportedNpadIdType(ids: *const HidNpadIdType, count: usize) -> Result;

    /// Gets an Event which is signaled when the `hidGetNpadStyleSet` output is updated for the specified controller.
    ///
    /// # Arguments
    /// - `id` (in): `HidNpadIdType`
    /// - `out_event` (out): Output Event.
    /// - `autoclear` (in): The autoclear for the Event.
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn hidAcquireNpadStyleSetUpdateEventHandle(id: HidNpadIdType, out_event: *mut Event, autoclear: bool) -> Result;

    /// DisconnectNpad
    ///
    /// # Arguments
    /// - `id` (in): `HidNpadIdType`
    pub fn hidDisconnectNpad(id: HidNpadIdType) -> Result;

    /// GetPlayerLedPattern
    ///
    /// # Arguments
    /// - `id` (in): `HidNpadIdType`
    /// - `out` (out): Output value.
    pub fn hidGetPlayerLedPattern(id: HidNpadIdType, out: *mut u8) -> Result;

    /// Sets the `HidNpadJoyHoldType`.
    ///
    /// # Arguments
    /// - `type` (in): `HidNpadJoyHoldType`
    ///
    /// **Note:** Used automatically by `hidScanInput` when required.
    pub fn hidSetNpadJoyHoldType(type_: HidNpadJoyHoldType) -> Result;

    /// Gets the `HidNpadJoyHoldType`.
    ///
    /// # Arguments
    /// - `type` (out): `HidNpadJoyHoldType`
    pub fn hidGetNpadJoyHoldType(type_: *mut HidNpadJoyHoldType) -> Result;

    /// This is the same as `hidSetNpadJoyAssignmentModeSingle`, except ::HidNpadJoyDeviceType_Left is used for the type.
    ///
    /// # Arguments
    /// - `id` (in): `HidNpadIdType`, must be HidNpadIdType_No*.
    pub fn hidSetNpadJoyAssignmentModeSingleByDefault(id: HidNpadIdType) -> Result;

    /// This is the same as `hidSetNpadJoyAssignmentModeSingleWithDestination`, except without the output params.
    ///
    /// # Arguments
    /// - `id` (in): `HidNpadIdType`, must be HidNpadIdType_No*.
    /// - `type` (in): `HidNpadJoyDeviceType`
    pub fn hidSetNpadJoyAssignmentModeSingle(id: HidNpadIdType, type_: HidNpadJoyDeviceType) -> Result;

    /// Use this if you want to use a pair of joy-cons as a single HidNpadIdType_No*. When used, both joy-cons in a pair should be used with this (HidNpadIdType_No1 and HidNpadIdType_No2 for example).
    ///
    /// # Arguments
    /// - `id` (in): `HidNpadIdType`, must be HidNpadIdType_No*.
    ///
    /// **Note:** Used automatically by `hidScanInput` when required.
    pub fn hidSetNpadJoyAssignmentModeDual(id: HidNpadIdType) -> Result;

    /// Merge two single joy-cons into a dual-mode controller. Use this after `hidSetNpadJoyAssignmentModeDual`, when `hidSetNpadJoyAssignmentModeSingleByDefault` was previously used (this includes using this manually at application exit).
    /// To be successful, id0/id1 must correspond to controllers supporting styles HidNpadStyleTag_NpadJoyLeft/Right, or HidNpadStyleTag_NpadJoyRight/Left.
    /// If successful, the id of the resulting dual controller is set to id0.
    ///
    /// # Arguments
    /// - `id0` (in): `HidNpadIdType`
    /// - `id1` (in): `HidNpadIdType`
    pub fn hidMergeSingleJoyAsDualJoy(id0: HidNpadIdType, id1: HidNpadIdType) -> Result;

    /// StartLrAssignmentMode
    pub fn hidStartLrAssignmentMode() -> Result;

    /// StopLrAssignmentMode
    pub fn hidStopLrAssignmentMode() -> Result;

    /// Sets the `HidNpadHandheldActivationMode`.
    ///
    /// # Arguments
    /// - `mode` (in): `HidNpadHandheldActivationMode`
    pub fn hidSetNpadHandheldActivationMode(mode: HidNpadHandheldActivationMode) -> Result;

    /// Gets the `HidNpadHandheldActivationMode`.
    ///
    /// # Arguments
    /// - `out` (out): `HidNpadHandheldActivationMode`
    pub fn hidGetNpadHandheldActivationMode(out: *mut HidNpadHandheldActivationMode) -> Result;

    /// SwapNpadAssignment
    ///
    /// # Arguments
    /// - `id0` (in): `HidNpadIdType`
    /// - `id1` (in): `HidNpadIdType`
    pub fn hidSwapNpadAssignment(id0: HidNpadIdType, id1: HidNpadIdType) -> Result;

    /// EnableUnintendedHomeButtonInputProtection
    ///
    /// # Arguments
    /// - `id` (in): `HidNpadIdType`
    /// - `flag` (in): Whether UnintendedHomeButtonInputProtection is enabled.
    ///
    /// **Note:** To get the state of this, use `hidGetNpadSystemButtonProperties` to access HidNpadSystemButtonProperties::is_unintended_home_button_input_protection_enabled.
    pub fn hidEnableUnintendedHomeButtonInputProtection(id: HidNpadIdType, flag: bool) -> Result;

    /// Use this if you want to use a single joy-con as a dedicated HidNpadIdType_No*. When used, both joy-cons in a pair should be used with this (HidNpadIdType_No1 and HidNpadIdType_No2 for example).
    ///
    /// # Arguments
    /// - `id` (in): `HidNpadIdType`, must be HidNpadIdType_No*.
    /// - `type` (in): `HidNpadJoyDeviceType`
    /// - `flag` (out): Whether the dest output is set.
    /// - `dest` (out): `HidNpadIdType`
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn hidSetNpadJoyAssignmentModeSingleWithDestination(id: HidNpadIdType, type_: HidNpadJoyDeviceType, flag: *mut bool, dest: *mut HidNpadIdType) -> Result;

    /// SetNpadAnalogStickUseCenterClamp
    ///
    /// # Arguments
    /// - `flag` (in): Flag
    ///
    /// **Note:** Only available on [6.1.0+].
    pub fn hidSetNpadAnalogStickUseCenterClamp(flag: bool) -> Result;

    /// Assigns the button(s) which trigger the CaptureButton.
    ///
    /// # Arguments
    /// - `style` (in): `HidNpadStyleTag`, exactly 1 bit must be set.
    /// - `buttons` (in): Bitfield of `HidNpadButton`, multiple bits can be set.
    ///
    /// **Note:** Only available on [8.0.0+].
    pub fn hidSetNpadCaptureButtonAssignment(style: HidNpadStyleTag, buttons: u64) -> Result;

    /// ClearNpadCaptureButtonAssignment
    ///
    /// **Note:** Only available on [8.0.0+].
    pub fn hidClearNpadCaptureButtonAssignment() -> Result;

    /// Gets and initializes vibration handles.
    ///
    /// # Arguments
    /// - `handles` (out): Output array of `HidVibrationDeviceHandle`.
    /// - `total_handles` (in): Total handles for the handles array. Must be 1 or 2, if 2 handles aren't supported by the specified style an error is thrown.
    /// - `id` (in): `HidNpadIdType`
    /// - `style` (in): `HidNpadStyleTag`
    ///
    /// **Note:** Only the following styles support total_handles 2: ::HidNpadStyleTag_NpadFullKey, ::HidNpadStyleTag_NpadHandheld, ::HidNpadStyleTag_NpadJoyDual, ::HidNpadStyleTag_NpadHandheldLark, ::HidNpadStyleTag_NpadSystem, ::HidNpadStyleTag_NpadSystemExt.
    pub fn hidInitializeVibrationDevices(handles: *mut HidVibrationDeviceHandle, total_handles: s32, id: HidNpadIdType, style: HidNpadStyleTag) -> Result;

    /// Gets `HidVibrationDeviceInfo` for the specified device.
    ///
    /// # Arguments
    /// - `handle` (in): `HidVibrationDeviceHandle`
    /// - `out` (out): `HidVibrationDeviceInfo`
    pub fn hidGetVibrationDeviceInfo(handle: HidVibrationDeviceHandle, out: *mut HidVibrationDeviceInfo) -> Result;

    /// Sends the `HidVibrationDeviceHandle` to the specified device.
    ///
    /// # Arguments
    /// - `handle` (in): `HidVibrationDeviceHandle`
    /// - `value` (in): `HidVibrationValue`
    ///
    /// **Note:** With ::HidVibrationDeviceType_GcErm, use `hidSendVibrationGcErmCommand` instead.
    pub fn hidSendVibrationValue(handle: HidVibrationDeviceHandle, value: *const HidVibrationValue) -> Result;

    /// Gets the current `HidVibrationValue` for the specified device.
    ///
    /// # Arguments
    /// - `handle` (in): `HidVibrationDeviceHandle`
    /// - `out` (out): `HidVibrationValue`
    ///
    /// **Note:** With ::HidVibrationDeviceType_GcErm, use `hidGetActualVibrationGcErmCommand` instead.
    pub fn hidGetActualVibrationValue(handle: HidVibrationDeviceHandle, out: *mut HidVibrationValue) -> Result;

    /// Sets whether vibration is allowed, this also affects the config displayed by System Settings.
    ///
    /// # Arguments
    /// - `flag` (in): Flag
    pub fn hidPermitVibration(flag: bool) -> Result;

    /// Gets whether vibration is allowed.
    ///
    /// # Arguments
    /// - `flag` (out): Flag
    pub fn hidIsVibrationPermitted(flag: *mut bool) -> Result;

    /// Send vibration values\[index\] to handles\[index\].
    ///
    /// # Arguments
    /// - `handles` (in): Input array of `HidVibrationDeviceHandle`.
    /// - `values` (in): Input array of `HidVibrationValue`.
    /// - `count` (in): Total entries in the handles/values arrays.
    ///
    /// **Note:** With ::HidVibrationDeviceType_GcErm, use `hidSendVibrationGcErmCommand` instead.
    pub fn hidSendVibrationValues(handles: *const HidVibrationDeviceHandle, values: *const HidVibrationValue, count: s32) -> Result;

    /// Send `HidVibrationGcErmCommand` to the specified device, for ::HidVibrationDeviceType_GcErm.
    ///
    /// # Arguments
    /// - `handle` (in): `HidVibrationDeviceHandle`
    /// - `cmd` (in): `HidVibrationGcErmCommand`
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn hidSendVibrationGcErmCommand(handle: HidVibrationDeviceHandle, cmd: HidVibrationGcErmCommand) -> Result;

    /// Get `HidVibrationGcErmCommand` for the specified device, for ::HidVibrationDeviceType_GcErm.
    ///
    /// # Arguments
    /// - `handle` (in): `HidVibrationDeviceHandle`
    /// - `out` (out): `HidVibrationGcErmCommand`
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn hidGetActualVibrationGcErmCommand(handle: HidVibrationDeviceHandle, out: *mut HidVibrationGcErmCommand) -> Result;

    /// Begins a forced-permitted vibration session.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn hidBeginPermitVibrationSession() -> Result;

    /// Ends the session started by `hidBeginPermitVibrationSession`.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn hidEndPermitVibrationSession() -> Result;

    /// Gets whether vibration is available with the specified device.
    ///
    /// # Arguments
    /// - `handle` (in): `HidVibrationDeviceHandle`
    /// - `flag` (out): Flag
    ///
    /// **Note:** Only available on [7.0.0+].
    pub fn hidIsVibrationDeviceMounted(handle: HidVibrationDeviceHandle, flag: *mut bool) -> Result;

    /// Starts the SevenSixAxisSensor.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn hidStartSevenSixAxisSensor() -> Result;

    /// Stops the SevenSixAxisSensor.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn hidStopSevenSixAxisSensor() -> Result;

    /// Initializes the SevenSixAxisSensor.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn hidInitializeSevenSixAxisSensor() -> Result;

    /// Finalizes the SevenSixAxisSensor.
    ///
    /// **Note:** This must be called before `hidExit`.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn hidFinalizeSevenSixAxisSensor() -> Result;

    /// Sets the SevenSixAxisSensor FusionStrength.
    ///
    /// # Arguments
    /// - `strength` (in): Strength
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn hidSetSevenSixAxisSensorFusionStrength(strength: f32) -> Result;

    /// Gets the SevenSixAxisSensor FusionStrength.
    ///
    /// # Arguments
    /// - `strength` (out): Strength
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn hidGetSevenSixAxisSensorFusionStrength(strength: *mut f32) -> Result;

    /// Resets the timestamp for the SevenSixAxisSensor.
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn hidResetSevenSixAxisSensorTimestamp() -> Result;

    /// GetSevenSixAxisSensorStates
    ///
    /// # Arguments
    /// - `states` (out): Output array of `HidSevenSixAxisSensorState`.
    /// - `count` (in): Size of the states array in entries.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** Only available when `hidInitializeSevenSixAxisSensor` was previously used.
    pub fn hidGetSevenSixAxisSensorStates(states: *mut HidSevenSixAxisSensorState, count: usize, total_out: *mut usize) -> Result;

    /// IsSevenSixAxisSensorAtRest
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available when `hidInitializeSevenSixAxisSensor` was previously used.
    pub fn hidIsSevenSixAxisSensorAtRest(out: *mut bool) -> Result;

    /// GetSensorFusionError
    ///
    /// # Arguments
    /// - `out` (out): Output data.
    ///
    /// **Note:** Only available when `hidInitializeSevenSixAxisSensor` was previously used.
    pub fn hidGetSensorFusionError(out: *mut f32) -> Result;

    /// GetGyroBias
    ///
    /// # Arguments
    /// - `out` (out): `UtilFloat3`
    ///
    /// **Note:** Only available when `hidInitializeSevenSixAxisSensor` was previously used.
    pub fn hidGetGyroBias(out: *mut UtilFloat3) -> Result;

    /// IsUsbFullKeyControllerEnabled
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn hidIsUsbFullKeyControllerEnabled(out: *mut bool) -> Result;

    /// EnableUsbFullKeyController
    ///
    /// # Arguments
    /// - `flag` (in): Flag
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn hidEnableUsbFullKeyController(flag: bool) -> Result;

    /// IsUsbFullKeyControllerConnected
    ///
    /// # Arguments
    /// - `id` (in): `HidNpadIdType`
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn hidIsUsbFullKeyControllerConnected(id: HidNpadIdType, out: *mut bool) -> Result;

    /// Gets the `HidNpadInterfaceType` for the specified controller.
    ///
    /// # Arguments
    /// - `id` (in): `HidNpadIdType`
    /// - `out` (out): `HidNpadInterfaceType`
    ///
    /// **Note:** When available, `hidsysGetNpadInterfaceType` should be used instead.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn hidGetNpadInterfaceType(id: HidNpadIdType, out: *mut u8) -> Result;

    /// GetNpadOfHighestBatteryLevel
    ///
    /// # Arguments
    /// - `ids` (in): Input array of `HidNpadIdType`, ::HidNpadIdType_Handheld is ignored.
    /// - `count` (in): Total entries in the ids array.
    /// - `out` (out): `HidNpadIdType`
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn hidGetNpadOfHighestBatteryLevel(ids: *const HidNpadIdType, count: usize, out: *mut HidNpadIdType) -> Result;

    /// GetPalmaConnectionHandle
    ///
    /// # Arguments
    /// - `id` (in): `HidNpadIdType`
    /// - `out` (out): `HidPalmaConnectionHandle`
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn hidGetPalmaConnectionHandle(id: HidNpadIdType, out: *mut HidPalmaConnectionHandle) -> Result;

    /// InitializePalma
    ///
    /// # Arguments
    /// - `handle` (in): `HidPalmaConnectionHandle`
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn hidInitializePalma(handle: HidPalmaConnectionHandle) -> Result;

    /// Gets an Event which is signaled when data is available with `hidGetPalmaOperationInfo`.
    ///
    /// # Arguments
    /// - `handle` (in): `HidPalmaConnectionHandle`
    /// - `out_event` (out): Output Event.
    /// - `autoclear` (in): The autoclear for the Event.
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn hidAcquirePalmaOperationCompleteEvent(handle: HidPalmaConnectionHandle, out_event: *mut Event, autoclear: bool) -> Result;

    /// Gets `HidPalmaOperationInfo` for a completed operation.
    ///
    /// # Arguments
    /// - `handle` (in): `HidPalmaConnectionHandle`
    /// - `out` (out): `HidPalmaOperationInfo`
    ///
    /// **Note:** This must be used at some point following using any of the other Palma cmds which trigger an Operation, once the Event from `hidAcquirePalmaOperationCompleteEvent` is signaled. Up to 4 Operations can be queued at once, the other cmds will throw an error once there's too many operations.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn hidGetPalmaOperationInfo(handle: HidPalmaConnectionHandle, out: *mut HidPalmaOperationInfo) -> Result;

    /// PlayPalmaActivity
    ///
    /// # Arguments
    /// - `handle` (in): `HidPalmaConnectionHandle`
    /// - `val` (in): Input value.
    ///
    /// **Note:** See `hidGetPalmaOperationInfo`.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn hidPlayPalmaActivity(handle: HidPalmaConnectionHandle, val: u16) -> Result;

    /// SetPalmaFrModeType
    ///
    /// # Arguments
    /// - `handle` (in): `HidPalmaConnectionHandle`
    /// - `type` (in): `HidPalmaFrModeType`
    ///
    /// **Note:** See `hidGetPalmaOperationInfo`.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn hidSetPalmaFrModeType(handle: HidPalmaConnectionHandle, type_: HidPalmaFrModeType) -> Result;

    /// ReadPalmaStep
    ///
    /// # Arguments
    /// - `handle` (in): `HidPalmaConnectionHandle`
    ///
    /// **Note:** See `hidGetPalmaOperationInfo`.
    ///
    /// **Note:** `hidEnablePalmaStep` should be used before this.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn hidReadPalmaStep(handle: HidPalmaConnectionHandle) -> Result;

    /// EnablePalmaStep
    ///
    /// # Arguments
    /// - `handle` (in): `HidPalmaConnectionHandle`
    /// - `flag` (in): Flag
    ///
    /// **Note:** See `hidGetPalmaOperationInfo`.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn hidEnablePalmaStep(handle: HidPalmaConnectionHandle, flag: bool) -> Result;

    /// ResetPalmaStep
    ///
    /// # Arguments
    /// - `handle` (in): `HidPalmaConnectionHandle`
    ///
    /// **Note:** See `hidGetPalmaOperationInfo`.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn hidResetPalmaStep(handle: HidPalmaConnectionHandle) -> Result;

    /// ReadPalmaApplicationSection
    ///
    /// # Arguments
    /// - `handle` (in): `HidPalmaConnectionHandle`
    /// - `inval0` (in): First input value.
    /// - `size` (in): This must be within the size of `HidPalmaApplicationSectionAccessBuffer`.
    ///
    /// **Note:** See `hidGetPalmaOperationInfo`.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn hidReadPalmaApplicationSection(handle: HidPalmaConnectionHandle, inval0: s32, size: u64) -> Result;

    /// WritePalmaApplicationSection
    ///
    /// # Arguments
    /// - `handle` (in): `HidPalmaConnectionHandle`
    /// - `inval0` (in): First input value.
    /// - `size` (in): Size of the data in `HidPalmaApplicationSectionAccessBuffer`.
    /// - `buf` (in): `HidPalmaApplicationSectionAccessBuffer`
    ///
    /// **Note:** See `hidGetPalmaOperationInfo`.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn hidWritePalmaApplicationSection(handle: HidPalmaConnectionHandle, inval0: s32, size: u64, buf: *const HidPalmaApplicationSectionAccessBuffer) -> Result;

    /// ReadPalmaUniqueCode
    ///
    /// # Arguments
    /// - `handle` (in): `HidPalmaConnectionHandle`
    ///
    /// **Note:** See `hidGetPalmaOperationInfo`.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn hidReadPalmaUniqueCode(handle: HidPalmaConnectionHandle) -> Result;

    /// SetPalmaUniqueCodeInvalid
    ///
    /// # Arguments
    /// - `handle` (in): `HidPalmaConnectionHandle`
    ///
    /// **Note:** See `hidGetPalmaOperationInfo`.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn hidSetPalmaUniqueCodeInvalid(handle: HidPalmaConnectionHandle) -> Result;

    /// WritePalmaActivityEntry
    ///
    /// # Arguments
    /// - `handle` (in): `HidPalmaConnectionHandle`
    /// - `unk` (in): Unknown
    /// - `entry` (in): `HidPalmaActivityEntry`
    ///
    /// **Note:** See `hidGetPalmaOperationInfo`.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn hidWritePalmaActivityEntry(handle: HidPalmaConnectionHandle, unk: u16, entry: *const HidPalmaActivityEntry) -> Result;

    /// WritePalmaRgbLedPatternEntry
    ///
    /// # Arguments
    /// - `handle` (in): `HidPalmaConnectionHandle`
    /// - `unk` (in): Unknown
    /// - `buffer` (in): Input buffer.
    /// - `size` (in): Input buffer size.
    ///
    /// **Note:** See `hidGetPalmaOperationInfo`.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn hidWritePalmaRgbLedPatternEntry(handle: HidPalmaConnectionHandle, unk: u16, buffer: *const c_void, size: usize) -> Result;

    /// WritePalmaWaveEntry
    ///
    /// # Arguments
    /// - `handle` (in): `HidPalmaConnectionHandle`
    /// - `wave_set` (in): `HidPalmaWaveSet`
    /// - `unk` (in): Unknown
    /// - `buffer` (in): TransferMemory buffer, must be 0x1000-byte aligned.
    /// - `tmem_size` (in): TransferMemory buffer size, must be 0x1000-byte aligned.
    /// - `size` (in): Actual size of the data in the buffer.
    ///
    /// **Note:** See `hidGetPalmaOperationInfo`.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn hidWritePalmaWaveEntry(handle: HidPalmaConnectionHandle, wave_set: HidPalmaWaveSet, unk: u16, buffer: *const c_void, tmem_size: usize, size: usize) -> Result;

    /// SetPalmaDataBaseIdentificationVersion
    ///
    /// # Arguments
    /// - `handle` (in): `HidPalmaConnectionHandle`
    /// - `version` (in): Version
    ///
    /// **Note:** See `hidGetPalmaOperationInfo`.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn hidSetPalmaDataBaseIdentificationVersion(handle: HidPalmaConnectionHandle, version: s32) -> Result;

    /// GetPalmaDataBaseIdentificationVersion
    ///
    /// # Arguments
    /// - `handle` (in): `HidPalmaConnectionHandle`
    ///
    /// **Note:** See `hidGetPalmaOperationInfo`.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn hidGetPalmaDataBaseIdentificationVersion(handle: HidPalmaConnectionHandle) -> Result;

    /// SuspendPalmaFeature
    ///
    /// # Arguments
    /// - `handle` (in): `HidPalmaConnectionHandle`
    /// - `features` (in): Bitfield of `HidPalmaFeature`.
    ///
    /// **Note:** See `hidGetPalmaOperationInfo`.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn hidSuspendPalmaFeature(handle: HidPalmaConnectionHandle, features: u32) -> Result;

    /// ReadPalmaPlayLog
    ///
    /// # Arguments
    /// - `handle` (in): `HidPalmaConnectionHandle`
    /// - `unk` (in): Unknown
    ///
    /// **Note:** See `hidGetPalmaOperationInfo`.
    ///
    /// **Note:** Only available on [5.1.0+].
    pub fn hidReadPalmaPlayLog(handle: HidPalmaConnectionHandle, unk: u16) -> Result;

    /// ResetPalmaPlayLog
    ///
    /// # Arguments
    /// - `handle` (in): `HidPalmaConnectionHandle`
    /// - `unk` (in): Unknown
    ///
    /// **Note:** See `hidGetPalmaOperationInfo`.
    ///
    /// **Note:** Only available on [5.1.0+].
    pub fn hidResetPalmaPlayLog(handle: HidPalmaConnectionHandle, unk: u16) -> Result;

    /// Sets whether any Palma can connect.
    ///
    /// # Arguments
    /// - `flag` (in): Flag
    ///
    /// **Note:** Only available on [5.1.0+].
    pub fn hidSetIsPalmaAllConnectable(flag: bool) -> Result;

    /// Sets whether paired Palma can connect.
    ///
    /// # Arguments
    /// - `flag` (in): Flag
    ///
    /// **Note:** Only available on [5.1.0+].
    pub fn hidSetIsPalmaPairedConnectable(flag: bool) -> Result;

    /// PairPalma
    ///
    /// # Arguments
    /// - `handle` (in): `HidPalmaConnectionHandle`
    ///
    /// **Note:** Only available on [5.1.0+].
    pub fn hidPairPalma(handle: HidPalmaConnectionHandle) -> Result;

    /// CancelWritePalmaWaveEntry
    ///
    /// # Arguments
    /// - `handle` (in): `HidPalmaConnectionHandle`
    ///
    /// **Note:** Only available on [7.0.0+].
    pub fn hidCancelWritePalmaWaveEntry(handle: HidPalmaConnectionHandle) -> Result;

    /// EnablePalmaBoostMode
    ///
    /// # Arguments
    /// - `flag` (in): Flag
    ///
    /// **Note:** Only available on [5.1.0+]. Uses cmd EnablePalmaBoostMode on [8.0.0+], otherwise cmd SetPalmaBoostMode is used.
    pub fn hidEnablePalmaBoostMode(flag: bool) -> Result;

    /// GetPalmaBluetoothAddress
    ///
    /// # Arguments
    /// - `handle` (in): `HidPalmaConnectionHandle`
    /// - `out` (out): `BtdrvAddress`
    ///
    /// **Note:** Only available on [8.0.0+].
    pub fn hidGetPalmaBluetoothAddress(handle: HidPalmaConnectionHandle, out: *mut BtdrvAddress) -> Result;

    /// SetDisallowedPalmaConnection
    ///
    /// # Arguments
    /// - `addrs` (in): Input array of `BtdrvAddress`.
    /// - `count` (in): Total entries in the addrs array.
    ///
    /// **Note:** Only available on [8.0.0+].
    pub fn hidSetDisallowedPalmaConnection(addrs: *const BtdrvAddress, count: s32) -> Result;

    /// SetNpadCommunicationMode
    ///
    /// # Arguments
    /// - `mode` (in): `HidNpadCommunicationMode`
    ///
    /// **Note:** [2.0.0+] Stubbed, just returns 0.
    pub fn hidSetNpadCommunicationMode(mode: HidNpadCommunicationMode) -> Result;

    /// GetNpadCommunicationMode
    ///
    /// # Arguments
    /// - `out` (out): `HidNpadCommunicationMode`
    ///
    /// **Note:** [2.0.0+] Stubbed, always returns output mode ::HidNpadCommunicationMode_Default.
    pub fn hidGetNpadCommunicationMode(out: *mut HidNpadCommunicationMode) -> Result;

    /// SetTouchScreenConfiguration
    ///
    /// # Arguments
    /// - `config` (in): `HidTouchScreenConfigurationForNx`
    ///
    /// **Note:** Only available on [9.0.0+].
    pub fn hidSetTouchScreenConfiguration(config: *const HidTouchScreenConfigurationForNx) -> Result;

    /// IsFirmwareUpdateNeededForNotification
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [9.0.0+].
    pub fn hidIsFirmwareUpdateNeededForNotification(out: *mut bool) -> Result;
}
