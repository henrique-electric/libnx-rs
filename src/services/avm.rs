//! `switch/services/avm.h`

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
pub struct AvmVersionListEntry {
    pub application_id: u64,
    pub version: u32,
    pub required: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AvmRequiredVersionEntry {
    pub application_id: u64,
    pub version: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AvmVersionListImporter {
    pub s: Service,
}

impl_zeroed_default!(AvmVersionListEntry, AvmRequiredVersionEntry, AvmVersionListImporter);

unsafe extern "C" {
    pub fn avmInitialize() -> Result;

    pub fn avmExit();

    pub fn avmGetServiceSession() -> *mut Service;

    pub fn avmGetHighestAvailableVersion(id_1: u64, id_2: u64, version: *mut u32) -> Result;

    pub fn avmGetHighestRequiredVersion(id_1: u64, id_2: u64, version: *mut u32) -> Result;

    pub fn avmGetVersionListEntry(application_id: u64, entry: *mut AvmVersionListEntry) -> Result;

    pub fn avmGetVersionListImporter(out: *mut AvmVersionListImporter) -> Result;

    pub fn avmGetLaunchRequiredVersion(application_id: u64, version: *mut u32) -> Result;

    pub fn avmUpgradeLaunchRequiredVersion(application_id: u64, version: u32) -> Result;

    pub fn avmPushLaunchVersion(application_id: u64, version: u32) -> Result;

    pub fn avmListVersionList(buffer: *mut AvmVersionListEntry, count: usize, out: *mut u32) -> Result;

    pub fn avmListRequiredVersion(buffer: *mut AvmRequiredVersionEntry, count: usize, out: *mut u32) -> Result;

    pub fn avmVersionListImporterClose(srv: *mut AvmVersionListImporter);

    pub fn avmVersionListImporterSetTimestamp(srv: *mut AvmVersionListImporter, timestamp: u64) -> Result;

    pub fn avmVersionListImporterSetData(srv: *mut AvmVersionListImporter, entries: *const AvmVersionListEntry, count: u32) -> Result;

    pub fn avmVersionListImporterFlush(srv: *mut AvmVersionListImporter) -> Result;
}
