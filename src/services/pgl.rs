//! `switch/services/pgl.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// LaunchFlag
pub type PglLaunchFlag = u32;
pub const PglLaunchFlag_None: PglLaunchFlag = 0;
pub const PglLaunchFlag_EnableDetailedCrashReport: PglLaunchFlag = BIT(0);
pub const PglLaunchFlag_EnableCrashReportScreenShotForProduction: PglLaunchFlag = BIT(1);
pub const PglLaunchFlag_EnableCrashReportScreenShotForDevelop: PglLaunchFlag = BIT(2);

/// SnapShotDumpType
pub type PglSnapShotDumpType = u32;
pub const PglSnapShotDumpType_None: PglSnapShotDumpType = 0;
pub const PglSnapShotDumpType_Auto: PglSnapShotDumpType = 1;
pub const PglSnapShotDumpType_Full: PglSnapShotDumpType = 2;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PglContentMetaInfo {
    /// Program Id
    pub id: u64,
    /// Version
    pub version: u32,
    /// NcmContentType
    pub content_type: u8,
    /// Id Offset
    pub id_offset: u8,
    /// Padding
    pub reserved_0E: [u8; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union PglEventObserver {
    pub s: Service,
    pub t: TipcService,
}

impl_zeroed_default!(PglContentMetaInfo, PglEventObserver);

unsafe extern "C" {
    /// Initialize pgl.
    pub fn pglInitialize() -> Result;

    /// Exit pgl.
    pub fn pglExit();

    /// Gets the Service object for the actual pgl service session. Requires &lt; 12.0.0
    pub fn pglGetServiceSessionCmif() -> *mut Service;

    /// Gets the TipcService object for the actual pgl service session. Requires 12.0.0+
    pub fn pglGetServiceSessionTipc() -> *mut TipcService;

    pub fn pglLaunchProgram(out_pid: *mut u64, loc: *const NcmProgramLocation, pm_launch_flags: u32, pgl_launch_flags: u8) -> Result;

    pub fn pglTerminateProcess(pid: u64) -> Result;

    pub fn pglLaunchProgramFromHost(out_pid: *mut u64, content_path: *const c_char, pm_launch_flags: u32) -> Result;

    pub fn pglGetHostContentMetaInfo(out: *mut PglContentMetaInfo, content_path: *const c_char) -> Result;

    pub fn pglGetApplicationProcessId(out: *mut u64) -> Result;

    pub fn pglBoostSystemMemoryResourceLimit(size: u64) -> Result;

    pub fn pglIsProcessTracked(out: *mut bool, pid: u64) -> Result;

    pub fn pglEnableApplicationCrashReport(en: bool) -> Result;

    pub fn pglIsApplicationCrashReportEnabled(out: *mut bool) -> Result;

    pub fn pglEnableApplicationAllThreadDumpOnCrash(en: bool) -> Result;

    pub fn pglTriggerApplicationSnapShotDumper(dump_type: PglSnapShotDumpType, arg: *const c_char) -> Result;

    pub fn pglGetEventObserver(out: *mut PglEventObserver) -> Result;

    pub fn pglEventObserverGetProcessEvent(observer: *mut PglEventObserver, out: *mut Event) -> Result;

    pub fn pglEventObserverGetProcessEventInfo(observer: *mut PglEventObserver, out: *mut PmProcessEventInfo) -> Result;

    pub fn pglEventObserverClose(observer: *mut PglEventObserver);
}
