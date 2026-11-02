//! `switch/services/nifm.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub type NifmServiceType = u32;
/// Initializes nifm:u.
pub const NifmServiceType_User: NifmServiceType = 0;
/// Initializes nifm:s.
pub const NifmServiceType_System: NifmServiceType = 1;
/// Initializes nifm:a.
pub const NifmServiceType_Admin: NifmServiceType = 2;

pub type NifmInternetConnectionType = u32;
/// Wi-Fi connection is used.
pub const NifmInternetConnectionType_WiFi: NifmInternetConnectionType = 1;
/// Ethernet connection is used.
pub const NifmInternetConnectionType_Ethernet: NifmInternetConnectionType = 2;

pub type NifmInternetConnectionStatus = u32;
/// Unknown internet connection status 1.
pub const NifmInternetConnectionStatus_ConnectingUnknown1: NifmInternetConnectionStatus = 0;
/// Unknown internet connection status 2.
pub const NifmInternetConnectionStatus_ConnectingUnknown2: NifmInternetConnectionStatus = 1;
/// Unknown internet connection status 3 (conntest?).
pub const NifmInternetConnectionStatus_ConnectingUnknown3: NifmInternetConnectionStatus = 2;
/// Unknown internet connection status 4.
pub const NifmInternetConnectionStatus_ConnectingUnknown4: NifmInternetConnectionStatus = 3;
/// Internet is connected.
pub const NifmInternetConnectionStatus_Connected: NifmInternetConnectionStatus = 4;

pub type NifmRequestState = u32;
/// Error.
pub const NifmRequestState_Invalid: NifmRequestState = 0;
/// Not yet submitted or error.
pub const NifmRequestState_Unknown1: NifmRequestState = 1;
/// OnHold
pub const NifmRequestState_OnHold: NifmRequestState = 2;
/// Available
pub const NifmRequestState_Available: NifmRequestState = 3;
/// Unknown
pub const NifmRequestState_Unknown4: NifmRequestState = 4;
/// Unknown
pub const NifmRequestState_Unknown5: NifmRequestState = 5;

/// Request
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NifmRequest {
    /// IRequest
    pub s: Service,
    /// First Event from cmd GetSystemEventReadableHandles, autoclear=true. Signaled when the RequestState changes.
    pub event_request_state: Event,
    /// Second Event from cmd GetSystemEventReadableHandles.
    pub event1: Event,
    /// **Ref:** NifmRequestState from the GetRequestState cmd.
    pub request_state: NifmRequestState,
    /// Result from the GetResult cmd.
    pub res: Result,
}

/// ClientId
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NifmClientId {
    /// ClientId
    pub id: u32,
}

/// IpV4Address
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NifmIpV4Address {
    /// IPv4 address, aka struct in_addr.
    pub addr: [u8; 4],
}

/// IpAddressSetting
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NifmIpAddressSetting {
    /// Whether this setting is automatic. Ignored by `nifmGetCurrentIpConfigInfo`.
    pub is_automatic: u8,
    /// Current address.
    pub current_addr: NifmIpV4Address,
    /// Subnet Mask.
    pub subnet_mask: NifmIpV4Address,
    /// Gateway.
    pub gateway: NifmIpV4Address,
}

/// DnsSetting
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NifmDnsSetting {
    /// Whether this setting is automatic. Ignored by `nifmGetCurrentIpConfigInfo`.
    pub is_automatic: u8,
    /// Primary DNS server.
    pub primary_dns_server: NifmIpV4Address,
    /// Secondary DNS server.
    pub secondary_dns_server: NifmIpV4Address,
}

/// ProxySetting
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NifmProxySetting {
    /// Enables using the proxy when set.
    pub enabled: u8,
    /// Padding
    pub pad: u8,
    /// Port
    pub port: u16,
    /// Server string, NUL-terminated.
    pub server: [c_char; 100],
    /// Enables auto-authentication when set, which uses the following two strings.
    pub auto_auth_enabled: u8,
    /// User string, NUL-terminated.
    pub user: [c_char; 32],
    /// Password string, NUL-terminated.
    pub password: [c_char; 32],
    /// Padding
    pub pad2: u8,
}

