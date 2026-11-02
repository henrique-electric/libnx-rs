//! `switch/services/vi.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ViDisplayName {
    pub data: [c_char; 64],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ViDisplay {
    pub display_id: u64,
    pub display_name: ViDisplayName,
    pub initialized: bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ViLayer {
    pub layer_id: u64,
    pub igbp_binder_obj_id: u32,
    pub _bitfield_0: [u8; 1],
}

impl ViLayer {
    bitfields!(_bitfield_0 {
        initialized / set_initialized: 0, 1 => bool;
        stray_layer / set_stray_layer: 1, 1 => bool;
    });
}

pub type ViServiceType = i32;
pub const ViServiceType_Default: ViServiceType = -1;
pub const ViServiceType_Application: ViServiceType = 0;
pub const ViServiceType_System: ViServiceType = 1;
pub const ViServiceType_Manager: ViServiceType = 2;

/// Used by viCreateLayer when CreateStrayLayer is used internally.
pub type ViLayerFlags = u32;
pub const ViLayerFlags_Default: ViLayerFlags = 0x1;

/// Used with viSetLayerScalingMode.
pub type ViScalingMode = u32;
pub const ViScalingMode_None: ViScalingMode = 0x0;
pub const ViScalingMode_FitToLayer: ViScalingMode = 0x2;
pub const ViScalingMode_PreserveAspectRatio: ViScalingMode = 0x4;
pub const ViScalingMode_Default: ViScalingMode = ViScalingMode_FitToLayer;

/// Used with viSetDisplayPowerState.
pub type ViPowerState = u32;
/// Screen is off.
pub const ViPowerState_Off: ViPowerState = 0;
/// [3.0.0+] Screen is on, but not scanning content.
pub const ViPowerState_NotScanning: ViPowerState = 1;
/// [3.0.0+] Screen is on.
pub const ViPowerState_On: ViPowerState = 2;
/// [1.0.0 - 2.3.0] Screen is on.
pub const ViPowerState_On_Deprecated: ViPowerState = 1;

/// Used as argument to many capture functions.
pub type ViLayerStack = u32;
/// Default layer stack, includes all layers.
pub const ViLayerStack_Default: ViLayerStack = 0;
/// Includes only layers for the LCD.
pub const ViLayerStack_Lcd: ViLayerStack = 1;
/// Includes only layers for user screenshots.
pub const ViLayerStack_Screenshot: ViLayerStack = 2;
/// Includes only layers for recording videos.
pub const ViLayerStack_Recording: ViLayerStack = 3;
/// Includes only layers for the last applet-transition frame.
pub const ViLayerStack_LastFrame: ViLayerStack = 4;
/// Captures some arbitrary layer. This is normally only for am.
pub const ViLayerStack_Arbitrary: ViLayerStack = 5;
/// Captures layers for the current application. This is normally used by creport/debugging tools.
pub const ViLayerStack_ApplicationForDebug: ViLayerStack = 6;
/// Layer stack for the empty display.
pub const ViLayerStack_Null: ViLayerStack = 10;

/// Used as argument for certain drawing commands.
pub type ViColorRgba4444 = u16;

pub type ViColorRgba8888 = u32;

/// # Safety
/// The vi service must be initialized.
#[inline(always)]
pub unsafe fn viOpenDefaultDisplay(display: &mut ViDisplay) -> Result {
    unsafe { viOpenDisplay(c"Default".as_ptr(), display) }
}

impl_zeroed_default!(ViDisplayName, ViDisplay, ViLayer);

unsafe extern "C" {
    pub fn viInitialize(service_type: ViServiceType) -> Result;

    pub fn viExit();

    pub fn viGetSession_IApplicationDisplayService() -> *mut Service;

    pub fn viGetSession_IHOSBinderDriverRelay() -> *mut Service;

    pub fn viGetSession_ISystemDisplayService() -> *mut Service;

    pub fn viGetSession_IManagerDisplayService() -> *mut Service;

    pub fn viGetSession_IHOSBinderDriverIndirect() -> *mut Service;

    pub fn viSetContentVisibility(v: bool) -> Result;

    pub fn viOpenDisplay(display_name: *const c_char, display: *mut ViDisplay) -> Result;

    pub fn viCloseDisplay(display: *mut ViDisplay) -> Result;

    pub fn viGetDisplayResolution(display: *mut ViDisplay, width: *mut s32, height: *mut s32) -> Result;

    pub fn viGetDisplayLogicalResolution(display: *mut ViDisplay, width: *mut s32, height: *mut s32) -> Result;

    /// Only available on [3.0.0+].
    pub fn viSetDisplayMagnification(display: *mut ViDisplay, x: s32, y: s32, width: s32, height: s32) -> Result;

    pub fn viGetDisplayVsyncEvent(display: *mut ViDisplay, event_out: *mut Event) -> Result;

    pub fn viSetDisplayPowerState(display: *mut ViDisplay, state: ViPowerState) -> Result;

    pub fn viSetDisplayAlpha(display: *mut ViDisplay, alpha: f32) -> Result;

    pub fn viGetZOrderCountMin(display: *mut ViDisplay, z: *mut s32) -> Result;

    pub fn viGetZOrderCountMax(display: *mut ViDisplay, z: *mut s32) -> Result;

    pub fn viCreateLayer(display: *const ViDisplay, layer: *mut ViLayer) -> Result;

    pub fn viCreateManagedLayer(display: *const ViDisplay, layer_flags: ViLayerFlags, aruid: u64, layer_id: *mut u64) -> Result;

    pub fn viSetLayerSize(layer: *mut ViLayer, width: s32, height: s32) -> Result;

    pub fn viSetLayerZ(layer: *mut ViLayer, z: s32) -> Result;

    pub fn viSetLayerPosition(layer: *mut ViLayer, x: f32, y: f32) -> Result;

    pub fn viCloseLayer(layer: *mut ViLayer) -> Result;

    pub fn viDestroyManagedLayer(layer: *mut ViLayer) -> Result;

    pub fn viSetLayerScalingMode(layer: *mut ViLayer, scaling_mode: ViScalingMode) -> Result;

    pub fn viGetIndirectLayerImageMap(buffer: *mut c_void, size: usize, width: s32, height: s32, IndirectLayerConsumerHandle: u64, out_size: *mut u64, out_stride: *mut u64) -> Result;

    pub fn viGetIndirectLayerImageRequiredMemoryInfo(width: s32, height: s32, out_size: *mut u64, out_alignment: *mut u64) -> Result;

    /// [16.0.0+]
    pub fn viManagerPrepareFatal() -> Result;

    /// [16.0.0+]
    pub fn viManagerShowFatal() -> Result;

    /// [16.0.0+]
    pub fn viManagerDrawFatalRectangle(x: s32, y: s32, end_x: s32, end_y: s32, color: ViColorRgba4444) -> Result;

    /// [16.0.0+]
    pub fn viManagerDrawFatalText32(out_advance: *mut s32, x: s32, y: s32, utf32_codepoints: *const u32, num_codepoints: usize, scale_x: f32, scale_y: f32, font_type: PlSharedFontType, bg_color: ViColorRgba8888, fg_color: ViColorRgba8888, initial_advance: s32) -> Result;
}
