//! `switch/services/lp2p.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub type Lp2pServiceType = u32;
/// Initializes lp2p:app.
pub const Lp2pServiceType_App: Lp2pServiceType = 0;
/// Initializes lp2p:sys.
pub const Lp2pServiceType_System: Lp2pServiceType = 1;

/// MacAddress
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Lp2pMacAddress {
    /// Address
    pub addr: [u8; 6],
}

/// GroupId
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Lp2pGroupId {
    /// BSSID
    pub id: [u8; 6],
}

/// GroupInfo
///
/// **Ref:** lp2pScan only uses the following fields for the cmd input struct: supported_platform/priority, frequency/channel, and preshared_key_binary_size/preshared_key.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Lp2pGroupInfo {
    /// When zero, this is set to randomly-generated data. Used during key derivation.
    pub unk_x0: [u8; 16],
    /// LocalCommunicationId. When zero, the value from the user-process control.nacp is loaded. This is later validated by `lp2pJoin` / `lp2pCreateGroup` the same way as LdnNetworkConfig::local_communication_id. Used during key derivation.
    pub local_communication_id: u64,
    /// Should be all-zero for the input struct so that the default is used.
    pub group_id: Lp2pGroupId,
    /// ServiceName. NUL-terminated string for the SSID. These characters must be '-' or alphanumeric (lowercase/uppercase). '_' must not be used, unless you generate valid data for that. The data for '_' will be automatically generated if it's not present.
    pub service_name: [c_char; 33],
    /// Must be &lt;=0x3F.
    pub flags_count: s8,
    /// Array of s8 with the above count. Each entry value must be &lt;=0x3F. Each entry is an array index used to load a set of flags from a global array with the specified index.
    pub flags: [s8; 64],
    /// SupportedPlatform. Must match value 1. 0 is PlatformIdNX, 1 is PlatformIdFuji.
    pub supported_platform: u8,
    /// MemberCountMax. Must be &lt;=0x8. If zero during group-creation, a default of value 1 is used for the value passed to a service-cmd.
    pub member_count_max: s8,
    /// Unknown
    pub unk_x82: u8,
    /// Unknown
    pub unk_x83: u8,
    /// Wifi frequency: 24 = 2.4GHz, 50 = 5GHz.
    pub frequency: u16,
    /// Wifi channel number. 0 = use default, otherwise this must be one of the following depending on the frequency field. 24: 1, 6, 11. 50: 36, 40, 44, 48.
    pub channel: s16,
    /// NetworkMode
    pub network_mode: u8,
    /// PerformanceRequirement
    pub performance_requirement: u8,
    /// Security type, used during key derivation. 0 = use defaults, 1 = plaintext, 2 = encrypted. [11.0.0+] 3: Standard WPA2-PSK.
    pub security_type: u8,
    /// StaticAesKeyIndex. Used as the array-index for selecting the KeySource used with GenerateAesKek during key derivation. Should be 1-2, otherwise GenerateAesKek is skipped and zeros are used for the AccessKey instead.
    pub static_aes_key_index: s8,
    /// Unknown
    pub unk_x8C: u8,
    /// Priority. Must match one of the following, depending on the used service (doesn't apply to `lp2pJoin`): 55 = SystemPriority (lp2p:sys), 90 = ApplicationPriority (lp2p:app and lp2p:sys).
    pub priority: u8,
    /// StealthEnabled. Bool flag, controls whether the SSID is hidden.
    pub stealth_enabled: u8,
    /// If zero, a default value of 0x20 is used.
    pub unk_x8F: u8,
    /// Unknown
    pub unk_x90: [u8; 304],
    /// PresharedKeyBinarySize
    pub preshared_key_binary_size: u8,
    /// PresharedKey. Used during key derivation.
    pub preshared_key: [u8; 63],
}

/// ScanResult
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Lp2pScanResult {
    /// **Ref:** Lp2pGroupInfo
    pub group_info: Lp2pGroupInfo,
    /// Unknown
    pub unk_x200: u8,
    /// Unknown
    pub unk_x201: [u8; 5],
    /// Size of the following AdvertiseData.
    pub advertise_data_size: u16,
    /// AdvertiseData, with the above size. This originates from `lp2pSetAdvertiseData`.
    pub advertise_data: [u8; 128],
    /// Unknown
    pub unk_x288: [u8; 120],
}

