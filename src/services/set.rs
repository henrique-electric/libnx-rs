//! `switch/services/set.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const SET_MAX_NAME_SIZE: u32 = 0x48;

pub type ColorSetId = u32;
pub const ColorSetId_Light: ColorSetId = 0;
pub const ColorSetId_Dark: ColorSetId = 1;

/// Console Product Models
pub type SetSysProductModel = u32;
/// Invalid Model
pub const SetSysProductModel_Invalid: SetSysProductModel = 0;
/// Erista Model
pub const SetSysProductModel_Nx: SetSysProductModel = 1;
/// Erista "Simulation" Model
pub const SetSysProductModel_Copper: SetSysProductModel = 2;
/// Mariko Model
pub const SetSysProductModel_Iowa: SetSysProductModel = 3;
/// Mariko Lite Model
pub const SetSysProductModel_Hoag: SetSysProductModel = 4;
/// Mariko "Simulation" Model
pub const SetSysProductModel_Calcio: SetSysProductModel = 5;
/// Mariko OLED Model
pub const SetSysProductModel_Aula: SetSysProductModel = 6;

/// IDs for Language.
pub type SetLanguage = u32;
/// Japanese
pub const SetLanguage_JA: SetLanguage = 0;
/// US English ("AmericanEnglish")
pub const SetLanguage_ENUS: SetLanguage = 1;
/// French
pub const SetLanguage_FR: SetLanguage = 2;
/// German
pub const SetLanguage_DE: SetLanguage = 3;
/// Italian
pub const SetLanguage_IT: SetLanguage = 4;
/// Spanish
pub const SetLanguage_ES: SetLanguage = 5;
/// Simplified Chinese ("Chinese")
pub const SetLanguage_ZHCN: SetLanguage = 6;
/// Korean
pub const SetLanguage_KO: SetLanguage = 7;
/// Dutch
pub const SetLanguage_NL: SetLanguage = 8;
/// Portuguese
pub const SetLanguage_PT: SetLanguage = 9;
/// Russian
pub const SetLanguage_RU: SetLanguage = 10;
/// Traditional Chinese ("Taiwanese")
pub const SetLanguage_ZHTW: SetLanguage = 11;
/// GB English ("BritishEnglish")
pub const SetLanguage_ENGB: SetLanguage = 12;
/// CA French ("CanadianFrench")
pub const SetLanguage_FRCA: SetLanguage = 13;
/// "LatinAmericanSpanish"
pub const SetLanguage_ES419: SetLanguage = 14;
/// [4.0.0+] ChineseSimplified
pub const SetLanguage_ZHHANS: SetLanguage = 15;
/// [4.0.0+] ChineseTraditional
pub const SetLanguage_ZHHANT: SetLanguage = 16;
/// [10.1.0+] "BrazilianPortuguese"
pub const SetLanguage_PTBR: SetLanguage = 17;
/// Total languages supported by this enum.
pub const SetLanguage_Total: SetLanguage = 18;

/// Region codes.
pub type SetRegion = u32;
/// Japan
pub const SetRegion_JPN: SetRegion = 0;
/// The Americas
pub const SetRegion_USA: SetRegion = 1;
/// Europe
pub const SetRegion_EUR: SetRegion = 2;
/// Australia/New Zealand
pub const SetRegion_AUS: SetRegion = 3;
/// Hong Kong/Taiwan/Korea
pub const SetRegion_HTK: SetRegion = 4;
/// China
pub const SetRegion_CHN: SetRegion = 5;

/// ConnectionFlag
pub type SetSysConnectionFlag = u32;
pub const SetSysConnectionFlag_ConnectAutomaticallyFlag: SetSysConnectionFlag = BIT(0);
pub const SetSysConnectionFlag_Unknown: SetSysConnectionFlag = BIT(1);

/// AccessPointSecurityType
pub type SetSysAccessPointSecurityType = u32;
pub const SetSysAccessPointSecurityType_None: SetSysAccessPointSecurityType = 0;
pub const SetSysAccessPointSecurityType_Shared: SetSysAccessPointSecurityType = 1;
pub const SetSysAccessPointSecurityType_Wpa: SetSysAccessPointSecurityType = 2;
pub const SetSysAccessPointSecurityType_Wpa2: SetSysAccessPointSecurityType = 3;

/// AccessPointSecurityStandard
pub type SetSysAccessPointSecurityStandard = u32;
pub const SetSysAccessPointSecurityStandard_None: SetSysAccessPointSecurityStandard = 0;
pub const SetSysAccessPointSecurityStandard_Wep: SetSysAccessPointSecurityStandard = 1;
pub const SetSysAccessPointSecurityStandard_Wpa: SetSysAccessPointSecurityStandard = 2;

/// AutoSettings
pub type SetSysAutoSettings = u32;
pub const SetSysAutoSettings_AutoIp: SetSysAutoSettings = BIT(0);
pub const SetSysAutoSettings_AutoDns: SetSysAutoSettings = BIT(1);

/// ProxyFlags
pub type SetSysProxyFlags = u32;
pub const SetSysProxyFlags_UseProxyFlag: SetSysProxyFlags = BIT(0);
pub const SetSysProxyFlags_ProxyAutoAuthenticateFlag: SetSysProxyFlags = BIT(1);

/// UserSelectorFlag
pub type SetSysUserSelectorFlag = u32;
pub const SetSysUserSelectorFlag_SkipsIfSingleUser: SetSysUserSelectorFlag = BIT(0);

/// EulaVersionClockType
pub type SetSysEulaVersionClockType = u32;
pub const SetSysEulaVersionClockType_NetworkSystemClock: SetSysEulaVersionClockType = 0;
pub const SetSysEulaVersionClockType_SteadyClock: SetSysEulaVersionClockType = 1;

/// NotificationVolume
pub type SetSysNotificationVolume = u32;
pub const SetSysNotificationVolume_Mute: SetSysNotificationVolume = 0;
pub const SetSysNotificationVolume_Low: SetSysNotificationVolume = 1;
pub const SetSysNotificationVolume_High: SetSysNotificationVolume = 2;

/// FriendPresenceOverlayPermission
pub type SetSysFriendPresenceOverlayPermission = u32;
pub const SetSysFriendPresenceOverlayPermission_NotConfirmed: SetSysFriendPresenceOverlayPermission = 0;
pub const SetSysFriendPresenceOverlayPermission_NoDisplay: SetSysFriendPresenceOverlayPermission = 1;
pub const SetSysFriendPresenceOverlayPermission_FavoriteFriends: SetSysFriendPresenceOverlayPermission = 2;
pub const SetSysFriendPresenceOverlayPermission_Friends: SetSysFriendPresenceOverlayPermission = 3;

/// AudioDevice
pub type SetSysAudioDevice = u32;
pub const SetSysAudioDevice_Console: SetSysAudioDevice = 0;
pub const SetSysAudioDevice_Headphone: SetSysAudioDevice = 1;
pub const SetSysAudioDevice_Tv: SetSysAudioDevice = 2;

/// PrimaryAlbumStorage
pub type SetSysPrimaryAlbumStorage = u32;
pub const SetSysPrimaryAlbumStorage_Nand: SetSysPrimaryAlbumStorage = 0;
pub const SetSysPrimaryAlbumStorage_SdCard: SetSysPrimaryAlbumStorage = 1;

/// HandheldSleepPlan
pub type SetSysHandheldSleepPlan = u32;
pub const SetSysHandheldSleepPlan_1Min: SetSysHandheldSleepPlan = 0;
pub const SetSysHandheldSleepPlan_3Min: SetSysHandheldSleepPlan = 1;
pub const SetSysHandheldSleepPlan_5Min: SetSysHandheldSleepPlan = 2;
pub const SetSysHandheldSleepPlan_10Min: SetSysHandheldSleepPlan = 3;
pub const SetSysHandheldSleepPlan_30Min: SetSysHandheldSleepPlan = 4;
pub const SetSysHandheldSleepPlan_Never: SetSysHandheldSleepPlan = 5;

/// ConsoleSleepPlan
pub type SetSysConsoleSleepPlan = u32;
pub const SetSysConsoleSleepPlan_1Hour: SetSysConsoleSleepPlan = 0;
pub const SetSysConsoleSleepPlan_2Hour: SetSysConsoleSleepPlan = 1;
pub const SetSysConsoleSleepPlan_3Hour: SetSysConsoleSleepPlan = 2;
pub const SetSysConsoleSleepPlan_6Hour: SetSysConsoleSleepPlan = 3;
pub const SetSysConsoleSleepPlan_12Hour: SetSysConsoleSleepPlan = 4;
pub const SetSysConsoleSleepPlan_Never: SetSysConsoleSleepPlan = 5;

/// AudioOutputModeTarget
pub type SetSysAudioOutputModeTarget = u32;
pub const SetSysAudioOutputModeTarget_Unknown0: SetSysAudioOutputModeTarget = 0;
pub const SetSysAudioOutputModeTarget_Unknown1: SetSysAudioOutputModeTarget = 1;
pub const SetSysAudioOutputModeTarget_Unknown2: SetSysAudioOutputModeTarget = 2;
pub const SetSysAudioOutputModeTarget_Unknown3: SetSysAudioOutputModeTarget = 3;

/// AudioOutputMode
pub type SetSysAudioOutputMode = u32;
/// Default value.
pub const SetSysAudioOutputMode_Unknown1: SetSysAudioOutputMode = 1;

/// ServiceDiscoveryControlSettings
pub type SetSysServiceDiscoveryControlSettings = u32;
pub const SetSysServiceDiscoveryControlSettings_IsChangeEnvironmentIdentifierDisabled: SetSysServiceDiscoveryControlSettings = BIT(0);

/// ErrorReportSharePermission
pub type SetSysErrorReportSharePermission = u32;
pub const SetSysErrorReportSharePermission_NotConfirmed: SetSysErrorReportSharePermission = 0;
pub const SetSysErrorReportSharePermission_Granted: SetSysErrorReportSharePermission = 1;
pub const SetSysErrorReportSharePermission_Denied: SetSysErrorReportSharePermission = 2;

/// KeyboardLayout
pub type SetKeyboardLayout = u32;
pub const SetKeyboardLayout_Japanese: SetKeyboardLayout = 0;
pub const SetKeyboardLayout_EnglishUs: SetKeyboardLayout = 1;
pub const SetKeyboardLayout_EnglishUsInternational: SetKeyboardLayout = 2;
pub const SetKeyboardLayout_EnglishUk: SetKeyboardLayout = 3;
pub const SetKeyboardLayout_French: SetKeyboardLayout = 4;
pub const SetKeyboardLayout_FrenchCa: SetKeyboardLayout = 5;
pub const SetKeyboardLayout_Spanish: SetKeyboardLayout = 6;
pub const SetKeyboardLayout_SpanishLatin: SetKeyboardLayout = 7;
pub const SetKeyboardLayout_German: SetKeyboardLayout = 8;
pub const SetKeyboardLayout_Italian: SetKeyboardLayout = 9;
pub const SetKeyboardLayout_Portuguese: SetKeyboardLayout = 10;
pub const SetKeyboardLayout_Russian: SetKeyboardLayout = 11;
pub const SetKeyboardLayout_Korean: SetKeyboardLayout = 12;
pub const SetKeyboardLayout_ChineseSimplified: SetKeyboardLayout = 13;
pub const SetKeyboardLayout_ChineseTraditional: SetKeyboardLayout = 14;

/// ChineseTraditionalInputMethod
pub type SetChineseTraditionalInputMethod = u32;
pub const SetChineseTraditionalInputMethod_Unknown1: SetChineseTraditionalInputMethod = 1;
pub const SetChineseTraditionalInputMethod_Unknown2: SetChineseTraditionalInputMethod = 2;

/// PtmCycleCountReliability
pub type SetSysPtmCycleCountReliability = u32;
pub const PtmCycleCountReliability_Default: SetSysPtmCycleCountReliability = 0;
pub const PtmCycleCountReliability_Unk: SetSysPtmCycleCountReliability = 1;

/// PlatformRegion. Other values not listed here should be handled as "Unknown".
pub type SetSysPlatformRegion = u32;
pub const SetSysPlatformRegion_Global: SetSysPlatformRegion = 1;
pub const SetSysPlatformRegion_China: SetSysPlatformRegion = 2;

/// TouchScreenMode, for "Touch-Screen Sensitivity".
pub type SetSysTouchScreenMode = u32;
/// Stylus.
pub const SetSysTouchScreenMode_Stylus: SetSysTouchScreenMode = 0;
/// Standard, the default.
pub const SetSysTouchScreenMode_Standard: SetSysTouchScreenMode = 1;

/// BlockType
pub type SetSysBlockType = u32;
pub const SetSysBlockType_Audio: SetSysBlockType = 1;
pub const SetSysBlockType_Video: SetSysBlockType = 2;
pub const SetSysBlockType_VendorSpecific: SetSysBlockType = 3;
pub const SetSysBlockType_Speaker: SetSysBlockType = 4;

/// ControllerType
pub type SetSysControllerType = u32;
pub const SetSysControllerType_JoyConR: SetSysControllerType = 1;
pub const SetSysControllerType_JoyConL: SetSysControllerType = 2;
pub const SetSysControllerType_ProCon: SetSysControllerType = 3;

/// BatteryLot
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetBatteryLot {
    /// BatteryLot string.
    pub lot: [c_char; 24],
}

