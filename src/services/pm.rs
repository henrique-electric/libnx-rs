//! `switch/services/pm.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// LaunchFlag
pub type PmLaunchFlag = u32;
pub const PmLaunchFlag_None: PmLaunchFlag = 0;
pub const PmLaunchFlag_SignalOnExit: PmLaunchFlag = (1 << 0);
pub const PmLaunchFlag_SignalOnStart: PmLaunchFlag = (1 << 1);
pub const PmLaunchFlag_SignalOnCrash: PmLaunchFlag = (1 << 2);
pub const PmLaunchFlag_SignalOnDebug: PmLaunchFlag = (1 << 3);
pub const PmLaunchFlag_StartSuspended: PmLaunchFlag = (1 << 4);
pub const PmLaunchFlag_DisableAslr: PmLaunchFlag = (1 << 5);
pub const PmLaunchFlagOld_SignalOnExit: PmLaunchFlag = (1 << 0);
pub const PmLaunchFlagOld_StartSuspended: PmLaunchFlag = (1 << 1);
pub const PmLaunchFlagOld_SignalOnCrash: PmLaunchFlag = (1 << 2);
pub const PmLaunchFlagOld_DisableAslr: PmLaunchFlag = (1 << 3);
pub const PmLaunchFlagOld_SignalOnDebug: PmLaunchFlag = (1 << 4);
pub const PmLaunchFlagOld_SignalOnStart: PmLaunchFlag = (1 << 5);

/// ProcessEvent
pub type PmProcessEvent = u32;
pub const PmProcessEvent_None: PmProcessEvent = 0;
pub const PmProcessEvent_Exit: PmProcessEvent = 1;
pub const PmProcessEvent_Start: PmProcessEvent = 2;
pub const PmProcessEvent_Crash: PmProcessEvent = 3;
pub const PmProcessEvent_DebugStart: PmProcessEvent = 4;
pub const PmProcessEvent_DebugBreak: PmProcessEvent = 5;

/// ProcessEventInfo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PmProcessEventInfo {
    pub event: PmProcessEvent,
    pub process_id: u64,
}

/// BootMode
pub type PmBootMode = u32;
/// Normal
pub const PmBootMode_Normal: PmBootMode = 0;
/// Maintenance
pub const PmBootMode_Maintenance: PmBootMode = 1;
/// SafeMode
pub const PmBootMode_SafeMode: PmBootMode = 2;

/// ResourceLimitValues
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PmResourceLimitValues {
    pub physical_memory: u64,
    pub thread_count: u32,
    pub event_count: u32,
    pub transfer_memory_count: u32,
    pub session_count: u32,
}

impl_zeroed_default!(PmProcessEventInfo, PmResourceLimitValues);

unsafe extern "C" {
    /// Initialize pm:dmnt.
    pub fn pmdmntInitialize() -> Result;

    /// Exit pm:dmnt.
    pub fn pmdmntExit();

    /// Gets the Service object for the actual pm:dmnt service session.
    pub fn pmdmntGetServiceSession() -> *mut Service;

    /// Initialize pm:info.
    pub fn pminfoInitialize() -> Result;

    /// Exit pm:info.
    pub fn pminfoExit();

    /// Gets the Service object for the actual pm:info service session.
    pub fn pminfoGetServiceSession() -> *mut Service;

    /// Initialize pm:shell.
    pub fn pmshellInitialize() -> Result;

    /// Exit pm:shell.
    pub fn pmshellExit();

    /// Gets the Service object for the actual pm:shell service session.
    pub fn pmshellGetServiceSession() -> *mut Service;

    /// Initialize pm:bm.
    pub fn pmbmInitialize() -> Result;

    /// Exit pm:bm.
    pub fn pmbmExit();

    /// Gets the Service object for the actual pm:bm service session.
    pub fn pmbmGetServiceSession() -> *mut Service;

    /// Gets the `PmBootMode`.
    ///
    /// # Arguments
    /// - `out` (out): `PmBootMode`
    pub fn pmbmGetBootMode(out: *mut PmBootMode) -> Result;

    /// Sets the `PmBootMode` to ::PmBootMode_Maintenance.
    pub fn pmbmSetMaintenanceBoot() -> Result;

    pub fn pmdmntGetJitDebugProcessIdList(out_count: *mut u32, out_pids: *mut u64, max_pids: usize) -> Result;

    pub fn pmdmntStartProcess(pid: u64) -> Result;

    pub fn pmdmntGetProcessId(pid_out: *mut u64, program_id: u64) -> Result;

    pub fn pmdmntHookToCreateProcess(out: *mut Event, program_id: u64) -> Result;

    pub fn pmdmntGetApplicationProcessId(pid_out: *mut u64) -> Result;

    pub fn pmdmntHookToCreateApplicationProcess(out: *mut Event) -> Result;

    pub fn pmdmntClearHook(which: u32) -> Result;

    pub fn pmdmntGetProgramId(program_id_out: *mut u64, pid: u64) -> Result;

    pub fn pminfoGetProgramId(program_id_out: *mut u64, pid: u64) -> Result;

    pub fn pminfoGetAppletCurrentResourceLimitValues(out: *mut PmResourceLimitValues) -> Result;

    pub fn pminfoGetAppletPeakResourceLimitValues(out: *mut PmResourceLimitValues) -> Result;

    pub fn pmshellLaunchProgram(launch_flags: u32, location: *const NcmProgramLocation, pid: *mut u64) -> Result;

    pub fn pmshellTerminateProcess(processID: u64) -> Result;

    pub fn pmshellTerminateProgram(program_id: u64) -> Result;

    pub fn pmshellGetProcessEventHandle(out: *mut Event) -> Result;

    pub fn pmshellGetProcessEventInfo(out: *mut PmProcessEventInfo) -> Result;

    pub fn pmshellCleanupProcess(pid: u64) -> Result;

    pub fn pmshellClearJitDebugOccured(pid: u64) -> Result;

    pub fn pmshellNotifyBootFinished() -> Result;

    pub fn pmshellGetApplicationProcessIdForShell(pid_out: *mut u64) -> Result;

    pub fn pmshellBoostSystemMemoryResourceLimit(boost_size: u64) -> Result;

    pub fn pmshellBoostApplicationThreadResourceLimit() -> Result;

    pub fn pmshellBoostSystemThreadResourceLimit() -> Result;

    pub fn pmshellGetProcessId(pid_out: *mut u64, program_id: u64) -> Result;
}
