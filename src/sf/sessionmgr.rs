//! `switch/sf/sessionmgr.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const NX_SESSION_MGR_MAX_SESSIONS: u32 = 16;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SessionMgr {
    pub sessions: [Handle; 16],
    pub num_sessions: u32,
    pub free_mask: u32,
    pub mutex: Mutex,
    pub condvar: CondVar,
    pub num_waiters: u32,
}

#[inline(always)]
pub fn sessionmgrGetClientSession(mgr: &SessionMgr, slot: c_int) -> Handle {
    mgr.sessions[slot as usize]
}

impl_zeroed_default!(SessionMgr);

unsafe extern "C" {
    pub fn sessionmgrCreate(mgr: *mut SessionMgr, root_session: Handle, num_sessions: u32) -> Result;

    pub fn sessionmgrClose(mgr: *mut SessionMgr);

    pub fn sessionmgrAttachClient(mgr: *mut SessionMgr) -> c_int;

    pub fn sessionmgrDetachClient(mgr: *mut SessionMgr, slot: c_int);
}
