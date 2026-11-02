//! `switch/services/time.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Values for __nx_time_service_type.
pub type TimeServiceType = u32;
/// Default. Initializes time:u.
pub const TimeServiceType_User: TimeServiceType = 0;
/// Initializes time:a
pub const TimeServiceType_Menu: TimeServiceType = 1;
/// Initializes time:s.
pub const TimeServiceType_System: TimeServiceType = 2;
/// Initializes time:r. Only available with [9.0.0+].
pub const TimeServiceType_Repair: TimeServiceType = 3;
/// Initializes time:su. Only available with [9.0.0+].
pub const TimeServiceType_SystemUser: TimeServiceType = 4;

/// Time clock type.
pub type TimeType = u32;
pub const TimeType_UserSystemClock: TimeType = 0;
pub const TimeType_NetworkSystemClock: TimeType = 1;
pub const TimeType_LocalSystemClock: TimeType = 2;
pub const TimeType_Default: TimeType = TimeType_UserSystemClock;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TimeCalendarTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub pad: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TimeCalendarAdditionalInfo {
    /// 0-based day-of-week.
    pub wday: u32,
    /// 0-based day-of-year.
    pub yday: u32,
    /// Timezone name string.
    pub timezoneName: [c_char; 8],
    /// 0 = no DST, 1 = DST.
    pub DST: u32,
    /// Seconds relative to UTC for this timezone.
    pub offset: s32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TimeZoneRule {
    pub data: [u8; 16384],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TimeLocationName {
    pub name: [c_char; 36],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TimeSteadyClockTimePoint {
    /// Monotonic count in seconds.
    pub time_point: s64,
    /// An ID representing the clock source.
    pub source_id: Uuid,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TimeStandardSteadyClockTimePointType {
    pub base_time: s64,
    pub source_id: Uuid,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TimeSystemClockContext {
    pub offset: s64,
    pub timestamp: TimeSteadyClockTimePoint,
}

impl_zeroed_default!(TimeCalendarTime, TimeCalendarAdditionalInfo, TimeZoneRule, TimeLocationName, TimeSteadyClockTimePoint, TimeStandardSteadyClockTimePointType, TimeSystemClockContext);

unsafe extern "C" {
    /// Initialize time. Used automatically during app startup.
    pub fn timeInitialize() -> Result;

    /// Exit time. Used automatically during app startup.
    pub fn timeExit();

    /// Gets the Service object for the actual time service session.
    pub fn timeGetServiceSession() -> *mut Service;

    /// Gets the Service object for ISystemClock with the specified `TimeType`. This will return NULL when the type is invalid.
    pub fn timeGetServiceSession_SystemClock(type_: TimeType) -> *mut Service;

    /// Gets the Service object for ISteadyClock.
    pub fn timeGetServiceSession_SteadyClock() -> *mut Service;

    /// Gets the Service object for ITimeZoneService.
    pub fn timeGetServiceSession_TimeZoneService() -> *mut Service;

    /// [6.0.0+] Gets the address of the SharedMemory.
    pub fn timeGetSharedmemAddr() -> *mut c_void;

    /// Gets the timepoint for the standard steady clock.
    ///
    /// # Arguments
    /// - `out` (out): Output timepoint (see `TimeSteadyClockTimePoint`)
    ///
    /// **Remark:** The standard steady clock counts time since the RTC was configured (usually this happens during manufacturing).
    ///
    /// **Returns:** Result code.
    pub fn timeGetStandardSteadyClockTimePoint(out: *mut TimeSteadyClockTimePoint) -> Result;

    /// [3.0.0+] Gets the internal offset for the standard steady clock.
    ///
    /// # Arguments
    /// - `out` (out): Output internal offset.
    ///
    /// **Returns:** Result code.
    pub fn timeGetStandardSteadyClockInternalOffset(out: *mut s64) -> Result;

    /// Gets the time for the specified clock.
    ///
    /// # Arguments
    /// - `type` (in): Clock to use.
    /// - `timestamp` (out): POSIX UTC timestamp.
    ///
    /// **Returns:** Result code.
    pub fn timeGetCurrentTime(type_: TimeType, timestamp: *mut u64) -> Result;

    /// Sets the time for the specified clock.
    ///
    /// # Arguments
    /// - `type` (in): Clock to use.
    /// - `timestamp` (in): POSIX UTC timestamp.
    ///
    /// **Returns:** Result code.
    pub fn timeSetCurrentTime(type_: TimeType, timestamp: u64) -> Result;

    pub fn timeGetDeviceLocationName(name: *mut TimeLocationName) -> Result;

    pub fn timeSetDeviceLocationName(name: *const TimeLocationName) -> Result;

    pub fn timeGetTotalLocationNameCount(total_location_name_count: *mut s32) -> Result;

    pub fn timeLoadLocationNameList(index: s32, location_name_array: *mut TimeLocationName, location_name_max: s32, location_name_count: *mut s32) -> Result;

    pub fn timeLoadTimeZoneRule(name: *const TimeLocationName, rule: *mut TimeZoneRule) -> Result;

    pub fn timeToCalendarTime(rule: *const TimeZoneRule, timestamp: u64, caltime: *mut TimeCalendarTime, info: *mut TimeCalendarAdditionalInfo) -> Result;

    pub fn timeToCalendarTimeWithMyRule(timestamp: u64, caltime: *mut TimeCalendarTime, info: *mut TimeCalendarAdditionalInfo) -> Result;

    pub fn timeToPosixTime(rule: *const TimeZoneRule, caltime: *const TimeCalendarTime, timestamp_list: *mut u64, timestamp_list_count: s32, timestamp_count: *mut s32) -> Result;

    pub fn timeToPosixTimeWithMyRule(caltime: *const TimeCalendarTime, timestamp_list: *mut u64, timestamp_list_count: s32, timestamp_count: *mut s32) -> Result;
}
