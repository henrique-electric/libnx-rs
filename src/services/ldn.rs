//! `switch/services/ldn.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// System priority for __nx_ldn_priority.
pub const LDN_PRIORITY_SYSTEM: u32 = 0x38;
/// User priority for __nx_ldn_priority.
pub const LDN_PRIORITY_USER: u32 = 0x5a;

pub type LdnServiceType = u32;
/// Initializes ldn:u.
pub const LdnServiceType_User: LdnServiceType = 0;
/// Initializes ldn:s.
pub const LdnServiceType_System: LdnServiceType = 1;

/// State loaded by `ldnmGetStateForMonitor` / `ldnGetState`.
pub type LdnState = u32;
/// None
pub const LdnState_None: LdnState = 0;
/// Initialized
pub const LdnState_Initialized: LdnState = 1;
/// AccessPoint (`ldnOpenAccessPoint`)
pub const LdnState_AccessPoint: LdnState = 2;
/// AccessPointCreated (`ldnCreateNetwork` / `ldnCreateNetworkPrivate`)
pub const LdnState_AccessPointCreated: LdnState = 3;
/// Station (`ldnOpenStation`)
pub const LdnState_Station: LdnState = 4;
/// StationConnected (`ldnConnect` / `ldnConnectPrivate`)
pub const LdnState_StationConnected: LdnState = 5;
/// Error
pub const LdnState_Error: LdnState = 6;

/// DisconnectReason loaded by `ldnGetDisconnectReason`.
pub type LdnDisconnectReason = u32;
/// None
pub const LdnDisconnectReason_None: LdnDisconnectReason = 0;
/// DisconnectedByUser
pub const LdnDisconnectReason_DisconnectedByUser: LdnDisconnectReason = 1;
/// DisconnectedBySystem
pub const LdnDisconnectReason_DisconnectedBySystem: LdnDisconnectReason = 2;
/// DestroyedByUser
pub const LdnDisconnectReason_DestroyedByUser: LdnDisconnectReason = 3;
/// DestroyedBySystem
pub const LdnDisconnectReason_DestroyedBySystem: LdnDisconnectReason = 4;
/// Rejected
pub const LdnDisconnectReason_Rejected: LdnDisconnectReason = 5;
/// SignalLost
pub const LdnDisconnectReason_SignalLost: LdnDisconnectReason = 6;

/// ScanFilterFlag
pub type LdnScanFilterFlag = u32;
/// Enables using LdnScanFilter::network_id LdnIntentId::local_communication_id.
pub const LdnScanFilterFlag_LocalCommunicationId: LdnScanFilterFlag = BIT(0);
/// Enables using LdnScanFilter::network_id session_id.
pub const LdnScanFilterFlag_SessionId: LdnScanFilterFlag = BIT(1);
/// Enables using LdnScanFilter::network_type.
pub const LdnScanFilterFlag_NetworkType: LdnScanFilterFlag = BIT(2);
/// Enables using LdnScanFilter::bssid. Only available with `ldnScanPrivate`.
pub const LdnScanFilterFlag_Bssid: LdnScanFilterFlag = BIT(3);
/// Enables using LdnScanFilter::ssid.
pub const LdnScanFilterFlag_Ssid: LdnScanFilterFlag = BIT(4);
/// Enables using LdnScanFilter::network_id LdnIntentId::scene_id.
pub const LdnScanFilterFlag_SceneId: LdnScanFilterFlag = BIT(5);
/// Enables using LdnScanFilter::network_id intent_id.
pub const LdnScanFilterFlag_IntentId: LdnScanFilterFlag = LdnScanFilterFlag_LocalCommunicationId | LdnScanFilterFlag_SceneId;
/// Enables using LdnScanFilter::network_id.
pub const LdnScanFilterFlag_NetworkId: LdnScanFilterFlag = LdnScanFilterFlag_IntentId | LdnScanFilterFlag_SessionId;