/// IpSettingData
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NifmIpSettingData {
    /// **Ref:** NifmIpAddressSetting
    pub ip_address_setting: NifmIpAddressSetting,
    /// **Ref:** NifmDnsSetting
    pub dns_setting: NifmDnsSetting,
    /// **Ref:** NifmProxySetting
    pub proxy_setting: NifmProxySetting,
    /// MTU
    pub mtu: u16,
}

/// WirelessSettingData
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NifmWirelessSettingData {
    /// NifmSfWirelessSettingData::ssid_len
    pub ssid_len: u8,
    /// NifmSfWirelessSettingData::ssid
    pub ssid: [c_char; 33],
    /// NifmSfWirelessSettingData::unk_x21
    pub unk_x22: u8,
    /// Padding
    pub pad: u8,
    /// NifmSfWirelessSettingData::unk_x22
    pub unk_x24: u32,
    /// NifmSfWirelessSettingData::unk_x23
    pub unk_x28: u32,
    /// NifmSfWirelessSettingData::passphrase
    pub passphrase: [u8; 65],
    /// Padding
    pub pad2: [u8; 3],
}

/// SfWirelessSettingData
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NifmSfWirelessSettingData {
    /// SSID length.
    pub ssid_len: u8,
    /// SSID string.
    pub ssid: [c_char; 32],
    /// Unknown
    pub unk_x21: u8,
    /// Unknown
    pub unk_x22: u8,
    /// Unknown
    pub unk_x23: u8,
    /// Passphrase
    pub passphrase: [u8; 65],
}

/// SfNetworkProfileData. Converted to/from `NifmNetworkProfileData`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NifmSfNetworkProfileData {
    /// **Ref:** NifmIpSettingData
    pub ip_setting_data: NifmIpSettingData,
    /// Uuid
    pub uuid: Uuid,
    /// NUL-terminated Network Name string.
    pub network_name: [c_char; 64],
    /// Unknown
    pub unk_x112: u8,
    /// Unknown
    pub unk_x113: u8,
    /// Unknown
    pub unk_x114: u8,
    /// Unknown
    pub unk_x115: u8,
    /// **Ref:** NifmSfWirelessSettingData
    pub wireless_setting_data: NifmSfWirelessSettingData,
    /// Padding
    pub pad: u8,
}

/// NetworkProfileData. Converted from/to `NifmSfNetworkProfileData`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NifmNetworkProfileData {
    /// NifmSfNetworkProfileData::uuid
    pub uuid: Uuid,
    /// NifmSfNetworkProfileData::network_name
    pub network_name: [c_char; 64],
    /// NifmSfNetworkProfileData::unk_x112
    pub unk_x50: u32,
    /// NifmSfNetworkProfileData::unk_x113
    pub unk_x54: u32,
    /// NifmSfNetworkProfileData::unk_x114
    pub unk_x58: u8,
    /// NifmSfNetworkProfileData::unk_x115
    pub unk_x59: u8,
    /// Padding
    pub pad: [u8; 2],
    /// **Ref:** NifmWirelessSettingData
    pub wireless_setting_data: NifmWirelessSettingData,
    /// **Ref:** NifmIpSettingData
    pub ip_setting_data: NifmIpSettingData,
}

impl_zeroed_default!(NifmRequest, NifmClientId, NifmIpV4Address, NifmIpAddressSetting, NifmDnsSetting, NifmProxySetting, NifmIpSettingData, NifmWirelessSettingData, NifmSfWirelessSettingData, NifmSfNetworkProfileData, NifmNetworkProfileData);

