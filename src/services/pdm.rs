//! `switch/services/pdm.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// PlayEventType
pub type PdmPlayEventType = u32;
/// Applet
pub const PdmPlayEventType_Applet: PdmPlayEventType = 0;
/// Account
pub const PdmPlayEventType_Account: PdmPlayEventType = 1;
/// PowerStateChange
pub const PdmPlayEventType_PowerStateChange: PdmPlayEventType = 2;
/// OperationModeChange
pub const PdmPlayEventType_OperationModeChange: PdmPlayEventType = 3;
/// Initialize. Used for the very first PlayEvent entry in the log.
pub const PdmPlayEventType_Initialize: PdmPlayEventType = 4;

/// AppletEventType
pub type PdmAppletEventType = u32;
/// "launch"
pub const PdmAppletEventType_Launch: PdmAppletEventType = 0;
/// "exit"
pub const PdmAppletEventType_Exit: PdmAppletEventType = 1;
/// "in_focus"
pub const PdmAppletEventType_InFocus: PdmAppletEventType = 2;
/// "out_of_focus"
pub const PdmAppletEventType_OutOfFocus: PdmAppletEventType = 3;
/// "out_of_focus"
pub const PdmAppletEventType_OutOfFocus4: PdmAppletEventType = 4;
/// "exit"
pub const PdmAppletEventType_Exit5: PdmAppletEventType = 5;
/// "exit"
pub const PdmAppletEventType_Exit6: PdmAppletEventType = 6;

/// PlayLogPolicy
pub type PdmPlayLogPolicy = u32;
/// All pdm:qry commands which require ::PdmPlayEventType_Applet and AppletId = Application will only return the entry when PlayLogPolicy matches this value.
pub const PdmPlayLogPolicy_All: PdmPlayLogPolicy = 0;
/// The above commands will filter out the entry with this.
pub const PdmPlayLogPolicy_LogOnly: PdmPlayLogPolicy = 1;
/// The pdm:ntfy commands which handle ::PdmPlayEventType_Applet logging will immediately return 0 when the input param matches this value.
pub const PdmPlayLogPolicy_None: PdmPlayLogPolicy = 2;
/// [10.0.0+] The cmds which require ::PdmPlayLogPolicy_All, now also allow value 3 if the cmd input flag is set.
pub const PdmPlayLogPolicy_Unknown3: PdmPlayLogPolicy = 3;

/// AppletEventV1. AppletEvent for [1.0.0-15.0.1], converted to `PdmAppletEvent` when needed.
/// Timestamp format, converted from PosixTime: total minutes since epoch UTC 1999/12/31 00:00:00.
/// See `pdmPlayTimestampToPosix`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PdmAppletEventV1 {
    /// ProgramId.
    pub program_id: u64,
    /// Entry index.
    pub entry_index: u32,
    /// See PdmPlayEvent::timestamp_user, with the above timestamp format.
    pub timestamp_user: u32,
    /// See PdmPlayEvent::timestamp_network, with the above timestamp format.
    pub timestamp_network: u32,
    /// **Ref:** PdmAppletEventType
    pub event_type: u8,
    /// Padding.
    pub pad: [u8; 3],
}

/// AppletEvent. AppletEvent for [16.0.0+], converted from `PdmAppletEventV1` on [1.0.0-15.0.1].
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PdmAppletEvent {
    /// ProgramId.
    pub program_id: u64,
    /// Entry index.
    pub entry_index: u32,
    /// Padding
    pub pad: u32,
    /// See PdmPlayEvent::timestamp_user.
    pub timestamp_user: u64,
    /// See PdmPlayEvent::timestamp_network.
    pub timestamp_network: u64,
    /// **Ref:** PdmAppletEventType
    pub event_type: u8,
    /// Padding.
    pub pad2: [u8; 7],
}

