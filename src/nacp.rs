//! `switch/nacp.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Language entry. These strings are UTF-8.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NacpLanguageEntry {
    pub name: [c_char; 512],
    pub author: [c_char; 256],
}

/// ApplicationNeighborDetectionGroupConfiguration
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NacpApplicationNeighborDetectionGroupConfiguration {
    /// GroupId
    pub group_id: u64,
    pub key: [u8; 16],
}

/// NeighborDetectionClientConfiguration
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NacpNeighborDetectionClientConfiguration {
    /// SendGroupConfiguration
    pub send_group_configuration: NacpApplicationNeighborDetectionGroupConfiguration,
    /// ReceivableGroupConfigurations
    pub receivable_group_configurations: [NacpApplicationNeighborDetectionGroupConfiguration; 16],
}

/// ApplicationJitConfiguration
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NacpApplicationJitConfiguration {
    /// Flags
    pub flags: u64,
    /// MemorySize
    pub memory_size: u64,
}

/// ns ApplicationControlProperty
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NacpStruct {
    /// **Ref:** NacpLanguageEntry
    pub lang: [NacpLanguageEntry; 16],
    /// Isbn
    pub isbn: [u8; 37],
    /// StartupUserAccount
    pub startup_user_account: u8,
    /// UserAccountSwitchLock
    pub user_account_switch_lock: u8,
    /// AddOnContentRegistrationType
    pub add_on_content_registration_type: u8,
    /// AttributeFlag
    pub attribute_flag: u32,
    /// SupportedLanguageFlag
    pub supported_language_flag: u32,
    /// ParentalControlFlag
    pub parental_control_flag: u32,
    /// Screenshot
    pub screenshot: u8,
    /// VideoCapture
    pub video_capture: u8,
    /// DataLossConfirmation
    pub data_loss_confirmation: u8,
    /// PlayLogPolicy
    pub play_log_policy: u8,
    /// PresenceGroupId
    pub presence_group_id: u64,
    /// RatingAge
    pub rating_age: [s8; 32],
    /// DisplayVersion
    pub display_version: [c_char; 16],
    /// AddOnContentBaseId
    pub add_on_content_base_id: u64,
    /// SaveDataOwnerId
    pub save_data_owner_id: u64,
    /// UserAccountSaveDataSize
    pub user_account_save_data_size: u64,
    /// UserAccountSaveDataJournalSize
    pub user_account_save_data_journal_size: u64,
    /// DeviceSaveDataSize
    pub device_save_data_size: u64,
    /// DeviceSaveDataJournalSize
    pub device_save_data_journal_size: u64,
    /// BcatDeliveryCacheStorageSize
    pub bcat_delivery_cache_storage_size: u64,
    /// ApplicationErrorCodeCategory
    pub application_error_code_category: u64,
    /// LocalCommunicationId
    pub local_communication_id: [u64; 8],
    /// LogoType
    pub logo_type: u8,
    /// LogoHandling
    pub logo_handling: u8,
    /// RuntimeAddOnContentInstall
    pub runtime_add_on_content_install: u8,
    /// RuntimeParameterDelivery
    pub runtime_parameter_delivery: u8,
    /// Reserved
    pub reserved_x30f4: [u8; 2],
    /// CrashReport
    pub crash_report: u8,
    /// Hdcp
    pub hdcp: u8,
    /// SeedForPseudoDeviceId
    pub pseudo_device_id_seed: u64,
    /// BcatPassphrase
    pub bcat_passphrase: [c_char; 65],
    /// StartupUserAccountOption
    pub startup_user_account_option: u8,
    /// ReservedForUserAccountSaveDataOperation
    pub reserved_for_user_account_save_data_operation: [u8; 6],
    /// UserAccountSaveDataSizeMax
    pub user_account_save_data_size_max: u64,
    /// UserAccountSaveDataJournalSizeMax
    pub user_account_save_data_journal_size_max: u64,
    /// DeviceSaveDataSizeMax
    pub device_save_data_size_max: u64,
    /// DeviceSaveDataJournalSizeMax
    pub device_save_data_journal_size_max: u64,
    /// TemporaryStorageSize
    pub temporary_storage_size: u64,
    /// CacheStorageSize
    pub cache_storage_size: u64,
    /// CacheStorageJournalSize
    pub cache_storage_journal_size: u64,
    /// CacheStorageDataAndJournalSizeMax
    pub cache_storage_data_and_journal_size_max: u64,
    /// CacheStorageIndexMax
    pub cache_storage_index_max: u16,
    /// Reserved
    pub reserved_x318a: [u8; 6],
    /// PlayLogQueryableApplicationId
    pub play_log_queryable_application_id: [u64; 16],
    /// PlayLogQueryCapability
    pub play_log_query_capability: u8,
    /// RepairFlag
    pub repair_flag: u8,
    /// ProgramIndex
    pub program_index: u8,
    /// RequiredNetworkServiceLicenseOnLaunchFlag
    pub required_network_service_license_on_launch: u8,
    /// Reserved
    pub reserved_x3214: u32,
    /// NeighborDetectionClientConfiguration
    pub neighbor_detection_client_configuration: NacpNeighborDetectionClientConfiguration,
    /// JitConfiguration
    pub jit_configuration: NacpApplicationJitConfiguration,
    /// Reserved
    pub reserved_x33c0: [u8; 3136],
}

impl_zeroed_default!(NacpLanguageEntry, NacpApplicationNeighborDetectionGroupConfiguration, NacpNeighborDetectionClientConfiguration, NacpApplicationJitConfiguration, NacpStruct);

unsafe extern "C" {
    /// Get the NacpLanguageEntry from the input nacp corresponding to the current system language (this may fallback to other languages when needed). Output langentry is NULL if none found / content of entry is empty.
    /// If you're using ns you may want to use `nsGetApplicationDesiredLanguage` instead.
    pub fn nacpGetLanguageEntry(nacp: *mut NacpStruct, langentry: *mut *mut NacpLanguageEntry) -> Result;
}
