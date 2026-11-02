//! `switch/services/lr.h`

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
pub struct LrLocationResolver {
    pub s: Service,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LrRegisteredLocationResolver {
    pub s: Service,
}

impl_zeroed_default!(LrLocationResolver, LrRegisteredLocationResolver);

unsafe extern "C" {
    /// Initialize lr.
    pub fn lrInitialize() -> Result;

    /// Exit lr.
    pub fn lrExit();

    /// Gets the Service object for the actual lr service session.
    pub fn lrGetServiceSession() -> *mut Service;

    pub fn lrOpenLocationResolver(storage: NcmStorageId, out: *mut LrLocationResolver) -> Result;

    pub fn lrOpenRegisteredLocationResolver(out: *mut LrRegisteredLocationResolver) -> Result;

    pub fn lrLrResolveProgramPath(lr: *mut LrLocationResolver, tid: u64, out: *mut c_char) -> Result;

    pub fn lrLrRedirectProgramPath(lr: *mut LrLocationResolver, tid: u64, path: *const c_char) -> Result;

    pub fn lrLrResolveApplicationControlPath(lr: *mut LrLocationResolver, tid: u64, out: *mut c_char) -> Result;

    pub fn lrLrResolveApplicationHtmlDocumentPath(lr: *mut LrLocationResolver, tid: u64, out: *mut c_char) -> Result;

    pub fn lrLrResolveDataPath(lr: *mut LrLocationResolver, tid: u64, out: *mut c_char) -> Result;

    pub fn lrLrRedirectApplicationControlPath(lr: *mut LrLocationResolver, tid: u64, tid2: u64, path: *const c_char) -> Result;

    pub fn lrLrRedirectApplicationHtmlDocumentPath(lr: *mut LrLocationResolver, tid: u64, tid2: u64, path: *const c_char) -> Result;

    pub fn lrLrResolveApplicationLegalInformationPath(lr: *mut LrLocationResolver, tid: u64, out: *mut c_char) -> Result;

    pub fn lrLrRedirectApplicationLegalInformationPath(lr: *mut LrLocationResolver, tid: u64, tid2: u64, path: *const c_char) -> Result;

    pub fn lrLrRefresh(lr: *mut LrLocationResolver) -> Result;

    /// Only available on [5.0.0+].
    pub fn lrLrEraseProgramRedirection(lr: *mut LrLocationResolver, tid: u64) -> Result;

    pub fn lrRegLrResolveProgramPath(reg: *mut LrRegisteredLocationResolver, tid: u64, out: *mut c_char) -> Result;
}