/// AcceptPolicy
pub type LdnAcceptPolicy = u32;
/// AlwaysAccept
pub const LdnAcceptPolicy_AlwaysAccept: LdnAcceptPolicy = 0;
/// AlwaysReject
pub const LdnAcceptPolicy_AlwaysReject: LdnAcceptPolicy = 1;
/// BlackList, addresses in the list (`ldnAddAcceptFilterEntry`) are not allowed.
pub const LdnAcceptPolicy_BlackList: LdnAcceptPolicy = 2;
/// WhiteList, only addresses in the list (`ldnAddAcceptFilterEntry`) are allowed.
pub const LdnAcceptPolicy_WhiteList: LdnAcceptPolicy = 3;

/// SecurityMode. LdnSecurityMode_Product can be used as the default value. The ldn cmds which use an input SecurityMode normally overwrite the SecurityMode.
pub type LdnSecurityMode = u32;
/// Any
pub const LdnSecurityMode_Any: LdnSecurityMode = 0;
/// Product
pub const LdnSecurityMode_Product: LdnSecurityMode = 1;
/// Debug
pub const LdnSecurityMode_Debug: LdnSecurityMode = 2;
/// SystemDebug
pub const LdnSecurityMode_SystemDebug: LdnSecurityMode = 3;

/// OperationMode
pub type LdnOperationMode = u32;
/// Stable
pub const LdnOperationMode_Stable: LdnOperationMode = 0;
/// HighSpeed
pub const LdnOperationMode_HighSpeed: LdnOperationMode = 1;

/// WirelessControllerRestriction
pub type LdnWirelessControllerRestriction = u32;
/// Disabled
pub const LdnWirelessControllerRestriction_Disabled: LdnWirelessControllerRestriction = 0;
/// Enabled
pub const LdnWirelessControllerRestriction_Enabled: LdnWirelessControllerRestriction = 1;

/// Protocol
pub type LdnProtocol = u32;
/// NX (default)
pub const LdnProtocol_NX: LdnProtocol = 1;
/// (NXAndOunce?)
pub const LdnProtocol_Unknown3: LdnProtocol = 3;

/// Ipv4Address. This is essentially the same as struct in_addr - hence this can be used with standard sockets (byteswap required).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LdnIpv4Address {
    /// Address
    pub addr: u32,
}

/// SubnetMask. This is essentially the same as struct in_addr - hence this can be used with standard sockets (byteswap required).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LdnSubnetMask {
    /// Mask
    pub mask: u32,
}

/// MacAddress
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LdnMacAddress {
    /// Address
    pub addr: [u8; 6],
}

/// Ssid
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LdnSsid {
    /// Length excluding NUL-terminator, must be 0x1-0x20.
    pub len: u8,
    /// SSID string including NUL-terminator, str\[len_field\] must be 0. The chars in this string must be be in the range of 0x20-0x7F, for when the Ssid is converted to a string (otherwise the byte written to the string will be 0).
    pub str: [c_char; 33],
}

/// NodeLatestUpdate
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LdnNodeLatestUpdate {
    /// StateChange. The field in state is reset to zero by `ldnGetNetworkInfoLatestUpdate` after loading it.
    pub state_change: u8,
    /// Reserved
    pub reserved: [u8; 7],
}

/// AddressEntry
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LdnAddressEntry {
    /// **Ref:** LdnIpv4Address
    pub ip_addr: LdnIpv4Address,
    /// **Ref:** LdnMacAddress
    pub mac_addr: LdnMacAddress,
    /// Reserved
    pub reserved: [u8; 2],
}

/// NodeInfo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LdnNodeInfo {
    /// **Ref:** LdnIpv4Address
    pub ip_addr: LdnIpv4Address,
    /// **Ref:** LdnMacAddress
    pub mac_addr: LdnMacAddress,
    /// NodeId
    pub node_id: s8,
    /// IsConnected flag
    pub is_connected: u8,
    /// LdnUserConfig::user_name
    pub user_name: [c_char; 33],
    /// [19.0.0+] Platform? (0 = NX, 1 = Ounce)
    pub platform: u8,
    /// LocalCommunicationVersion
    pub local_communication_version: s16,
    /// Reserved
    pub reserved_x30: [u8; 16],
}