/// PlayStatisticsV1. PlayStatistics for [1.0.0-15.0.1], converted to `PdmPlayStatistics` when needed.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PdmPlayStatisticsV1 {
    /// ProgramId.
    pub program_id: u64,
    /// Entry index for the first time the program was played.
    pub first_entry_index: u32,
    /// See PdmAppletEventV1::timestamp_user. This is for the first time the program was played.
    pub first_timestamp_user: u32,
    /// See PdmAppletEventV1::timestamp_network. This is for the first time the program was played.
    pub first_timestamp_network: u32,
    /// Entry index for the last time the program was played.
    pub last_entry_index: u32,
    /// See PdmAppletEventV1::timestamp_user. This is for the last time the program was played.
    pub last_timestamp_user: u32,
    /// See PdmAppletEventV1::timestamp_network. This is for the last time the program was played.
    pub last_timestamp_network: u32,
    /// Total play-time in minutes.
    pub playtime_minutes: u32,
    /// Total times the program was launched.
    pub total_launches: u32,
}

/// PlayStatistics. PlayStatistics for [16.0.0+], converted from `PdmPlayStatisticsV1` on [1.0.0-15.0.1].
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PdmPlayStatistics {
    /// ProgramId.
    pub program_id: u64,
    /// Entry index for the first time the program was played.
    pub first_entry_index: u32,
    /// Padding
    pub pad: u32,
    /// See PdmAppletEvent::timestamp_user. This is for the first time the program was played, in PosixTime.
    pub first_timestamp_user: u64,
    /// See PdmAppletEvent::timestamp_network. This is for the first time the program was played, in PosixTime.
    pub first_timestamp_network: u64,
    /// Entry index for the last time the program was played.
    pub last_entry_index: u32,
    /// Padding
    pub pad2: u32,
    /// See PdmAppletEvent::timestamp_user. This is for the last time the program was played, in PosixTime.
    pub last_timestamp_user: u64,
    /// See PdmAppletEvent::timestamp_network. This is for the last time the program was played, in PosixTime.
    pub last_timestamp_network: u64,
    /// Total play-time in nanoseconds.
    pub playtime: u64,
    /// Total times the program was launched.
    pub total_launches: u32,
    /// Padding
    pub pad3: u32,
}