/// NodeInfo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Lp2pNodeInfo {
    /// struct sockaddr for the IP address.
    pub ip_addr: [u8; 32],
    /// Unknown
    pub unk_x20: [u8; 4],
    /// **Ref:** Lp2pMacAddress
    pub mac_addr: Lp2pMacAddress,
    /// Unknown
    pub unk_x2A: [u8; 86],
}

/// IpConfig. Only contains IPv4 addresses.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Lp2pIpConfig {
    /// Always zeros.
    pub unk_x0: [u8; 32],
    /// struct sockaddr for the IP address.
    pub ip_addr: [u8; 32],
    /// struct sockaddr for the subnet-mask.
    pub subnet_mask: [u8; 32],
    /// struct sockaddr for the gateway(?).
    pub gateway: [u8; 32],
    /// Always zeros.
    pub unk_x80: [u8; 128],
}

/// Sets Lp2pGroupInfo::member_count_max.
///
/// # Arguments
/// - `info`: `Lp2pGroupInfo`
/// - `count` (in): MemberCountMax
#[inline(always)]
pub fn lp2pGroupInfoSetMemberCountMax(info: &mut Lp2pGroupInfo, count: usize) {
    info.member_count_max = count as _;
}

/// Sets Lp2pGroupInfo::frequency and Lp2pGroupInfo::channel.
///
/// # Arguments
/// - `info`: `Lp2pGroupInfo`
/// - `frequency` (in): Lp2pGroupInfo::frequency
/// - `channel` (in): Lp2pGroupInfo::channel
#[inline(always)]
pub fn lp2pGroupInfoSetFrequencyChannel(info: &mut Lp2pGroupInfo, frequency: u16, channel: s16) {
    info.frequency = frequency;
    info.channel = channel;
}

/// Sets Lp2pGroupInfo::stealth_enabled.
///
/// # Arguments
/// - `info`: `Lp2pGroupInfo`
/// - `flag` (in): Lp2pGroupInfo::stealth_enabled
#[inline(always)]
pub fn lp2pGroupInfoSetStealthEnabled(info: &mut Lp2pGroupInfo, flag: bool) {
    info.stealth_enabled = flag as _;
}

impl_zeroed_default!(Lp2pMacAddress, Lp2pGroupId, Lp2pGroupInfo, Lp2pScanResult, Lp2pNodeInfo, Lp2pIpConfig);