/// UserConfig. The input struct is copied to a tmp struct, which is then used with the cmd.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LdnUserConfig {
    /// UserName (NUL-terminated string for the user name)
    pub user_name: [c_char; 33],
    /// Reserved
    pub reserved: [u8; 15],
}

/// IntentId
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LdnIntentId {
    /// LocalCommunicationId. `ldnCreateNetwork`, `ldnCreateNetworkPrivate`, `ldnConnect`, `ldnConnectPrivate` (also `LdnScanFilter` when enabled): When -1, this is overwritten with the first LocalCommunicationId from the user-process NACP, if loading fails value 0 is written instead. Otherwise when not -1, if NACP loading is successful, this field must match one of the LocalCommunicationIds from there.
    pub local_communication_id: s64,
    /// Reserved
    pub reserved_x8: [u8; 2],
    /// SceneId. Arbitrary user data, this can be used for filtering with `LdnScanFilter` (::LdnScanFilterFlag_SceneId) for example.
    pub scene_id: u16,
    /// Reserved
    pub reserved_xC: [u8; 4],
}

/// SessionId
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LdnSessionId {
    /// Random. Used to generate/overwrite the ssid when needed.
    pub random: [u8; 16],
}

/// NetworkId
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LdnNetworkId {
    /// IntentId
    pub intent_id: LdnIntentId,
    /// SessionId
    pub session_id: LdnSessionId,
}

/// CommonNetworkInfo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LdnCommonNetworkInfo {
    /// **Ref:** LdnMacAddress
    pub bssid: LdnMacAddress,
    /// **Ref:** LdnSsid
    pub ssid: LdnSsid,
    /// Channel
    pub channel: s16,
    /// LinkLevel
    pub link_level: s8,
    /// NetworkType
    pub network_type: u8,
    /// Reserved
    pub reserved: [u8; 4],
}

/// NetworkInfo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LdnNetworkInfo {
    /// NetworkId
    pub network_id: LdnNetworkId,
    /// CommonNetworkInfo
    pub common: LdnCommonNetworkInfo,
    /// LdnSecurityParameter::server_random
    pub server_random: [u8; 16],
    /// LdnSecurityConfig::security_mode
    pub security_mode: u16,
    /// **Ref:** LdnAcceptPolicy
    pub station_accept_policy: u8,
    /// Version
    pub version: u8,
    /// Reserved
    pub reserved_x14: [u8; 2],
    /// NodeCountMax
    pub node_count_max: s8,
    /// NodeCount, number of set entries in nodes.
    pub node_count: u8,
    /// Array of `LdnNodeInfo`, starting with the AccessPoint node.
    pub nodes: [LdnNodeInfo; 8],
    /// Reserved
    pub reserved_x218: [u8; 2],
    /// AdvertiseDataSize (`ldnSetAdvertiseData`)
    pub advertise_data_size: u16,
    /// AdvertiseData (`ldnSetAdvertiseData`)
    pub advertise_data: [u8; 384],
    /// Reserved
    pub reserved_x39C: [u8; 140],
    /// Reserved
    pub reserved_x428: u64,
}

/// ScanFilter. `ldnScan` / `ldnScanPrivate` will only return the `LdnNetworkInfo` when validating the `LdnNetworkInfo` with LdnScanFilter was sucessful, with the fields enabled by `LdnScanFilterFlag`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LdnScanFilter {
    /// NetworkId
    pub network_id: LdnNetworkId,
    /// NetworkType (::LdnScanFilterFlag_NetworkType)
    pub network_type: u32,
    /// Bssid (::LdnScanFilterFlag_Bssid)
    pub bssid: LdnMacAddress,
    /// Ssid (::LdnScanFilterFlag_Ssid)
    pub ssid: LdnSsid,
    /// Reserved
    pub reserved: [u8; 16],
    /// Bitmask for `LdnScanFilterFlag`.
    pub flags: u32,
}

/// SecurityConfig
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LdnSecurityConfig {
    /// **Ref:** LdnSecurityMode Overwritten by `ldnCreateNetwork`, `ldnCreateNetworkPrivate`, `ldnConnect`, `ldnConnectPrivate`.
    pub security_mode: u16,
    /// PassphraseSize. Must be 0x10-0x40.
    pub passphrase_size: u16,
    /// Passphrase, used with key derivation.
    pub passphrase: [u8; 64],
}

