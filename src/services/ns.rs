//! `switch/services/ns.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Default size for `nssuControlSetupCardUpdate` / `nssuControlSetupCardUpdateViaSystemUpdater`. This is the size used by qlaunch for SetupCardUpdate.
pub const NSSU_CARDUPDATE_TMEM_SIZE_DEFAULT: u32 = 0x100000;

/// ShellEvent
pub type NsShellEvent = u32;
/// None
pub const NsShellEvent_None: NsShellEvent = 0;
/// Exit
pub const NsShellEvent_Exit: NsShellEvent = 1;
/// Start
pub const NsShellEvent_Start: NsShellEvent = 2;
/// Crash
pub const NsShellEvent_Crash: NsShellEvent = 3;
/// Debug
pub const NsShellEvent_Debug: NsShellEvent = 4;

/// ApplicationControlSource
pub type NsApplicationControlSource = u32;
/// Returns data from cache.
pub const NsApplicationControlSource_CacheOnly: NsApplicationControlSource = 0;
/// Returns data from storage if not present in cache.
pub const NsApplicationControlSource_Storage: NsApplicationControlSource = 1;
/// Returns data from storage without using cache.
pub const NsApplicationControlSource_StorageOnly: NsApplicationControlSource = 2;

/// BackgroundNetworkUpdateState
pub type NsBackgroundNetworkUpdateState = u32;
/// No sysupdate task exists.
pub const NsBackgroundNetworkUpdateState_None: NsBackgroundNetworkUpdateState = 0;
/// Sysupdate download in progress.
pub const NsBackgroundNetworkUpdateState_Downloading: NsBackgroundNetworkUpdateState = 1;
/// Sysupdate ready, pending install.
pub const NsBackgroundNetworkUpdateState_Ready: NsBackgroundNetworkUpdateState = 2;

/// LatestSystemUpdate
pub type NsLatestSystemUpdate = u32;
/// UpToDate
pub const NsLatestSystemUpdate_UpToDate: NsLatestSystemUpdate = 0;
/// Downloaded
pub const NsLatestSystemUpdate_Downloaded: NsLatestSystemUpdate = 1;
/// NeedsDownload
pub const NsLatestSystemUpdate_NeedsDownload: NsLatestSystemUpdate = 2;

/// RequestServerStopper
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NsRequestServerStopper {
    /// IRequestServerStopper
    pub s: Service,
}

/// ProgressMonitorForDeleteUserSaveDataAll
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NsProgressMonitorForDeleteUserSaveDataAll {
    /// IProgressMonitorForDeleteUserSaveDataAll
    pub s: Service,
}

/// ProgressAsyncResult
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NsProgressAsyncResult {
    /// IProgressAsyncResult
    pub s: Service,
    /// Event with autoclear=false.
    pub event: Event,
}

/// SystemUpdateControl
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NsSystemUpdateControl {
    /// ISystemUpdateControl
    pub s: Service,
    /// TransferMemory for SetupCardUpdate/SetupCardUpdateViaSystemUpdater.
    pub tmem: TransferMemory,
}

/// ApplicationControlData
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NsApplicationControlData {
    /// **Ref:** NacpStruct
    pub nacp: NacpStruct,
    /// JPEG
    pub icon: [u8; 131072],
}

/// ApplicationOccupiedSize
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NsApplicationOccupiedSize {
    /// Unknown.
    pub unk_x0: [u8; 128],
}

/// NsApplicationContentMetaStatus
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NsApplicationContentMetaStatus {
    /// **Ref:** NcmContentMetaType
    pub meta_type: u8,
    /// **Ref:** NcmStorageId
    pub storageID: u8,
    /// RightsCheck.
    pub rights_check: u8,
    /// Reserved.
    pub reserved: u8,
    /// Application version.
    pub version: u32,
    /// ApplicationId.
    pub application_id: u64,
}

/// ApplicationRecord
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NsApplicationRecord {
    /// ApplicationId.
    pub application_id: u64,
    /// LastEvent.
    pub last_event: u8,
    /// Attributes.
    pub attributes: u8,
    /// Reserved.
    pub reserved: [u8; 6],
    /// LastUpdated.
    pub last_updated: u64,
}

/// ProgressForDeleteUserSaveDataAll
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NsProgressForDeleteUserSaveDataAll {
    /// Unknown.
    pub unk_x0: [u8; 40],
}

/// ApplicationViewDeprecated. The below comments are for the `NsApplicationView` to NsApplicationViewDeprecated conversion done by `nsGetApplicationViewDeprecated` on newer system-versions.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NsApplicationViewDeprecated {
    /// Same as NsApplicationView::application_id.
    pub application_id: u64,
    /// Same as NsApplicationView::unk_x8.
    pub unk_x8: [u8; 4],
    /// Same as NsApplicationView::flags.
    pub flags: u32,
    /// Same as NsApplicationView::unk_x10.
    pub unk_x10: [u8; 16],
    /// Same as NsApplicationView::unk_x20.
    pub unk_x20: u32,
    /// Same as NsApplicationView::unk_x24.
    pub unk_x24: u16,
    /// Cleared to zero.
    pub unk_x26: [u8; 2],
    /// Same as NsApplicationView::unk_x30.
    pub unk_x28: [u8; 16],
    /// Same as NsApplicationView::unk_x40.
    pub unk_x38: u32,
    /// Same as NsApplicationView::unk_x44.
    pub unk_x3c: u8,
    /// Cleared to zero.
    pub unk_x3d: [u8; 3],
}

/// ApplicationView
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NsApplicationView {
    /// ApplicationId.
    pub application_id: u64,
    /// Unknown.
    pub unk_x8: [u8; 4],
    /// Flags.
    pub flags: u32,
    /// Unknown.
    pub unk_x10: [u8; 16],
    /// Unknown.
    pub unk_x20: u32,
    /// Unknown.
    pub unk_x24: u16,
    /// Unknown.
    pub unk_x26: [u8; 2],
    /// Unknown.
    pub unk_x28: [u8; 8],
    /// Unknown.
    pub unk_x30: [u8; 16],
    /// Unknown.
    pub unk_x40: u32,
    /// Unknown.
    pub unk_x44: u8,
    /// Unknown.
    pub unk_x45: [u8; 11],
}

/// NsPromotionInfo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NsPromotionInfo {
    /// POSIX timestamp for the promotion start.
    pub start_timestamp: u64,
    /// POSIX timestamp for the promotion end.
    pub end_timestamp: u64,
    /// Remaining time until the promotion ends, in nanoseconds ({end_timestamp - current_time} converted to nanoseconds).
    pub remaining_time: s64,
    /// Not set, left at zero.
    pub unk_x18: [u8; 4],
    /// Flags. Bit0: whether the PromotionInfo is valid (including bit1). Bit1 clear: remaining_time is set.
    pub flags: u8,
    /// Padding.
    pub pad: [u8; 3],
}

/// NsApplicationViewWithPromotionInfo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NsApplicationViewWithPromotionInfo {
    /// **Ref:** NsApplicationView
    pub view: NsApplicationView,
    /// **Ref:** NsPromotionInfo
    pub promotion: NsPromotionInfo,
}

/// LaunchProperties
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NsLaunchProperties {
    /// program_id.
    pub program_id: u64,
    /// Program version.
    pub version: u32,
    /// **Ref:** NcmStorageId
    pub storageID: u8,
    /// Index.
    pub index: u8,
    /// Whether this is an Application.
    pub is_application: u8,
}

/// ShellEventInfo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NsShellEventInfo {
    /// **Ref:** NsShellEvent
    pub event: NsShellEvent,
    /// processID.
    pub process_id: u64,
}

/// SystemUpdateProgress. Commands which have this as output will return 0 with the output cleared, when no task is available.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NsSystemUpdateProgress {
    /// Current size. This value can be larger than total_size when the async operation is finishing. When total_size is &lt;=0, this current_size field may contain a progress value for when the total_size is not yet determined.
    pub current_size: s64,
    /// Total size, this field is only valid when >0.
    pub total_size: s64,
}

/// ReceiveApplicationProgress. Same as `NsSystemUpdateProgress`, except cmds which return this will return actual errors on failure, instead of returning 0 with a cleared struct.
pub type NsReceiveApplicationProgress = NsSystemUpdateProgress;

/// SendApplicationProgress. Same as `NsSystemUpdateProgress`, except cmds which return this will return actual errors on failure, instead of returning 0 with a cleared struct.
pub type NsSendApplicationProgress = NsSystemUpdateProgress;