/// LastPlayTime.
/// This contains data from the last time the application was played.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PdmLastPlayTime {
    /// ApplicationId.
    pub application_id: u64,
    /// See PdmAppletEventV1::timestamp_user.
    pub timestamp_user: u32,
    /// See PdmAppletEventV1::timestamp_network.
    pub timestamp_network: u32,
    /// Total minutes since the application was last played.
    pub last_played_minutes: u32,
    /// Flag indicating whether the above field is set.
    pub flag: u8,
    /// Padding.
    pub pad: [u8; 3],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PdmPlayEvent_event_data_applet_unk_x8_application {
    /// Application version.
    pub version: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PdmPlayEvent_event_data_applet_unk_x8_applet {
    /// Set to 0x1 by pdm:ntfy cmd8, indicating that the below field is set to an input param.
    pub flag: u8,
    /// Input value from pdm:ntfy cmd8, see `LibAppletMode`.
    pub mode: u8,
    /// Padding.
    pub pad: [u8; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union PdmPlayEvent_event_data_applet_unk_x8 {
    /// For AppletId == ::AppletId_application.
    pub application: PdmPlayEvent_event_data_applet_unk_x8_application,
    /// For AppletId != ::AppletId_application.
    pub applet: PdmPlayEvent_event_data_applet_unk_x8_applet,
    pub data: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PdmPlayEvent_event_data_applet {
    /// ProgramId.
    pub program_id: [u32; 2],
    pub unk_x8: PdmPlayEvent_event_data_applet_unk_x8,
    /// **Ref:** AppletId
    pub applet_id: u8,
    /// **Ref:** NcmStorageId
    pub storage_id: u8,
    /// **Ref:** PdmPlayLogPolicy
    pub log_policy: u8,
    /// **Ref:** PdmAppletEventType
    pub event_type: u8,
    /// Unused.
    pub unused: [u8; 12],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PdmPlayEvent_event_data_account {
    /// userId.
    pub uid: [u32; 4],
    /// ApplicationId, see below.
    pub application_id: [u32; 2],
    /// 0-1 to be listed by `pdmqryQueryAccountEvent`, or 2 to include the above ApplicationId.
    pub type_: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PdmPlayEvent_event_data_power_state_change {
    /// Input value from the pdm:ntfy command.
    pub value: u8,
    /// Unused.
    pub unused: [u8; 27],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PdmPlayEvent_event_data_operation_mode_change {
    /// Input value from the pdm:ntfy command.
    pub value: u8,
    /// Unused.
    pub unused: [u8; 27],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union PdmPlayEvent_event_data {
    pub applet: PdmPlayEvent_event_data_applet,
    pub account: PdmPlayEvent_event_data_account,
    pub power_state_change: PdmPlayEvent_event_data_power_state_change,
    pub operation_mode_change: PdmPlayEvent_event_data_operation_mode_change,
    pub data: [u8; 28],
}

/// PlayEvent.
/// This is the raw entry struct directly read from FS, without any entry filtering.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PdmPlayEvent {
    /// ProgramId/ApplicationId/userId stored within here have the u32 low/high swapped in each u64.
    pub event_data: PdmPlayEvent_event_data,
    /// **Ref:** PdmPlayEventType. Controls which struct in the above event_data is used. ::PdmPlayEventType_Initialize doesn't use event_data.
    pub play_event_type: u8,
    /// Padding.
    pub pad: [u8; 3],
    /// PosixTime timestamp from StandardUserSystemClock.
    pub timestamp_user: u64,
    /// PosixTime timestamp from StandardNetworkSystemClock.
    pub timestamp_network: u64,
    /// Timestamp in seconds derived from StandardSteadyClock.
    pub timestamp_steady: u64,
}

/// AccountEventV3. AccountEvent for [3.0.0-9.2.0], converted to `PdmAccountEvent` when needed.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PdmAccountEventV3 {
    /// **Ref:** AccountUid
    pub uid: AccountUid,
    /// Entry index.
    pub entry_index: u32,
    /// Padding.
    pub pad: [u8; 4],
    /// See PdmPlayEvent::timestamp_user.
    pub timestamp_user: u64,
    /// See PdmPlayEvent::timestamp_network.
    pub timestamp_network: u64,
    /// See PdmPlayEvent::timestamp_steady.
    pub timestamp_steady: u64,
    /// See PdmPlayEvent::event_data::account::type.
    pub type_: u8,
    /// Padding.
    pub pad2: [u8; 7],
}

/// AccountEventV10. AccountEvent for [10.0.0-15.0.1], converted to `PdmAccountEvent` when needed.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PdmAccountEventV10 {
    /// **Ref:** AccountUid
    pub uid: AccountUid,
    /// ProgramId
    pub program_id: u64,
    /// Entry index.
    pub entry_index: u32,
    /// Padding.
    pub pad: [u8; 4],
    /// See PdmPlayEvent::timestamp_user.
    pub timestamp_user: u64,
    /// See PdmPlayEvent::timestamp_network.
    pub timestamp_network: u64,
    /// See PdmPlayEvent::timestamp_steady.
    pub timestamp_steady: u64,
    /// See PdmPlayEvent::event_data::account::type.
    pub type_: u8,
    /// Padding.
    pub pad2: [u8; 7],
}

/// AccountEvent. AccountEvent for [16.0.0+], converted from the older structs when needed.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PdmAccountEvent {
    /// **Ref:** AccountUid
    pub uid: AccountUid,
    /// [10.0.0+] ProgramId
    pub program_id: u64,
    /// Entry index.
    pub entry_index: u32,
    /// Padding.
    pub pad: [u8; 4],
    /// See PdmPlayEvent::timestamp_user.
    pub timestamp_user: u64,
    /// See PdmPlayEvent::timestamp_network.
    pub timestamp_network: u64,
    /// See PdmPlayEvent::event_data::account::type.
    pub type_: u8,
    /// Padding.
    pub pad2: [u8; 7],
}

/// AccountPlayEvent.
/// This is the raw entry struct directly read from FS, without any entry filtering. This is separate from `PdmPlayEvent`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PdmAccountPlayEvent {
    /// Unknown.
    pub unk_x0: [u8; 4],
    /// ApplicationId, with the u32 low/high words swapped.
    pub application_id: [u32; 2],
    /// Unknown.
    pub unk_xc: [u8; 12],
    /// POSIX timestamp.
    pub timestamp0: u64,
    /// POSIX timestamp.
    pub timestamp1: u64,
}

/// ApplicationPlayStatistics
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PdmApplicationPlayStatistics {
    /// ApplicationId.
    pub application_id: u64,
    /// Total play-time in nanoseconds.
    pub playtime: u64,
    /// Total times the application was launched.
    pub total_launches: u64,
}

/// Helper function which converts a Play timestamp from the Pdm*Event structs to POSIX.
///
/// # Arguments
/// - `timestamp` (in): Input timestamp.
#[inline(always)]
pub const fn pdmPlayTimestampToPosix(timestamp: u32) -> u64 {
    (timestamp as u64) * 60 + 946598400
}

impl_zeroed_default!(PdmAppletEventV1, PdmAppletEvent, PdmPlayStatisticsV1, PdmPlayStatistics, PdmLastPlayTime, PdmPlayEvent_event_data_applet_unk_x8_application, PdmPlayEvent_event_data_applet_unk_x8_applet, PdmPlayEvent_event_data_applet_unk_x8, PdmPlayEvent_event_data_applet, PdmPlayEvent_event_data_account, PdmPlayEvent_event_data_power_state_change, PdmPlayEvent_event_data_operation_mode_change, PdmPlayEvent_event_data, PdmPlayEvent, PdmAccountEventV3, PdmAccountEventV10, PdmAccountEvent, PdmAccountPlayEvent, PdmApplicationPlayStatistics);

unsafe extern "C" {
    /// Initialize pdm:qry.
    pub fn pdmqryInitialize() -> Result;

    /// Exit pdm:qry.
    pub fn pdmqryExit();

    /// Gets the Service object for the actual pdm:qry service session.
    pub fn pdmqryGetServiceSession() -> *mut Service;

    /// Gets a list of `PdmAppletEvent`.
    ///
    /// # Arguments
    /// - `entry_index` (in): Start entry index.
    /// - `flag` (in): [10.0.0+] Whether to additionally allow using entries with ::PdmPlayLogPolicy_Unknown3.
    /// - `events` (out): Output `PdmAppletEvent` array.
    /// - `count` (in): Max entries in the output array.
    /// - `total_out` (out): Total output entries.
    pub fn pdmqryQueryAppletEvent(entry_index: s32, flag: bool, events: *mut PdmAppletEvent, count: s32, total_out: *mut s32) -> Result;

    /// Gets `PdmPlayStatistics` for the specified ApplicationId.
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId
    /// - `flag` (in): [10.0.0+] Whether to additionally allow using entries with ::PdmPlayLogPolicy_Unknown3.
    /// - `stats` (out): `PdmPlayStatistics`
    pub fn pdmqryQueryPlayStatisticsByApplicationId(application_id: u64, flag: bool, stats: *mut PdmPlayStatistics) -> Result;

    /// Gets `PdmPlayStatistics` for the specified ApplicationId and account userId.
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId
    /// - `uid` (in): `AccountUid`
    /// - `flag` (in): [10.0.0+] Whether to additionally allow using entries with ::PdmPlayLogPolicy_Unknown3.
    /// - `stats` (out): `PdmPlayStatistics`
    pub fn pdmqryQueryPlayStatisticsByApplicationIdAndUserAccountId(application_id: u64, uid: AccountUid, flag: bool, stats: *mut PdmPlayStatistics) -> Result;

    /// Gets `PdmLastPlayTime` for the specified applications.
    ///
    /// # Arguments
    /// - `flag` (in): [10.0.0+] Whether to additionally allow using entries with ::PdmPlayLogPolicy_Unknown3.
    /// - `playtimes` (out): Output `PdmLastPlayTime` array.
    /// - `application_ids` (in): Input ApplicationIds array.
    /// - `count` (in): Total entries in the input/output arrays.
    /// - `total_out` (out): Total output entries.
    pub fn pdmqryQueryLastPlayTime(flag: bool, playtimes: *mut PdmLastPlayTime, application_ids: *const u64, count: s32, total_out: *mut s32) -> Result;

    /// Gets a list of `PdmPlayEvent`.
    ///
    /// # Arguments
    /// - `entry_index` (in): Start entry index.
    /// - `events` (out): Output `PdmPlayEvent` array.
    /// - `count` (in): Max entries in the output array.
    /// - `total_out` (out): Total output entries.
    pub fn pdmqryQueryPlayEvent(entry_index: s32, events: *mut PdmPlayEvent, count: s32, total_out: *mut s32) -> Result;

    /// Gets range fields which can then be used with the other pdmqry funcs, except for `pdmqryQueryAccountPlayEvent`.
    ///
    /// # Arguments
    /// - `total_entries` (out): Total entries.
    /// - `start_entry_index` (out): Start entry index.
    /// - `end_entry_index` (out): End entry index.
    pub fn pdmqryGetAvailablePlayEventRange(total_entries: *mut s32, start_entry_index: *mut s32, end_entry_index: *mut s32) -> Result;

    /// Gets a list of `PdmAccountEvent`.
    ///
    /// # Arguments
    /// - `entry_index` (in): Start entry index.
    /// - `events` (out): Output `PdmAccountEvent` array.
    /// - `count` (in): Max entries in the output array.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** Only available with [3.0.0+].
    pub fn pdmqryQueryAccountEvent(entry_index: s32, events: *mut PdmAccountEvent, count: s32, total_out: *mut s32) -> Result;

    /// Gets a list of `PdmAccountPlayEvent`.
    ///
    /// # Arguments
    /// - `entry_index` (in): Start entry index.
    /// - `uid` (in): `AccountUid`
    /// - `events` (out): Output `PdmAccountPlayEvent` array.
    /// - `count` (in): Max entries in the output array.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** Only available with [4.0.0+].
    pub fn pdmqryQueryAccountPlayEvent(entry_index: s32, uid: AccountUid, events: *mut PdmAccountPlayEvent, count: s32, total_out: *mut s32) -> Result;

    /// Gets range fields which can then be used with `pdmqryQueryAccountPlayEvent`.
    ///
    /// # Arguments
    /// - `uid` (in): `AccountUid`
    /// - `total_entries` (out): Total entries.
    /// - `start_entry_index` (out): Start entry index.
    /// - `end_entry_index` (out): End entry index.
    pub fn pdmqryGetAvailableAccountPlayEventRange(uid: AccountUid, total_entries: *mut s32, start_entry_index: *mut s32, end_entry_index: *mut s32) -> Result;

    /// Gets a list of applications played by the specified user.
    ///
    /// # Arguments
    /// - `uid` (in): `AccountUid`
    /// - `flag` (in): [10.0.0+] Whether to additionally allow using entries with ::PdmPlayLogPolicy_Unknown3.
    /// - `application_ids` (out): Output ApplicationIds array.
    /// - `count` (in): Max entries in the output array.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** Only available with [6.0.0-14.1.2].
    pub fn pdmqryQueryRecentlyPlayedApplication(uid: AccountUid, flag: bool, application_ids: *mut u64, count: s32, total_out: *mut s32) -> Result;

    /// Gets an Event which is signaled when logging a new `PdmPlayEvent` which would be available via `pdmqryQueryAccountEvent`, where PdmPlayEvent::event_data::account::type is 0.
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=false.
    ///
    /// **Note:** Only available with [6.0.0-14.1.2].
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn pdmqryGetRecentlyPlayedApplicationUpdateEvent(out_event: *mut Event) -> Result;
}