/// SecurityParameter. The struct used by `ldnCreateNetwork` internally is randomly-generated.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LdnSecurityParameter {
    /// ServerRandom, used with the same key derivation as `LdnSecurityConfig`.
    pub server_random: [u8; 16],
    /// SessionId
    pub session_id: LdnSessionId,
}

/// NetworkConfig. The input struct is copied to a tmp struct, which is then used with the cmd (`ldnCreateNetwork`, `ldnCreateNetworkPrivate`, `ldnConnectPrivate`).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LdnNetworkConfig {
    /// IntentId
    pub intent_id: LdnIntentId,
    /// LdnCommonNetworkInfo::channel. Channel, can be zero. Overwritten internally by `ldnCreateNetwork`.
    pub channel: s16,
    /// LdnNetworkInfo::node_count_max. `ldnCreateNetwork` / `ldnCreateNetworkPrivate`: Must be 0x1-0x8.
    pub node_count_max: s8,
    /// Cleared to zero for the tmp struct.
    pub reserved_x13: u8,
    /// LdnNodeInfo::local_communication_version, for the first entry in LdnNetworkInfo::nodes. Must not be negative.
    pub local_communication_version: s16,
    /// Cleared to zero for the tmp struct.
    pub reserved_x16: [u8; 10],
}

/// ActionFrameSettings
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LdnActionFrameSettings {
    /// LocalCommunicationId (Same handling as LdnIntentId::local_communication_id)
    pub local_communication_id: s64,
    /// Reserved
    pub reserved: [u8; 52],
    /// **Ref:** LdnSecurityMode (Must be ::LdnSecurityMode_Product or ::LdnSecurityMode_Debug, internally this is overriden)
    pub security_mode: u16,
    /// PassphraseSize (Must be 0x10-0x40)
    pub passphrase_size: u16,
    /// Passphrase
    pub passphrase: [u8; 64],
}

impl_zeroed_default!(LdnIpv4Address, LdnSubnetMask, LdnMacAddress, LdnSsid, LdnNodeLatestUpdate, LdnAddressEntry, LdnNodeInfo, LdnUserConfig, LdnIntentId, LdnSessionId, LdnNetworkId, LdnCommonNetworkInfo, LdnNetworkInfo, LdnScanFilter, LdnSecurityConfig, LdnSecurityParameter, LdnNetworkConfig, LdnActionFrameSettings);

