//! `switch/kernel/barrier.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Barrier structure.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Barrier {
    /// Number of threads to reach the barrier.
    pub count: u64,
    /// Number of threads to wait on.
    pub total: u64,
    pub mutex: Mutex,
    pub condvar: CondVar,
}

impl_zeroed_default!(Barrier);

unsafe extern "C" {
    /// Initializes a barrier and the number of threads to wait on.
    ///
    /// # Arguments
    /// - `b`: Barrier object.
    /// - `thread_count`: Initial value for the number of threads the barrier must wait for.
    pub fn barrierInit(b: *mut Barrier, thread_count: u64);

    /// Forces threads to wait until all threads have called barrierWait.
    ///
    /// # Arguments
    /// - `b`: Barrier object.
    pub fn barrierWait(b: *mut Barrier);
}