unsafe extern "C" {
    /// Initialize nifm. This is used automatically by gethostid().
    pub fn nifmInitialize(service_type: NifmServiceType) -> Result;

    /// Exit nifm. This is used automatically by gethostid().
    pub fn nifmExit();

    /// Gets the Service object for the actual nifm:* service session.
    pub fn nifmGetServiceSession_StaticService() -> *mut Service;

    /// Gets the Service object for IGeneralService.
    pub fn nifmGetServiceSession_GeneralService() -> *mut Service;

    /// GetClientId
    pub fn nifmGetClientId() -> NifmClientId;

    /// CreateRequest
    ///
    /// # Arguments
    /// - `r` (out): `NifmRequest`
    /// - `autoclear` (in): Event autoclear to use for NifmRequest::event1, a default of true can be used for this.
    pub fn nifmCreateRequest(r: *mut NifmRequest, autoclear: bool) -> Result;

    /// GetCurrentNetworkProfile
    ///
    /// # Arguments
    /// - `profile` (out): `NifmNetworkProfileData`
    pub fn nifmGetCurrentNetworkProfile(profile: *mut NifmNetworkProfileData) -> Result;

    /// GetNetworkProfile
    ///
    /// # Arguments
    /// - `uuid` (in): Uuid
    /// - `profile` (out): `NifmNetworkProfileData`
    pub fn nifmGetNetworkProfile(uuid: Uuid, profile: *mut NifmNetworkProfileData) -> Result;

    /// SetNetworkProfile
    ///
    /// # Arguments
    /// - `profile` (in): `NifmNetworkProfileData`
    /// - `uuid` (out): Uuid
    ///
    /// **Note:** Only available with ::NifmServiceType_Admin.
    pub fn nifmSetNetworkProfile(profile: *const NifmNetworkProfileData, uuid: *mut Uuid) -> Result;

    /// GetCurrentIpAddress
    ///
    /// # Arguments
    /// - `out` (out): IPv4 address (struct in_addr).
    pub fn nifmGetCurrentIpAddress(out: *mut u32) -> Result;

    /// GetCurrentIpConfigInfo
    ///
    /// # Arguments
    /// - `current_addr` (out): Same as `nifmGetCurrentIpAddress` output.
    /// - `subnet_mask` (out): Subnet Mask (struct in_addr).
    /// - `gateway` (out): Gateway (struct in_addr).
    /// - `primary_dns_server` (out): Primary DNS server IPv4 address (struct in_addr).
    /// - `secondary_dns_server` (out): Secondary DNS server IPv4 address (struct in_addr).
    pub fn nifmGetCurrentIpConfigInfo(current_addr: *mut u32, subnet_mask: *mut u32, gateway: *mut u32, primary_dns_server: *mut u32, secondary_dns_server: *mut u32) -> Result;

    /// **Note:** Works only if called from nifm:a or nifm:s.
    pub fn nifmSetWirelessCommunicationEnabled(enable: bool) -> Result;

    pub fn nifmIsWirelessCommunicationEnabled(out: *mut bool) -> Result;

    /// # Arguments
    /// - `wifiStrength`: Strength of the Wi-Fi signal in number of bars from 0 to 3.
    ///
    /// **Note:** Will fail with 0xd46ed if Internet is neither connecting or connected (airplane mode or no known network in reach).
    pub fn nifmGetInternetConnectionStatus(connectionType: *mut NifmInternetConnectionType, wifiStrength: *mut u32, connectionStatus: *mut NifmInternetConnectionStatus) -> Result;

    pub fn nifmIsEthernetCommunicationEnabled(out: *mut bool) -> Result;

    /// IsAnyInternetRequestAccepted
    ///
    /// # Arguments
    /// - `id` (in): `NifmClientId`
    pub fn nifmIsAnyInternetRequestAccepted(id: NifmClientId) -> bool;

    pub fn nifmIsAnyForegroundRequestAccepted(out: *mut bool) -> Result;

    pub fn nifmPutToSleep() -> Result;

    pub fn nifmWakeUp() -> Result;

    /// SetWowlDelayedWakeTime
    ///
    /// # Arguments
    /// - `val` (in): Input value.
    ///
    /// **Note:** Only available with ::NifmServiceType_System or ::NifmServiceType_Admin.
    ///
    /// **Note:** Only available on [9.0.0+].
    pub fn nifmSetWowlDelayedWakeTime(val: s32) -> Result;

    /// Close a `NifmRequest`.
    ///
    /// # Arguments
    /// - `r`: `NifmRequest`
    pub fn nifmRequestClose(r: *mut NifmRequest);

    /// GetRequestState
    ///
    /// # Arguments
    /// - `r`: `NifmRequest`
    /// - `out` (out): `NifmRequestState`
    pub fn nifmGetRequestState(r: *mut NifmRequest, out: *mut NifmRequestState) -> Result;

    /// GetResult
    ///
    /// # Arguments
    /// - `r`: `NifmRequest`
    pub fn nifmGetResult(r: *mut NifmRequest) -> Result;

    /// Cancel
    ///
    /// # Arguments
    /// - `r`: `NifmRequest`
    pub fn nifmRequestCancel(r: *mut NifmRequest) -> Result;

    /// Submit
    ///
    /// # Arguments
    /// - `r`: `NifmRequest`
    pub fn nifmRequestSubmit(r: *mut NifmRequest) -> Result;

    /// SubmitAndWait
    ///
    /// # Arguments
    /// - `r`: `NifmRequest`
    pub fn nifmRequestSubmitAndWait(r: *mut NifmRequest) -> Result;

    /// GetAppletInfo
    ///
    /// # Arguments
    /// - `r`: `NifmRequest`
    /// - `theme_color` (in): ThemeColor
    /// - `buffer` (out): Output buffer for storage data.
    /// - `size` (in): Output buffer size.
    /// - `applet_id` (out): `AppletId`
    /// - `mode` (out): `LibAppletMode`
    /// - `out_size` (out): Total data size written to the output buffer.
    ///
    /// **Note:** This is used by `nifmLaHandleNetworkRequestResult`.
    pub fn nifmRequestGetAppletInfo(r: *mut NifmRequest, theme_color: u32, buffer: *mut c_void, size: usize, applet_id: *mut u32, mode: *mut u32, out_size: *mut u32) -> Result;

    /// SetKeptInSleep
    ///
    /// # Arguments
    /// - `r`: `NifmRequest`
    /// - `flag` (in): Flag
    ///
    /// **Note:** Only available on [3.0.0+].
    ///
    /// **Note:** ::NifmRequestState must be ::NifmRequestState_Unknown1.
    pub fn nifmRequestSetKeptInSleep(r: *mut NifmRequest, flag: bool) -> Result;

    /// RegisterSocketDescriptor. Only 1 socket can be registered at a time with a NifmRequest. Do not use directly, use `socketNifmRequestRegisterSocketDescriptor` instead.
    ///
    /// # Arguments
    /// - `r`: `NifmRequest`
    /// - `sockfd` (in): Socket fd
    ///
    /// **Note:** Only available on [3.0.0+].
    ///
    /// **Note:** ::NifmRequestState must be ::NifmRequestState_Available.
    pub fn nifmRequestRegisterSocketDescriptor(r: *mut NifmRequest, sockfd: c_int) -> Result;

    /// UnregisterSocketDescriptor. Do not use directly, use `socketNifmRequestUnregisterSocketDescriptor` instead.
    ///
    /// # Arguments
    /// - `r`: `NifmRequest`
    /// - `sockfd` (in): Socket fd, must match the fd previously registered with `nifmRequestRegisterSocketDescriptor`.
    ///
    /// **Note:** Only available on [3.0.0+].
    ///
    /// **Note:** ::NifmRequestState must be ::NifmRequestState_Available.
    pub fn nifmRequestUnregisterSocketDescriptor(r: *mut NifmRequest, sockfd: c_int) -> Result;
}