unsafe extern "C" {
    /// Initialize ldn:m.
    pub fn ldnmInitialize() -> Result;

    /// Exit ldn:m.
    pub fn ldnmExit();

    /// Gets the Service object for IMonitorService.
    pub fn ldnmGetServiceSession_MonitorService() -> *mut Service;

    /// GetState
    ///
    /// # Arguments
    /// - `out` (out): `LdnState`
    pub fn ldnmGetState(out: *mut LdnState) -> Result;

    /// GetNetworkInfo
    ///
    /// # Arguments
    /// - `out` (out): `LdnNetworkInfo`
    pub fn ldnmGetNetworkInfo(out: *mut LdnNetworkInfo) -> Result;

    /// GetIpv4Address
    ///
    /// # Arguments
    /// - `addr` (out): `LdnIpv4Address`
    /// - `mask` (out): `LdnSubnetMask`
    pub fn ldnmGetIpv4Address(addr: *mut LdnIpv4Address, mask: *mut LdnSubnetMask) -> Result;

    /// GetSecurityParameter
    ///
    /// # Arguments
    /// - `out` (out): `LdnSecurityParameter`
    ///
    /// **Note:** Not exposed by official sw.
    pub fn ldnmGetSecurityParameter(out: *mut LdnSecurityParameter) -> Result;

    /// GetNetworkConfig
    ///
    /// # Arguments
    /// - `out` (out): `LdnNetworkConfig`
    ///
    /// **Note:** Not exposed by official sw.
    pub fn ldnmGetNetworkConfig(out: *mut LdnNetworkConfig) -> Result;

    /// Initialize ldn.
    /// The priority is only used with ::LdnServiceType_System on [19.0.0+]. To optionally set this, define "s32 __nx_ldn_priority". The priority must be `LDN_PRIORITY_SYSTEM` (default) or `LDN_PRIORITY_USER`.
    pub fn ldnInitialize(service_type: LdnServiceType) -> Result;

    /// Exit ldn.
    pub fn ldnExit();

    /// Gets the Service object for IUserLocalCommunicationService/ISystemLocalCommunicationService.
    pub fn ldnGetServiceSession_LocalCommunicationService() -> *mut Service;

    /// Gets the Service object for IClientProcessMonitor, only valid with [18.0.0+].
    pub fn ldnGetServiceSession_IClientProcessMonitor() -> *mut Service;

    /// GetState
    ///
    /// # Arguments
    /// - `out` (out): `LdnState`
    pub fn ldnGetState(out: *mut LdnState) -> Result;

    /// GetNetworkInfo
    ///
    /// # Arguments
    /// - `out` (out): `LdnNetworkInfo`
    pub fn ldnGetNetworkInfo(out: *mut LdnNetworkInfo) -> Result;

    /// GetIpv4Address
    ///
    /// # Arguments
    /// - `addr` (out): `LdnIpv4Address`
    /// - `mask` (out): `LdnSubnetMask`
    pub fn ldnGetIpv4Address(addr: *mut LdnIpv4Address, mask: *mut LdnSubnetMask) -> Result;

    /// GetDisconnectReason
    ///
    /// # Arguments
    /// - `out` (out): `LdnDisconnectReason`
    pub fn ldnGetDisconnectReason(out: *mut LdnDisconnectReason) -> Result;

    /// GetSecurityParameter
    ///
    /// # Arguments
    /// - `out` (out): `LdnSecurityParameter`
    pub fn ldnGetSecurityParameter(out: *mut LdnSecurityParameter) -> Result;

    /// GetNetworkConfig
    ///
    /// # Arguments
    /// - `out` (out): `LdnNetworkConfig`
    pub fn ldnGetNetworkConfig(out: *mut LdnNetworkConfig) -> Result;

    /// GetStateChangeEvent
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=true.
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    ///
    /// **Note:** This is signaled when the data returned by `ldnGetNetworkInfo` / `ldnGetNetworkInfoLatestUpdate` is updated.
    pub fn ldnGetStateChangeEvent(out_event: *mut Event) -> Result;

    /// GetNetworkInfoAndHistory
    ///
    /// # Arguments
    /// - `network_info` (out): `LdnNetworkInfo`
    /// - `nodes` (out): Output array of `LdnNodeLatestUpdate`.
    /// - `count` (in): Size of the nodes array in entries, must be 8.
    pub fn ldnGetNetworkInfoAndHistory(network_info: *mut LdnNetworkInfo, nodes: *mut LdnNodeLatestUpdate, count: s32) -> Result;

    /// Scan
    ///
    /// # Arguments
    /// - `channel` (in): Channel, value 0 can be used for this.
    /// - `filter` (in): `LdnScanFilter`
    /// - `network_info` (out): Output array of `LdnNetworkInfo`.
    /// - `count` (in): Size of the network_info array in entries. Must be at least 1, this is clamped to a maximum of 0x18 internally.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** `LdnState` must be ::LdnState_AccessPointCreated, ::LdnState_Station, or ::LdnState_StationConnected.
    ///
    /// **Note:** This is the same as `ldnScanPrivate` (minus the masking for LdnScanFilter::flags), except this has the same channel-override functionality as `ldnCreateNetwork`.
    pub fn ldnScan(channel: s32, filter: *const LdnScanFilter, network_info: *mut LdnNetworkInfo, count: s32, total_out: *mut s32) -> Result;

    /// ScanPrivate
    ///
    /// # Arguments
    /// - `channel` (in): Channel, value 0 can be used for this.
    /// - `filter` (in): `LdnScanFilter`
    /// - `network_info` (out): Output array of `LdnNetworkInfo`.
    /// - `count` (in): Size of the network_info array in entries. Must be at least 1, this is clamped to a maximum of 0x18 internally.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** `LdnState` must be ::LdnState_AccessPointCreated, ::LdnState_Station, or ::LdnState_StationConnected.
    ///
    /// **Note:** See `ldnScan`.
    pub fn ldnScanPrivate(channel: s32, filter: *const LdnScanFilter, network_info: *mut LdnNetworkInfo, count: s32, total_out: *mut s32) -> Result;

    /// SetWirelessControllerRestriction
    ///
    /// # Arguments
    /// - `restriction` (in): `LdnWirelessControllerRestriction`
    ///
    /// **Note:** Only available on [5.0.0+].
    ///
    /// **Note:** `LdnState` must be ::LdnState_Initialized.
    pub fn ldnSetWirelessControllerRestriction(restriction: LdnWirelessControllerRestriction) -> Result;

    /// SetProtocol
    ///
    /// # Arguments
    /// - `protocol` (in): `LdnProtocol`
    ///
    /// **Note:** This is only usable with [20.0.0+] (with [18.0.0-19-0.1] this is available but not usable).
    ///
    /// **Note:** `LdnState` must be ::LdnState_Initialized. If a non-default Protocol is wanted, use this after `ldnInitialize`.
    ///
    /// **Note:** This is used by `ldnInitialize` with LdnProtocol_NX on [20.0.0+].
    pub fn ldnSetProtocol(protocol: LdnProtocol) -> Result;

    /// OpenAccessPoint
    ///
    /// **Note:** `LdnState` must be ::LdnState_Initialized, this eventually sets the State to ::LdnState_AccessPoint.
    pub fn ldnOpenAccessPoint() -> Result;

    /// CloseAccessPoint
    ///
    /// **Note:** `LdnState` must be ::LdnState_AccessPoint or ::LdnState_AccessPointCreated, this eventually sets the State to ::LdnState_Initialized.
    ///
    /// **Note:** Used automatically internally by `ldnExit` if needed.
    pub fn ldnCloseAccessPoint() -> Result;

    /// CreateNetwork
    ///
    /// # Arguments
    /// - `sec_config` (in): `LdnSecurityConfig`
    /// - `user_config` (in): `LdnUserConfig`
    /// - `network_config` (in): `LdnNetworkConfig`
    ///
    /// **Note:** `LdnState` must be ::LdnState_AccessPoint, this eventually sets the State to ::LdnState_AccessPointCreated.
    pub fn ldnCreateNetwork(sec_config: *const LdnSecurityConfig, user_config: *const LdnUserConfig, network_config: *const LdnNetworkConfig) -> Result;

    /// CreateNetworkPrivate
    ///
    /// # Arguments
    /// - `sec_config` (in): `LdnSecurityConfig`
    /// - `sec_param` (in): `LdnSecurityParameter`
    /// - `user_config` (in): `LdnUserConfig`
    /// - `network_config` (in): `LdnNetworkConfig`
    /// - `addrs` (in): Input array of `LdnAddressEntry`. This can be NULL.
    /// - `count` (in): Size of the addrs array in entries. This must be &lt;=8. This can be 0, in which case the network will be non-Private like `ldnCreateNetwork`.
    ///
    /// **Note:** `LdnState` must be ::LdnState_AccessPoint, this eventually sets the State to ::LdnState_AccessPointCreated.
    ///
    /// **Note:** This is the same as `ldnCreateNetwork` besides the additional user-specified params, and with this cmd LdnNetworkConfig::channel is not overwritten (unlike `ldnCreateNetwork`).
    pub fn ldnCreateNetworkPrivate(sec_config: *const LdnSecurityConfig, sec_param: *const LdnSecurityParameter, user_config: *const LdnUserConfig, network_config: *const LdnNetworkConfig, addrs: *const LdnAddressEntry, count: s32) -> Result;

    /// DestroyNetwork
    ///
    /// **Note:** `LdnState` must be ::LdnState_AccessPointCreated, this eventually sets the State to ::LdnState_AccessPoint.
    pub fn ldnDestroyNetwork() -> Result;

    /// Reject
    ///
    /// # Arguments
    /// - `addr` (in): `LdnIpv4Address`
    ///
    /// **Note:** `LdnState` must be ::LdnState_AccessPointCreated.
    pub fn ldnReject(addr: LdnIpv4Address) -> Result;

    /// SetAdvertiseData
    ///
    /// # Arguments
    /// - `buffer` (in): Input buffer containing arbitrary user data.
    /// - `size` (in): Input buffer size, must be &lt;=0x180.
    ///
    /// **Note:** An empty buffer (buffer=NULL/size=0) can be used to reset the AdvertiseData size in state to zero.
    ///
    /// **Note:** `LdnState` must be ::LdnState_AccessPoint or ::LdnState_AccessPointCreated.
    pub fn ldnSetAdvertiseData(buffer: *const c_void, size: usize) -> Result;

    /// SetStationAcceptPolicy
    ///
    /// # Arguments
    /// - `policy` (in): `LdnAcceptPolicy`
    ///
    /// **Note:** `LdnState` must be ::LdnState_AccessPoint or ::LdnState_AccessPointCreated.
    pub fn ldnSetStationAcceptPolicy(policy: LdnAcceptPolicy) -> Result;

    /// AddAcceptFilterEntry
    ///
    /// # Arguments
    /// - `addr` (in): `LdnMacAddress`. If you want, you can also pass LdnNodeInfo::mac_addr for this.
    ///
    /// **Note:** `LdnState` must be ::LdnState_AccessPoint or ::LdnState_AccessPointCreated.
    ///
    /// **Note:** See `LdnAcceptPolicy`.
    pub fn ldnAddAcceptFilterEntry(addr: LdnMacAddress) -> Result;

    /// ClearAcceptFilter
    ///
    /// **Note:** `LdnState` must be ::LdnState_AccessPoint or ::LdnState_AccessPointCreated.
    pub fn ldnClearAcceptFilter() -> Result;

    /// OpenStation
    ///
    /// **Note:** `LdnState` must be ::LdnState_Initialized, this eventually sets the State to ::LdnState_Station.
    pub fn ldnOpenStation() -> Result;

    /// CloseStation
    ///
    /// **Note:** `LdnState` must be ::LdnState_Station or ::LdnState_StationConnected, this eventually sets the State to ::LdnState_Initialized.
    ///
    /// **Note:** Used automatically internally by `ldnExit` if needed.
    pub fn ldnCloseStation() -> Result;

    /// Connect
    ///
    /// # Arguments
    /// - `sec_config` (in): `LdnSecurityConfig`
    /// - `user_config` (in): `LdnUserConfig`
    /// - `version` (in): LocalCommunicationVersion, this must be 0x0-0x7FFF.
    /// - `option` (in): ConnectOption bitmask, must be &lt;=0x1. You can use value 0 for example here.
    /// - `network_info` (in): `LdnNetworkInfo`
    ///
    /// **Note:** `LdnState` must be ::LdnState_Station, this eventually sets the State to ::LdnState_StationConnected.
    ///
    /// **Note:** This is identical to `ldnConnectPrivate` besides the used params, the code overwriting LdnSecurityConfig::type also differs.
    pub fn ldnConnect(sec_config: *const LdnSecurityConfig, user_config: *const LdnUserConfig, version: s32, option: u32, network_info: *const LdnNetworkInfo) -> Result;

    /// ConnectPrivate
    ///
    /// # Arguments
    /// - `sec_config` (in): `LdnSecurityConfig`
    /// - `sec_param` (in): `LdnSecurityParameter`
    /// - `user_config` (in): `LdnUserConfig`
    /// - `version` (in): LocalCommunicationVersion, this must be 0x0-0x7FFF.
    /// - `option` (in): ConnectOption bitmask, must be &lt;=0x1. You can use value 0 for example here.
    /// - `network_config` (in): `LdnNetworkConfig`
    ///
    /// **Note:** `LdnState` must be ::LdnState_Station, this eventually sets the State to ::LdnState_StationConnected.
    ///
    /// **Note:** See `ldnConnect`.
    pub fn ldnConnectPrivate(sec_config: *const LdnSecurityConfig, sec_param: *const LdnSecurityParameter, user_config: *const LdnUserConfig, version: s32, option: u32, network_config: *const LdnNetworkConfig) -> Result;

    /// Disconnect
    ///
    /// **Note:** `LdnState` must be ::LdnState_StationConnected, this eventually sets the State to ::LdnState_Station.
    pub fn ldnDisconnect() -> Result;

    /// SetOperationMode
    ///
    /// # Arguments
    /// - `mode` (in): `LdnOperationMode`
    ///
    /// **Note:** With ::LdnServiceType_System this is only available on [4.0.0+].
    ///
    /// **Note:** With ::LdnServiceType_User this is only available on [19.0.0+].
    ///
    /// **Note:** `LdnState` must be ::LdnState_Initialized.
    pub fn ldnSetOperationMode(mode: LdnOperationMode) -> Result;

    /// EnableActionFrame
    ///
    /// # Arguments
    /// - `settings` (in): `LdnActionFrameSettings`
    ///
    /// **Note:** Only available on [18.0.0+].
    ///
    /// **Note:** `LdnState` must be ::LdnState_Initialized.
    pub fn ldnEnableActionFrame(settings: *const LdnActionFrameSettings) -> Result;

    /// DisableActionFrame
    ///
    /// **Note:** Only available on [18.0.0+].
    ///
    /// **Note:** `LdnState` must be ::LdnState_Initialized.
    pub fn ldnDisableActionFrame() -> Result;

    /// SendActionFrame
    ///
    /// # Arguments
    /// - `data` (in): Data buffer.
    /// - `size` (in): Data buffer size.
    /// - `destination` (in): Destination `LdnMacAddress`.
    /// - `bssid` (in): Bssid `LdnMacAddress`.
    /// - `channel` (in): Channel, must be non-zero.
    /// - `flags` (in): MessageFlag bit0 clear = block until the data can be sent, set = return error when the data can't be sent.
    ///
    /// **Note:** Only available on [18.0.0+].
    ///
    /// **Note:** `LdnState` must be ::LdnState_AccessPointCreated / ::LdnState_Station.
    pub fn ldnSendActionFrame(data: *const c_void, size: usize, destination: LdnMacAddress, bssid: LdnMacAddress, channel: s16, flags: u32) -> Result;

    /// RecvActionFrame
    ///
    /// # Arguments
    /// - `data` (out): Output data buffer.
    /// - `size` (in): Max size of the data buffer.
    /// - `addr0` (out): First `LdnMacAddress`.
    /// - `addr1` (out): Second `LdnMacAddress`.
    /// - `channel` (out): Channel
    /// - `out_size` (out): Output size.
    /// - `link_level` (out): LinkLevel
    /// - `flags` (in): MessageFlag bit0 clear = block until data is available, set = return error when data is not available.
    ///
    /// **Note:** Only available on [18.0.0+].
    ///
    /// **Note:** `ldnEnableActionFrame` must be used prior to this.
    pub fn ldnRecvActionFrame(data: *mut c_void, size: usize, addr0: *mut LdnMacAddress, addr1: *mut LdnMacAddress, channel: *mut s16, out_size: *mut u32, link_level: *mut s32, flags: u32) -> Result;

    /// SetHomeChannel
    ///
    /// # Arguments
    /// - `channel` (in): Channel, must be non-zero.
    ///
    /// **Note:** Only available on [18.0.0+].
    ///
    /// **Note:** `LdnState` must be ::LdnState_Station.
    pub fn ldnSetHomeChannel(channel: s16) -> Result;

    /// SetTxPower
    ///
    /// # Arguments
    /// - `power` (in): Power, must be 0x0..0xFF.
    ///
    /// **Note:** Only available on [18.0.0+].
    ///
    /// **Note:** `LdnState` must be ::LdnState_AccessPoint* / ::LdnState_Station*.
    pub fn ldnSetTxPower(power: s16) -> Result;

    /// ResetTxPower
    ///
    /// **Note:** Only available on [18.0.0+].
    ///
    /// **Note:** `LdnState` must be ::LdnState_AccessPoint* / ::LdnState_Station*.
    pub fn ldnResetTxPower() -> Result;
}