unsafe extern "C" {
    /// Initialize lp2p.
    pub fn lp2pInitialize(service_type: Lp2pServiceType) -> Result;

    /// Exit lp2p.
    pub fn lp2pExit();

    /// Gets the Service object for INetworkService.
    pub fn lp2pGetServiceSession_INetworkService() -> *mut Service;

    /// Gets the Service object for INetworkServiceMonitor.
    pub fn lp2pGetServiceSession_INetworkServiceMonitor() -> *mut Service;

    /// Creates a default `Lp2pGroupInfo` for use with `lp2pCreateGroup` / `lp2pJoin`.
    ///
    /// # Arguments
    /// - `info`: `Lp2pGroupInfo`
    pub fn lp2pCreateGroupInfo(info: *mut Lp2pGroupInfo);

    /// Creates a default `Lp2pGroupInfo` for use with `lp2pScan`.
    ///
    /// # Arguments
    /// - `info`: `Lp2pGroupInfo`
    pub fn lp2pCreateGroupInfoScan(info: *mut Lp2pGroupInfo);

    /// Sets Lp2pGroupInfo::service_name.
    ///
    /// # Arguments
    /// - `info`: `Lp2pGroupInfo`
    /// - `name` (in): ServiceName / SSID.
    pub fn lp2pGroupInfoSetServiceName(info: *mut Lp2pGroupInfo, name: *const c_char);

    /// Sets Lp2pGroupInfo::flags_count and Lp2pGroupInfo::flags.
    ///
    /// # Arguments
    /// - `info`: `Lp2pGroupInfo`
    /// - `flags` (in): Lp2pGroupInfo::flags
    /// - `count` (in): Lp2pGroupInfo::flags_count
    ///
    /// **Note:** The default is count=1 flags\[0\]=1, which is used by `lp2pCreateGroupInfo`. [11.0.0+] To use standard WPA2-PSK, you can use flags\[0\]=0.
    pub fn lp2pGroupInfoSetFlags(info: *mut Lp2pGroupInfo, flags: *mut s8, count: usize);

    /// Sets the PresharedKey for the specified `Lp2pGroupInfo`.
    ///
    /// # Arguments
    /// - `info`: `Lp2pGroupInfo`
    /// - `key` (in): Data for the PresharedKey.
    /// - `size` (in): Size to copy into the PresharedKey, max is 0x20.
    ///
    /// **Note:** Using this is required before using the `Lp2pGroupInfo` as input for any cmds, so that Lp2pGroupInfo::preshared_key_binary_size gets initialized.
    ///
    /// **Note:** If standard WPA2-PSK is being used, use `lp2pGroupInfoSetPassphrase` instead.
    pub fn lp2pGroupInfoSetPresharedKey(info: *mut Lp2pGroupInfo, key: *const c_void, size: usize);

    /// Sets the passphrase, for when standard WPA2-PSK is being used.
    ///
    /// # Arguments
    /// - `info`: `Lp2pGroupInfo`
    /// - `passphrase` (in): Passphrase string, the required length is 0x8-0x3F.
    ///
    /// **Note:** Configure standard WPA2-PSK usage via `lp2pGroupInfoSetFlags` / Lp2pGroupInfo::security_type.
    ///
    /// **Note:** Only available on [11.0.0+].
    pub fn lp2pGroupInfoSetPassphrase(info: *mut Lp2pGroupInfo, passphrase: *const c_char) -> Result;

    /// Scan
    ///
    /// # Arguments
    /// - `info` (in): `Lp2pGroupInfo`
    /// - `results` (out): Output array of `Lp2pScanResult`.
    /// - `count` (in): Size of the results array in entries.
    /// - `total_out` (out): Total output entries.
    pub fn lp2pScan(info: *const Lp2pGroupInfo, results: *mut Lp2pScanResult, count: s32, total_out: *mut s32) -> Result;

    /// CreateGroup
    ///
    /// # Arguments
    /// - `info` (in): `Lp2pGroupInfo`
    ///
    /// **Note:** The role (`lp2pGetRole`) must be 0. This eventually sets the role to value 1.
    pub fn lp2pCreateGroup(info: *const Lp2pGroupInfo) -> Result;

    /// This destroys the previously created group from `lp2pCreateGroup`.
    ///
    /// **Note:** If no group was previously created (role from `lp2pGetRole` is not 1), this just returns 0.
    pub fn lp2pDestroyGroup() -> Result;

    /// SetAdvertiseData
    ///
    /// # Arguments
    /// - `buffer` (out): Input buffer containing arbitrary user data.
    /// - `size` (in): Input buffer size, must be &lt;=0x80.
    ///
    /// **Note:** The role (`lp2pGetRole`) must be &lt;=1.
    ///
    /// **Note:** An empty buffer (buffer=NULL/size=0) can be used to reset the AdvertiseData size in state to zero.
    pub fn lp2pSetAdvertiseData(buffer: *const c_void, size: usize) -> Result;

    /// This sends an Action frame to the specified `Lp2pGroupId`, with the specified destination `Lp2pMacAddress`.
    ///
    /// # Arguments
    /// - `buffer` (in): Input buffer containing arbitrary user data.
    /// - `size` (in): Input buffer size, must be &lt;=0x400.
    /// - `addr` (in): `Lp2pMacAddress`, this can be a broadcast address. This must be non-zero.
    /// - `group_id` (in): `Lp2pGroupId`
    /// - `frequency` (in): Must be >=1. See Lp2pGroupInfo::frequency.
    /// - `channel` (in): Must be >=1. See Lp2pGroupInfo::channel.
    /// - `flags` (in): Only bit0 is used: clear = block until the data can be sent, set = return error when the data can't be sent.
    ///
    /// **Note:** The role (`lp2pGetRole`) must be non-zero.
    ///
    /// **Note:** The error from `lp2pGetNetworkInterfaceLastError` will be returned if it's set.
    ///
    /// **Note:** [11.0.0+] Lp2pGroupInfo::security_type must be value 2 (default encryption), otherwise an error is returned.
    pub fn lp2pSendToOtherGroup(buffer: *const c_void, size: usize, addr: Lp2pMacAddress, group_id: Lp2pGroupId, frequency: s16, channel: s16, flags: u32) -> Result;

    /// This receives an Action frame.
    ///
    /// # Arguments
    /// - `buffer` (out): Output buffer containing arbitrary user data.
    /// - `size` (in): Output buffer size.
    /// - `flags` (in): Only bit0 is used: clear = block until data is available, set = return error when data is not available.
    /// - `addr` (in): `Lp2pMacAddress`
    /// - `unk0` (in): Unknown
    /// - `unk1` (in): Unknown
    /// - `out_size` (out): This is the original size used for copying to the output buffer, before it's clamped to the output-buffer size.
    /// - `unk2` (out): Unknown
    ///
    /// **Note:** The role (`lp2pGetRole`) must be non-zero.
    ///
    /// **Note:** When data is not available, the error from `lp2pGetNetworkInterfaceLastError` will be returned if it's set.
    pub fn lp2pRecvFromOtherGroup(buffer: *mut c_void, size: usize, flags: u32, addr: *mut Lp2pMacAddress, unk0: *mut u16, unk1: *mut s32, out_size: *mut u64, unk2: *mut s32) -> Result;

    /// AddAcceptableGroupId
    ///
    /// # Arguments
    /// - `group_id` (in): `Lp2pGroupId`
    pub fn lp2pAddAcceptableGroupId(group_id: Lp2pGroupId) -> Result;

    /// RemoveAcceptableGroupId
    pub fn lp2pRemoveAcceptableGroupId() -> Result;

    /// AttachNetworkInterfaceStateChangeEvent
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=false.
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn lp2pAttachNetworkInterfaceStateChangeEvent(out_event: *mut Event) -> Result;

    /// GetNetworkInterfaceLastError
    pub fn lp2pGetNetworkInterfaceLastError() -> Result;

    /// GetRole
    ///
    /// # Arguments
    /// - `out` (out): Output Role.
    pub fn lp2pGetRole(out: *mut u8) -> Result;

    /// GetAdvertiseData
    ///
    /// # Arguments
    /// - `buffer` (out): Output buffer data.
    /// - `size` (in): Output buffer size.
    /// - `transfer_size` (out): Size of the data copied into the buffer.
    /// - `original_size` (out): Original size from state.
    ///
    /// **Note:** The role from `lp2pGetRole` must be value 2.
    pub fn lp2pGetAdvertiseData(buffer: *mut c_void, size: usize, transfer_size: *mut u16, original_size: *mut u16) -> Result;

    /// GetAdvertiseData2
    ///
    /// # Arguments
    /// - `buffer` (out): Output buffer data.
    /// - `size` (in): Output buffer size.
    /// - `transfer_size` (out): Size of the data copied into the buffer.
    /// - `original_size` (out): Original size from state.
    ///
    /// **Note:** This is identical to `lp2pGetAdvertiseData` except this doesn't run the role validation.
    pub fn lp2pGetAdvertiseData2(buffer: *mut c_void, size: usize, transfer_size: *mut u16, original_size: *mut u16) -> Result;

    /// GetGroupInfo
    ///
    /// # Arguments
    /// - `out` (out): `Lp2pGroupInfo`
    ///
    /// **Note:** The role from `lp2pGetRole` must be non-zero.
    pub fn lp2pGetGroupInfo(out: *mut Lp2pGroupInfo) -> Result;

    /// This runs the same code as `lp2pCreateGroup` to generate the `Lp2pGroupInfo` for the input struct.
    ///
    /// # Arguments
    /// - `out` (out): `Lp2pGroupInfo`
    /// - `info` (in): `Lp2pGroupInfo`
    pub fn lp2pJoin(out: *mut Lp2pGroupInfo, info: *const Lp2pGroupInfo) -> Result;

    /// GetGroupOwner
    ///
    /// # Arguments
    /// - `out` (out): `Lp2pNodeInfo`
    ///
    /// **Note:** The role from `lp2pGetRole` must be non-zero.
    pub fn lp2pGetGroupOwner(out: *mut Lp2pNodeInfo) -> Result;

    /// GetIpConfig
    ///
    /// # Arguments
    /// - `out` (out): `Lp2pIpConfig`
    ///
    /// **Note:** The role from `lp2pGetRole` must be non-zero.
    pub fn lp2pGetIpConfig(out: *mut Lp2pIpConfig) -> Result;

    /// Leave
    ///
    /// # Arguments
    /// - `out` (out): Output value.
    pub fn lp2pLeave(out: *mut u32) -> Result;

    /// AttachJoinEvent
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=false.
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn lp2pAttachJoinEvent(out_event: *mut Event) -> Result;

    /// GetMembers
    ///
    /// # Arguments
    /// - `members` (out): Output array of `Lp2pNodeInfo`.
    /// - `count` (in): Size of the members array in entries. A maximum of 8 entries can be returned.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** The role from `lp2pGetRole` must be value 1.
    pub fn lp2pGetMembers(members: *mut Lp2pNodeInfo, count: s32, total_out: *mut s32) -> Result;
}