/// NetworkSettings
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysNetworkSettings {
    pub name: [c_char; 64],
    pub uuid: Uuid,
    /// Bitmask with `SetSysConnectionFlag`.
    pub connection_flags: u32,
    pub wired_flag: u32,
    /// Bitmask with UseStealthNetworkFlag.
    pub connect_to_hidden_network: u32,
    pub access_point_ssid: [c_char; 32],
    pub access_point_ssid_len: u32,
    /// Bitmask with `SetSysAccessPointSecurityType`.
    pub access_point_security_type: u32,
    /// Bitmask with `SetSysAccessPointSecurityStandard`.
    pub access_point_security_standard: u32,
    pub access_point_passphrase: [c_char; 64],
    pub access_point_passphrase_len: u32,
    /// Bitmask with `SetSysAutoSettings`.
    pub auto_settings: u32,
    pub manual_ip_address: u32,
    pub manual_subnet_mask: u32,
    pub manual_gateway: u32,
    pub primary_dns: u32,
    pub secondary_dns: u32,
    /// Bitmask with `SetSysProxyFlags`.
    pub proxy_flags: u32,
    pub proxy_server: [c_char; 128],
    pub proxy_port: u16,
    pub padding1: u16,
    pub proxy_autoauth_user: [c_char; 32],
    pub proxy_autoauth_pass: [c_char; 32],
    pub mtu: u16,
    pub padding2: u16,
}

/// LcdBacklightBrightnessMapping
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysLcdBacklightBrightnessMapping {
    pub brightness_applied_to_backlight: f32,
    pub ambient_light_sensor_value: f32,
    pub unk_x8: f32,
}

/// BacklightSettings
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysBacklightSettings {
    pub auto_brightness_flags: u32,
    pub screen_brightness: f32,
    pub brightness_mapping: SetSysLcdBacklightBrightnessMapping,
    pub unk_x14: f32,
    pub unk_x18: f32,
    pub unk_x1C: f32,
    pub unk_x20: f32,
    pub unk_x24: f32,
}

/// BacklightSettingsEx
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysBacklightSettingsEx {
    pub auto_brightness_flags: u32,
    pub screen_brightness: f32,
    pub current_brightness_for_vr_mode: f32,
    pub brightness_mapping: SetSysLcdBacklightBrightnessMapping,
    pub unk_x18: f32,
    pub unk_x1C: f32,
    pub unk_x20: f32,
    pub unk_x24: f32,
    pub unk_x28: f32,
}

/// BluetoothDevicesSettings
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysBluetoothDevicesSettings {
    /// **Ref:** BtdrvAddress
    pub addr: BtdrvAddress,
    /// [1.0.0-12.1.0] BdName. On 13.0.0+ name2 is used instead.
    pub name: BtmBdName,
    /// ClassOfDevice
    pub class_of_device: BtmClassOfDevice,
    /// LinkKey
    pub link_key: [u8; 16],
    /// LinkKeyPresent
    pub link_key_present: u8,
    /// Version
    pub version: u16,
    /// TrustedServices
    pub trusted_services: u32,
    /// Vid
    pub vid: u16,
    /// Pid
    pub pid: u16,
    /// SubClass
    pub sub_class: u8,
    /// AttributeMask
    pub attribute_mask: u8,
    /// DescriptorLength
    pub descriptor_length: u16,
    /// Descriptor
    pub descriptor: [u8; 128],
    /// KeyType
    pub key_type: u8,
    /// DeviceType
    pub device_type: u8,
    /// BrrSize
    pub brr_size: u16,
    /// Brr
    pub brr: [u8; 9],
    /// [13.0.0+] AudioSourceVolume
    pub audio_source_volume: u8,
    /// [13.0.0+] Name
    pub name2: [c_char; 249],
    /// [15.0.0+] AudioSinkVolume
    pub audio_sink_volume: u8,
    /// [14.0.0+] AudioFlags
    pub audio_flags: u32,
    /// Reserved
    pub reserved: [u8; 44],
}

/// Structure returned by `setsysGetFirmwareVersion`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysFirmwareVersion {
    pub major: u8,
    pub minor: u8,
    pub micro: u8,
    pub padding1: u8,
    pub revision_major: u8,
    pub revision_minor: u8,
    pub padding2: u8,
    pub padding3: u8,
    pub platform: [c_char; 32],
    pub version_hash: [c_char; 64],
    pub display_version: [c_char; 24],
    pub display_title: [c_char; 128],
}

/// Structure returned by `setsysGetFirmwareVersionDigest`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysFirmwareVersionDigest {
    pub digest: [c_char; 64],
}

/// Structure returned by `setsysGetSerialNumber`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysSerialNumber {
    pub number: [c_char; 24],
}

/// DeviceNickName
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysDeviceNickName {
    pub nickname: [c_char; 128],
}

/// UserSelectorSettings
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysUserSelectorSettings {
    /// Bitmask with `SetSysUserSelectorFlag`.
    pub flags: u32,
}