/// EulaDataPath
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NsEulaDataPath {
    /// Path.
    pub path: [c_char; 256],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NsSystemDeliveryInfo_data {
    /// SystemDeliveryProtocolVersion.
    pub system_delivery_protocol_version: u32,
    /// ApplicationDeliveryProtocolVersion.
    pub application_delivery_protocol_version: u32,
    /// HasExFat.
    pub has_exfat: u8,
    /// Reserved.
    pub reserved: [u8; 3],
    /// SystemUpdateVersion.
    pub system_update_version: u32,
    /// OldSystemUpdateId.
    pub old_system_update_id: u64,
    /// FirmwareVariationId.
    pub firmware_variation_id: u8,
    /// UpdatableFirmwareGroupId.
    pub updatable_firmware_group_id: u8,
    /// PlatformRegion.
    pub platform_region: u8,
    /// [20.0.0+] SystemDeliveryInfoPlatform.
    pub system_delivery_info_platform: u8,
    /// [20.0.0+] SystemUpdateIdFlag. When non-zero, SystemUpdateId is used instead of OldSystemUpdateId.
    pub system_update_id_flag: u8,
    /// Padding.
    pub pad: [u8; 3],
    /// [20.0.0+] SystemUpdateId.
    pub system_update_id: u64,
    /// Reserved.
    pub reserved_x28: [u8; 184],
}

/// SystemDeliveryInfo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NsSystemDeliveryInfo {
    /// Data used with the below hmac.
    pub data: NsSystemDeliveryInfo_data,
    /// HMAC-SHA256 over the above data.
    pub hmac: [u8; 32],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NsApplicationDeliveryInfo_data {
    /// ApplicationDeliveryProtocolVersion.
    pub application_delivery_protocol_version: u32,
    /// Padding.
    pub pad: [u8; 4],
    /// ApplicationId.
    pub application_id: u64,
    /// ApplicationVersion.
    pub application_version: u32,
    /// RequiredApplicationVersion.
    pub required_application_version: u32,
    /// RequiredSystemVersion.
    pub required_system_version: u32,
    /// Bitfield of ApplicationDeliveryAttributeTag.
    pub attributes: u32,
    /// [20.0.0+] `NcmContentMetaPlatform`
    pub platform: u8,
    /// [20.0.0+] ProperProgramExists.
    pub proper_program_exists: u8,
    /// Reserved.
    pub reserved: [u8; 190],
}

/// ApplicationDeliveryInfo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NsApplicationDeliveryInfo {
    /// Data used with the below hmac.
    pub data: NsApplicationDeliveryInfo_data,
    /// HMAC-SHA256 over the above data.
    pub hmac: [u8; 32],
}

/// NsApplicationRightsOnClient
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NsApplicationRightsOnClient {
    /// ApplicationId.
    pub application_id: u64,
    /// **Ref:** AccountUid
    pub uid: AccountUid,
    /// qlaunch uses bit0-bit4 and bit7 from here.
    pub flags_x18: u8,
    /// qlaunch uses bit0 from here.
    pub flags_x19: u8,
    /// Unknown.
    pub unk_x1a: [u8; 6],
}

/// DownloadTaskStatus
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NsDownloadTaskStatus {
    /// Unknown.
    pub unk_x0: [u8; 32],
}

impl_zeroed_default!(NsRequestServerStopper, NsProgressMonitorForDeleteUserSaveDataAll, NsProgressAsyncResult, NsSystemUpdateControl, NsApplicationControlData, NsApplicationOccupiedSize, NsApplicationContentMetaStatus, NsApplicationRecord, NsProgressForDeleteUserSaveDataAll, NsApplicationViewDeprecated, NsApplicationView, NsPromotionInfo, NsApplicationViewWithPromotionInfo, NsLaunchProperties, NsShellEventInfo, NsSystemUpdateProgress, NsEulaDataPath, NsSystemDeliveryInfo_data, NsSystemDeliveryInfo, NsApplicationDeliveryInfo_data, NsApplicationDeliveryInfo, NsApplicationRightsOnClient, NsDownloadTaskStatus);

