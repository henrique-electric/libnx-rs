//! `switch/services/nim.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// SystemUpdateTaskId
#[repr(C, align(8))]
#[derive(Debug, Clone, Copy)]
pub struct NimSystemUpdateTaskId {
    /// UUID
    pub uuid: Uuid,
}

impl_zeroed_default!(NimSystemUpdateTaskId);

unsafe extern "C" {
    /// Initialize nim.
    pub fn nimInitialize() -> Result;

    /// Exit nim.
    pub fn nimExit();

    /// Gets the Service object for the actual nim service session.
    pub fn nimGetServiceSession() -> *mut Service;

    pub fn nimListSystemUpdateTask(out_count: *mut s32, out_task_ids: *mut NimSystemUpdateTaskId, max_task_ids: usize) -> Result;

    pub fn nimDestroySystemUpdateTask(task_id: *const NimSystemUpdateTaskId) -> Result;
}
