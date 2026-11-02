//! `switch/services/news.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub type NewsServiceType = u32;
/// Initializes news:a
pub const NewsServiceType_Administrator: NewsServiceType = 0;
/// Initializes news:c
pub const NewsServiceType_Configuration: NewsServiceType = 1;
/// Initializes news:m
pub const NewsServiceType_Manager: NewsServiceType = 2;
/// Initializes news:p
pub const NewsServiceType_Post: NewsServiceType = 3;
/// Initializes news:v
pub const NewsServiceType_Viewer: NewsServiceType = 4;
pub const NewsServiceType_Count: NewsServiceType = 5;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NewsTopicName {
    pub name: [c_char; 32],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NewsNewlyArrivedEventHolder {
    pub s: Service,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NewsDataService {
    pub s: Service,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NewsDatabaseService {
    pub s: Service,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NewsOverwriteEventHolder {
    pub s: Service,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NewsRecordV1 {
    pub news_id: [c_char; 24],
    pub user_id: [c_char; 24],
    pub received_at: s64,
    pub read: s32,
    pub newly: s32,
    pub displayed: s32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NewsRecord {
    pub news_id: [c_char; 24],
    pub user_id: [c_char; 24],
    pub topic_id: NewsTopicName,
    pub received_at: s64,
    pub pad_0: s64,
    pub decoration_type: s32,
    pub read: s32,
    pub newly: s32,
    pub displayed: s32,
    pub feedback: s32,
    pub pad_1: s32,
    pub extra_1: s32,
    pub extra_2: s32,
}

impl_zeroed_default!(NewsTopicName, NewsNewlyArrivedEventHolder, NewsDataService, NewsDatabaseService, NewsOverwriteEventHolder, NewsRecordV1, NewsRecord);

unsafe extern "C" {
    pub fn newsInitialize(service_type: NewsServiceType) -> Result;

    pub fn newsExit();

    pub fn newsGetServiceSession() -> *mut Service;

    pub fn newsCreateNewlyArrivedEventHolder(out: *mut NewsNewlyArrivedEventHolder) -> Result;

    pub fn newsCreateNewsDataService(out: *mut NewsDataService) -> Result;

    pub fn newsCreateNewsDatabaseService(out: *mut NewsDatabaseService) -> Result;

    /// [2.0.0+]
    pub fn newsCreateOverwriteEventHolder(out: *mut NewsOverwriteEventHolder) -> Result;

    pub fn newsPostLocalNews(news: *const c_void, size: usize) -> Result;

    pub fn newsSetPassphrase(program_id: u64, passphrase: *const c_char) -> Result;

    pub fn newsGetSubscriptionStatus(filter: *const c_char, status: *mut u32) -> Result;

    /// [3.0.0+]
    pub fn newsGetTopicList(channel: u32, out_count: *mut u32, out: *mut NewsTopicName, max_count: u32) -> Result;

    /// [6.0.0+]
    pub fn newsGetSavedataUsage(current: *mut u64, total: *mut u64) -> Result;

    pub fn newsIsSystemUpdateRequired(out: *mut bool) -> Result;

    /// [10.0.0+]
    pub fn newsGetDatabaseVersion(version: *mut u32) -> Result;

    pub fn newsRequestImmediateReception(filter: *const c_char) -> Result;

    pub fn newsSetSubscriptionStatus(filter: *const c_char, status: u32) -> Result;

    pub fn newsClearStorage() -> Result;

    pub fn newsClearSubscriptionStatusAll() -> Result;

    pub fn newsGetNewsDatabaseDump(buffer: *mut c_void, size: u64, out: *mut u64) -> Result;

    pub fn newsNewlyArrivedEventHolderClose(srv: *mut NewsNewlyArrivedEventHolder);

    pub fn newsNewlyArrivedEventHolderGet(srv: *mut NewsNewlyArrivedEventHolder, out: *mut Event) -> Result;

    pub fn newsDataClose(srv: *mut NewsDataService);

    pub fn newsDataOpen(srv: *mut NewsDataService, file_name: *const c_char) -> Result;

    pub fn newsDataOpenWithNewsRecordV1(srv: *mut NewsDataService, record: *mut NewsRecordV1) -> Result;

    pub fn newsDataRead(srv: *mut NewsDataService, bytes_read: *mut u64, offset: u64, out: *mut c_void, out_size: usize) -> Result;

    pub fn newsDataGetSize(srv: *mut NewsDataService, size: *mut u64) -> Result;

    /// [6.0.0+]
    pub fn newsDataOpenWithNewsRecord(srv: *mut NewsDataService, record: *mut NewsRecord) -> Result;

    pub fn newsDatabaseClose(srv: *mut NewsDatabaseService);

    pub fn newsDatabaseGetListV1(srv: *mut NewsDatabaseService, out: *mut NewsRecordV1, max_count: u32, where_: *const c_char, order: *const c_char, count: *mut u32, offset: u32) -> Result;

    pub fn newsDatabaseCount(srv: *mut NewsDatabaseService, filter: *const c_char, count: *mut u32) -> Result;

    /// [6.0.0+]
    pub fn newsDatabaseGetList(srv: *mut NewsDatabaseService, out: *mut NewsRecord, max_count: u32, where_: *const c_char, order: *const c_char, count: *mut u32, offset: u32) -> Result;

    pub fn newsOverwriteEventHolderClose(srv: *mut NewsOverwriteEventHolder);

    pub fn newsOverwriteEventHolderGet(srv: *mut NewsOverwriteEventHolder, out: *mut Event) -> Result;
}