/// AccountSettings
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysAccountSettings {
    pub settings: SetSysUserSelectorSettings,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysAudioVolume {
    /// 0 for Console and Tv, 2 for Headphones.
    pub unk_x0: u32,
    /// From 0-15.
    pub volume: u8,
}

/// EulaVersion
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysEulaVersion {
    pub version: u32,
    pub region_code: s32,
    /// **Ref:** SetSysEulaVersionClockType
    pub clock_type: s32,
    pub pad: u32,
    /// POSIX timestamp.
    pub network_clock_time: u64,
    /// **Ref:** TimeSteadyClockTimePoint
    pub steady_clock_time: TimeSteadyClockTimePoint,
}

/// NotificationTime
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysNotificationTime {
    pub hour: s32,
    pub minute: s32,
}

/// NotificationSettings
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysNotificationSettings {
    /// Bitmask with NotificationFlag.
    pub flags: u32,
    /// **Ref:** SetSysNotificationVolume
    pub volume: s32,
    /// **Ref:** SetSysNotificationTime
    pub start_time: SetSysNotificationTime,
    /// **Ref:** SetSysNotificationTime
    pub end_time: SetSysNotificationTime,
}

/// AccountNotificationSettings
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysAccountNotificationSettings {
    /// **Ref:** AccountUid
    pub uid: AccountUid,
    /// Bitmask with AccountNotificationFlag.
    pub flags: u32,
    /// **Ref:** SetSysFriendPresenceOverlayPermission
    pub friend_presence_overlay_permission: s8,
    /// Padding.
    pub pad: [u8; 3],
}

/// TvSettings
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysTvSettings {
    /// Bitmask with TvFlag.
    pub flags: u32,
    /// **Ref:** SetSysTvResolution
    pub tv_resolution: s32,
    /// **Ref:** SetSysHdmiContentType
    pub hdmi_content_type: s32,
    /// **Ref:** SetSysRgbRange
    pub rgb_range: s32,
    /// **Ref:** SetSysCmuMode
    pub cmu_mode: s32,
    /// Underscan.
    pub underscan: u32,
    /// Gamma.
    pub gamma: f32,
    /// Contrast.
    pub contrast: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysModeLine {
    /// In 10 kHz units.
    pub pixel_clock: u16,
    pub horizontal_active_pixels_lsb: u8,
    pub horizontal_blanking_pixels_lsb: u8,
    pub _bitfield_0: [u8; 1],
    pub vertical_active_lines_lsb: u8,
    pub vertical_blanking_lines_lsb: u8,
    pub _bitfield_1: [u8; 1],
    pub horizontal_sync_offset_pixels_lsb: u8,
    pub horizontal_sync_pulse_width_pixels_lsb: u8,
    pub _bitfield_2: [u8; 2],
    pub horizontal_image_size_mm_lsb: u8,
    pub vertical_image_size_mm_lsb: u8,
    pub _bitfield_3: [u8; 1],
    pub horizontal_border_pixels: u8,
    pub vertical_border_lines: u8,
    pub _bitfield_4: [u8; 1],
}

impl SetSysModeLine {
    bitfields!(_bitfield_0 {
        horizontal_blanking_pixels_msb / set_horizontal_blanking_pixels_msb: 0, 4 => u8;
        horizontal_active_pixels_msb / set_horizontal_active_pixels_msb: 4, 4 => u8;
    });
    bitfields!(_bitfield_1 {
        vertical_blanking_lines_msb / set_vertical_blanking_lines_msb: 0, 4 => u8;
        vertical_active_lines_msb / set_vertical_active_lines_msb: 4, 4 => u8;
    });
    bitfields!(_bitfield_2 {
        vertical_sync_pulse_width_lines_lsb / set_vertical_sync_pulse_width_lines_lsb: 0, 4 => u8;
        vertical_sync_offset_lines_lsb / set_vertical_sync_offset_lines_lsb: 4, 4 => u8;
        vertical_sync_pulse_width_lines_msb / set_vertical_sync_pulse_width_lines_msb: 8, 2 => u8;
        vertical_sync_offset_lines_msb / set_vertical_sync_offset_lines_msb: 10, 2 => u8;
        horizontal_sync_pulse_width_pixels_msb / set_horizontal_sync_pulse_width_pixels_msb: 12, 2 => u8;
        horizontal_sync_offset_pixels_msb / set_horizontal_sync_offset_pixels_msb: 14, 2 => u8;
    });
    bitfields!(_bitfield_3 {
        vertical_image_size_mm_msb / set_vertical_image_size_mm_msb: 0, 4 => u8;
        horizontal_image_size_mm_msb / set_horizontal_image_size_mm_msb: 4, 4 => u8;
    });
    bitfields!(_bitfield_4 {
        features_bitmap_0 / set_features_bitmap_0: 0, 1 => u8;
        features_bitmap_1 / set_features_bitmap_1: 1, 1 => u8;
        features_bitmap_2 / set_features_bitmap_2: 2, 1 => u8;
        features_bitmap_34 / set_features_bitmap_34: 3, 2 => u8;
        features_bitmap_56 / set_features_bitmap_56: 5, 2 => u8;
        interlaced / set_interlaced: 7, 1 => u8;
    });
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysDataBlock_video_svd {
    pub _bitfield_0: [u8; 1],
}

impl SetSysDataBlock_video_svd {
    bitfields!(_bitfield_0 {
        svd_index / set_svd_index: 0, 7 => u8;
        native_flag / set_native_flag: 7, 1 => u8;
    });
}

#[repr(C, align(4))]
#[derive(Debug, Clone, Copy)]
pub struct SetSysDataBlock_video {
    pub _bitfield_0: [u8; 1],
    pub svd: [SetSysDataBlock_video_svd; 12],
}

impl SetSysDataBlock_video {
    bitfields!(_bitfield_0 {
        size / set_size: 0, 5 => u8;
        block_type / set_block_type: 5, 3 => SetSysBlockType;
    });
}

#[repr(C, align(4))]
#[derive(Debug, Clone, Copy)]
pub struct SetSysDataBlock_audio {
    pub _bitfield_0: [u8; 2],
    pub sampling_rates_bitmap: u8,
    pub bitrate: u8,
}

impl SetSysDataBlock_audio {
    bitfields!(_bitfield_0 {
        size / set_size: 0, 5 => u8;
        block_type / set_block_type: 5, 3 => SetSysBlockType;
        channel_count / set_channel_count: 8, 3 => u8;
        format_code / set_format_code: 11, 4 => u8;
        padding1 / set_padding1: 15, 1 => u8;
    });
}

#[repr(C, align(4))]
#[derive(Debug, Clone, Copy)]
pub struct SetSysDataBlock_vendor_specific {
    pub _bitfield_0: [u8; 1],
    pub ieee_registration_id: [u8; 3],
    pub source_physical_address: u16,
    pub mode_bitmap: u8,
    pub max_tmds_frequency: u8,
    pub latency_bitmap: u8,
}

impl SetSysDataBlock_vendor_specific {
    bitfields!(_bitfield_0 {
        size / set_size: 0, 5 => u8;
        block_type / set_block_type: 5, 3 => SetSysBlockType;
    });
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysDataBlock {
    pub video: SetSysDataBlock_video,
    pub audio: SetSysDataBlock_audio,
    pub vendor_specific: SetSysDataBlock_vendor_specific,
    pub padding: [u8; 2],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysExtensionBlock_extension_block_cea {
    pub revision: u8,
    pub dtd_start: u8,
    pub _bitfield_0: [u8; 1],
    pub data_block: SetSysDataBlock,
    pub extended_timing_descriptor: [SetSysModeLine; 5],
    pub padding: [u8; 5],
}

impl SetSysExtensionBlock_extension_block_cea {
    bitfields!(_bitfield_0 {
        native_dtd_count / set_native_dtd_count: 0, 4 => u8;
        native_dtd_feature_bitmap / set_native_dtd_feature_bitmap: 4, 4 => u8;
    });
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysExtensionBlock_extension_block_display_id {
    pub data: [u8; 126],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysExtensionBlock_extension_block_block_map {
    pub second_block_extension_tag: u8,
    pub third_block_extension_tag: u8,
    pub padding: [u8; 124],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union SetSysExtensionBlock_extension_block {
    /// CEA EDID timing extension
    pub cea: SetSysExtensionBlock_extension_block_cea,
    /// [13.0.0+] DisplayID Extension Block
    pub display_id: SetSysExtensionBlock_extension_block_display_id,
    /// [13.0.0+] EDID Extension Block Map
    pub block_map: SetSysExtensionBlock_extension_block_block_map,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SetSysExtensionBlock {
    /// 2 = CEA EDID timing extension, 0x70 = DisplayID Extension Block, 0xF0 = EDID Extension Block Map
    pub extension_tag: u8,
    pub extension_block: SetSysExtensionBlock_extension_block,
    /// Sum of 128 extended bytes should equal 0 mod 256.
    pub extended_checksum: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysEdid_chromaticity {
    pub _bitfield_0: [u8; 2],
    pub red_x_msb: u8,
    pub red_y_msb: u8,
    pub green_x_msb: u8,
    pub green_y_msb: u8,
    pub blue_x_msb: u8,
    pub blue_y_msb: u8,
    pub white_x_msb: u8,
    pub white_y_msb: u8,
}

impl SetSysEdid_chromaticity {
    bitfields!(_bitfield_0 {
        green_y_lsb / set_green_y_lsb: 0, 2 => u8;
        green_x_lsb / set_green_x_lsb: 2, 2 => u8;
        red_y_lsb / set_red_y_lsb: 4, 2 => u8;
        red_x_lsb / set_red_x_lsb: 6, 2 => u8;
        blue_lsb / set_blue_lsb: 8, 4 => u8;
        white_lsb / set_white_lsb: 12, 4 => u8;
    });
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysEdid_timing_info {
    /// Real value is (val + 31) * 8 pixels.
    pub x_resolution: u8,
    pub _bitfield_0: [u8; 1],
}

impl SetSysEdid_timing_info {
    bitfields!(_bitfield_0 {
        /// Real value is val + 60 Hz.
        vertical_frequency / set_vertical_frequency: 0, 6 => u8;
        /// 0 = 16:10, 1 = 4:3, 2 = 5:4, 3 = 16:9.
        aspect_ratio / set_aspect_ratio: 6, 2 => u8;
    });
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysEdid_display_descriptor_name {
    pub display_descriptor_zero: u16,
    pub padding1: u8,
    pub descriptor_type: u8,
    pub padding2: u8,
    pub name: [c_char; 13],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysEdid_display_descriptor_range_limits {
    pub display_descriptor_zero: u16,
    pub padding1: u8,
    pub descriptor_type: u8,
    pub range_limit_offsets: u8,
    pub vertical_field_rate_min: u8,
    pub vertical_field_rate_max: u8,
    pub horizontal_line_rate_min: u8,
    pub horizontal_line_rate_max: u8,
    /// Rounded up to multiples of 10 MHz.
    pub pixel_clock_rate_max: u8,
    pub extended_timing_info: u8,
    pub padding: [u8; 7],
}

/// Edid
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SetSysEdid {
    /// Fixed pattern 00 FF FF FF FF FF FF 00.
    pub pattern: [u8; 8],
    /// Big-endian set of 3 5-bit values representing letters, 1 = A .. 26 = Z.
    pub pnp_id: u16,
    pub product_code: u16,
    pub serial_number: u32,
    pub manufacture_week: u8,
    /// Real year is 1990 + val.
    pub manufacture_year: u8,
    pub edid_version: u8,
    pub edid_revision: u8,
    pub video_input_parameters_bitmap: u8,
    pub display_width: u8,
    pub display_height: u8,
    pub display_gamma: u8,
    pub supported_features_bitmap: u8,
    pub chromaticity: SetSysEdid_chromaticity,
    pub timing_bitmap: [u8; 3],
    pub timing_info: [SetSysEdid_timing_info; 8],
    pub timing_descriptor: [SetSysModeLine; 2],
    pub display_descriptor_name: SetSysEdid_display_descriptor_name,
    pub display_descriptor_range_limits: SetSysEdid_display_descriptor_range_limits,
    pub extension_count: u8,
    /// Sum of all 128 bytes should equal 0 mod 256.
    pub checksum: u8,
    pub data1: SetSysExtensionBlock,
    /// [13.0.0+]
    pub data2: SetSysExtensionBlock,
    /// [13.0.0+]
    pub data3: SetSysExtensionBlock,
}

/// DataDeletionSettings
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysDataDeletionSettings {
    /// Bitmask with DataDeletionFlag.
    pub flags: u32,
    /// Use count.
    pub use_count: s32,
}

/// SleepSettings
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysSleepSettings {
    /// Bitmask with SleepFlag.
    pub flags: u32,
    /// **Ref:** SetSysHandheldSleepPlan
    pub handheld_sleep_plan: s32,
    /// **Ref:** SetSysConsoleSleepPlan
    pub console_sleep_plan: s32,
}

/// InitialLaunchSettings
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysInitialLaunchSettings {
    /// Bitmask with InitialLaunchFlag.
    pub flags: u32,
    /// Padding.
    pub pad: u32,
    /// **Ref:** TimeSteadyClockTimePoint timestamp.
    pub timestamp: TimeSteadyClockTimePoint,
}

/// PtmFuelGaugeParameter
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysPtmFuelGaugeParameter {
    pub rcomp0: u16,
    pub tempc0: u16,
    pub fullcap: u16,
    pub fullcapnom: u16,
    pub lavgempty: u16,
    pub qresidual00: u16,
    pub qresidual10: u16,
    pub qresidual20: u16,
    pub qresidual30: u16,
    /// Normally keeps the cycles reg. Unused and contains stack garbage.
    pub cycles: u16,
    /// Keeps track of cycles. The fuel gauge cycles reg is reset if > 2.00 cycles and added here.
    pub cycles_actual: u32,
}

/// Actually nn::util::Color4u8Type.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysColor4u8Type {
    pub field: [u8; 4],
}

/// NxControllerLegacySettings
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysNxControllerLegacySettings {
    pub address: BtdrvAddress,
    /// **Ref:** SetSysControllerType.
    pub type_: u8,
    pub serial: [c_char; 16],
    pub body_color: SetSysColor4u8Type,
    pub button_color: SetSysColor4u8Type,
    pub unk_x1F: [u8; 8],
    pub unk_x27: u8,
    /// Bitmask with `XcdInterfaceType`.
    pub interface_type: u8,
}

/// NxControllerSettings
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysNxControllerSettings {
    pub address: BtdrvAddress,
    /// **Ref:** SetSysControllerType.
    pub type_: u8,
    pub serial: [c_char; 16],
    pub body_color: SetSysColor4u8Type,
    pub button_color: SetSysColor4u8Type,
    pub unk_x1F: [u8; 8],
    pub unk_x27: u8,
    /// Bitmask with `XcdInterfaceType`.
    pub interface_type: u8,
    /// Unknown
    pub unk_x29: [u8; 1027],
}

/// ConsoleSixAxisSensorAccelerationBias
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysConsoleSixAxisSensorAccelerationBias {
    pub unk_x0: f32,
    pub unk_x4: f32,
    pub unk_x8: f32,
}

/// ConsoleSixAxisSensorAngularVelocityBias
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysConsoleSixAxisSensorAngularVelocityBias {
    pub unk_x0: f32,
    pub unk_x4: f32,
    pub unk_x8: f32,
}

/// ConsoleSixAxisSensorAccelerationGain
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysConsoleSixAxisSensorAccelerationGain {
    pub unk_x0: f32,
    pub unk_x4: f32,
    pub unk_x8: f32,
    pub unk_xC: f32,
    pub unk_x10: f32,
    pub unk_x14: f32,
    pub unk_x18: f32,
    pub unk_x1C: f32,
    pub unk_x20: f32,
}

/// ConsoleSixAxisSensorAngularVelocityGain
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysConsoleSixAxisSensorAngularVelocityGain {
    pub unk_x0: f32,
    pub unk_x4: f32,
    pub unk_x8: f32,
    pub unk_xC: f32,
    pub unk_x10: f32,
    pub unk_x14: f32,
    pub unk_x18: f32,
    pub unk_x1C: f32,
    pub unk_x20: f32,
}

/// AllowedSslHosts
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysAllowedSslHosts {
    pub hosts: [u8; 256],
}

/// HostFsMountPoint
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysHostFsMountPoint {
    pub mount: [c_char; 256],
}

/// BlePairingSettings
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysBlePairingSettings {
    pub address: BtdrvAddress,
    pub unk_x6: u16,
    pub unk_x8: u16,
    pub unk_xA: u8,
    pub unk_xB: u8,
    pub unk_xC: u8,
    pub unk_xD: u8,
    pub unk_xE: u8,
    pub unk_xF: u8,
    pub padding: [u8; 112],
}

/// ConsoleSixAxisSensorAngularVelocityTimeBias
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysConsoleSixAxisSensorAngularVelocityTimeBias {
    pub unk_x0: f32,
    pub unk_x4: f32,
    pub unk_x8: f32,
}

/// ConsoleSixAxisSensorAngularAcceleration
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysConsoleSixAxisSensorAngularAcceleration {
    pub unk_x0: f32,
    pub unk_x4: f32,
    pub unk_x8: f32,
    pub unk_xC: f32,
    pub unk_x10: f32,
    pub unk_x14: f32,
    pub unk_x18: f32,
    pub unk_x1C: f32,
    pub unk_x20: f32,
}

/// RebootlessSystemUpdateVersion. This is the content of the RebootlessSystemUpdateVersion SystemData, in the "/version" file.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysRebootlessSystemUpdateVersion {
    pub version: u32,
    pub reserved: [u8; 28],
    pub display_version: [c_char; 32],
}

/// AccountOnlineStorageSettings
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysAccountOnlineStorageSettings {
    /// **Ref:** AccountUid
    pub uid: AccountUid,
    pub unk_x10: u32,
    pub unk_x14: u32,
}

/// AnalogStickUserCalibration
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysAnalogStickUserCalibration {
    pub unk_x0: u16,
    pub unk_x2: u16,
    pub unk_x4: u16,
    pub unk_x6: u16,
    pub unk_x8: u16,
    pub unk_xA: u16,
    pub unk_xC: u16,
    pub unk_xE: u16,
}

/// ThemeId
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysThemeId {
    pub theme_id: [u64; 16],
}

/// ThemeSettings
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysThemeSettings {
    pub theme_settings: u64,
}

/// Output from `setsysGetHomeMenuScheme`. This contains RGBA8 colors which correspond with the physical shell of the system.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysHomeMenuScheme {
    /// Main Color.
    pub main_color: u32,
    /// Back Color.
    pub back_color: u32,
    /// Sub Color.
    pub sub_color: u32,
    /// Bezel Color.
    pub bezel_color: u32,
    /// Extra Color.
    pub extra_color: u32,
}

/// ButtonConfigSettings
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysButtonConfigSettings {
    pub settings: [u8; 1448],
}

/// ButtonConfigRegisteredSettings
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetSysButtonConfigRegisteredSettings {
    pub settings: [u8; 1480],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetCalAccelerometerOffset {
    pub offset: [u8; 6],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetCalAccelerometerScale {
    pub scale: [u8; 6],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetCalAmiiboEcdsaCertificate {
    pub size: u32,
    pub cert: [u8; 112],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetCalAmiiboEcqvBlsCertificate {
    pub size: u32,
    pub cert: [u8; 32],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetCalAmiiboEcqvBlsKey {
    pub size: u32,
    pub key: [u8; 64],
    pub generation: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetCalAmiiboEcqvBlsRootCertificate {
    pub size: u32,
    pub cert: [u8; 144],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetCalAmiiboEcqvCertificate {
    pub size: u32,
    pub cert: [u8; 20],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetCalAmiiboKey {
    pub size: u32,
    pub key: [u8; 80],
    pub generation: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetCalAnalogStickFactoryCalibration {
    pub calibration: [u8; 9],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetCalAnalogStickModelParameter {
    pub parameter: [u8; 18],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetCalBdAddress {
    pub bd_addr: [u8; 6],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetCalConfigurationId1 {
    pub cfg: [u8; 30],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetCalConsoleSixAxisSensorHorizontalOffset {
    pub offset: [u8; 6],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetCalCountryCode {
    /// Country code.
    pub code: [c_char; 3],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetCalEccB233DeviceCertificate {
    pub cert: [u8; 384],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetCalEccB233DeviceKey {
    pub size: u32,
    pub key: [u8; 80],
    pub generation: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetCalGameCardCertificate {
    pub cert: [u8; 1024],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetCalGameCardKey {
    /// Size of the entire key.
    pub size: u32,
    pub key: [u8; 304],
    pub generation: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetCalGyroscopeOffset {
    pub offset: [u8; 6],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetCalGyroscopeScale {
    pub scale: [u8; 6],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetCalMacAddress {
    /// Mac address.
    pub addr: [u8; 6],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetCalRsa2048DeviceCertificate {
    pub cert: [u8; 576],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetCalRsa2048DeviceKey {
    /// Size of the entire key.
    pub size: u32,
    pub key: [u8; 576],
    pub generation: u32,
}

pub type SetCalSerialNumber = SetSysSerialNumber;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetCalSpeakerParameter {
    pub parameter: [u8; 90],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetCalSslCertificate {
    /// Size of the certificate data.
    pub size: u32,
    pub cert: [u8; 2048],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetCalSslKey {
    /// Size of the entire key.
    pub size: u32,
    pub key: [u8; 304],
    pub generation: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetCalRegionCode {
    /// Region code.
    pub code: u32,
}

impl_zeroed_default!(SetBatteryLot, SetSysNetworkSettings, SetSysLcdBacklightBrightnessMapping, SetSysBacklightSettings, SetSysBacklightSettingsEx, SetSysBluetoothDevicesSettings, SetSysFirmwareVersion, SetSysFirmwareVersionDigest, SetSysSerialNumber, SetSysDeviceNickName, SetSysUserSelectorSettings, SetSysAccountSettings, SetSysAudioVolume, SetSysEulaVersion, SetSysNotificationTime, SetSysNotificationSettings, SetSysAccountNotificationSettings, SetSysTvSettings, SetSysModeLine, SetSysDataBlock_video_svd, SetSysDataBlock_video, SetSysDataBlock_audio, SetSysDataBlock_vendor_specific, SetSysDataBlock, SetSysExtensionBlock_extension_block_cea, SetSysExtensionBlock_extension_block_display_id, SetSysExtensionBlock_extension_block_block_map, SetSysExtensionBlock_extension_block, SetSysExtensionBlock, SetSysEdid_chromaticity, SetSysEdid_timing_info, SetSysEdid_display_descriptor_name, SetSysEdid_display_descriptor_range_limits, SetSysEdid, SetSysDataDeletionSettings, SetSysSleepSettings, SetSysInitialLaunchSettings, SetSysPtmFuelGaugeParameter, SetSysColor4u8Type, SetSysNxControllerLegacySettings, SetSysNxControllerSettings, SetSysConsoleSixAxisSensorAccelerationBias, SetSysConsoleSixAxisSensorAngularVelocityBias, SetSysConsoleSixAxisSensorAccelerationGain, SetSysConsoleSixAxisSensorAngularVelocityGain, SetSysAllowedSslHosts, SetSysHostFsMountPoint, SetSysBlePairingSettings, SetSysConsoleSixAxisSensorAngularVelocityTimeBias, SetSysConsoleSixAxisSensorAngularAcceleration, SetSysRebootlessSystemUpdateVersion, SetSysAccountOnlineStorageSettings, SetSysAnalogStickUserCalibration, SetSysThemeId, SetSysThemeSettings, SetSysHomeMenuScheme, SetSysButtonConfigSettings, SetSysButtonConfigRegisteredSettings, SetCalAccelerometerOffset, SetCalAccelerometerScale, SetCalAmiiboEcdsaCertificate, SetCalAmiiboEcqvBlsCertificate, SetCalAmiiboEcqvBlsKey, SetCalAmiiboEcqvBlsRootCertificate, SetCalAmiiboEcqvCertificate, SetCalAmiiboKey, SetCalAnalogStickFactoryCalibration, SetCalAnalogStickModelParameter, SetCalBdAddress, SetCalConfigurationId1, SetCalConsoleSixAxisSensorHorizontalOffset, SetCalCountryCode, SetCalEccB233DeviceCertificate, SetCalEccB233DeviceKey, SetCalGameCardCertificate, SetCalGameCardKey, SetCalGyroscopeOffset, SetCalGyroscopeScale, SetCalMacAddress, SetCalRsa2048DeviceCertificate, SetCalRsa2048DeviceKey, SetCalSpeakerParameter, SetCalSslCertificate, SetCalSslKey, SetCalRegionCode);

unsafe extern "C" {
    /// Initialize set.
    pub fn setInitialize() -> Result;

    /// Exit set.
    pub fn setExit();

    /// Gets the Service object for the actual set service session.
    pub fn setGetServiceSession() -> *mut Service;

    /// Converts LanguageCode to `SetLanguage`.
    pub fn setMakeLanguage(LanguageCode: u64, Language: *mut SetLanguage) -> Result;

    /// Converts `SetLanguage` to LanguageCode.
    pub fn setMakeLanguageCode(Language: SetLanguage, LanguageCode: *mut u64) -> Result;

    /// Gets the current system LanguageCode.
    /// Normally this should be used instead of `setGetLanguageCode`.
    /// LanguageCode is a string, see here: <https://switchbrew.org/wiki/Settings_services#LanguageCode>
    pub fn setGetSystemLanguage(LanguageCode: *mut u64) -> Result;

    /// Gets the current LanguageCode, `setGetSystemLanguage` should be used instead normally.
    pub fn setGetLanguageCode(LanguageCode: *mut u64) -> Result;

    /// Gets available LanguageCodes.
    /// On system-version &lt;4.0.0, max_entries is set to the output from `setGetAvailableLanguageCodeCount` if max_entries is larger than that.
    pub fn setGetAvailableLanguageCodes(total_entries: *mut s32, LanguageCodes: *mut u64, max_entries: usize) -> Result;

    /// Gets total available LanguageCodes.
    /// Output total is overridden with value 0 if the total is &lt;0.
    pub fn setGetAvailableLanguageCodeCount(total: *mut s32) -> Result;

    /// Gets the RegionCode.
    pub fn setGetRegionCode(out: *mut SetRegion) -> Result;

    /// GetQuestFlag
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn setGetQuestFlag(out: *mut bool) -> Result;

    /// Gets the system's nickname.
    ///
    /// # Arguments
    /// - `nickname` (out): `SetSysDeviceNickName`
    ///
    /// **Note:** Only available on [10.1.0+].
    pub fn setGetDeviceNickname(nickname: *mut SetSysDeviceNickName) -> Result;

    /// Initialize setsys.
    pub fn setsysInitialize() -> Result;

    /// Exit setsys.
    pub fn setsysExit();

    /// Gets the Service object for the actual setsys service session.
    pub fn setsysGetServiceSession() -> *mut Service;

    /// SetLanguageCode
    ///
    /// # Arguments
    /// - `LanguageCode` (in): LanguageCode.
    pub fn setsysSetLanguageCode(LanguageCode: u64) -> Result;

    /// SetNetworkSettings
    ///
    /// # Arguments
    /// - `settings` (in): Input array of `SetSysNetworkSettings`.
    /// - `count` (in): Size of the settings array in entries.
    pub fn setsysSetNetworkSettings(settings: *const SetSysNetworkSettings, count: s32) -> Result;

    /// GetNetworkSettings
    ///
    /// # Arguments
    /// - `total_out` (out): Total output entries.
    /// - `versions` (out): Output array of `SetSysNetworkSettings`.
    /// - `count` (in): Size of the settings array in entries.
    pub fn setsysGetNetworkSettings(total_out: *mut s32, settings: *mut SetSysNetworkSettings, count: s32) -> Result;

    /// Gets the system firmware version.
    ///
    /// # Arguments
    /// - `out` (out): Firmware version to populate.
    pub fn setsysGetFirmwareVersion(out: *mut SetSysFirmwareVersion) -> Result;

    /// GetFirmwareVersionDigest
    ///
    /// # Arguments
    /// - `out` (out): `SetSysFirmwareVersionDigest`
    pub fn setsysGetFirmwareVersionDigest(out: *mut SetSysFirmwareVersionDigest) -> Result;

    /// GetLockScreenFlag
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    pub fn setsysGetLockScreenFlag(out: *mut bool) -> Result;

    /// SetLockScreenFlag
    ///
    /// # Arguments
    /// - `flag` (in): Input flag.
    pub fn setsysSetLockScreenFlag(flag: bool) -> Result;

    /// GetBacklightSettings
    ///
    /// # Arguments
    /// - `out` (out): `SetSysBacklightSettings`
    pub fn setsysGetBacklightSettings(out: *mut SetSysBacklightSettings) -> Result;

    /// SetBacklightSettings
    ///
    /// # Arguments
    /// - `settings` (in): `SetSysBacklightSettings`
    pub fn setsysSetBacklightSettings(settings: *const SetSysBacklightSettings) -> Result;

    /// SetBluetoothDevicesSettings
    ///
    /// # Arguments
    /// - `settings` (in): Input array of `SetSysBluetoothDevicesSettings`.
    /// - `count` (in): Size of the settings array in entries.
    pub fn setsysSetBluetoothDevicesSettings(settings: *const SetSysBluetoothDevicesSettings, count: s32) -> Result;

    /// GetBluetoothDevicesSettings
    ///
    /// # Arguments
    /// - `total_out` (out): Total output entries.
    /// - `settings` (out): Output array of `SetSysBluetoothDevicesSettings`.
    /// - `count` (in): Size of the settings array in entries.
    pub fn setsysGetBluetoothDevicesSettings(total_out: *mut s32, settings: *mut SetSysBluetoothDevicesSettings, count: s32) -> Result;

    /// GetExternalSteadyClockSourceId
    ///
    /// # Arguments
    /// - `out` (out): `Uuid`
    pub fn setsysGetExternalSteadyClockSourceId(out: *mut Uuid) -> Result;

    /// SetExternalSteadyClockSourceId
    ///
    /// # Arguments
    /// - `uuid` (in): `Uuid`
    pub fn setsysSetExternalSteadyClockSourceId(uuid: *const Uuid) -> Result;

    /// GetUserSystemClockContext
    ///
    /// # Arguments
    /// - `out` (out): `TimeSystemClockContext`
    pub fn setsysGetUserSystemClockContext(out: *mut TimeSystemClockContext) -> Result;

    /// SetUserSystemClockContext
    ///
    /// # Arguments
    /// - `context` (in): `TimeSystemClockContext`
    pub fn setsysSetUserSystemClockContext(context: *const TimeSystemClockContext) -> Result;

    /// GetAccountSettings
    ///
    /// # Arguments
    /// - `out` (out): `SetSysAccountSettings`
    pub fn setsysGetAccountSettings(out: *mut SetSysAccountSettings) -> Result;

    /// SetAccountSettings
    ///
    /// # Arguments
    /// - `settings` (in): `SetSysAccountSettings`
    pub fn setsysSetAccountSettings(settings: SetSysAccountSettings) -> Result;

    /// GetAudioVolume
    ///
    /// # Arguments
    /// - `device` (in): `SetSysAudioDevice`
    /// - `out` (out): `SetSysAudioVolume`
    pub fn setsysGetAudioVolume(device: SetSysAudioDevice, out: *mut SetSysAudioVolume) -> Result;

    /// SetAudioVolume
    ///
    /// # Arguments
    /// - `device` (in): `SetSysAudioDevice`
    /// - `volume` (in): `SetSysAudioVolume`
    pub fn setsysSetAudioVolume(device: SetSysAudioDevice, volume: *const SetSysAudioVolume) -> Result;

    /// GetEulaVersions
    ///
    /// # Arguments
    /// - `total_out` (out): Total output entries.
    /// - `versions` (out): Output array of `SetSysEulaVersion`.
    /// - `count` (in): Size of the versions array in entries.
    pub fn setsysGetEulaVersions(total_out: *mut s32, versions: *mut SetSysEulaVersion, count: s32) -> Result;

    /// SetEulaVersions
    ///
    /// # Arguments
    /// - `versions` (in): Input array of `SetSysEulaVersion`.
    /// - `count` (in): Size of the versions array in entries.
    pub fn setsysSetEulaVersions(versions: *const SetSysEulaVersion, count: s32) -> Result;

    /// Gets the current system theme.
    pub fn setsysGetColorSetId(out: *mut ColorSetId) -> Result;

    /// Sets the current system theme.
    pub fn setsysSetColorSetId(id: ColorSetId) -> Result;

    /// GetConsoleInformationUploadFlag
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    pub fn setsysGetConsoleInformationUploadFlag(out: *mut bool) -> Result;

    /// SetConsoleInformationUploadFlag
    ///
    /// # Arguments
    /// - `flag` (in): Input flag.
    pub fn setsysSetConsoleInformationUploadFlag(flag: bool) -> Result;

    /// GetAutomaticApplicationDownloadFlag
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    pub fn setsysGetAutomaticApplicationDownloadFlag(out: *mut bool) -> Result;

    /// SetAutomaticApplicationDownloadFlag
    ///
    /// # Arguments
    /// - `flag` (in): Input flag.
    pub fn setsysSetAutomaticApplicationDownloadFlag(flag: bool) -> Result;

    /// GetNotificationSettings
    ///
    /// # Arguments
    /// - `out` (out): `SetSysNotificationSettings`
    pub fn setsysGetNotificationSettings(out: *mut SetSysNotificationSettings) -> Result;

    /// SetNotificationSettings
    ///
    /// # Arguments
    /// - `settings` (in): `SetSysNotificationSettings`
    pub fn setsysSetNotificationSettings(settings: *const SetSysNotificationSettings) -> Result;

    /// GetAccountNotificationSettings
    ///
    /// # Arguments
    /// - `total_out` (out): Total output entries.
    /// - `settings` (out): Output array of `SetSysAccountNotificationSettings`.
    /// - `count` (in): Size of the settings array in entries.
    pub fn setsysGetAccountNotificationSettings(total_out: *mut s32, settings: *mut SetSysAccountNotificationSettings, count: s32) -> Result;

    /// SetAccountNotificationSettings
    ///
    /// # Arguments
    /// - `settings` (in): Input array of `SetSysAccountNotificationSettings`.
    /// - `count` (in): Size of the settings array in entries.
    pub fn setsysSetAccountNotificationSettings(settings: *const SetSysAccountNotificationSettings, count: s32) -> Result;

    /// GetVibrationMasterVolume
    ///
    /// # Arguments
    /// - `out` (out): Output volume.
    pub fn setsysGetVibrationMasterVolume(out: *mut f32) -> Result;

    /// SetVibrationMasterVolume
    ///
    /// # Arguments
    /// - `volume` (in): Input volume.
    pub fn setsysSetVibrationMasterVolume(volume: f32) -> Result;

    /// Gets the size of a settings item value.
    ///
    /// # Arguments
    /// - `name`: Name string.
    /// - `item_key`: Item key string.
    /// - `size_out`: Pointer to output the size to.
    pub fn setsysGetSettingsItemValueSize(name: *const c_char, item_key: *const c_char, size_out: *mut u64) -> Result;

    /// Gets the value of a settings item.
    ///
    /// # Arguments
    /// - `name`: Name string.
    /// - `item_key`: Item key string.
    /// - `value_out`: Pointer to output the value to.
    /// - `value_out_size`: Size of the value_out buffer.
    /// - `size_out`: Total size which was copied to value_out.
    pub fn setsysGetSettingsItemValue(name: *const c_char, item_key: *const c_char, value_out: *mut c_void, value_out_size: usize, size_out: *mut u64) -> Result;

    /// GetTvSettings
    ///
    /// # Arguments
    /// - `out` (out): `SetSysTvSettings`
    pub fn setsysGetTvSettings(out: *mut SetSysTvSettings) -> Result;

    /// SetTvSettings
    ///
    /// # Arguments
    /// - `settings` (in): `SetSysTvSettings`
    pub fn setsysSetTvSettings(settings: *const SetSysTvSettings) -> Result;

    /// GetEdid
    ///
    /// # Arguments
    /// - `out` (out): `SetSysEdid`
    pub fn setsysGetEdid(out: *mut SetSysEdid) -> Result;

    /// SetEdid
    ///
    /// # Arguments
    /// - `edid` (in): `SetSysEdid`
    pub fn setsysSetEdid(edid: *const SetSysEdid) -> Result;

    /// GetAudioOutputMode
    ///
    /// # Arguments
    /// - `target` (in): `SetSysAudioOutputModeTarget`
    /// - `out` (out): `SetSysAudioOutputMode`
    pub fn setsysGetAudioOutputMode(target: SetSysAudioOutputModeTarget, out: *mut SetSysAudioOutputMode) -> Result;

    /// SetAudioOutputMode
    ///
    /// # Arguments
    /// - `target` (in): `SetSysAudioOutputModeTarget`
    /// - `mode` (in): `SetSysAudioOutputMode`
    pub fn setsysSetAudioOutputMode(target: SetSysAudioOutputModeTarget, mode: SetSysAudioOutputMode) -> Result;

    /// GetSpeakerAutoMuteFlag
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    pub fn setsysGetSpeakerAutoMuteFlag(out: *mut bool) -> Result;

    /// SetSpeakerAutoMuteFlag
    ///
    /// # Arguments
    /// - `flag` (in): Input flag.
    pub fn setsysSetSpeakerAutoMuteFlag(flag: bool) -> Result;

    /// GetQuestFlag
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    pub fn setsysGetQuestFlag(out: *mut bool) -> Result;

    /// SetQuestFlag
    ///
    /// # Arguments
    /// - `flag` (in): Input flag.
    pub fn setsysSetQuestFlag(flag: bool) -> Result;

    /// GetDataDeletionSettings
    ///
    /// # Arguments
    /// - `out` (out): `SetSysDataDeletionSettings`
    pub fn setsysGetDataDeletionSettings(out: *mut SetSysDataDeletionSettings) -> Result;

    /// SetDataDeletionSettings
    ///
    /// # Arguments
    /// - `settings` (in): `SetSysDataDeletionSettings`
    pub fn setsysSetDataDeletionSettings(settings: *const SetSysDataDeletionSettings) -> Result;

    /// GetInitialSystemAppletProgramId
    ///
    /// # Arguments
    /// - `out` (out): output ProgramId.
    pub fn setsysGetInitialSystemAppletProgramId(out: *mut u64) -> Result;

    /// GetOverlayDispProgramId
    ///
    /// # Arguments
    /// - `out` (out): output ProgramId.
    pub fn setsysGetOverlayDispProgramId(out: *mut u64) -> Result;

    /// GetDeviceTimeZoneLocationName
    ///
    /// # Arguments
    /// - `out` (out): `TimeLocationName`
    pub fn setsysGetDeviceTimeZoneLocationName(out: *mut TimeLocationName) -> Result;

    /// SetDeviceTimeZoneLocationName
    ///
    /// # Arguments
    /// - `name` (in): `TimeLocationName`
    pub fn setsysSetDeviceTimeZoneLocationName(name: *const TimeLocationName) -> Result;

    /// GetWirelessCertificationFileSize
    ///
    /// # Arguments
    /// - `out_size` (out): Output size.
    pub fn setsysGetWirelessCertificationFileSize(out_size: *mut u64) -> Result;

    /// GetWirelessCertificationFile
    ///
    /// # Arguments
    /// - `buffer` (out): Output buffer.
    /// - `size` (in): Output buffer size.
    /// - `out_size` (out): Output size.
    pub fn setsysGetWirelessCertificationFile(buffer: *mut c_void, size: usize, out_size: *mut u64) -> Result;

    /// SetRegionCode
    ///
    /// # Arguments
    /// - `region` (in): `SetRegion`
    pub fn setsysSetRegionCode(region: SetRegion) -> Result;

    /// GetNetworkSystemClockContext
    ///
    /// # Arguments
    /// - `out` (out): `TimeSystemClockContext`
    pub fn setsysGetNetworkSystemClockContext(out: *mut TimeSystemClockContext) -> Result;

    /// SetNetworkSystemClockContext
    ///
    /// # Arguments
    /// - `context` (in): `TimeSystemClockContext`
    pub fn setsysSetNetworkSystemClockContext(context: *const TimeSystemClockContext) -> Result;

    /// IsUserSystemClockAutomaticCorrectionEnabled
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    pub fn setsysIsUserSystemClockAutomaticCorrectionEnabled(out: *mut bool) -> Result;

    /// SetUserSystemClockAutomaticCorrectionEnabled
    ///
    /// # Arguments
    /// - `flag` (in): Input flag.
    pub fn setsysSetUserSystemClockAutomaticCorrectionEnabled(flag: bool) -> Result;

    /// GetDebugModeFlag
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    pub fn setsysGetDebugModeFlag(out: *mut bool) -> Result;

    /// GetPrimaryAlbumStorage
    ///
    /// # Arguments
    /// - `out` (out): `GetPrimaryAlbumStorage`
    pub fn setsysGetPrimaryAlbumStorage(out: *mut SetSysPrimaryAlbumStorage) -> Result;

    /// SetPrimaryAlbumStorage
    ///
    /// # Arguments
    /// - `storage` (in): `SetSysPrimaryAlbumStorage`
    pub fn setsysSetPrimaryAlbumStorage(storage: SetSysPrimaryAlbumStorage) -> Result;

    /// GetUsb30EnableFlag
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    pub fn setsysGetUsb30EnableFlag(out: *mut bool) -> Result;

    /// SetUsb30EnableFlag
    ///
    /// # Arguments
    /// - `flag` (in): Input flag.
    pub fn setsysSetUsb30EnableFlag(flag: bool) -> Result;

    /// Gets the `SetBatteryLot`.
    ///
    /// # Arguments
    /// - `out` (out): `SetBatteryLot`
    pub fn setsysGetBatteryLot(out: *mut SetBatteryLot) -> Result;

    /// Gets the system's serial number.
    ///
    /// # Arguments
    /// - `out` (out): `SetSysSerialNumber`
    pub fn setsysGetSerialNumber(out: *mut SetSysSerialNumber) -> Result;

    /// GetNfcEnableFlag
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    pub fn setsysGetNfcEnableFlag(out: *mut bool) -> Result;

    /// SetNfcEnableFlag
    ///
    /// # Arguments
    /// - `flag` (in): Input flag.
    pub fn setsysSetNfcEnableFlag(flag: bool) -> Result;

    /// GetSleepSettings
    ///
    /// # Arguments
    /// - `out` (out): `SetSysSleepSettings`
    pub fn setsysGetSleepSettings(out: *mut SetSysSleepSettings) -> Result;

    /// SetSleepSettings
    ///
    /// # Arguments
    /// - `settings` (in): `SetSysSleepSettings`
    pub fn setsysSetSleepSettings(settings: *const SetSysSleepSettings) -> Result;

    /// GetWirelessLanEnableFlag
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    pub fn setsysGetWirelessLanEnableFlag(out: *mut bool) -> Result;

    /// SetWirelessLanEnableFlag
    ///
    /// # Arguments
    /// - `flag` (in): Input flag.
    pub fn setsysSetWirelessLanEnableFlag(flag: bool) -> Result;

    /// GetInitialLaunchSettings
    ///
    /// # Arguments
    /// - `out` (out): `SetSysInitialLaunchSettings`
    pub fn setsysGetInitialLaunchSettings(out: *mut SetSysInitialLaunchSettings) -> Result;

    /// SetInitialLaunchSettings
    ///
    /// # Arguments
    /// - `settings` (in): `SetSysInitialLaunchSettings`
    pub fn setsysSetInitialLaunchSettings(settings: *const SetSysInitialLaunchSettings) -> Result;

    /// Gets the system's nickname.
    ///
    /// # Arguments
    /// - `nickname` (out): `SetSysDeviceNickName`
    ///
    /// **Note:** Same as `setGetDeviceNickname`, which official sw uses instead on [10.1.0+].
    pub fn setsysGetDeviceNickname(nickname: *mut SetSysDeviceNickName) -> Result;

    /// Sets the system's nickname.
    ///
    /// # Arguments
    /// - `nickname` (in): `SetSysDeviceNickName`
    pub fn setsysSetDeviceNickname(nickname: *const SetSysDeviceNickName) -> Result;

    /// GetProductModel
    ///
    /// # Arguments
    /// - `model` (out): Output SetSysProductModel.
    pub fn setsysGetProductModel(model: *mut SetSysProductModel) -> Result;

    /// GetLdnChannel
    ///
    /// # Arguments
    /// - `out` (out): Output LdnChannel.
    pub fn setsysGetLdnChannel(out: *mut s32) -> Result;

    /// SetLdnChannel
    ///
    /// # Arguments
    /// - `channel` (in): Input LdnChannel.
    pub fn setsysSetLdnChannel(channel: s32) -> Result;

    /// Gets an event that settings will signal on flag change.
    ///
    /// # Arguments
    /// - `out_event`: Event to bind. Output event will have autoclear=false.
    pub fn setsysAcquireTelemetryDirtyFlagEventHandle(out_event: *mut Event) -> Result;

    /// Gets the settings flags that have changed.
    ///
    /// # Arguments
    /// - `flags_0`: Pointer to populate with first 64 flags.
    /// - `flags_1`: Pointer to populate with second 64 flags.
    pub fn setsysGetTelemetryDirtyFlags(flags_0: *mut u64, flags_1: *mut u64) -> Result;

    /// GetPtmBatteryLot
    ///
    /// # Arguments
    /// - `out` (out): `SetBatteryLot`
    pub fn setsysGetPtmBatteryLot(out: *mut SetBatteryLot) -> Result;

    /// SetPtmBatteryLot
    ///
    /// # Arguments
    /// - `lot` (in): `SetBatteryLot`
    pub fn setsysSetPtmBatteryLot(lot: *const SetBatteryLot) -> Result;

    /// GetPtmFuelGaugeParameter
    ///
    /// # Arguments
    /// - `out` (out): `SetSysPtmFuelGaugeParameter`
    pub fn setsysGetPtmFuelGaugeParameter(out: *mut SetSysPtmFuelGaugeParameter) -> Result;

    /// SetPtmFuelGaugeParameter
    ///
    /// # Arguments
    /// - `parameter` (in): `SetSysPtmFuelGaugeParameter`
    pub fn setsysSetPtmFuelGaugeParameter(parameter: *const SetSysPtmFuelGaugeParameter) -> Result;

    /// GetBluetoothEnableFlag
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    pub fn setsysGetBluetoothEnableFlag(out: *mut bool) -> Result;

    /// SetBluetoothEnableFlag
    ///
    /// # Arguments
    /// - `flag` (in): Input flag.
    pub fn setsysSetBluetoothEnableFlag(flag: bool) -> Result;

    /// GetMiiAuthorId
    ///
    /// # Arguments
    /// - `out` (out): Output MiiAuthorId.
    pub fn setsysGetMiiAuthorId(out: *mut Uuid) -> Result;

    /// SetShutdownRtcValue
    ///
    /// # Arguments
    /// - `value` (in): Input value.
    pub fn setsysSetShutdownRtcValue(value: u64) -> Result;

    /// GetShutdownRtcValue
    ///
    /// # Arguments
    /// - `out` (out): Output value.
    pub fn setsysGetShutdownRtcValue(out: *mut u64) -> Result;

    /// Gets an event that settings will signal on flag change.
    ///
    /// # Arguments
    /// - `out_event`: Event to bind. Output event will have autoclear=false.
    pub fn setsysAcquireFatalDirtyFlagEventHandle(out_event: *mut Event) -> Result;

    /// Gets the settings flags that have changed.
    ///
    /// # Arguments
    /// - `flags_0`: Pointer to populate with first 64 flags.
    /// - `flags_1`: Pointer to populate with second 64 flags.
    pub fn setsysGetFatalDirtyFlags(flags_0: *mut u64, flags_1: *mut u64) -> Result;

    /// GetAutoUpdateEnableFlag
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn setsysGetAutoUpdateEnableFlag(out: *mut bool) -> Result;

    /// SetAutoUpdateEnableFlag
    ///
    /// # Arguments
    /// - `flag` (in): Input flag.
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn setsysSetAutoUpdateEnableFlag(flag: bool) -> Result;

    /// GetNxControllerSettings
    ///
    /// # Arguments
    /// - `total_out` (out): Total output entries.
    /// - `settings` (out): Output array of `SetSysNxControllerLegacySettings`.
    /// - `count` (in): Size of the settings array in entries.
    ///
    /// **Note:** On [13.0.0+] `setsysGetNxControllerSettingsEx` should be used instead.
    pub fn setsysGetNxControllerSettings(total_out: *mut s32, settings: *mut SetSysNxControllerLegacySettings, count: s32) -> Result;

    /// SetNxControllerSettings
    ///
    /// # Arguments
    /// - `settings` (in): Input array of `SetSysNxControllerLegacySettings`.
    /// - `count` (in): Size of the settings array in entries.
    ///
    /// **Note:** On [13.0.0+] `setsysSetNxControllerSettingsEx` should be used instead.
    pub fn setsysSetNxControllerSettings(settings: *const SetSysNxControllerLegacySettings, count: s32) -> Result;

    /// GetBatteryPercentageFlag
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn setsysGetBatteryPercentageFlag(out: *mut bool) -> Result;

    /// SetBatteryPercentageFlag
    ///
    /// # Arguments
    /// - `flag` (in): Input flag.
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn setsysSetBatteryPercentageFlag(flag: bool) -> Result;

    /// GetExternalRtcResetFlag
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn setsysGetExternalRtcResetFlag(out: *mut bool) -> Result;

    /// SetExternalRtcResetFlag
    ///
    /// # Arguments
    /// - `flag` (in): Input flag.
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn setsysSetExternalRtcResetFlag(flag: bool) -> Result;

    /// GetUsbFullKeyEnableFlag
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn setsysGetUsbFullKeyEnableFlag(out: *mut bool) -> Result;

    /// SetUsbFullKeyEnableFlag
    ///
    /// # Arguments
    /// - `flag` (in): Input flag.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn setsysSetUsbFullKeyEnableFlag(flag: bool) -> Result;

    /// SetExternalSteadyClockInternalOffset
    ///
    /// # Arguments
    /// - `offset` (in): Input offset.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn setsysSetExternalSteadyClockInternalOffset(offset: u64) -> Result;

    /// GetExternalSteadyClockInternalOffset
    ///
    /// # Arguments
    /// - `out` (out): Output offset.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn setsysGetExternalSteadyClockInternalOffset(out: *mut u64) -> Result;

    /// GetBacklightSettingsEx
    ///
    /// # Arguments
    /// - `out` (out): `SetSysBacklightSettingsEx`
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn setsysGetBacklightSettingsEx(out: *mut SetSysBacklightSettingsEx) -> Result;

    /// SetBacklightSettingsEx
    ///
    /// # Arguments
    /// - `settings` (in): `SetSysBacklightSettingsEx`
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn setsysSetBacklightSettingsEx(settings: *const SetSysBacklightSettingsEx) -> Result;

    /// GetHeadphoneVolumeWarningCount
    ///
    /// # Arguments
    /// - `out` (out): Output count.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn setsysGetHeadphoneVolumeWarningCount(out: *mut u32) -> Result;

    /// SetHeadphoneVolumeWarningCount
    ///
    /// # Arguments
    /// - `count` (in): Input count.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn setsysSetHeadphoneVolumeWarningCount(count: u32) -> Result;

    /// GetBluetoothAfhEnableFlag
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn setsysGetBluetoothAfhEnableFlag(out: *mut bool) -> Result;

    /// SetBluetoothAfhEnableFlag
    ///
    /// # Arguments
    /// - `flag` (in): Input flag.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn setsysSetBluetoothAfhEnableFlag(flag: bool) -> Result;

    /// GetBluetoothBoostEnableFlag
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn setsysGetBluetoothBoostEnableFlag(out: *mut bool) -> Result;

    /// SetBluetoothBoostEnableFlag
    ///
    /// # Arguments
    /// - `flag` (in): Input flag.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn setsysSetBluetoothBoostEnableFlag(flag: bool) -> Result;

    /// GetInRepairProcessEnableFlag
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn setsysGetInRepairProcessEnableFlag(out: *mut bool) -> Result;

    /// SetInRepairProcessEnableFlag
    ///
    /// # Arguments
    /// - `flag` (in): Input flag.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn setsysSetInRepairProcessEnableFlag(flag: bool) -> Result;

    /// GetHeadphoneVolumeUpdateFlag
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn setsysGetHeadphoneVolumeUpdateFlag(out: *mut bool) -> Result;

    /// SetHeadphoneVolumeUpdateFlag
    ///
    /// # Arguments
    /// - `flag` (in): Input flag.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn setsysSetHeadphoneVolumeUpdateFlag(flag: bool) -> Result;

    /// NeedsToUpdateHeadphoneVolume
    ///
    /// # Arguments
    /// - `a0` (out): Output arg.
    /// - `a1` (out): Output arg.
    /// - `a2` (out): Output arg.
    /// - `flag` (in): Input flag.
    ///
    /// **Note:** Only available on [3.0.0-14.1.2].
    pub fn setsysNeedsToUpdateHeadphoneVolume(a0: *mut u8, a1: *mut u8, a2: *mut u8, flag: bool) -> Result;

    /// GetPushNotificationActivityModeOnSleep
    ///
    /// # Arguments
    /// - `out` (out): Output mode.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn setsysGetPushNotificationActivityModeOnSleep(out: *mut u32) -> Result;

    /// SetPushNotificationActivityModeOnSleep
    ///
    /// # Arguments
    /// - `mode` (in): Input mode.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn setsysSetPushNotificationActivityModeOnSleep(mode: u32) -> Result;

    /// GetServiceDiscoveryControlSettings
    ///
    /// # Arguments
    /// - `out` (out): `ServiceDiscoveryControlSettings`
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn setsysGetServiceDiscoveryControlSettings(out: *mut SetSysServiceDiscoveryControlSettings) -> Result;

    /// SetServiceDiscoveryControlSettings
    ///
    /// # Arguments
    /// - `settings` (in): `ServiceDiscoveryControlSettings`
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn setsysSetServiceDiscoveryControlSettings(settings: SetSysServiceDiscoveryControlSettings) -> Result;

    /// GetErrorReportSharePermission
    ///
    /// # Arguments
    /// - `out` (out): `SetSysErrorReportSharePermission`
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn setsysGetErrorReportSharePermission(out: *mut SetSysErrorReportSharePermission) -> Result;

    /// SetErrorReportSharePermission
    ///
    /// # Arguments
    /// - `permission` (in): `SetSysErrorReportSharePermission`
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn setsysSetErrorReportSharePermission(permission: SetSysErrorReportSharePermission) -> Result;

    /// GetAppletLaunchFlags
    ///
    /// # Arguments
    /// - `out` (out): Output AppletLaunchFlags bitmask.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn setsysGetAppletLaunchFlags(out: *mut u32) -> Result;

    /// SetAppletLaunchFlags
    ///
    /// # Arguments
    /// - `flags` (in): Input AppletLaunchFlags bitmask.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn setsysSetAppletLaunchFlags(flags: u32) -> Result;

    /// GetConsoleSixAxisSensorAccelerationBias
    ///
    /// # Arguments
    /// - `out` (out): `SetSysConsoleSixAxisSensorAccelerationBias`
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn setsysGetConsoleSixAxisSensorAccelerationBias(out: *mut SetSysConsoleSixAxisSensorAccelerationBias) -> Result;

    /// SetConsoleSixAxisSensorAccelerationBias
    ///
    /// # Arguments
    /// - `bias` (in): `SetSysConsoleSixAxisSensorAccelerationBias`
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn setsysSetConsoleSixAxisSensorAccelerationBias(bias: *const SetSysConsoleSixAxisSensorAccelerationBias) -> Result;

    /// GetConsoleSixAxisSensorAngularVelocityBias
    ///
    /// # Arguments
    /// - `out` (out): `SetSysConsoleSixAxisSensorAngularVelocityBias`
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn setsysGetConsoleSixAxisSensorAngularVelocityBias(out: *mut SetSysConsoleSixAxisSensorAngularVelocityBias) -> Result;

    /// SetConsoleSixAxisSensorAngularVelocityBias
    ///
    /// # Arguments
    /// - `bias` (in): `SetSysConsoleSixAxisSensorAngularVelocityBias`
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn setsysSetConsoleSixAxisSensorAngularVelocityBias(bias: *const SetSysConsoleSixAxisSensorAngularVelocityBias) -> Result;

    /// GetConsoleSixAxisSensorAccelerationGain
    ///
    /// # Arguments
    /// - `out` (out): `SetSysConsoleSixAxisSensorAccelerationGain`
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn setsysGetConsoleSixAxisSensorAccelerationGain(out: *mut SetSysConsoleSixAxisSensorAccelerationGain) -> Result;

    /// SetConsoleSixAxisSensorAccelerationGain
    ///
    /// # Arguments
    /// - `gain` (in): `SetSysConsoleSixAxisSensorAccelerationGain`
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn setsysSetConsoleSixAxisSensorAccelerationGain(gain: *const SetSysConsoleSixAxisSensorAccelerationGain) -> Result;

    /// GetConsoleSixAxisSensorAngularVelocityGain
    ///
    /// # Arguments
    /// - `out` (out): `SetSysConsoleSixAxisSensorAngularVelocityGain`
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn setsysGetConsoleSixAxisSensorAngularVelocityGain(out: *mut SetSysConsoleSixAxisSensorAngularVelocityGain) -> Result;

    /// SetConsoleSixAxisSensorAngularVelocityGain
    ///
    /// # Arguments
    /// - `gain` (in): `SetSysConsoleSixAxisSensorAngularVelocityGain`
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn setsysSetConsoleSixAxisSensorAngularVelocityGain(gain: *const SetSysConsoleSixAxisSensorAngularVelocityGain) -> Result;

    /// GetKeyboardLayout
    ///
    /// # Arguments
    /// - `out` (out): `SetKeyboardLayout`
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn setsysGetKeyboardLayout(out: *mut SetKeyboardLayout) -> Result;

    /// SetKeyboardLayout
    ///
    /// # Arguments
    /// - `layout` (in): `SetKeyboardLayout`
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn setsysSetKeyboardLayout(layout: SetKeyboardLayout) -> Result;

    /// GetWebInspectorFlag
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn setsysGetWebInspectorFlag(out: *mut bool) -> Result;

    /// GetAllowedSslHosts
    ///
    /// # Arguments
    /// - `total_out` (out): Total output entries.
    /// - `out` (out): Output array of `SetSysAllowedSslHosts`.
    /// - `count` (in): Size of the hosts array in entries.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn setsysGetAllowedSslHosts(total_out: *mut s32, out: *mut SetSysAllowedSslHosts, count: s32) -> Result;

    /// GetHostFsMountPoint
    ///
    /// # Arguments
    /// - `out` (out): `SetSysHostFsMountPoint`
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn setsysGetHostFsMountPoint(out: *mut SetSysHostFsMountPoint) -> Result;

    /// GetRequiresRunRepairTimeReviser
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn setsysGetRequiresRunRepairTimeReviser(out: *mut bool) -> Result;

    /// SetRequiresRunRepairTimeReviser
    ///
    /// # Arguments
    /// - `flag` (in): Input flag.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn setsysSetRequiresRunRepairTimeReviser(flag: bool) -> Result;

    /// SetBlePairingSettings
    ///
    /// # Arguments
    /// - `settings` (in): Input array of `SetSysBlePairingSettings`.
    /// - `count` (in): Size of the settings array in entries.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn setsysSetBlePairingSettings(settings: *const SetSysBlePairingSettings, count: s32) -> Result;

    /// GetBlePairingSettings
    ///
    /// # Arguments
    /// - `total_out` (out): Total output entries.
    /// - `settings` (out): Output array of `SetSysBlePairingSettings`.
    /// - `count` (in): Size of the hosts array in entries.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn setsysGetBlePairingSettings(total_out: *mut s32, settings: *mut SetSysBlePairingSettings, count: s32) -> Result;

    /// GetConsoleSixAxisSensorAngularVelocityTimeBias
    ///
    /// # Arguments
    /// - `out` (out): `SetSysConsoleSixAxisSensorAngularVelocityTimeBias`
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn setsysGetConsoleSixAxisSensorAngularVelocityTimeBias(out: *mut SetSysConsoleSixAxisSensorAngularVelocityTimeBias) -> Result;

    /// SetConsoleSixAxisSensorAngularVelocityTimeBias
    ///
    /// # Arguments
    /// - `bias` (in): `SetSysConsoleSixAxisSensorAngularVelocityTimeBias`
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn setsysSetConsoleSixAxisSensorAngularVelocityTimeBias(bias: *const SetSysConsoleSixAxisSensorAngularVelocityTimeBias) -> Result;

    /// GetConsoleSixAxisSensorAngularAcceleration
    ///
    /// # Arguments
    /// - `out` (out): `SetSysConsoleSixAxisSensorAngularAcceleration`
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn setsysGetConsoleSixAxisSensorAngularAcceleration(out: *mut SetSysConsoleSixAxisSensorAngularAcceleration) -> Result;

    /// SetConsoleSixAxisSensorAngularAcceleration
    ///
    /// # Arguments
    /// - `acceleration` (in): `SetSysConsoleSixAxisSensorAngularAcceleration`
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn setsysSetConsoleSixAxisSensorAngularAcceleration(acceleration: *const SetSysConsoleSixAxisSensorAngularAcceleration) -> Result;

    /// GetRebootlessSystemUpdateVersion
    ///
    /// # Arguments
    /// - `out` (out): `SetSysRebootlessSystemUpdateVersion`
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn setsysGetRebootlessSystemUpdateVersion(out: *mut SetSysRebootlessSystemUpdateVersion) -> Result;

    /// GetDeviceTimeZoneLocationUpdatedTime
    ///
    /// # Arguments
    /// - `out` (out): `TimeSteadyClockTimePoint`
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn setsysGetDeviceTimeZoneLocationUpdatedTime(out: *mut TimeSteadyClockTimePoint) -> Result;

    /// SetDeviceTimeZoneLocationUpdatedTime
    ///
    /// # Arguments
    /// - `time_point` (in): `TimeSteadyClockTimePoint`
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn setsysSetDeviceTimeZoneLocationUpdatedTime(time_point: *const TimeSteadyClockTimePoint) -> Result;

    /// GetUserSystemClockAutomaticCorrectionUpdatedTime
    ///
    /// # Arguments
    /// - `out` (out): `TimeSteadyClockTimePoint`
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn setsysGetUserSystemClockAutomaticCorrectionUpdatedTime(out: *mut TimeSteadyClockTimePoint) -> Result;

    /// SetUserSystemClockAutomaticCorrectionUpdatedTime
    ///
    /// # Arguments
    /// - `time_point` (in): `TimeSteadyClockTimePoint`
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn setsysSetUserSystemClockAutomaticCorrectionUpdatedTime(time_point: *const TimeSteadyClockTimePoint) -> Result;

    /// GetAccountOnlineStorageSettings
    ///
    /// # Arguments
    /// - `total_out` (out): Total output entries.
    /// - `settings` (out): Output array of `SetSysAccountOnlineStorageSettings`.
    /// - `count` (in): Size of the settings array in entries.
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn setsysGetAccountOnlineStorageSettings(total_out: *mut s32, settings: *mut SetSysAccountOnlineStorageSettings, count: s32) -> Result;

    /// SetAccountOnlineStorageSettings
    ///
    /// # Arguments
    /// - `settings` (in): Input array of `SetSysAccountOnlineStorageSettings`.
    /// - `count` (in): Size of the settings array in entries.
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn setsysSetAccountOnlineStorageSettings(settings: *const SetSysAccountOnlineStorageSettings, count: s32) -> Result;

    /// GetPctlReadyFlag
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn setsysGetPctlReadyFlag(out: *mut bool) -> Result;

    /// SetPctlReadyFlag
    ///
    /// # Arguments
    /// - `flag` (in): Input flag.
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn setsysSetPctlReadyFlag(flag: bool) -> Result;

    /// GetAnalogStickUserCalibrationL
    ///
    /// # Arguments
    /// - `out` (out): `SetSysAnalogStickUserCalibration`
    ///
    /// **Note:** Only available on [8.1.1+].
    pub fn setsysGetAnalogStickUserCalibrationL(out: *mut SetSysAnalogStickUserCalibration) -> Result;

    /// SetAnalogStickUserCalibrationL
    ///
    /// # Arguments
    /// - `calibration` (in): `SetSysAnalogStickUserCalibration`
    ///
    /// **Note:** Only available on [8.1.1+].
    pub fn setsysSetAnalogStickUserCalibrationL(calibration: *const SetSysAnalogStickUserCalibration) -> Result;

    /// GetAnalogStickUserCalibrationR
    ///
    /// # Arguments
    /// - `out` (out): `SetSysAnalogStickUserCalibration`
    ///
    /// **Note:** Only available on [8.1.1+].
    pub fn setsysGetAnalogStickUserCalibrationR(out: *mut SetSysAnalogStickUserCalibration) -> Result;

    /// SetAnalogStickUserCalibrationR
    ///
    /// # Arguments
    /// - `calibration` (in): `SetSysAnalogStickUserCalibration`
    ///
    /// **Note:** Only available on [8.1.1+].
    pub fn setsysSetAnalogStickUserCalibrationR(calibration: *const SetSysAnalogStickUserCalibration) -> Result;

    /// GetPtmBatteryVersion
    ///
    /// # Arguments
    /// - `out` (out): Output version.
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn setsysGetPtmBatteryVersion(out: *mut u8) -> Result;

    /// SetPtmBatteryVersion
    ///
    /// # Arguments
    /// - `version` (in): Input version.
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn setsysSetPtmBatteryVersion(version: u8) -> Result;

    /// GetUsb30HostEnableFlag
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn setsysGetUsb30HostEnableFlag(out: *mut bool) -> Result;

    /// SetUsb30HostEnableFlag
    ///
    /// # Arguments
    /// - `flag` (in): Input flag.
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn setsysSetUsb30HostEnableFlag(flag: bool) -> Result;

    /// GetUsb30DeviceEnableFlag
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn setsysGetUsb30DeviceEnableFlag(out: *mut bool) -> Result;

    /// SetUsb30DeviceEnableFlag
    ///
    /// # Arguments
    /// - `flag` (in): Input flag.
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn setsysSetUsb30DeviceEnableFlag(flag: bool) -> Result;

    /// GetThemeId
    ///
    /// # Arguments
    /// - `type` (in): Input theme id type.
    /// - `out` (out): `SetSysThemeId`
    ///
    /// **Note:** Only available on [7.0.0+].
    pub fn setsysGetThemeId(type_: s32, out: *mut SetSysThemeId) -> Result;

    /// SetThemeId
    ///
    /// # Arguments
    /// - `type` (in): Input theme id type.
    /// - `theme_id` (in): `SetSysThemeId`
    ///
    /// **Note:** Only available on [7.0.0+].
    pub fn setsysSetThemeId(type_: s32, theme_id: *const SetSysThemeId) -> Result;

    /// GetChineseTraditionalInputMethod
    ///
    /// # Arguments
    /// - `out` (out): `SetChineseTraditionalInputMethod`
    ///
    /// **Note:** Only available on [7.0.0+].
    pub fn setsysGetChineseTraditionalInputMethod(out: *mut SetChineseTraditionalInputMethod) -> Result;

    /// SetChineseTraditionalInputMethod
    ///
    /// # Arguments
    /// - `method` (in): `SetChineseTraditionalInputMethod`
    ///
    /// **Note:** Only available on [7.0.0+].
    pub fn setsysSetChineseTraditionalInputMethod(method: SetChineseTraditionalInputMethod) -> Result;

    /// GetPtmCycleCountReliability
    ///
    /// # Arguments
    /// - `out` (out): `SetSysPtmCycleCountReliability`
    ///
    /// **Note:** Only available on [7.0.0+].
    pub fn setsysGetPtmCycleCountReliability(out: *mut SetSysPtmCycleCountReliability) -> Result;

    /// SetPtmCycleCountReliability
    ///
    /// # Arguments
    /// - `reliability` (in): `SetSysPtmCycleCountReliability`
    ///
    /// **Note:** Only available on [7.0.0+].
    pub fn setsysSetPtmCycleCountReliability(reliability: SetSysPtmCycleCountReliability) -> Result;

    /// Gets the `SetSysHomeMenuScheme`.
    ///
    /// # Arguments
    /// - `out` (out): `SetSysHomeMenuScheme`
    ///
    /// **Note:** Only available on [8.1.1+].
    pub fn setsysGetHomeMenuScheme(out: *mut SetSysHomeMenuScheme) -> Result;

    /// GetThemeSettings
    ///
    /// # Arguments
    /// - `out` (out): `SetSysThemeSettings`
    ///
    /// **Note:** Only available on [7.0.0+].
    pub fn setsysGetThemeSettings(out: *mut SetSysThemeSettings) -> Result;

    /// SetThemeSettings
    ///
    /// # Arguments
    /// - `settings` (in): `SetSysThemeSettings`
    ///
    /// **Note:** Only available on [7.0.0+].
    pub fn setsysSetThemeSettings(settings: *const SetSysThemeSettings) -> Result;

    /// GetThemeKey
    ///
    /// # Arguments
    /// - `out` (out): `FsArchiveMacKey`
    ///
    /// **Note:** Only available on [7.0.0+].
    pub fn setsysGetThemeKey(out: *mut FsArchiveMacKey) -> Result;

    /// SetThemeKey
    ///
    /// # Arguments
    /// - `key` (in): `FsArchiveMacKey`
    ///
    /// **Note:** Only available on [7.0.0+].
    pub fn setsysSetThemeKey(key: *const FsArchiveMacKey) -> Result;

    /// GetZoomFlag
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [8.0.0+].
    pub fn setsysGetZoomFlag(out: *mut bool) -> Result;

    /// SetZoomFlag
    ///
    /// # Arguments
    /// - `flag` (in): Input flag.
    ///
    /// **Note:** Only available on [8.0.0+].
    pub fn setsysSetZoomFlag(flag: bool) -> Result;

    /// Returns Terra platform type flag.
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** On [9.0.0+], this is a wrapper for `setsysGetPlatFormRegion()` == 2.
    ///
    /// **Note:** Only available on [8.0.0+].
    pub fn setsysGetT(out: *mut bool) -> Result;

    /// Sets Terra platform type flag.
    ///
    /// # Arguments
    /// - `flag` (in): Input flag.
    ///
    /// **Note:** On [9.0.0+], this is a wrapper for `setsysSetPlatFormRegion`(1 + (IsT & 1)).
    ///
    /// **Note:** Only available on [8.0.0+].
    pub fn setsysSetT(flag: bool) -> Result;

    /// Gets the `SetSysPlatformRegion`.
    ///
    /// # Arguments
    /// - `out` (out): `SetSysPlatformRegion`
    ///
    /// **Note:** This is used internally by `appletGetSettingsPlatformRegion`.
    ///
    /// **Note:** Only available on [9.0.0+].
    pub fn setsysGetPlatformRegion(out: *mut SetSysPlatformRegion) -> Result;

    /// Sets the `SetSysPlatformRegion`.
    ///
    /// # Arguments
    /// - `region` (in): `SetSysPlatformRegion`
    ///
    /// **Note:** Only available on [9.0.0+].
    pub fn setsysSetPlatformRegion(region: SetSysPlatformRegion) -> Result;

    /// GetHomeMenuSchemeModel
    ///
    /// # Arguments
    /// - `out` (out): HomeMenuSchemeModel.
    ///
    /// **Note:** This will throw an error when loading the "settings_debug!{...}" system-setting which is used with this fails.
    ///
    /// **Note:** Only available on [9.0.0+].
    pub fn setsysGetHomeMenuSchemeModel(out: *mut u32) -> Result;

    /// GetMemoryUsageRateFlag
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [9.0.0+].
    pub fn setsysGetMemoryUsageRateFlag(out: *mut bool) -> Result;

    /// Gets the `SetSysTouchScreenMode`.
    ///
    /// # Arguments
    /// - `out` (out): `SetSysTouchScreenMode`
    ///
    /// **Note:** Only available on [9.0.0+].
    pub fn setsysGetTouchScreenMode(out: *mut SetSysTouchScreenMode) -> Result;

    /// Sets the `SetSysTouchScreenMode`.
    ///
    /// # Arguments
    /// - `mode` (in): `SetSysTouchScreenMode`
    ///
    /// **Note:** Only available on [9.0.0+].
    pub fn setsysSetTouchScreenMode(mode: SetSysTouchScreenMode) -> Result;

    /// GetButtonConfigSettingsFull
    ///
    /// # Arguments
    /// - `total_out` (out): Total output entries.
    /// - `settings` (out): Output array of `SetSysButtonConfigSettings`.
    /// - `count` (in): Size of the settings array in entries.
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn setsysGetButtonConfigSettingsFull(total_out: *mut s32, settings: *mut SetSysButtonConfigSettings, count: s32) -> Result;

    /// SetButtonConfigSettingsFull
    ///
    /// # Arguments
    /// - `settings` (in): Input array of `SetSysButtonConfigSettings`.
    /// - `count` (in): Size of the settings array in entries.
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn setsysSetButtonConfigSettingsFull(settings: *const SetSysButtonConfigSettings, count: s32) -> Result;

    /// GetButtonConfigSettingsEmbedded
    ///
    /// # Arguments
    /// - `total_out` (out): Total output entries.
    /// - `settings` (out): Output array of `SetSysButtonConfigSettings`.
    /// - `count` (in): Size of the settings array in entries.
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn setsysGetButtonConfigSettingsEmbedded(total_out: *mut s32, settings: *mut SetSysButtonConfigSettings, count: s32) -> Result;

    /// SetButtonConfigSettingsEmbedded
    ///
    /// # Arguments
    /// - `settings` (in): Input array of `SetSysButtonConfigSettings`.
    /// - `count` (in): Size of the settings array in entries.
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn setsysSetButtonConfigSettingsEmbedded(settings: *const SetSysButtonConfigSettings, count: s32) -> Result;

    /// GetButtonConfigSettingsLeft
    ///
    /// # Arguments
    /// - `total_out` (out): Total output entries.
    /// - `settings` (out): Output array of `SetSysButtonConfigSettings`.
    /// - `count` (in): Size of the settings array in entries.
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn setsysGetButtonConfigSettingsLeft(total_out: *mut s32, settings: *mut SetSysButtonConfigSettings, count: s32) -> Result;

    /// SetButtonConfigSettingsLeft
    ///
    /// # Arguments
    /// - `settings` (in): Input array of `SetSysButtonConfigSettings`.
    /// - `count` (in): Size of the settings array in entries.
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn setsysSetButtonConfigSettingsLeft(settings: *const SetSysButtonConfigSettings, count: s32) -> Result;

    /// GetButtonConfigSettingsRight
    ///
    /// # Arguments
    /// - `total_out` (out): Total output entries.
    /// - `settings` (out): Output array of `SetSysButtonConfigSettings`.
    /// - `count` (in): Size of the settings array in entries.
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn setsysGetButtonConfigSettingsRight(total_out: *mut s32, settings: *mut SetSysButtonConfigSettings, count: s32) -> Result;

    /// SetButtonConfigSettingsRight
    ///
    /// # Arguments
    /// - `settings` (in): Input array of `SetSysButtonConfigSettings`.
    /// - `count` (in): Size of the settings array in entries.
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn setsysSetButtonConfigSettingsRight(settings: *const SetSysButtonConfigSettings, count: s32) -> Result;

    /// GetButtonConfigRegisteredSettingsEmbedded
    ///
    /// # Arguments
    /// - `settings` (out): `SetSysButtonConfigRegisteredSettings`
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn setsysGetButtonConfigRegisteredSettingsEmbedded(settings: *mut SetSysButtonConfigRegisteredSettings) -> Result;

    /// SetButtonConfigRegisteredSettingsEmbedded
    ///
    /// # Arguments
    /// - `settings` (in): `SetSysButtonConfigRegisteredSettings`
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn setsysSetButtonConfigRegisteredSettingsEmbedded(settings: *const SetSysButtonConfigRegisteredSettings) -> Result;

    /// GetButtonConfigRegisteredSettings
    ///
    /// # Arguments
    /// - `settings` (out): `SetSysButtonConfigRegisteredSettings`
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn setsysGetButtonConfigRegisteredSettings(total_out: *mut s32, settings: *mut SetSysButtonConfigRegisteredSettings, count: s32) -> Result;

    /// SetButtonConfigRegisteredSettings
    ///
    /// # Arguments
    /// - `settings` (in): `SetSysButtonConfigRegisteredSettings`
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn setsysSetButtonConfigRegisteredSettings(settings: *const SetSysButtonConfigRegisteredSettings, count: s32) -> Result;

    /// GetFieldTestingFlag
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [10.1.0+].
    pub fn setsysGetFieldTestingFlag(out: *mut bool) -> Result;

    /// SetFieldTestingFlag
    ///
    /// # Arguments
    /// - `flag` (in): Input flag.
    ///
    /// **Note:** Only available on [10.1.0+].
    pub fn setsysSetFieldTestingFlag(flag: bool) -> Result;

    /// GetNxControllerSettingsEx
    ///
    /// # Arguments
    /// - `total_out` (out): Total output entries.
    /// - `settings` (out): Output array of `SetSysNxControllerSettings`.
    /// - `count` (in): Size of the settings array in entries.
    pub fn setsysGetNxControllerSettingsEx(total_out: *mut s32, settings: *mut SetSysNxControllerSettings, count: s32) -> Result;

    /// SetNxControllerSettingsEx
    ///
    /// # Arguments
    /// - `settings` (in): Input array of `SetSysNxControllerSettings`.
    /// - `count` (in): Size of the settings array in entries.
    pub fn setsysSetNxControllerSettingsEx(settings: *const SetSysNxControllerSettings, count: s32) -> Result;

    /// Initialize setcal.
    pub fn setcalInitialize() -> Result;

    /// Exit setcal.
    pub fn setcalExit();

    /// Gets the Service object for the actual setcal service session.
    pub fn setcalGetServiceSession() -> *mut Service;

    /// Gets the `SetCalBdAddress`.
    ///
    /// # Arguments
    /// - `out` (out): `SetCalBdAddress`
    pub fn setcalGetBdAddress(out: *mut SetCalBdAddress) -> Result;

    /// Gets the `SetCalConfigurationId1`.
    ///
    /// # Arguments
    /// - `out` (out): `SetCalConfigurationId1`
    pub fn setcalGetConfigurationId1(out: *mut SetCalConfigurationId1) -> Result;

    /// Gets the `SetCalAccelerometerOffset`.
    ///
    /// # Arguments
    /// - `out` (out): `SetCalAccelerometerOffset`
    pub fn setcalGetAccelerometerOffset(out: *mut SetCalAccelerometerOffset) -> Result;

    /// Gets the `SetCalAccelerometerScale`.
    ///
    /// # Arguments
    /// - `out` (out): `SetCalAccelerometerScale`
    pub fn setcalGetAccelerometerScale(out: *mut SetCalAccelerometerScale) -> Result;

    /// Gets the `SetCalGyroscopeOffset`.
    ///
    /// # Arguments
    /// - `out` (out): `SetCalGyroscopeOffset`
    pub fn setcalGetGyroscopeOffset(out: *mut SetCalGyroscopeOffset) -> Result;

    /// Gets the `SetCalGyroscopeScale`.
    ///
    /// # Arguments
    /// - `out` (out): `SetCalGyroscopeScale`
    pub fn setcalGetGyroscopeScale(out: *mut SetCalGyroscopeScale) -> Result;

    /// Gets the `SetCalMacAddress`.
    ///
    /// # Arguments
    /// - `out` (out): `SetCalMacAddress`
    pub fn setcalGetWirelessLanMacAddress(out: *mut SetCalMacAddress) -> Result;

    /// GetWirelessLanCountryCodeCount
    ///
    /// # Arguments
    /// - `out_count` (out): Output count
    pub fn setcalGetWirelessLanCountryCodeCount(out_count: *mut s32) -> Result;

    /// GetWirelessLanCountryCodes
    ///
    /// # Arguments
    /// - `total_out` (out): Total output entries.
    /// - `codes` (out): Output array of `SetCalCountryCode`.
    /// - `count` (in): Size of the versions array in entries.
    pub fn setcalGetWirelessLanCountryCodes(total_out: *mut s32, codes: *mut SetCalCountryCode, count: s32) -> Result;

    /// Gets the `SetCalSerialNumber`.
    ///
    /// # Arguments
    /// - `out` (out): `SetCalSerialNumber`
    pub fn setcalGetSerialNumber(out: *mut SetCalSerialNumber) -> Result;

    /// SetInitialSystemAppletProgramId
    ///
    /// # Arguments
    /// - `program_id` (in): input ProgramId.
    pub fn setcalSetInitialSystemAppletProgramId(program_id: u64) -> Result;

    /// SetOverlayDispProgramId
    ///
    /// # Arguments
    /// - `program_id` (in): input ProgramId.
    pub fn setcalSetOverlayDispProgramId(program_id: u64) -> Result;

    /// Gets the `SetBatteryLot`.
    ///
    /// # Arguments
    /// - `out` (out): `SetBatteryLot`
    pub fn setcalGetBatteryLot(out: *mut SetBatteryLot) -> Result;

    /// Gets the `SetCalEccB233DeviceCertificate`.
    ///
    /// # Arguments
    /// - `out` (out): `SetCalEccB233DeviceCertificate`
    pub fn setcalGetEciDeviceCertificate(out: *mut SetCalEccB233DeviceCertificate) -> Result;

    /// Gets the `SetCalRsa2048DeviceCertificate`.
    ///
    /// # Arguments
    /// - `out` (out): `SetCalRsa2048DeviceCertificate`
    pub fn setcalGetEticketDeviceCertificate(out: *mut SetCalRsa2048DeviceCertificate) -> Result;

    /// Gets the `SetCalSslKey`.
    ///
    /// # Arguments
    /// - `out` (out): `SetCalSslKey`
    pub fn setcalGetSslKey(out: *mut SetCalSslKey) -> Result;

    /// Gets the `SetCalSslCertificate`.
    ///
    /// # Arguments
    /// - `out` (out): `SetCalSslCertificate`
    pub fn setcalGetSslCertificate(out: *mut SetCalSslCertificate) -> Result;

    /// Gets the `SetCalGameCardKey`.
    ///
    /// # Arguments
    /// - `out` (out): `SetCalGameCardKey`
    pub fn setcalGetGameCardKey(out: *mut SetCalGameCardKey) -> Result;

    /// Gets the `SetCalGameCardCertificate`.
    ///
    /// # Arguments
    /// - `out` (out): `SetCalGameCardCertificate`
    pub fn setcalGetGameCardCertificate(out: *mut SetCalGameCardCertificate) -> Result;

    /// Gets the `SetCalEccB233DeviceKey`.
    ///
    /// # Arguments
    /// - `out` (out): `SetCalEccB233DeviceKey`
    pub fn setcalGetEciDeviceKey(out: *mut SetCalEccB233DeviceKey) -> Result;

    /// Gets the `SetCalRsa2048DeviceKey`.
    ///
    /// # Arguments
    /// - `out` (out): `SetCalRsa2048DeviceKey`
    pub fn setcalGetEticketDeviceKey(out: *mut SetCalRsa2048DeviceKey) -> Result;

    /// Gets the `SetCalSpeakerParameter`.
    ///
    /// # Arguments
    /// - `out` (out): `SetCalSpeakerParameter`
    pub fn setcalGetSpeakerParameter(out: *mut SetCalSpeakerParameter) -> Result;

    /// GetLcdVendorId
    ///
    /// # Arguments
    /// - `out_vendor_id` (out): Output LcdVendorId.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn setcalGetLcdVendorId(out_vendor_id: *mut u32) -> Result;

    /// Gets the `SetCalRsa2048DeviceCertificate`.
    ///
    /// # Arguments
    /// - `out` (out): `SetCalRsa2048DeviceCertificate`
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn setcalGetEciDeviceCertificate2(out: *mut SetCalRsa2048DeviceCertificate) -> Result;

    /// Gets the `SetCalRsa2048DeviceKey`.
    ///
    /// # Arguments
    /// - `out` (out): `SetCalRsa2048DeviceKey`
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn setcalGetEciDeviceKey2(out: *mut SetCalRsa2048DeviceKey) -> Result;

    /// Gets the `SetCalAmiiboKey`.
    ///
    /// # Arguments
    /// - `out` (out): `SetCalAmiiboKey`
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn setcalGetAmiiboKey(out: *mut SetCalAmiiboKey) -> Result;

    /// Gets the `SetCalAmiiboEcqvCertificate`.
    ///
    /// # Arguments
    /// - `out` (out): `SetCalAmiiboEcqvCertificate`
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn setcalGetAmiiboEcqvCertificate(out: *mut SetCalAmiiboEcqvCertificate) -> Result;

    /// Gets the `SetCalAmiiboEcdsaCertificate`.
    ///
    /// # Arguments
    /// - `out` (out): `SetCalAmiiboEcdsaCertificate`
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn setcalGetAmiiboEcdsaCertificate(out: *mut SetCalAmiiboEcdsaCertificate) -> Result;

    /// Gets the `SetCalAmiiboEcqvBlsKey`.
    ///
    /// # Arguments
    /// - `out` (out): `SetCalAmiiboEcqvBlsKey`
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn setcalGetAmiiboEcqvBlsKey(out: *mut SetCalAmiiboEcqvBlsKey) -> Result;

    /// Gets the `SetCalAmiiboEcqvBlsCertificate`.
    ///
    /// # Arguments
    /// - `out` (out): `SetCalAmiiboEcqvBlsCertificate`
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn setcalGetAmiiboEcqvBlsCertificate(out: *mut SetCalAmiiboEcqvBlsCertificate) -> Result;

    /// Gets the `SetCalAmiiboEcqvBlsRootCertificate`.
    ///
    /// # Arguments
    /// - `out` (out): `SetCalAmiiboEcqvBlsRootCertificate`
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn setcalGetAmiiboEcqvBlsRootCertificate(out: *mut SetCalAmiiboEcqvBlsRootCertificate) -> Result;

    /// GetUsbTypeCPowerSourceCircuitVersion
    ///
    /// # Arguments
    /// - `out_version` (out): Output UsbTypeCPowerSourceCircuitVersion.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn setcalGetUsbTypeCPowerSourceCircuitVersion(out_version: *mut u8) -> Result;

    /// GetAnalogStickModuleTypeL
    ///
    /// # Arguments
    /// - `out_version` (out): Output AnalogStickModuleType.
    ///
    /// **Note:** Only available on [8.1.1+].
    pub fn setcalGetAnalogStickModuleTypeL(out_type: *mut u8) -> Result;

    /// Gets the `SetCalAnalogStickModelParameter`.
    ///
    /// # Arguments
    /// - `out` (out): `SetCalAnalogStickModelParameter`
    ///
    /// **Note:** Only available on [8.1.1+].
    pub fn setcalGetAnalogStickModelParameterL(out: *mut SetCalAnalogStickModelParameter) -> Result;

    /// Gets the `SetCalAnalogStickFactoryCalibration`.
    ///
    /// # Arguments
    /// - `out` (out): `SetCalAnalogStickFactoryCalibration`
    ///
    /// **Note:** Only available on [8.1.1+].
    pub fn setcalGetAnalogStickFactoryCalibrationL(out: *mut SetCalAnalogStickFactoryCalibration) -> Result;

    /// GetAnalogStickModuleTypeR
    ///
    /// # Arguments
    /// - `out_version` (out): Output AnalogStickModuleType.
    ///
    /// **Note:** Only available on [8.1.1+].
    pub fn setcalGetAnalogStickModuleTypeR(out_type: *mut u8) -> Result;

    /// Gets the `SetCalAnalogStickModelParameter`.
    ///
    /// # Arguments
    /// - `out` (out): `SetCalAnalogStickModelParameter`
    ///
    /// **Note:** Only available on [8.1.1+].
    pub fn setcalGetAnalogStickModelParameterR(out: *mut SetCalAnalogStickModelParameter) -> Result;

    /// Gets the `SetCalAnalogStickFactoryCalibration`.
    ///
    /// # Arguments
    /// - `out` (out): `SetCalAnalogStickFactoryCalibration`
    ///
    /// **Note:** Only available on [8.1.1+].
    pub fn setcalGetAnalogStickFactoryCalibrationR(out: *mut SetCalAnalogStickFactoryCalibration) -> Result;

    /// GetConsoleSixAxisSensorModuleType
    ///
    /// # Arguments
    /// - `out_version` (out): Output ConsoleSixAxisSensorModuleType.
    ///
    /// **Note:** Only available on [8.1.1+].
    pub fn setcalGetConsoleSixAxisSensorModuleType(out_type: *mut u8) -> Result;

    /// Gets the `SetCalConsoleSixAxisSensorHorizontalOffset`.
    ///
    /// # Arguments
    /// - `out` (out): `SetCalConsoleSixAxisSensorHorizontalOffset`
    ///
    /// **Note:** Only available on [8.1.1+].
    pub fn setcalGetConsoleSixAxisSensorHorizontalOffset(out: *mut SetCalConsoleSixAxisSensorHorizontalOffset) -> Result;

    /// GetBatteryVersion
    ///
    /// # Arguments
    /// - `out_version` (out): Output BatteryVersion.
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn setcalGetBatteryVersion(out_version: *mut u8) -> Result;

    /// GetDeviceId
    ///
    /// # Arguments
    /// - `out_type` (out): Output DeviceId.
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn setcalGetDeviceId(out_device_id: *mut u64) -> Result;

    /// GetConsoleSixAxisSensorMountType
    ///
    /// # Arguments
    /// - `out_type` (out): Output ConsoleSixAxisSensorMountType.
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn setcalGetConsoleSixAxisSensorMountType(out_type: *mut u8) -> Result;
}
