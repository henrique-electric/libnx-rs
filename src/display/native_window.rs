//! `switch/display/native_window.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Native window structure.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NWindow {
    pub magic: u32,
    pub bq: Binder,
    pub event: Event,
    pub mutex: Mutex,
    pub slots_configured: u64,
    pub slots_requested: u64,
    pub cur_slot: s32,
    pub width: u32,
    pub height: u32,
    pub format: u32,
    pub usage: u32,
    pub crop: BqRect,
    pub scaling_mode: u32,
    pub transform: u32,
    pub sticky_transform: u32,
    pub default_width: u32,
    pub default_height: u32,
    pub swap_interval: u32,
    pub is_connected: bool,
    pub producer_controlled_by_app: bool,
    pub consumer_running_behind: bool,
}

/// Checks whether the consumer of a `NWindow` is running behind.
#[inline(always)]
pub fn nwindowIsConsumerRunningBehind(nw: &NWindow) -> bool {
    nw.consumer_running_behind
}

impl_zeroed_default!(NWindow);

unsafe extern "C" {
    /// Checks whether a pointer refers to a valid `NWindow` object.
    pub fn nwindowIsValid(nw: *mut NWindow) -> bool;

    /// Retrieves the default `NWindow` object.
    ///
    /// **Returns:** Pointer to the default `NWindow` object.
    ///
    /// **Note:** When this function is used/referenced, libnx will initialize VI services
    /// and create a `NWindow` object from a `ViLayer` created on the default `ViDisplay`;
    /// all of this happening automatically during application startup (i.e. before main is called).
    /// If creating the default `NWindow` fails, libnx will throw a LibnxError_BadGfxInit fatal error.
    /// Likewise, after main returns (or exit is called) libnx will clean up all resources used by it.
    pub fn nwindowGetDefault() -> *mut NWindow;

    /// Creates a `NWindow`.
    ///
    /// # Arguments
    /// - `nw` (out): Output `NWindow` structure.
    /// - `binder_session` (in): Service object for the Android IGraphicBufferProducer binder session.
    /// - `binder_id` (in): Android IGraphicBufferProducer binder session ID.
    /// - `producer_controlled_by_app` (in): Specifies whether the producer is controlled by the application.
    pub fn nwindowCreate(nw: *mut NWindow, binder_session: *mut Service, binder_id: s32, producer_controlled_by_app: bool) -> Result;

    /// Creates a `NWindow` operating on a `ViLayer`.
    ///
    /// # Arguments
    /// - `nw` (out): Output `NWindow` structure.
    /// - `layer` (in): Pointer to `ViLayer` structure (such as the one returned by `viCreateLayer`).
    pub fn nwindowCreateFromLayer(nw: *mut NWindow, layer: *const ViLayer) -> Result;

    /// Closes a `NWindow`, freeing all resources associated with it.
    pub fn nwindowClose(nw: *mut NWindow);

    /// Retrieves the dimensions of a `NWindow`.
    ///
    /// # Arguments
    /// - `nw` (in): Pointer to `NWindow` structure.
    /// - `out_width` (out): Output variable containing the width of the `NWindow`.
    /// - `out_height` (out): Output variable containing the height of the `NWindow`.
    ///
    /// **Note:** After creation, a `NWindow` reports a default size (usually 1280x720).
    /// This size can be overriden by calling `nwindowSetDimensions`.
    pub fn nwindowGetDimensions(nw: *mut NWindow, out_width: *mut u32, out_height: *mut u32) -> Result;

    /// Sets the dimensions of a `NWindow`.
    ///
    /// # Arguments
    /// - `nw` (in): Pointer to `NWindow` structure.
    /// - `width` (in): Desired width of the `NWindow`.
    /// - `height` (in): Desired width of the `NWindow`.
    ///
    /// **Note:** This function cannot be called when there are buffers registered with the `NWindow`.
    pub fn nwindowSetDimensions(nw: *mut NWindow, width: u32, height: u32) -> Result;

    /// Configures the crop applied to images presented through a `NWindow`.
    ///
    /// # Arguments
    /// - `nw` (in): Pointer to `NWindow` structure.
    /// - `left` (in): X coordinate of the left margin of the crop bounding box.
    /// - `top` (in): Y coordinate of the top margin of the crop bounding box.
    /// - `right` (in): X coordinate of the right margin of the crop bounding box.
    /// - `bottom` (in): Y coordinate of the bottom margin of the crop bounding box.
    ///
    /// **Note:** Passing zero to all parameters disables the crop functionality. This is also the default.
    ///
    /// **Note:** The bounding box defined by the parameters will be adjusted to fit within the dimensions of the `NWindow`.
    ///
    /// **Note:** `left` must be less or equal than `right`.
    ///
    /// **Note:** `top` must be less or equal than `bottom`.
    pub fn nwindowSetCrop(nw: *mut NWindow, left: s32, top: s32, right: s32, bottom: s32) -> Result;

    /// Configures the transformation applied to images presented through a `NWindow`.
    ///
    /// # Arguments
    /// - `nw` (in): Pointer to `NWindow` structure.
    /// - `transform` (in): Android transformation mode (see NATIVE_WINDOW_TRANSFORM_* enum)
    ///
    /// **Note:** The default transformation is 0 (i.e. no transformation applied)
    pub fn nwindowSetTransform(nw: *mut NWindow, transform: u32) -> Result;

    /// Configures the swap interval of a `NWindow`.
    ///
    /// # Arguments
    /// - `nw` (in): Pointer to `NWindow` structure.
    /// - `swap_interval` (in): Value specifying the number of display refreshes (VBlanks) that must occur between presenting images.
    ///
    /// **Note:** The default swap interval is 1.
    ///
    /// **Note:** If the `NWindow` has three or more buffers configured (with `nwindowConfigureBuffer`), it is possible to pass 0
    /// to disable the swap interval feature and present images as fast as allowed by the compositor. Otherwise, the system
    /// enforces a minimum of 1 as the swap interval.
    pub fn nwindowSetSwapInterval(nw: *mut NWindow, swap_interval: u32) -> Result;

    /// Registers a `NvGraphicBuffer` with a `NWindow`.
    ///
    /// # Arguments
    /// - `nw` (in): Pointer to `NWindow` structure.
    /// - `slot` (in): ID of the slot to configure (starting from 0).
    /// - `buf` (in): Pointer to `NvGraphicBuffer` structure.
    ///
    /// **Note:** When a buffer is registered, it is added to the internal queue of buffers used for presenting.
    ///
    /// **Note:** All buffers registered with a `NWindow` must have the same dimensions, format and usage.
    /// If `nwindowSetDimensions` has not been previously called, the `NWindow` will automatically
    /// adopt the dimensions of the first buffer registered with it. Otherwise, said buffer will need
    /// to match the dimensions that were previously configured.
    pub fn nwindowConfigureBuffer(nw: *mut NWindow, slot: s32, buf: *mut NvGraphicBuffer) -> Result;

    /// Dequeues a buffer from a `NWindow`.
    ///
    /// # Arguments
    /// - `nw` (in): Pointer to `NWindow` structure.
    /// - `out_slot` (out): Output variable containing the ID of the slot that has been dequeued.
    /// - `out_fence` (out): Output variable containing a `NvMultiFence` that will be signalled by
    ///   the compositor when the buffer is ready to be written to. Pass NULL to wait instead
    ///   on this fence before this function returns.
    ///
    /// **Note:** For `out_fence`=NULL to work, `nvFenceInit` must have been previously called.
    pub fn nwindowDequeueBuffer(nw: *mut NWindow, out_slot: *mut s32, out_fence: *mut NvMultiFence) -> Result;

    /// Cancels a buffer previously dequeued with `nwindowDequeueBuffer`.
    ///
    /// # Arguments
    /// - `nw` (in): Pointer to `NWindow` structure.
    /// - `slot` (in): ID of the slot to cancel. This must match the output of the previous `nwindowDequeueBuffer` call.
    /// - `fence` (in): Pointer to the `NvMultiFence` that will be waited on by the compositor before cancelling the buffer.
    ///   Pass NULL if there is no such fence.
    pub fn nwindowCancelBuffer(nw: *mut NWindow, slot: s32, fence: *const NvMultiFence) -> Result;

    /// Queues a buffer previously dequeued with `nwindowDequeueBuffer`, making it ready for presentation.
    ///
    /// # Arguments
    /// - `nw` (in): Pointer to `NWindow` structure.
    /// - `slot` (in): ID of the slot to queue. This must match the output of the previous `nwindowDequeueBuffer` call.
    /// - `fence` (in): Pointer to the `NvMultiFence` that will be waited on by the compositor before queuing/presenting the buffer.
    ///   Pass NULL if there is no such fence.
    pub fn nwindowQueueBuffer(nw: *mut NWindow, slot: s32, fence: *const NvMultiFence) -> Result;

    /// Releases all buffers registered with a `NWindow`.
    pub fn nwindowReleaseBuffers(nw: *mut NWindow) -> Result;
}