unsafe extern "C" {
    /// Initialize ns services. Uses ns:am on pre-3.0.0, ns:am2 on [3.0.0+].
    pub fn nsInitialize() -> Result;

    /// Exit ns services.
    pub fn nsExit();

    /// Gets the Service object for the actual ns:* service session. Only initialized on [3.0.0+], on pre-3.0.0 see `nsGetServiceSession_ApplicationManagerInterface`.
    pub fn nsGetServiceSession_GetterInterface() -> *mut Service;

    /// Gets the Service object for IApplicationManagerInterface. Only initialized on pre-3.0.0, on [3.0.0+] use `nsGetApplicationManagerInterface`.
    pub fn nsGetServiceSession_ApplicationManagerInterface() -> *mut Service;

    /// Gets the Service object for IDynamicRightsInterface via the cmd for that.
    /// Only available on [6.0.0+].
    pub fn nsGetDynamicRightsInterface(srv_out: *mut Service) -> Result;

    /// Gets the Service object for IReadOnlyApplicationControlDataInterface via the cmd for that.
    /// Only available on [5.1.0+].
    pub fn nsGetReadOnlyApplicationControlDataInterface(srv_out: *mut Service) -> Result;

    /// Gets the Service object for IReadOnlyApplicationRecordInterface via the cmd for that.
    /// Only available on [5.0.0+].
    pub fn nsGetReadOnlyApplicationRecordInterface(srv_out: *mut Service) -> Result;

    /// Gets the Service object for IECommerceInterface via the cmd for that.
    /// Only available on [4.0.0+].
    pub fn nsGetECommerceInterface(srv_out: *mut Service) -> Result;

    /// Gets the Service object for IApplicationVersionInterface via the cmd for that.
    /// Only available on [4.0.0+].
    pub fn nsGetApplicationVersionInterface(srv_out: *mut Service) -> Result;

    /// Gets the Service object for IFactoryResetInterface via the cmd for that.
    /// Only available on [3.0.0+].
    pub fn nsGetFactoryResetInterface(srv_out: *mut Service) -> Result;

    /// Gets the Service object for IAccountProxyInterface via the cmd for that.
    /// Only available on [3.0.0+].
    pub fn nsGetAccountProxyInterface(srv_out: *mut Service) -> Result;

    /// Gets the Service object for IApplicationManagerInterface via the cmd for that.
    /// Only available on [3.0.0+], on prior sysvers use `nsGetServiceSession_ApplicationManagerInterface`.
    pub fn nsGetApplicationManagerInterface(srv_out: *mut Service) -> Result;

    /// Gets the Service object for IDownloadTaskInterface via the cmd for that.
    /// Only available on [3.0.0+].
    pub fn nsGetDownloadTaskInterface(srv_out: *mut Service) -> Result;

    /// Gets the Service object for IContentManagementInterface via the cmd for that.
    /// Only available on [3.0.0+].
    pub fn nsGetContentManagementInterface(srv_out: *mut Service) -> Result;

    /// Gets the Service object for IDocumentInterface via the cmd for that.
    /// Only available on [3.0.0+].
    pub fn nsGetDocumentInterface(srv_out: *mut Service) -> Result;

    /// Gets the `NsApplicationControlData` for the specified application.
    ///
    /// # Arguments
    /// - `source` (in): Source, official sw uses ::NsApplicationControlSource_Storage.
    /// - `application_id` (in): ApplicationId.
    /// - `buffer` (out): `NsApplicationControlData`
    /// - `size` (in): Size of the buffer.
    /// - `actual_size` (out): Actual output size.
    ///
    /// **Note:** Uses `nsGetReadOnlyApplicationControlDataInterface` on [5.1.0+], otherwise IApplicationManagerInterface is used.
    pub fn nsGetApplicationControlData(source: NsApplicationControlSource, application_id: u64, buffer: *mut NsApplicationControlData, size: usize, actual_size: *mut u64) -> Result;

    /// Gets the `NsApplicationControlData` for the specified application.
    ///
    /// # Arguments
    /// - `source` (in): Source, official sw uses ::NsApplicationControlSource_Storage.
    /// - `application_id` (in): ApplicationId.
    /// - `buffer` (out): `NsApplicationControlData`
    /// - `size` (in): Size of the buffer.
    /// - `flag1` (in): Default is 0. 0xFF speeds up execution.
    /// - `flag2` (in): Default is 0.
    /// - `actual_size` (out): Actual output size.
    /// - `unk` (out): Returned with size, always 0.
    ///
    /// **Note:** Only available on [19.0.0+].
    pub fn nsGetApplicationControlData2(source: NsApplicationControlSource, application_id: u64, buffer: *mut NsApplicationControlData, size: usize, flag1: u8, flag2: u8, actual_size: *mut u64, unk: *mut u32) -> Result;

    /// GetApplicationDesiredLanguage. Selects a `NacpLanguageEntry` to use from the specified `NacpStruct`.
    ///
    /// # Arguments
    /// - `nacp` (in): `NacpStruct`
    /// - `langentry` (out): `NacpLanguageEntry`
    ///
    /// **Note:** Uses `nsGetReadOnlyApplicationControlDataInterface` on [5.1.0+], otherwise IApplicationManagerInterface is used.
    pub fn nsGetApplicationDesiredLanguage(nacp: *mut NacpStruct, langentry: *mut *mut NacpLanguageEntry) -> Result;

    /// RequestLinkDevice
    ///
    /// # Arguments
    /// - `a` (out): `AsyncResult`
    /// - `uid` (in): `AccountUid`
    ///
    /// **Note:** `nifmInitialize` must be used prior to this. Before using the cmd, this calls `nifmIsAnyInternetRequestAccepted` with the output from `nifmGetClientId`, an error is returned when that returns false.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nsRequestLinkDevice(a: *mut AsyncResult, uid: AccountUid) -> Result;

    /// RequestSyncRights
    ///
    /// # Arguments
    /// - `a` (out): `AsyncResult`
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn nsRequestSyncRights(a: *mut AsyncResult) -> Result;

    /// RequestUnlinkDevice
    ///
    /// # Arguments
    /// - `a` (out): `AsyncResult`
    /// - `uid` (in): `AccountUid`
    ///
    /// **Note:** `nifmInitialize` must be used prior to this. Before using the cmd, this calls `nifmIsAnyInternetRequestAccepted` with the output from `nifmGetClientId`, an error is returned when that returns false.
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn nsRequestUnlinkDevice(a: *mut AsyncResult, uid: AccountUid) -> Result;

    /// ResetToFactorySettings
    ///
    /// **Note:** Uses `nsGetFactoryResetInterface` on [3.0.0+], otherwise IApplicationManagerInterface is used.
    pub fn nsResetToFactorySettings() -> Result;

    /// ResetToFactorySettingsWithoutUserSaveData
    ///
    /// **Note:** Uses `nsGetFactoryResetInterface` on [3.0.0+], otherwise IApplicationManagerInterface is used.
    pub fn nsResetToFactorySettingsWithoutUserSaveData() -> Result;

    /// ResetToFactorySettingsForRefurbishment
    ///
    /// **Note:** Uses `nsGetFactoryResetInterface` on [3.0.0+], otherwise IApplicationManagerInterface is used.
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn nsResetToFactorySettingsForRefurbishment() -> Result;

    /// ResetToFactorySettingsWithPlatformRegion
    ///
    /// **Note:** Only available on [9.1.0+].
    pub fn nsResetToFactorySettingsWithPlatformRegion() -> Result;

    /// ResetToFactorySettingsWithPlatformRegionAuthentication
    ///
    /// **Note:** Only available on [9.1.0+].
    pub fn nsResetToFactorySettingsWithPlatformRegionAuthentication() -> Result;

    /// Gets an listing of `NsApplicationRecord`.
    ///
    /// # Arguments
    /// - `records` (out): Output array of `NsApplicationRecord`.
    /// - `count` (in): Size of the records array in entries.
    /// - `entry_offset` (in): Starting entry offset.
    /// - `out_entrycount` (out): Total output entries.
    pub fn nsListApplicationRecord(records: *mut NsApplicationRecord, count: s32, entry_offset: s32, out_entrycount: *mut s32) -> Result;

    /// GetApplicationRecordUpdateSystemEvent
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=true.
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn nsGetApplicationRecordUpdateSystemEvent(out_event: *mut Event) -> Result;

    /// GetApplicationViewDeprecated
    ///
    /// # Arguments
    /// - `out` (out): Output array of `NsApplicationViewDeprecated`.
    /// - `application_ids` (in): Input array of ApplicationIds.
    /// - `count` (in): Size of the input/output arrays in entries.
    ///
    /// **Note:** On [3.0.0+] you should generally use `nsGetApplicationView` instead.
    pub fn nsGetApplicationViewDeprecated(views: *mut NsApplicationViewDeprecated, application_ids: *const u64, count: s32) -> Result;

    /// DeleteApplicationEntity
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId.
    pub fn nsDeleteApplicationEntity(application_id: u64) -> Result;

    /// DeleteApplicationCompletely
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId.
    pub fn nsDeleteApplicationCompletely(application_id: u64) -> Result;

    /// DeleteRedundantApplicationEntity
    pub fn nsDeleteRedundantApplicationEntity() -> Result;

    /// IsApplicationEntityMovable
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId.
    /// - `storage_id` (in): `NcmStorageId`
    /// - `out` (out): Output flag.
    pub fn nsIsApplicationEntityMovable(application_id: u64, storage_id: NcmStorageId, out: *mut bool) -> Result;

    /// MoveApplicationEntity
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId.
    /// - `storage_id` (in): `NcmStorageId`
    ///
    /// **Note:** Only available on [1.0.0-9.2.0].
    pub fn nsMoveApplicationEntity(application_id: u64, storage_id: NcmStorageId) -> Result;

    /// RequestApplicationUpdateInfo
    ///
    /// # Arguments
    /// - `a` (out): `AsyncValue`. The data that can be read from this is u8 ApplicationUpdateInfo. qlaunch just checks whether this is 0.
    /// - `application_id`: ApplicationId.
    ///
    /// **Note:** `nifmInitialize` must be used prior to this. Before using the cmd, this calls `nifmIsAnyInternetRequestAccepted` with the output from `nifmGetClientId`, an error is returned when that returns false.
    pub fn nsRequestApplicationUpdateInfo(a: *mut AsyncValue, application_id: u64) -> Result;

    /// CancelApplicationDownload
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId.
    pub fn nsCancelApplicationDownload(application_id: u64) -> Result;

    /// ResumeApplicationDownload
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId.
    pub fn nsResumeApplicationDownload(application_id: u64) -> Result;

    /// CheckApplicationLaunchVersion
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId.
    pub fn nsCheckApplicationLaunchVersion(application_id: u64) -> Result;

    /// CalculateApplicationApplyDeltaRequiredSize
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId.
    /// - `storage_id` (out): Output `NcmStorageId`.
    /// - `size` (out): Output size.
    pub fn nsCalculateApplicationDownloadRequiredSize(application_id: u64, storage_id: *mut NcmStorageId, size: *mut s64) -> Result;

    /// CleanupSdCard
    pub fn nsCleanupSdCard() -> Result;

    /// GetSdCardMountStatusChangedEvent
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=false.
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn nsGetSdCardMountStatusChangedEvent(out_event: *mut Event) -> Result;

    /// GetGameCardUpdateDetectionEvent
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=false.
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn nsGetGameCardUpdateDetectionEvent(out_event: *mut Event) -> Result;

    /// DisableApplicationAutoDelete
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId.
    pub fn nsDisableApplicationAutoDelete(application_id: u64) -> Result;

    /// EnableApplicationAutoDelete
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId.
    pub fn nsEnableApplicationAutoDelete(application_id: u64) -> Result;

    /// SetApplicationTerminateResult
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId.
    /// - `res` (in): Result.
    pub fn nsSetApplicationTerminateResult(application_id: u64, res: Result) -> Result;

    /// ClearApplicationTerminateResult
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId.
    pub fn nsClearApplicationTerminateResult(application_id: u64) -> Result;

    /// GetLastSdCardMountUnexpectedResult
    pub fn nsGetLastSdCardMountUnexpectedResult() -> Result;

    /// Opens a `NsRequestServerStopper`.
    ///
    /// # Arguments
    /// - `r` (out): `NsRequestServerStopper`
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn nsGetRequestServerStopper(r: *mut NsRequestServerStopper) -> Result;

    /// CancelApplicationApplyDelta
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn nsCancelApplicationApplyDelta(application_id: u64) -> Result;

    /// ResumeApplicationApplyDelta
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn nsResumeApplicationApplyDelta(application_id: u64) -> Result;

    /// CalculateApplicationApplyDeltaRequiredSize
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId.
    /// - `storage_id` (out): Output `NcmStorageId`.
    /// - `size` (out): Output size.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn nsCalculateApplicationApplyDeltaRequiredSize(application_id: u64, storage_id: *mut NcmStorageId, size: *mut s64) -> Result;

    /// ResumeAll
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn nsResumeAll() -> Result;

    /// Temporarily mounts the specified fs ContentStorage, then uses fs GetTotalSpaceSize/GetFreeSpaceSize with that mounted ContentStorage.
    ///
    /// # Arguments
    /// - `storage_id` (in): `NcmStorageId`, must be ::NcmStorageId_BuiltInUser or ::NcmStorageId_SdCard.
    /// - `total_space_size` (out): Output from GetTotalSpaceSize.
    /// - `free_space_size` (out): Output from GetFreeSpaceSize.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn nsGetStorageSize(storage_id: NcmStorageId, total_space_size: *mut s64, free_space_size: *mut s64) -> Result;

    /// RequestUpdateApplication2
    ///
    /// # Arguments
    /// - `a` (out): `AsyncResult`
    /// - `application_id` (in): ApplicationId.
    ///
    /// **Note:** `nifmInitialize` must be used prior to this. Before using the cmd, this calls `nifmIsAnyInternetRequestAccepted` with the output from `nifmGetClientId`, an error is returned when that returns false.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nsRequestUpdateApplication2(a: *mut AsyncResult, application_id: u64) -> Result;

    /// DeleteUserSaveDataAll
    ///
    /// # Arguments
    /// - `p` (in): `NsProgressMonitorForDeleteUserSaveDataAll`
    /// - `uid` (in): `AccountUid`
    pub fn nsDeleteUserSaveDataAll(p: *mut NsProgressMonitorForDeleteUserSaveDataAll, uid: AccountUid) -> Result;

    /// DeleteUserSystemSaveData
    ///
    /// # Arguments
    /// - `uid` (in): `AccountUid`
    /// - `system_save_data_id` (in): SystemSaveDataId
    pub fn nsDeleteUserSystemSaveData(uid: AccountUid, system_save_data_id: u64) -> Result;

    /// DeleteSaveData
    ///
    /// # Arguments
    /// - `save_data_space_id` (in): `FsSaveDataSpaceId`
    /// - `save_data_id` (in): SaveDataId
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn nsDeleteSaveData(save_data_space_id: FsSaveDataSpaceId, save_data_id: u64) -> Result;

    /// UnregisterNetworkServiceAccount
    ///
    /// # Arguments
    /// - `uid` (in): `AccountUid`
    pub fn nsUnregisterNetworkServiceAccount(uid: AccountUid) -> Result;

    /// UnregisterNetworkServiceAccountWithUserSaveDataDeletion
    ///
    /// # Arguments
    /// - `uid` (in): `AccountUid`
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn nsUnregisterNetworkServiceAccountWithUserSaveDataDeletion(uid: AccountUid) -> Result;

    /// RequestDownloadApplicationControlData
    ///
    /// # Arguments
    /// - `a` (out): `AsyncResult`
    /// - `application_id` (in): ApplicationId.
    ///
    /// **Note:** `nifmInitialize` must be used prior to this. Before using the cmd, this calls `nifmIsAnyInternetRequestAccepted` with the output from `nifmGetClientId`, an error is returned when that returns false.
    pub fn nsRequestDownloadApplicationControlData(a: *mut AsyncResult, application_id: u64) -> Result;

    /// ListApplicationTitle
    ///
    /// # Arguments
    /// - `a` (out): `AsyncValue`
    /// - `source` (in): Source, qlaunch uses ::NsApplicationControlSource_Storage.
    /// - `application_ids` (in): Input array of ApplicationIds.
    /// - `count` (in): Size of the application_ids array in entries.
    /// - `buffer`: 0x1000-byte aligned buffer for TransferMemory. This buffer must not be accessed until the async operation finishes.
    /// - `size` (in): 0x1000-byte aligned buffer size for TransferMemory. This must be at least: count*sizeof(`NacpLanguageEntry`) + count*sizeof(u64) + count*sizeof(`NsApplicationControlData`).
    ///
    /// **Note:** The data available with `asyncValueGet` is a s32 for the offset within the buffer where the output data is located, `asyncValueGetSize` returns the total byte-size of the data located here. The data located here is the `NacpLanguageEntry` for each specified ApplicationId.
    ///
    /// **Note:** Only available on [8.0.0+].
    pub fn nsListApplicationTitle(a: *mut AsyncValue, source: NsApplicationControlSource, application_ids: *const u64, count: s32, buffer: *mut c_void, size: usize) -> Result;

    /// ListApplicationIcon
    ///
    /// # Arguments
    /// - `a` (out): `AsyncValue`
    /// - `source` (in): Source.
    /// - `application_ids` (in): Input array of ApplicationIds.
    /// - `count` (in): Size of the application_ids array in entries.
    /// - `buffer`: 0x1000-byte aligned buffer for TransferMemory. This buffer must not be accessed until the async operation finishes.
    /// - `size` (in): 0x1000-byte aligned buffer size for TransferMemory. This must be at least: 0x4 + count*sizeof(u64) + count*sizeof(`NsApplicationControlData`::icon) + count*sizeof(u64) + sizeof(`NsApplicationControlData`).
    ///
    /// **Note:** The data available with `asyncValueGet` is a s32 for the offset within the buffer where the output data is located, `asyncValueGetSize` returns the total byte-size of the data located here. This data is: an u64 for total entries, an array of u64s for each icon size, then the icon JPEGs for the specified ApplicationIds.
    ///
    /// **Note:** Only available on [8.0.0+].
    pub fn nsListApplicationIcon(a: *mut AsyncValue, source: NsApplicationControlSource, application_ids: *const u64, count: s32, buffer: *mut c_void, size: usize) -> Result;

    /// RequestCheckGameCardRegistration
    ///
    /// # Arguments
    /// - `a` (out): `AsyncResult`
    /// - `application_id` (in): ApplicationId.
    ///
    /// **Note:** `nifmInitialize` must be used prior to this. Before using the cmd, this calls `nifmIsAnyInternetRequestAccepted` with the output from `nifmGetClientId`, an error is returned when that returns false.
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn nsRequestCheckGameCardRegistration(a: *mut AsyncResult, application_id: u64) -> Result;

    /// RequestGameCardRegistrationGoldPoint
    ///
    /// # Arguments
    /// - `a` (out): `AsyncValue`. The data that can be read from this is 4-bytes.
    /// - `uid` (in): `AccountUid`
    /// - `application_id` (in): ApplicationId.
    ///
    /// **Note:** `nifmInitialize` must be used prior to this. Before using the cmd, this calls `nifmIsAnyInternetRequestAccepted` with the output from `nifmGetClientId`, an error is returned when that returns false.
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn nsRequestGameCardRegistrationGoldPoint(a: *mut AsyncValue, uid: AccountUid, application_id: u64) -> Result;

    /// RequestRegisterGameCard
    ///
    /// # Arguments
    /// - `a` (out): `AsyncResult`
    /// - `uid` (in): `AccountUid`
    /// - `application_id` (in): ApplicationId.
    /// - `inval` (in): Input value.
    ///
    /// **Note:** `nifmInitialize` must be used prior to this. Before using the cmd, this calls `nifmIsAnyInternetRequestAccepted` with the output from `nifmGetClientId`, an error is returned when that returns false.
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn nsRequestRegisterGameCard(a: *mut AsyncResult, uid: AccountUid, application_id: u64, inval: s32) -> Result;

    /// GetGameCardMountFailureEvent
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=false.
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn nsGetGameCardMountFailureEvent(out_event: *mut Event) -> Result;

    /// IsGameCardInserted
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn nsIsGameCardInserted(out: *mut bool) -> Result;

    /// EnsureGameCardAccess
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn nsEnsureGameCardAccess() -> Result;

    /// GetLastGameCardMountFailureResult
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn nsGetLastGameCardMountFailureResult() -> Result;

    /// ListApplicationIdOnGameCard
    ///
    /// # Arguments
    /// - `application_ids` (out): Output array of ApplicationIds.
    /// - `count` (in): Size of the application_ids array in entries.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn nsListApplicationIdOnGameCard(application_ids: *mut u64, count: s32, total_out: *mut s32) -> Result;

    /// TouchApplication
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId.
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn nsTouchApplication(application_id: u64) -> Result;

    /// IsApplicationUpdateRequested
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId.
    /// - `flag` (out): Output flag, indicating whether out is valid.
    /// - `out` (out): Output value.
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn nsIsApplicationUpdateRequested(application_id: u64, flag: *mut bool, out: *mut u32) -> Result;

    /// WithdrawApplicationUpdateRequest
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId.
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn nsWithdrawApplicationUpdateRequest(application_id: u64) -> Result;

    /// RequestVerifyAddOnContentsRights
    ///
    /// # Arguments
    /// - `a` (out): `NsProgressAsyncResult`
    /// - `application_id` (in): ApplicationId.
    ///
    /// **Note:** Only available on [3.0.0-9.2.0].
    pub fn nsRequestVerifyAddOnContentsRights(a: *mut NsProgressAsyncResult, application_id: u64) -> Result;

    /// RequestVerifyApplication
    ///
    /// # Arguments
    /// - `a` (out): `NsProgressAsyncResult`. The data available with `nsProgressAsyncResultGetProgress` is basically the same as `NsSystemUpdateProgress`.
    /// - `application_id` (in): ApplicationId.
    /// - `unk` (in): Unknown. A default value of 0x7 can be used (which is what qlaunch uses). Only used on [5.0.0+].
    /// - `buffer`: 0x1000-byte aligned buffer for TransferMemory. This buffer must not be accessed until the async operation finishes.
    /// - `size` (in): 0x1000-byte aligned buffer size for TransferMemory. qlaunch uses size 0x100000.
    ///
    /// **Note:** On pre-5.0.0 this uses cmd RequestVerifyApplicationDeprecated, otherwise cmd RequestVerifyApplication is used.
    pub fn nsRequestVerifyApplication(a: *mut NsProgressAsyncResult, application_id: u64, unk: u32, buffer: *mut c_void, size: usize) -> Result;

    /// IsAnyApplicationEntityInstalled
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId.
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn nsIsAnyApplicationEntityInstalled(application_id: u64, out: *mut bool) -> Result;

    /// CleanupUnavailableAddOnContents
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId.
    /// - `uid` (in): `AccountUid`
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn nsCleanupUnavailableAddOnContents(application_id: u64, uid: AccountUid) -> Result;

    /// EstimateSizeToMove
    ///
    /// # Arguments
    /// - `storage_ids` (in): Array of u8 `NcmStorageId`.
    /// - `count` (in): Size of the storage_ids array in entries.
    /// - `storage_id` (in): storage_id `NcmStorageId`
    /// - `flags` (in): Flags
    /// - `application_id` (in): ApplicationId.
    /// - `Out` (out): Output value.
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn nsEstimateSizeToMove(storage_ids: *mut u8, count: s32, storage_id: NcmStorageId, flags: u32, application_id: u64, out: *mut s64) -> Result;

    /// FormatSdCard
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn nsFormatSdCard() -> Result;

    /// NeedsSystemUpdateToFormatSdCard
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn nsNeedsSystemUpdateToFormatSdCard(out: *mut bool) -> Result;

    /// GetLastSdCardFormatUnexpectedResult
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn nsGetLastSdCardFormatUnexpectedResult() -> Result;

    /// GetApplicationView
    ///
    /// # Arguments
    /// - `out` (out): Output array of `NsApplicationView`.
    /// - `application_ids` (in): Input array of ApplicationIds.
    /// - `count` (in): Size of the input/output arrays in entries.
    ///
    /// **Note:** Only available on [3.0.0+], on prior system-versions use `nsGetApplicationViewDeprecated` instead.
    pub fn nsGetApplicationView(views: *mut NsApplicationView, application_ids: *const u64, count: s32) -> Result;

    /// GetApplicationViewDownloadErrorContext
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId
    /// - `context` (out): `ErrorContext`
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nsGetApplicationViewDownloadErrorContext(application_id: u64, context: *mut ErrorContext) -> Result;

    /// GetApplicationViewWithPromotionInfo
    ///
    /// # Arguments
    /// - `out` (out): Output array of `NsApplicationViewWithPromotionInfo`.
    /// - `application_ids` (in): Input array of ApplicationIds.
    /// - `count` (in): Size of the input/output arrays in entries.
    ///
    /// **Note:** Only available on [8.0.0+].
    pub fn nsGetApplicationViewWithPromotionInfo(out: *mut NsApplicationViewWithPromotionInfo, application_ids: *const u64, count: s32) -> Result;

    /// RequestDownloadApplicationPrepurchasedRights
    ///
    /// # Arguments
    /// - `a` (out): `AsyncResult`
    /// - `application_id` (in): ApplicationId.
    ///
    /// **Note:** `nifmInitialize` must be used prior to this. Before using the cmd, this calls `nifmIsAnyInternetRequestAccepted` with the output from `nifmGetClientId`, an error is returned when that returns false.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nsRequestDownloadApplicationPrepurchasedRights(a: *mut AsyncResult, application_id: u64) -> Result;

    /// Generates a `NsSystemDeliveryInfo` using the currently installed SystemUpdate meta.
    ///
    /// # Arguments
    /// - `info` (out): `NsSystemDeliveryInfo`
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nsGetSystemDeliveryInfo(info: *mut NsSystemDeliveryInfo) -> Result;

    /// SelectLatestSystemDeliveryInfo
    ///
    /// # Arguments
    /// - `sys_list` (in): Input array of `NsSystemDeliveryInfo`.
    /// - `sys_count` (in): Size of the sys_list array in entries.
    /// - `base_info` (in): `NsSystemDeliveryInfo`
    /// - `app_list` (in): Input array of `NsApplicationDeliveryInfo`. This can be NULL.
    /// - `app_count` (in): Size of the app_list array in entries. This can be 0.
    /// - `index` (out): Output index for the selected entry in sys_list, -1 if none found.
    ///
    /// **Note:** This selects the `NsSystemDeliveryInfo` with the latest version from sys_list, using minimum versions determined from app_list/state and base_info, etc.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nsSelectLatestSystemDeliveryInfo(sys_list: *const NsSystemDeliveryInfo, sys_count: s32, base_info: *const NsSystemDeliveryInfo, app_list: *const NsApplicationDeliveryInfo, app_count: s32, index: *mut s32) -> Result;

    /// VerifyDeliveryProtocolVersion
    ///
    /// # Arguments
    /// - `info` (in): `NsSystemDeliveryInfo`
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nsVerifyDeliveryProtocolVersion(info: *const NsSystemDeliveryInfo) -> Result;

    /// Generates `NsApplicationDeliveryInfo` for the specified ApplicationId.
    ///
    /// # Arguments
    /// - `info` (out): Output array of `NsApplicationDeliveryInfo`.
    /// - `count` (in): Size of the array in entries.
    /// - `application_id` (in): ApplicationId
    /// - `attr` (in): ApplicationDeliveryAttributeTag bitmask.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nsGetApplicationDeliveryInfo(info: *mut NsApplicationDeliveryInfo, count: s32, application_id: u64, attr: u32, total_out: *mut s32) -> Result;

    /// HasAllContentsToDeliver
    ///
    /// # Arguments
    /// - `info` (in): Input array of `NsApplicationDeliveryInfo`.
    /// - `count` (in): Size of the array in entries. Must be value 1.
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nsHasAllContentsToDeliver(info: *const NsApplicationDeliveryInfo, count: s32, out: *mut bool) -> Result;

    /// Both `NsApplicationDeliveryInfo` are validated, then the application_version in the first/second `NsApplicationDeliveryInfo` are compared.
    ///
    /// # Arguments
    /// - `info0` (in): First input array of `NsApplicationDeliveryInfo`.
    /// - `count0` (in): Size of the info0 array in entries. Must be value 1.
    /// - `info1` (in): Second input array of `NsApplicationDeliveryInfo`.
    /// - `count1` (in): Size of the info1 array in entries. Must be value 1.
    /// - `out` (out): Comparison result: -1 for less than, 0 for equal, and 1 for higher than.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nsCompareApplicationDeliveryInfo(info0: *const NsApplicationDeliveryInfo, count0: s32, info1: *const NsApplicationDeliveryInfo, count1: s32, out: *mut s32) -> Result;

    /// CanDeliverApplication
    ///
    /// # Arguments
    /// - `info0` (in): First input array of `NsApplicationDeliveryInfo`.
    /// - `count0` (in): Size of the info0 array in entries. Must be value &lt;=1, when 0 this will return 0 with out set to 0.
    /// - `info1` (in): Second input array of `NsApplicationDeliveryInfo`.
    /// - `count1` (in): Size of the info1 array in entries. Must be value 1.
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nsCanDeliverApplication(info0: *const NsApplicationDeliveryInfo, count0: s32, info1: *const NsApplicationDeliveryInfo, count1: s32, out: *mut bool) -> Result;

    /// ListContentMetaKeyToDeliverApplication
    ///
    /// # Arguments
    /// - `meta` (out): Output array of `NcmContentMetaKey`.
    /// - `meta_count` (in): Size of the meta array in entries. Must be at least 1, only 1 entry will be returned.
    /// - `meta_index` (in): Meta entry index. An output `NcmContentMetaKey` will not be returned when this value is larger than 0.
    /// - `info` (in): Input array of `NsApplicationDeliveryInfo`.
    /// - `info_count` (in): Size of the info array in entries. Must be value 1.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nsListContentMetaKeyToDeliverApplication(meta: *mut NcmContentMetaKey, meta_count: s32, meta_index: s32, info: *const NsApplicationDeliveryInfo, info_count: s32, total_out: *mut s32) -> Result;

    /// After validation, this sets the output bool by comparing system-version fields in the `NsSystemDeliveryInfo` / info-array and with a state field, etc.
    ///
    /// # Arguments
    /// - `info` (in): Input array of `NsApplicationDeliveryInfo`.
    /// - `count` (in): Size of the info array in entries. Must be value 1.
    /// - `sys_info` (in): `NsSystemDeliveryInfo`
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nsNeedsSystemUpdateToDeliverApplication(info: *const NsApplicationDeliveryInfo, count: s32, sys_info: *const NsSystemDeliveryInfo, out: *mut bool) -> Result;

    /// EstimateRequiredSize
    ///
    /// # Arguments
    /// - `meta` (in): Input array of `NcmContentMetaKey`.
    /// - `count` (in): Size of the meta array in entries. When less than 1, this will return 0 with out set to 0.
    /// - `out` (out): Output size.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nsEstimateRequiredSize(meta: *const NcmContentMetaKey, count: s32, out: *mut s64) -> Result;

    /// RequestReceiveApplication
    ///
    /// # Arguments
    /// - `a` (out): `AsyncResult`
    /// - `addr` (in): Server IPv4 address.
    /// - `port` (in): Socket port. qlaunch uses value 55556.
    /// - `application_id` (in): ApplicationId
    /// - `meta` (in): Input array of `NcmContentMetaKey`. The ::NcmContentMetaType must match ::NcmContentMetaType_Patch.
    /// - `count` (in): Size of the meta array in entries.
    /// - `storage_id` (in): `NcmStorageId`. qlaunch uses ::NcmStorageId_Any.
    ///
    /// **Note:** This is the Application version of `nssuControlRequestReceiveSystemUpdate`, see the notes for that.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nsRequestReceiveApplication(a: *mut AsyncResult, addr: u32, port: u16, application_id: u64, meta: *const NcmContentMetaKey, count: s32, storage_id: NcmStorageId) -> Result;

    /// CommitReceiveApplication
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nsCommitReceiveApplication(application_id: u64) -> Result;

    /// GetReceiveApplicationProgress
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId
    /// - `out` (out): `NsReceiveApplicationProgress`
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nsGetReceiveApplicationProgress(application_id: u64, out: *mut NsReceiveApplicationProgress) -> Result;

    /// RequestSendApplication
    ///
    /// # Arguments
    /// - `a` (out): `AsyncResult`
    /// - `addr` (in): Client IPv4 address.
    /// - `port` (in): Socket port. qlaunch uses value 55556.
    /// - `application_id` (in): ApplicationId
    /// - `meta` (in): Input array of `NcmContentMetaKey`. The ::NcmContentMetaType must match ::NcmContentMetaType_Patch.
    /// - `count` (in): Size of the meta array in entries.
    ///
    /// **Note:** This is the Application version of `nssuRequestSendSystemUpdate`, see the notes for that.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nsRequestSendApplication(a: *mut AsyncResult, addr: u32, port: u16, application_id: u64, meta: *const NcmContentMetaKey, count: s32) -> Result;

    /// GetSendApplicationProgress
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId
    /// - `out` (out): `NsSendApplicationProgress`
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nsGetSendApplicationProgress(application_id: u64, out: *mut NsSendApplicationProgress) -> Result;

    /// Both `NsSystemDeliveryInfo` are validated, then the system_update_version in the first/second `NsSystemDeliveryInfo` are compared.
    ///
    /// # Arguments
    /// - `info0` (in): First `NsSystemDeliveryInfo`.
    /// - `info1` (in): Second `NsSystemDeliveryInfo`.
    /// - `out` (out): Comparison result: -1 for less than, 0 for equal, and 1 for higher than.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nsCompareSystemDeliveryInfo(info0: *const NsSystemDeliveryInfo, info1: *const NsSystemDeliveryInfo, out: *mut s32) -> Result;

    /// ListNotCommittedContentMeta
    ///
    /// # Arguments
    /// - `meta` (out): Output array of `NcmContentMetaKey`.
    /// - `count` (in): Size of the meta array in entries.
    /// - `application_id` (in): ApplicationId
    /// - `unk` (in): Unknown.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nsListNotCommittedContentMeta(meta: *mut NcmContentMetaKey, count: s32, application_id: u64, unk: s32, total_out: *mut s32) -> Result;

    /// This extracts data from the input array for hashing with SHA256, with validation being done when handling each entry.
    ///
    /// # Arguments
    /// - `info` (in): Input array of `NsApplicationDeliveryInfo`.
    /// - `count` (in): Size of the array in entries.
    /// - `out_hash` (out): Output 0x20-byte SHA256 hash.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn nsGetApplicationDeliveryInfoHash(info: *const NsApplicationDeliveryInfo, count: s32, out_hash: *mut u8) -> Result;

    /// GetApplicationTerminateResult
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId.
    /// - `res` (out): Output Result.
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn nsGetApplicationTerminateResult(application_id: u64, res: *mut Result) -> Result;

    /// GetApplicationRightsOnClient
    ///
    /// # Arguments
    /// - `rights` (out): Output array of `NsApplicationRightsOnClient`.
    /// - `count` (in): Size of the rights array in entries. qlaunch uses value 3 for this.
    /// - `application_id` (in): ApplicationId
    /// - `uid` (in): `AccountUid`, can optionally be all-zero.
    /// - `flags` (in): Flags. Official sw hard-codes this to value 0x3.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn nsGetApplicationRightsOnClient(rights: *mut NsApplicationRightsOnClient, count: s32, application_id: u64, uid: AccountUid, flags: u32, total_out: *mut s32) -> Result;

    /// RequestNoDownloadRightsErrorResolution
    ///
    /// # Arguments
    /// - `a` (out): `AsyncValue`. The data that can be read from this is u8 NoDownloadRightsErrorResolution.
    /// - `application_id`: ApplicationId.
    ///
    /// **Note:** `nifmInitialize` must be used prior to this. Before using the cmd, this calls `nifmIsAnyInternetRequestAccepted` with the output from `nifmGetClientId`, an error is returned when that returns false.
    ///
    /// **Note:** Only available on [9.0.0+].
    pub fn nsRequestNoDownloadRightsErrorResolution(a: *mut AsyncValue, application_id: u64) -> Result;

    /// RequestResolveNoDownloadRightsError
    ///
    /// # Arguments
    /// - `a` (out): `AsyncValue`. The data that can be read from this is u8 NoDownloadRightsErrorResolution.
    /// - `application_id`: ApplicationId.
    ///
    /// **Note:** `nifmInitialize` must be used prior to this. Before using the cmd, this calls `nifmIsAnyInternetRequestAccepted` with the output from `nifmGetClientId`, an error is returned when that returns false.
    ///
    /// **Note:** Only available on [9.0.0+].
    pub fn nsRequestResolveNoDownloadRightsError(a: *mut AsyncValue, application_id: u64) -> Result;

    /// GetPromotionInfo
    ///
    /// # Arguments
    /// - `promotion` (out): `NsPromotionInfo`
    /// - `application_id`: ApplicationId.
    /// - `uid` (in): `AccountUid`
    ///
    /// **Note:** Only available on [8.0.0+].
    pub fn nsGetPromotionInfo(promotion: *mut NsPromotionInfo, application_id: u64, uid: AccountUid) -> Result;

    /// ClearTaskStatusList
    ///
    /// **Note:** Uses `nsGetDownloadTaskInterface` on [3.0.0+], otherwise IApplicationManagerInterface is used.
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn nsClearTaskStatusList() -> Result;

    /// RequestDownloadTaskList
    ///
    /// **Note:** Uses `nsGetDownloadTaskInterface` on [3.0.0+], otherwise IApplicationManagerInterface is used.
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn nsRequestDownloadTaskList() -> Result;

    /// RequestEnsureDownloadTask
    ///
    /// # Arguments
    /// - `a` (out): `AsyncResult`
    ///
    /// **Note:** Uses `nsGetDownloadTaskInterface` on [3.0.0+], otherwise IApplicationManagerInterface is used.
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn nsRequestEnsureDownloadTask(a: *mut AsyncResult) -> Result;

    /// ListDownloadTaskStatus
    ///
    /// # Arguments
    /// - `tasks` (out): Output array of `NsDownloadTaskStatus`.
    /// - `count` (in): Size of the tasks array in entries. A maximum of 0x100 tasks can be stored in state.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** Uses `nsGetDownloadTaskInterface` on [3.0.0+], otherwise IApplicationManagerInterface is used.
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn nsListDownloadTaskStatus(tasks: *mut NsDownloadTaskStatus, count: s32, total_out: *mut s32) -> Result;

    /// RequestDownloadTaskListData
    ///
    /// # Arguments
    /// - `a` (out): `AsyncValue`
    ///
    /// **Note:** Uses `nsGetDownloadTaskInterface` on [3.0.0+], otherwise IApplicationManagerInterface is used.
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn nsRequestDownloadTaskListData(a: *mut AsyncValue) -> Result;

    /// TryCommitCurrentApplicationDownloadTask
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nsTryCommitCurrentApplicationDownloadTask() -> Result;

    /// EnableAutoCommit
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nsEnableAutoCommit() -> Result;

    /// DisableAutoCommit
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nsDisableAutoCommit() -> Result;

    /// TriggerDynamicCommitEvent
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nsTriggerDynamicCommitEvent() -> Result;

    /// CalculateApplicationOccupiedSize
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId.
    /// - `out` (out): `NsApplicationOccupiedSize`
    ///
    /// **Note:** Uses `nsGetContentManagementInterface` on [3.0.0+], otherwise IApplicationManagerInterface is used.
    pub fn nsCalculateApplicationOccupiedSize(application_id: u64, out: *mut NsApplicationOccupiedSize) -> Result;

    /// CheckSdCardMountStatus
    ///
    /// **Note:** Uses `nsGetContentManagementInterface` on [3.0.0+], otherwise IApplicationManagerInterface is used.
    pub fn nsCheckSdCardMountStatus() -> Result;

    /// Returns the total storage capacity (used + free) from content manager services.
    ///
    /// # Arguments
    /// - `storage_id` (in): `NcmStorageId`. Must be ::NcmStorageId_SdCard.
    /// - `size` (out): Pointer to output the total storage size to.
    ///
    /// **Note:** Uses `nsGetContentManagementInterface` on [3.0.0+], otherwise IApplicationManagerInterface is used.
    pub fn nsGetTotalSpaceSize(storage_id: NcmStorageId, size: *mut s64) -> Result;

    /// Returns the available storage capacity from content manager services.
    ///
    /// # Arguments
    /// - `storage_id` (in): `NcmStorageId`. Must be ::NcmStorageId_SdCard.
    /// - `size` (out): Pointer to output the free storage size to.
    ///
    /// **Note:** Uses `nsGetContentManagementInterface` on [3.0.0+], otherwise IApplicationManagerInterface is used.
    pub fn nsGetFreeSpaceSize(storage_id: NcmStorageId, size: *mut s64) -> Result;

    /// CountApplicationContentMeta
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId.
    /// - `out` (out): Output count.
    ///
    /// **Note:** Uses `nsGetContentManagementInterface` on [3.0.0+], otherwise IApplicationManagerInterface is used.
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn nsCountApplicationContentMeta(application_id: u64, out: *mut s32) -> Result;

    /// Gets an listing of `NsApplicationContentMetaStatus`.
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId.
    /// - `index` (in): Starting entry index.
    /// - `list` (out): Output array of `NsApplicationContentMetaStatus`.
    /// - `count` (in): Size of the list array in entries.
    /// - `out_entrycount` (out): Total output entries.
    ///
    /// **Note:** Uses `nsGetContentManagementInterface` on [3.0.0+], otherwise IApplicationManagerInterface is used.
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn nsListApplicationContentMetaStatus(application_id: u64, index: s32, list: *mut NsApplicationContentMetaStatus, count: s32, out_entrycount: *mut s32) -> Result;

    /// IsAnyApplicationRunning
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn nsIsAnyApplicationRunning(out: *mut bool) -> Result;

    /// Close a `NsRequestServerStopper`.
    ///
    /// # Arguments
    /// - `r`: `NsRequestServerStopper`
    pub fn nsRequestServerStopperClose(r: *mut NsRequestServerStopper);

    /// Close a `NsProgressMonitorForDeleteUserSaveDataAll`. When initialized this will use `nsProgressMonitorForDeleteUserSaveDataAllIsFinished`, throwing errors on failure / when the operation isn't finished (without closing the object).
    ///
    /// # Arguments
    /// - `p`: `NsProgressMonitorForDeleteUserSaveDataAll`
    ///
    /// **Note:** Cancelling the operation before it's finished is not supported by `NsProgressMonitorForDeleteUserSaveDataAll`.
    pub fn nsProgressMonitorForDeleteUserSaveDataAllClose(p: *mut NsProgressMonitorForDeleteUserSaveDataAll) -> Result;

    /// GetSystemEvent
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=false.
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn nsProgressMonitorForDeleteUserSaveDataAllGetSystemEvent(p: *mut NsProgressMonitorForDeleteUserSaveDataAll, out_event: *mut Event) -> Result;

    /// IsFinished
    ///
    /// # Arguments
    /// - `p`: `NsProgressMonitorForDeleteUserSaveDataAll`
    /// - `out` (out): Whether the operation finished.
    pub fn nsProgressMonitorForDeleteUserSaveDataAllIsFinished(p: *mut NsProgressMonitorForDeleteUserSaveDataAll, out: *mut bool) -> Result;

    /// GetResult
    ///
    /// # Arguments
    /// - `p`: `NsProgressMonitorForDeleteUserSaveDataAll`
    pub fn nsProgressMonitorForDeleteUserSaveDataAllGetResult(p: *mut NsProgressMonitorForDeleteUserSaveDataAll) -> Result;

    /// GetProgress
    ///
    /// # Arguments
    /// - `p`: `NsProgressMonitorForDeleteUserSaveDataAll`
    /// - `progress` (out): Output `NsProgressForDeleteUserSaveDataAll`.
    pub fn nsProgressMonitorForDeleteUserSaveDataAllGetProgress(p: *mut NsProgressMonitorForDeleteUserSaveDataAll, progress: *mut NsProgressForDeleteUserSaveDataAll) -> Result;

    /// Close a `NsProgressAsyncResult`.
    ///
    /// # Arguments
    /// - `a`: `NsProgressAsyncResult`
    ///
    /// **Note:** When the object is initialized, this uses `nsProgressAsyncResultCancel` then `nsProgressAsyncResultWait` with timeout=UINT64_MAX.
    pub fn nsProgressAsyncResultClose(a: *mut NsProgressAsyncResult);

    /// Waits for the async operation to finish using the specified timeout.
    ///
    /// # Arguments
    /// - `a`: `NsProgressAsyncResult`
    /// - `timeout` (in): Timeout in nanoseconds. UINT64_MAX for no timeout.
    pub fn nsProgressAsyncResultWait(a: *mut NsProgressAsyncResult, timeout: u64) -> Result;

    /// Gets the Result.
    ///
    /// # Arguments
    /// - `a`: `NsProgressAsyncResult`
    ///
    /// **Note:** Prior to using the cmd, this uses `nsProgressAsyncResultWait` with timeout=UINT64_MAX.
    pub fn nsProgressAsyncResultGet(a: *mut NsProgressAsyncResult) -> Result;

    /// Cancels the async operation.
    ///
    /// # Arguments
    /// - `a`: `NsProgressAsyncResult`
    ///
    /// **Note:** Used automatically by `nsProgressAsyncResultClose`.
    pub fn nsProgressAsyncResultCancel(a: *mut NsProgressAsyncResult) -> Result;

    /// Gets the progress.
    ///
    /// # Arguments
    /// - `a`: `NsProgressAsyncResult`
    /// - `buffer` (out): Output buffer.
    /// - `size` (in): Output buffer size.
    pub fn nsProgressAsyncResultGetProgress(a: *mut NsProgressAsyncResult, buffer: *mut c_void, size: usize) -> Result;

    /// GetDetailResult
    ///
    /// # Arguments
    /// - `a`: `NsProgressAsyncResult`
    pub fn nsProgressAsyncResultGetDetailResult(a: *mut NsProgressAsyncResult) -> Result;

    /// Gets the `ErrorContext`.
    ///
    /// # Arguments
    /// - `a`: `NsProgressAsyncResult`
    /// - `context` (out): `ErrorContext`
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nsProgressAsyncResultGetErrorContext(a: *mut NsProgressAsyncResult, context: *mut ErrorContext) -> Result;

    /// Initialize ns:vm. On pre-3.0.0 this must be used with `nsInitialize`.
    pub fn nsvmInitialize() -> Result;

    /// Exit ns:vm.
    pub fn nsvmExit();

    /// Gets the Service object for ns:vm. This is only initialized on [3.0.0+].
    pub fn nsvmGetServiceSession() -> *mut Service;

    pub fn nsvmNeedsUpdateVulnerability(out: *mut bool) -> Result;

    /// [4.0.0+]
    pub fn nsvmGetSafeSystemVersion(out: *mut NcmContentMetaKey) -> Result;

    /// Initialize ns:dev.
    pub fn nsdevInitialize() -> Result;

    /// Initialize ns:dev.
    pub fn nsdevExit();

    /// Gets the Service object for ns:dev.
    pub fn nsdevGetServiceSession() -> *mut Service;

    /// [1.0.0-9.2.0]
    pub fn nsdevLaunchProgram(out_pid: *mut u64, properties: *const NsLaunchProperties, flags: u32) -> Result;

    pub fn nsdevTerminateProcess(pid: u64) -> Result;

    /// [1.0.0-9.2.0]
    pub fn nsdevTerminateProgram(tid: u64) -> Result;

    /// Autoclear for nsdevShellEvent is always true. [1.0.0-9.2.0]
    pub fn nsdevGetShellEvent(out_event: *mut Event) -> Result;

    /// [1.0.0-9.2.0]
    pub fn nsdevGetShellEventInfo(out: *mut NsShellEventInfo) -> Result;

    pub fn nsdevTerminateApplication() -> Result;

    /// [1.0.0-9.2.0]
    pub fn nsdevPrepareLaunchProgramFromHost(out: *mut NsLaunchProperties, path: *const c_char, path_len: usize) -> Result;

    /// [1.0.0-9.2.0]
    pub fn nsdevLaunchApplicationForDevelop(out_pid: *mut u64, application_id: u64, flags: u32) -> Result;

    /// [10.0.0-17.0.1]
    pub fn nsdevLaunchApplicationFromHost(out_pid: *mut u64, path: *const c_char, path_len: usize, flags: u32) -> Result;

    /// [1.0.0-17.0.1]
    pub fn nsdevLaunchApplicationWithStorageIdForDevelop(out_pid: *mut u64, application_id: u64, flags: u32, app_storage_id: u8, patch_storage_id: u8) -> Result;

    /// [6.0.0-8.1.0]
    pub fn nsdevIsSystemMemoryResourceLimitBoosted(out: *mut bool) -> Result;

    /// [6.0.0+]
    pub fn nsdevGetRunningApplicationProcessIdForDevelop(out_pid: *mut u64) -> Result;

    /// [6.0.0+]
    pub fn nsdevSetCurrentApplicationRightsEnvironmentCanBeActiveForDevelop(can_be_active: bool) -> Result;

    /// Initialize ns:su.
    pub fn nssuInitialize() -> Result;

    /// Exit ns:su.
    pub fn nssuExit();

    /// Gets the Service object for ns:su.
    pub fn nssuGetServiceSession() -> *mut Service;

    /// Gets the `NsBackgroundNetworkUpdateState`.
    ///
    /// # Arguments
    /// - `out` (out): `NsBackgroundNetworkUpdateState`
    ///
    /// **Note:** Internally this uses nim commands ListSystemUpdateTask and GetSystemUpdateTaskInfo to determine the output state.
    pub fn nssuGetBackgroundNetworkUpdateState(out: *mut NsBackgroundNetworkUpdateState) -> Result;

    /// Opens a `NsSystemUpdateControl`.
    ///
    /// # Arguments
    /// - `c` (out): `NsSystemUpdateControl`
    ///
    /// **Note:** Only 1 `NsSystemUpdateControl` can be open at a time.
    pub fn nssuOpenSystemUpdateControl(c: *mut NsSystemUpdateControl) -> Result;

    /// Uses nim ListSystemUpdateTask, then uses the task with DestroySystemUpdateTask if it exists. Then this runs ExFat handling, updates state, and sets the same state flag as `nssuRequestBackgroundNetworkUpdate`.
    ///
    /// **Note:** Only usable when a `NsSystemUpdateControl` isn't open.
    pub fn nssuNotifyExFatDriverRequired() -> Result;

    /// ClearExFatDriverStatusForDebug
    pub fn nssuClearExFatDriverStatusForDebug() -> Result;

    /// RequestBackgroundNetworkUpdate
    ///
    /// **Note:** Only usable when a `NsSystemUpdateControl` isn't open.
    pub fn nssuRequestBackgroundNetworkUpdate() -> Result;

    /// This checks whether a sysupdate is needed with the input `NcmContentMetaKey` using NCM commands, if not this will just return 0. Otherwise, this will then run code which is identical to `nssuRequestBackgroundNetworkUpdate`.
    ///
    /// # Arguments
    /// - `key` (in): `NcmContentMetaKey`
    ///
    /// **Note:** Only usable when a `NsSystemUpdateControl` isn't open.
    pub fn nssuNotifyBackgroundNetworkUpdate(key: *const NcmContentMetaKey) -> Result;

    /// NotifyExFatDriverDownloadedForDebug
    pub fn nssuNotifyExFatDriverDownloadedForDebug() -> Result;

    /// Gets an Event which can be signaled by `nssuNotifySystemUpdateForContentDelivery`.
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=false.
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn nssuGetSystemUpdateNotificationEventForContentDelivery(out_event: *mut Event) -> Result;

    /// Signals the event returned by `nssuGetSystemUpdateNotificationEventForContentDelivery`.
    pub fn nssuNotifySystemUpdateForContentDelivery() -> Result;

    /// This does shutdown preparation.
    ///
    /// **Note:** This is used by am-sysmodule, so generally there's no need to use this.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn nssuPrepareShutdown() -> Result;

    /// This uses nim ListSystemUpdateTask, then when a task is returned uses it with DestroySystemUpdateTask.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nssuDestroySystemUpdateTask() -> Result;

    /// RequestSendSystemUpdate
    ///
    /// # Arguments
    /// - `a` (out): `AsyncResult`
    /// - `addr` (in): Client IPv4 address.
    /// - `port` (in): Socket port. qlaunch uses value 55556.
    /// - `info` (in): `NsSystemDeliveryInfo`
    ///
    /// **Note:** addr/port are little-endian.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nssuRequestSendSystemUpdate(a: *mut AsyncResult, addr: u32, port: u16, info: *mut NsSystemDeliveryInfo) -> Result;

    /// GetSendSystemUpdateProgress
    ///
    /// # Arguments
    /// - `out` (out): `NsSystemUpdateProgress`
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nssuGetSendSystemUpdateProgress(out: *mut NsSystemUpdateProgress) -> Result;

    /// Close a `NsSystemUpdateControl`.
    ///
    /// # Arguments
    /// - `c`: `NsSystemUpdateControl`
    pub fn nssuControlClose(c: *mut NsSystemUpdateControl);

    /// Gets whether a network sysupdate was downloaded, with install pending.
    ///
    /// # Arguments
    /// - `c`: `NsSystemUpdateControl`
    /// - `out` (out): Output flag.
    pub fn nssuControlHasDownloaded(c: *mut NsSystemUpdateControl, out: *mut bool) -> Result;

    /// RequestCheckLatestUpdate
    ///
    /// # Arguments
    /// - `c`: `NsSystemUpdateControl`
    /// - `a` (out): `AsyncValue`. The data that can be read from this is u8 `NsLatestSystemUpdate`.
    pub fn nssuControlRequestCheckLatestUpdate(c: *mut NsSystemUpdateControl, a: *mut AsyncValue) -> Result;

    /// RequestDownloadLatestUpdate
    ///
    /// # Arguments
    /// - `c`: `NsSystemUpdateControl`
    /// - `a` (out): `AsyncResult`
    pub fn nssuControlRequestDownloadLatestUpdate(c: *mut NsSystemUpdateControl, a: *mut AsyncResult) -> Result;

    /// GetDownloadProgress
    ///
    /// # Arguments
    /// - `c`: `NsSystemUpdateControl`
    /// - `out` (out): `NsSystemUpdateProgress`
    pub fn nssuControlGetDownloadProgress(c: *mut NsSystemUpdateControl, out: *mut NsSystemUpdateProgress) -> Result;

    /// ApplyDownloadedUpdate
    ///
    /// # Arguments
    /// - `c`: `NsSystemUpdateControl`
    pub fn nssuControlApplyDownloadedUpdate(c: *mut NsSystemUpdateControl) -> Result;

    /// RequestPrepareCardUpdate
    ///
    /// # Arguments
    /// - `c`: `NsSystemUpdateControl`
    /// - `a` (out): `AsyncResult`
    pub fn nssuControlRequestPrepareCardUpdate(c: *mut NsSystemUpdateControl, a: *mut AsyncResult) -> Result;

    /// GetPrepareCardUpdateProgress
    ///
    /// # Arguments
    /// - `c`: `NsSystemUpdateControl`
    /// - `out` (out): `NsSystemUpdateProgress`
    ///
    /// **Note:** `nssuControlSetupCardUpdate` / `nssuControlSetupCardUpdateViaSystemUpdater` must have been used at some point prior to using this.
    pub fn nssuControlGetPrepareCardUpdateProgress(c: *mut NsSystemUpdateControl, out: *mut NsSystemUpdateProgress) -> Result;

    /// HasPreparedCardUpdate
    ///
    /// # Arguments
    /// - `c`: `NsSystemUpdateControl`
    /// - `out` (out): Output flag.
    ///
    /// **Note:** `nssuControlSetupCardUpdate` / `nssuControlSetupCardUpdateViaSystemUpdater` must have been used at some point prior to using this.
    pub fn nssuControlHasPreparedCardUpdate(c: *mut NsSystemUpdateControl, out: *mut bool) -> Result;

    /// ApplyCardUpdate
    ///
    /// # Arguments
    /// - `c`: `NsSystemUpdateControl`
    ///
    /// **Note:** `nssuControlSetupCardUpdate` / `nssuControlSetupCardUpdateViaSystemUpdater` must have been used at some point prior to using this.
    pub fn nssuControlApplyCardUpdate(c: *mut NsSystemUpdateControl) -> Result;

    /// Gets the filesize for the specified DownloadedEulaData.
    ///
    /// # Arguments
    /// - `c`: `NsSystemUpdateControl`
    /// - `path` (in): EulaData path.
    /// - `filesize` (out): Output filesize.
    ///
    /// **Note:** This mounts the Eula SystemData, then uses the file "&lt;mountname>:/&lt;input path>".
    pub fn nssuControlGetDownloadedEulaDataSize(c: *mut NsSystemUpdateControl, path: *const c_char, filesize: *mut u64) -> Result;

    /// Gets the specified DownloadedEulaData.
    ///
    /// # Arguments
    /// - `c`: `NsSystemUpdateControl`
    /// - `path` (in): EulaData path.
    /// - `buffer` (out): Output buffer.
    /// - `size` (in): Size of the output buffer, must be at least the output size from `nssuControlGetDownloadedEulaDataSize`.
    /// - `filesize` (out): Output filesize.
    ///
    /// **Note:** See the note for `nssuControlGetDownloadedEulaDataSize`.
    pub fn nssuControlGetDownloadedEulaData(c: *mut NsSystemUpdateControl, path: *const c_char, buffer: *mut c_void, size: usize, filesize: *mut u64) -> Result;

    /// SetupCardUpdate
    ///
    /// # Arguments
    /// - `c`: `NsSystemUpdateControl`
    /// - `buffer` (in): TransferMemory buffer, when NULL this is automatically allocated.
    /// - `size` (in): TransferMemory buffer size, see `NSSU_CARDUPDATE_TMEM_SIZE_DEFAULT`.
    pub fn nssuControlSetupCardUpdate(c: *mut NsSystemUpdateControl, buffer: *mut c_void, size: usize) -> Result;

    /// Gets the filesize for the specified PreparedCardUpdateEulaData.
    ///
    /// # Arguments
    /// - `c`: `NsSystemUpdateControl`
    /// - `path` (in): EulaData path.
    /// - `filesize` (out): Output filesize.
    ///
    /// **Note:** See the note for `nssuControlGetDownloadedEulaDataSize`.
    pub fn nssuControlGetPreparedCardUpdateEulaDataSize(c: *mut NsSystemUpdateControl, path: *const c_char, filesize: *mut u64) -> Result;

    /// Gets the specified PreparedCardUpdateEulaData.
    ///
    /// # Arguments
    /// - `c`: `NsSystemUpdateControl`
    /// - `path` (in): EulaData path.
    /// - `buffer` (out): Output buffer.
    /// - `size` (in): Size of the output buffer, must be at least the output size from `nssuControlGetPreparedCardUpdateEulaDataSize`.
    /// - `filesize` (out): Output filesize.
    ///
    /// **Note:** See the note for `nssuControlGetDownloadedEulaDataSize`.
    pub fn nssuControlGetPreparedCardUpdateEulaData(c: *mut NsSystemUpdateControl, path: *const c_char, buffer: *mut c_void, size: usize, filesize: *mut u64) -> Result;

    /// SetupCardUpdateViaSystemUpdater
    ///
    /// # Arguments
    /// - `c`: `NsSystemUpdateControl`
    /// - `buffer` (in): TransferMemory buffer, when NULL this is automatically allocated.
    /// - `size` (in): TransferMemory buffer size, see `NSSU_CARDUPDATE_TMEM_SIZE_DEFAULT`.
    ///
    /// **Note:** Same as `nssuControlSetupCardUpdate`, except this doesn't run the code for fs cmds GetGameCardHandle/GetGameCardUpdatePartitionInfo, and uses fs OpenRegisteredUpdatePartition instead of OpenGameCardFileSystem.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nssuControlSetupCardUpdateViaSystemUpdater(c: *mut NsSystemUpdateControl, buffer: *mut c_void, size: usize) -> Result;

    /// HasReceived
    ///
    /// # Arguments
    /// - `c`: `NsSystemUpdateControl`
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nssuControlHasReceived(c: *mut NsSystemUpdateControl, out: *mut bool) -> Result;

    /// RequestReceiveSystemUpdate
    ///
    /// # Arguments
    /// - `c`: `NsSystemUpdateControl`
    /// - `a` (out): `AsyncResult`
    /// - `addr` (in): Server IPv4 address.
    /// - `port` (in): Socket port. qlaunch uses value 55556.
    /// - `info` (in): `NsSystemDeliveryInfo`
    ///
    /// **Note:** addr/port are little-endian.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nssuControlRequestReceiveSystemUpdate(c: *mut NsSystemUpdateControl, a: *mut AsyncResult, addr: u32, port: u16, info: *mut NsSystemDeliveryInfo) -> Result;

    /// GetReceiveProgress
    ///
    /// # Arguments
    /// - `c`: `NsSystemUpdateControl`
    /// - `out` (out): `NsSystemUpdateProgress`
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nssuControlGetReceiveProgress(c: *mut NsSystemUpdateControl, out: *mut NsSystemUpdateProgress) -> Result;

    /// ApplyReceivedUpdate
    ///
    /// # Arguments
    /// - `c`: `NsSystemUpdateControl`
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nssuControlApplyReceivedUpdate(c: *mut NsSystemUpdateControl) -> Result;

    /// Gets the filesize for the specified ReceivedEulaData.
    ///
    /// # Arguments
    /// - `c`: `NsSystemUpdateControl`
    /// - `path` (in): EulaData path.
    /// - `filesize` (out): Output filesize.
    ///
    /// **Note:** See the note for `nssuControlGetDownloadedEulaDataSize`.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nssuControlGetReceivedEulaDataSize(c: *mut NsSystemUpdateControl, path: *const c_char, filesize: *mut u64) -> Result;

    /// Gets the specified ReceivedEulaData.
    ///
    /// # Arguments
    /// - `c`: `NsSystemUpdateControl`
    /// - `path` (in): EulaData path.
    /// - `buffer` (out): Output buffer.
    /// - `size` (in): Size of the output buffer, must be at least the output size from `nssuControlGetReceivedEulaDataSize`.
    /// - `filesize` (out): Output filesize.
    ///
    /// **Note:** See the note for `nssuControlGetDownloadedEulaDataSize`.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nssuControlGetReceivedEulaData(c: *mut NsSystemUpdateControl, path: *const c_char, buffer: *mut c_void, size: usize, filesize: *mut u64) -> Result;

    /// Does setup for ReceiveSystemUpdate by using the same nim cmds as `nssuDestroySystemUpdateTask`.
    ///
    /// # Arguments
    /// - `c`: `NsSystemUpdateControl`
    ///
    /// **Note:** qlaunch uses this before `nssuControlRequestReceiveSystemUpdate`.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn nssuControlSetupToReceiveSystemUpdate(c: *mut NsSystemUpdateControl) -> Result;

    /// RequestCheckLatestUpdateIncludesRebootlessUpdate
    ///
    /// # Arguments
    /// - `c`: `NsSystemUpdateControl`
    /// - `a` (out): `AsyncValue`
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn nssuControlRequestCheckLatestUpdateIncludesRebootlessUpdate(c: *mut NsSystemUpdateControl, a: *mut AsyncValue) -> Result;
}
