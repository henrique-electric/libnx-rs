//! `switch/services/ssl.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Values for __nx_ssl_service_type, controls which ssl service to initialize.
pub type SslServiceType = u32;
/// Initialize the ssl service.
pub const SslServiceType_Default: SslServiceType = 0;
/// [15.0.0+] Initialize the ssl:s service. On older versions this is the same as ::SslServiceType_Default.
pub const SslServiceType_System: SslServiceType = 1;

/// CaCertificateId
pub type SslCaCertificateId = i32;
/// [3.0.0+] All
pub const SslCaCertificateId_All: SslCaCertificateId = -1;
/// NintendoCAG3
pub const SslCaCertificateId_NintendoCAG3: SslCaCertificateId = 1;
/// NintendoClass2CAG3
pub const SslCaCertificateId_NintendoClass2CAG3: SslCaCertificateId = 2;
/// [16.0.0+] "Nintendo Root CA G4"
pub const SslCaCertificateId_NintendoRootCAG4: SslCaCertificateId = 3;
/// AmazonRootCA1
pub const SslCaCertificateId_AmazonRootCA1: SslCaCertificateId = 1000;
/// StarfieldServicesRootCertificateAuthorityG2
pub const SslCaCertificateId_StarfieldServicesRootCertificateAuthorityG2: SslCaCertificateId = 1001;
/// AddTrustExternalCARoot
pub const SslCaCertificateId_AddTrustExternalCARoot: SslCaCertificateId = 1002;
/// COMODOCertificationAuthority
pub const SslCaCertificateId_COMODOCertificationAuthority: SslCaCertificateId = 1003;
/// UTNDATACorpSGC
pub const SslCaCertificateId_UTNDATACorpSGC: SslCaCertificateId = 1004;
/// UTNUSERFirstHardware
pub const SslCaCertificateId_UTNUSERFirstHardware: SslCaCertificateId = 1005;
/// BaltimoreCyberTrustRoot
pub const SslCaCertificateId_BaltimoreCyberTrustRoot: SslCaCertificateId = 1006;
/// CybertrustGlobalRoot
pub const SslCaCertificateId_CybertrustGlobalRoot: SslCaCertificateId = 1007;
/// VerizonGlobalRootCA
pub const SslCaCertificateId_VerizonGlobalRootCA: SslCaCertificateId = 1008;
/// DigiCertAssuredIDRootCA
pub const SslCaCertificateId_DigiCertAssuredIDRootCA: SslCaCertificateId = 1009;
/// DigiCertAssuredIDRootG2
pub const SslCaCertificateId_DigiCertAssuredIDRootG2: SslCaCertificateId = 1010;
/// DigiCertGlobalRootCA
pub const SslCaCertificateId_DigiCertGlobalRootCA: SslCaCertificateId = 1011;
/// DigiCertGlobalRootG2
pub const SslCaCertificateId_DigiCertGlobalRootG2: SslCaCertificateId = 1012;
/// DigiCertHighAssuranceEVRootCA
pub const SslCaCertificateId_DigiCertHighAssuranceEVRootCA: SslCaCertificateId = 1013;
/// EntrustnetCertificationAuthority2048
pub const SslCaCertificateId_EntrustnetCertificationAuthority2048: SslCaCertificateId = 1014;
/// EntrustRootCertificationAuthority
pub const SslCaCertificateId_EntrustRootCertificationAuthority: SslCaCertificateId = 1015;
/// EntrustRootCertificationAuthorityG2
pub const SslCaCertificateId_EntrustRootCertificationAuthorityG2: SslCaCertificateId = 1016;
/// GeoTrustGlobalCA2 ([8.0.0+] ::SslTrustedCertStatus is ::SslTrustedCertStatus_EnabledNotTrusted)
pub const SslCaCertificateId_GeoTrustGlobalCA2: SslCaCertificateId = 1017;
/// GeoTrustGlobalCA ([8.0.0+] ::SslTrustedCertStatus is ::SslTrustedCertStatus_EnabledNotTrusted)
pub const SslCaCertificateId_GeoTrustGlobalCA: SslCaCertificateId = 1018;
/// GeoTrustPrimaryCertificationAuthorityG3 ([8.0.0+] ::SslTrustedCertStatus is ::SslTrustedCertStatus_EnabledNotTrusted)
pub const SslCaCertificateId_GeoTrustPrimaryCertificationAuthorityG3: SslCaCertificateId = 1019;
/// GeoTrustPrimaryCertificationAuthority ([8.0.0+] ::SslTrustedCertStatus is ::SslTrustedCertStatus_EnabledNotTrusted)
pub const SslCaCertificateId_GeoTrustPrimaryCertificationAuthority: SslCaCertificateId = 1020;
/// GlobalSignRootCA
pub const SslCaCertificateId_GlobalSignRootCA: SslCaCertificateId = 1021;
/// GlobalSignRootCAR2
pub const SslCaCertificateId_GlobalSignRootCAR2: SslCaCertificateId = 1022;
/// GlobalSignRootCAR3
pub const SslCaCertificateId_GlobalSignRootCAR3: SslCaCertificateId = 1023;
/// GoDaddyClass2CertificationAuthority
pub const SslCaCertificateId_GoDaddyClass2CertificationAuthority: SslCaCertificateId = 1024;
/// GoDaddyRootCertificateAuthorityG2
pub const SslCaCertificateId_GoDaddyRootCertificateAuthorityG2: SslCaCertificateId = 1025;
/// StarfieldClass2CertificationAuthority
pub const SslCaCertificateId_StarfieldClass2CertificationAuthority: SslCaCertificateId = 1026;
/// StarfieldRootCertificateAuthorityG2
pub const SslCaCertificateId_StarfieldRootCertificateAuthorityG2: SslCaCertificateId = 1027;
/// thawtePrimaryRootCAG3 ([8.0.0+] ::SslTrustedCertStatus is ::SslTrustedCertStatus_EnabledNotTrusted)
pub const SslCaCertificateId_thawtePrimaryRootCAG3: SslCaCertificateId = 1028;
/// thawtePrimaryRootCA ([8.0.0+] ::SslTrustedCertStatus is ::SslTrustedCertStatus_EnabledNotTrusted)
pub const SslCaCertificateId_thawtePrimaryRootCA: SslCaCertificateId = 1029;
/// VeriSignClass3PublicPrimaryCertificationAuthorityG3 ([8.0.0+] ::SslTrustedCertStatus is ::SslTrustedCertStatus_EnabledNotTrusted)
pub const SslCaCertificateId_VeriSignClass3PublicPrimaryCertificationAuthorityG3: SslCaCertificateId = 1030;
/// VeriSignClass3PublicPrimaryCertificationAuthorityG5 ([8.0.0+] ::SslTrustedCertStatus is ::SslTrustedCertStatus_EnabledNotTrusted)
pub const SslCaCertificateId_VeriSignClass3PublicPrimaryCertificationAuthorityG5: SslCaCertificateId = 1031;
/// VeriSignUniversalRootCertificationAuthority ([8.0.0+] ::SslTrustedCertStatus is ::SslTrustedCertStatus_EnabledNotTrusted)
pub const SslCaCertificateId_VeriSignUniversalRootCertificationAuthority: SslCaCertificateId = 1032;
/// [6.0.0+] DSTRootCAX3
pub const SslCaCertificateId_DSTRootCAX3: SslCaCertificateId = 1033;
/// [10.0.3+] "USERTrust RSA Certification Authority"
pub const SslCaCertificateId_USERTrustRsaCertificationAuthority: SslCaCertificateId = 1034;
/// [10.1.0+] "ISRG Root X10"
pub const SslCaCertificateId_ISRGRootX10: SslCaCertificateId = 1035;
/// [10.1.0+] "USERTrust ECC Certification Authority"
pub const SslCaCertificateId_USERTrustEccCertificationAuthority: SslCaCertificateId = 1036;
/// [10.1.0+] "COMODO RSA Certification Authority"
pub const SslCaCertificateId_COMODORsaCertificationAuthority: SslCaCertificateId = 1037;
/// [10.1.0+] "COMODO ECC Certification Authority"
pub const SslCaCertificateId_COMODOEccCertificationAuthority: SslCaCertificateId = 1038;
/// [11.0.0+] "Amazon Root CA 2"
pub const SslCaCertificateId_AmazonRootCA2: SslCaCertificateId = 1039;
/// [11.0.0+] "Amazon Root CA 3"
pub const SslCaCertificateId_AmazonRootCA3: SslCaCertificateId = 1040;
/// [11.0.0+] "Amazon Root CA 4"
pub const SslCaCertificateId_AmazonRootCA4: SslCaCertificateId = 1041;
/// [11.0.0+] "DigiCert Assured ID Root G3"
pub const SslCaCertificateId_DigiCertAssuredIDRootG3: SslCaCertificateId = 1042;
/// [11.0.0+] "DigiCert Global Root G3"
pub const SslCaCertificateId_DigiCertGlobalRootG3: SslCaCertificateId = 1043;
/// [11.0.0+] "DigiCert Trusted Root G4"
pub const SslCaCertificateId_DigiCertTrustedRootG4: SslCaCertificateId = 1044;
/// [11.0.0+] "Entrust Root Certification Authority - EC1"
pub const SslCaCertificateId_EntrustRootCertificationAuthorityEC1: SslCaCertificateId = 1045;
/// [11.0.0+] "Entrust Root Certification Authority - G4"
pub const SslCaCertificateId_EntrustRootCertificationAuthorityG4: SslCaCertificateId = 1046;
/// [11.0.0+] "GlobalSign ECC Root CA - R4"
pub const SslCaCertificateId_GlobalSignECCRootCAR4: SslCaCertificateId = 1047;
/// [11.0.0+] "GlobalSign ECC Root CA - R5"
pub const SslCaCertificateId_GlobalSignECCRootCAR5: SslCaCertificateId = 1048;
/// [11.0.0+] "GlobalSign ECC Root CA - R6"
pub const SslCaCertificateId_GlobalSignECCRootCAR6: SslCaCertificateId = 1049;
/// [11.0.0+] "GTS Root R1"
pub const SslCaCertificateId_GTSRootR1: SslCaCertificateId = 1050;
/// [11.0.0+] "GTS Root R2"
pub const SslCaCertificateId_GTSRootR2: SslCaCertificateId = 1051;
/// [11.0.0+] "GTS Root R3"
pub const SslCaCertificateId_GTSRootR3: SslCaCertificateId = 1052;
/// [11.0.0+] "GTS Root R4"
pub const SslCaCertificateId_GTSRootR4: SslCaCertificateId = 1053;
/// [12.0.0+] "Security Communication RootCA"
pub const SslCaCertificateId_SecurityCommunicationRootCA: SslCaCertificateId = 1054;
/// [15.0.0+] "GlobalSign Root E4"
pub const SslCaCertificateId_GlobalSignRootE4: SslCaCertificateId = 1055;
/// [15.0.0+] "GlobalSign Root R4"
pub const SslCaCertificateId_GlobalSignRootR4: SslCaCertificateId = 1056;
/// [15.0.0+] "T-TeleSec GlobalRoot Class 2"
pub const SslCaCertificateId_TTeleSecGlobalRootClass2: SslCaCertificateId = 1057;
/// [16.0.0+] "DigiCert TLS ECC P384 Root G5"
pub const SslCaCertificateId_DigiCertTLSECCP384RootG5: SslCaCertificateId = 1058;
/// [16.0.0+] "DigiCert TLS RSA4096 Root G5"
pub const SslCaCertificateId_DigiCertTLSRSA4096RootG5: SslCaCertificateId = 1059;

/// TrustedCertStatus
pub type SslTrustedCertStatus = i32;
/// Invalid
pub const SslTrustedCertStatus_Invalid: SslTrustedCertStatus = -1;
/// Removed
pub const SslTrustedCertStatus_Removed: SslTrustedCertStatus = 0;
/// EnabledTrusted
pub const SslTrustedCertStatus_EnabledTrusted: SslTrustedCertStatus = 1;
/// EnabledNotTrusted
pub const SslTrustedCertStatus_EnabledNotTrusted: SslTrustedCertStatus = 2;
/// Revoked
pub const SslTrustedCertStatus_Revoked: SslTrustedCertStatus = 3;

/// FlushSessionCacheOptionType
pub type SslFlushSessionCacheOptionType = u32;
/// SingleHost. Uses the input string.
pub const SslFlushSessionCacheOptionType_SingleHost: SslFlushSessionCacheOptionType = 0;
/// AllHosts. Doesn't use the input string.
pub const SslFlushSessionCacheOptionType_AllHosts: SslFlushSessionCacheOptionType = 1;

/// DebugOptionType
pub type SslDebugOptionType = u32;
/// AllowDisableVerifyOption
pub const SslDebugOptionType_AllowDisableVerifyOption: SslDebugOptionType = 0;

/// SslVersion. This is a bitmask which controls the min/max TLS versions to use, depending on which lowest/highest bits are set (if Auto* isn't set).
pub type SslVersion = u32;
/// TLS version min = 1.0, max = 1.2.
pub const SslVersion_Auto: SslVersion = BIT(0);
/// TLS 1.0.
pub const SslVersion_TlsV10: SslVersion = BIT(3);
/// TLS 1.1.
pub const SslVersion_TlsV11: SslVersion = BIT(4);
/// TLS 1.2.
pub const SslVersion_TlsV12: SslVersion = BIT(5);
/// [11.0.0+] TLS 1.3.
pub const SslVersion_TlsV13: SslVersion = BIT(6);
/// [11.0.0+] Same as Auto.
pub const SslVersion_Auto24: SslVersion = BIT(24);

/// CertificateFormat
pub type SslCertificateFormat = u32;
/// Pem
pub const SslCertificateFormat_Pem: SslCertificateFormat = 1;
/// Der
pub const SslCertificateFormat_Der: SslCertificateFormat = 2;

/// InternalPki
pub type SslInternalPki = u32;
/// DeviceClientCertDefault. Enables using the DeviceCert.
pub const SslInternalPki_DeviceClientCertDefault: SslInternalPki = 1;

/// ContextOption
pub type SslContextOption = u32;
/// CrlImportDateCheckEnable. The default value at the time of `sslCreateContext` is value 1.
pub const SslContextOption_CrlImportDateCheckEnable: SslContextOption = 1;

/// VerifyOption. The default bitmask value at the time of `sslContextCreateConnection` is ::SslVerifyOption_PeerCa | ::SslVerifyOption_HostName.
/// [5.0.0+] `sslConnectionSetVerifyOption`: (::SslVerifyOption_PeerCa | ::SslVerifyOption_HostName) must be set, unless: ::SslOptionType_SkipDefaultVerify is set, or [9.0.0+] ::SslDebugOptionType_AllowDisableVerifyOption is set.
/// [6.0.0+] `sslConnectionSetVerifyOption`: Following that, if ::SslVerifyOption_EvPolicyOid is set, then the following options must be set (besides the previously mentioned one): ::SslVerifyOption_PeerCa and ::SslVerifyOption_DateCheck.
pub type SslVerifyOption = u32;
/// PeerCa
pub const SslVerifyOption_PeerCa: SslVerifyOption = BIT(0);
/// HostName
pub const SslVerifyOption_HostName: SslVerifyOption = BIT(1);
/// DateCheck
pub const SslVerifyOption_DateCheck: SslVerifyOption = BIT(2);
/// EvCertPartial
pub const SslVerifyOption_EvCertPartial: SslVerifyOption = BIT(3);
/// [6.0.0+] EvPolicyOid
pub const SslVerifyOption_EvPolicyOid: SslVerifyOption = BIT(4);
/// [6.0.0+] EvCertFingerprint
pub const SslVerifyOption_EvCertFingerprint: SslVerifyOption = BIT(5);

/// IoMode. The default value at the time of `sslContextCreateConnection` is ::SslIoMode_Blocking.
/// The socket non-blocking flag is always set regardless of this field, this is only used internally for calculating the timeout used by various cmds.
pub type SslIoMode = u32;
/// Blocking. Timeout = 5 minutes.
pub const SslIoMode_Blocking: SslIoMode = 1;
/// NonBlocking. Timeout = 0.
pub const SslIoMode_NonBlocking: SslIoMode = 2;

/// PollEvent
pub type SslPollEvent = u32;
/// Read
pub const SslPollEvent_Read: SslPollEvent = BIT(0);
/// Write
pub const SslPollEvent_Write: SslPollEvent = BIT(1);
/// Except
pub const SslPollEvent_Except: SslPollEvent = BIT(2);

/// SessionCacheMode
pub type SslSessionCacheMode = u32;
/// None
pub const SslSessionCacheMode_None: SslSessionCacheMode = 0;
/// SessionId
pub const SslSessionCacheMode_SessionId: SslSessionCacheMode = 1;
/// SessionTicket
pub const SslSessionCacheMode_SessionTicket: SslSessionCacheMode = 2;

/// RenegotiationMode
pub type SslRenegotiationMode = u32;
/// None
pub const SslRenegotiationMode_None: SslRenegotiationMode = 0;
/// Secure
pub const SslRenegotiationMode_Secure: SslRenegotiationMode = 1;

/// OptionType. The default bool flags value for these at the time of `sslContextCreateConnection` is cleared.
pub type SslOptionType = u32;
/// DoNotCloseSocket. See `sslConnectionSetSocketDescriptor`. This is only available if `sslConnectionSetSocketDescriptor` wasn't used yet.
pub const SslOptionType_DoNotCloseSocket: SslOptionType = 0;
/// [3.0.0+] GetServerCertChain. See `sslConnectionDoHandshake`.
pub const SslOptionType_GetServerCertChain: SslOptionType = 1;
/// [5.0.0+] SkipDefaultVerify. Checked by `sslConnectionSetVerifyOption`, see `SslVerifyOption`.
pub const SslOptionType_SkipDefaultVerify: SslOptionType = 2;
/// [9.0.0+] EnableAlpn. Only available with `sslConnectionSetOption`. `sslConnectionSetSocketDescriptor` should have been used prior to this - this will optionally use state setup by that, without throwing an error if that cmd wasn't used.
pub const SslOptionType_EnableAlpn: SslOptionType = 3;

/// PrivateOptionType
pub type SslPrivateOptionType = u32;
/// **Ref:** sslConnectionSetSessionCacheMode will throw an error if the input ::SslSessionCacheMode is non-zero and this option flag is set.
pub const SslPrivateOptionType_DtlsSession: SslPrivateOptionType = 1;
/// [17.0.0+] This exclusively enables the cipher suite specified in the input u32 value passed to `sslConnectionSetPrivateOption` (all other ciphers disabled).
pub const SslPrivateOptionType_SetCipher: SslPrivateOptionType = 2;

/// AlpnProtoState
pub type SslAlpnProtoState = u32;
/// NoSupport
pub const SslAlpnProtoState_NoSupport: SslAlpnProtoState = 0;
/// Negotiated
pub const SslAlpnProtoState_Negotiated: SslAlpnProtoState = 1;
/// NoOverlap
pub const SslAlpnProtoState_NoOverlap: SslAlpnProtoState = 2;
/// Selected
pub const SslAlpnProtoState_Selected: SslAlpnProtoState = 3;
/// EarlyValue
pub const SslAlpnProtoState_EarlyValue: SslAlpnProtoState = 4;

/// SslContext
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SslContext {
    /// ISslContext
    pub s: Service,
}

/// SslConnection
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SslConnection {
    /// ISslConnection
    pub s: Service,
}

/// BuiltInCertificateInfo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SslBuiltInCertificateInfo {
    /// **Ref:** SslCaCertificateId
    pub cert_id: u32,
    /// **Ref:** SslTrustedCertStatus
    pub status: u32,
    /// CertificateSize
    pub cert_size: u64,
    /// CertificateData (converted from an offset to a ptr), in DER format.
    pub cert_data: *mut u8,
}

/// SslServerCertDetailHeader
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SslServerCertDetailHeader {
    /// Magicnum.
    pub magicnum: u64,
    /// Total certs.
    pub cert_total: u32,
    /// Padding.
    pub pad: u32,
}

/// SslServerCertDetailEntry
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SslServerCertDetailEntry {
    /// Size.
    pub size: u32,
    /// Offset.
    pub offset: u32,
}

/// CipherInfo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SslCipherInfo {
    /// Cipher string.
    pub cipher: [c_char; 64],
    /// Protocol version string.
    pub protocol_version: [c_char; 8],
}

/// KeyAndCertParams
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SslKeyAndCertParams {
    /// Must be value 1.
    pub unk_x0: u32,
    /// Key size in bits.
    pub key_size: s32,
    /// Public exponent, must be non-zero. Only the low 4-bytes are used.
    pub public_exponent: u64,
    /// CN (Common Name) NUL-terminated string.
    pub common_name: [c_char; 64],
    /// Length of common_name excluding NUL-terminator. Must be 0x1-0x3F.
    pub common_name_len: u32,
}

impl_zeroed_default!(SslContext, SslConnection, SslBuiltInCertificateInfo, SslServerCertDetailHeader, SslServerCertDetailEntry, SslCipherInfo, SslKeyAndCertParams);

unsafe extern "C" {
    /// Initialize ssl. A default value of 0x3 can be used for num_sessions. This must be 0x1-0x4.
    pub fn sslInitialize(num_sessions: u32) -> Result;

    /// Exit ssl.
    pub fn sslExit();

    /// Gets the Service object for the actual ssl service session.
    pub fn sslGetServiceSession() -> *mut Service;

    /// CreateContext
    ///
    /// # Arguments
    /// - `c` (out): `SslContext`
    /// - `ssl_version` (in): `SslVersion`
    ///
    /// **Note:** The CertStore is used automatically, regardless of what cmds are used.
    pub fn sslCreateContext(c: *mut SslContext, ssl_version: u32) -> Result;

    /// GetContextCount
    ///
    /// # Arguments
    /// - `out` (out): Output value.
    ///
    /// **Note:** Not used by official sw.
    pub fn sslGetContextCount(out: *mut u32) -> Result;

    /// GetCertificates
    ///
    /// # Arguments
    /// - `buffer` (in): Output buffer. The start of this buffer is an array of `SslBuiltInCertificateInfo`, with the specified count. The cert data (SslBuiltInCertificateInfo::data) is located after this array.
    /// - `size` (in): Output buffer size, this should be the size from `sslGetCertificateBufSize`.
    /// - `ca_cert_ids` (in): Input array of `SslCaCertificateId`.
    /// - `count` (in): Size of the ca_cert_ids array in entries.
    /// - `total_out` (out): [3.0.0+] Total output entries. Will always match count on pre-3.0.0. This will differ from count when ::SslCaCertificateId_All is used.
    pub fn sslGetCertificates(buffer: *mut c_void, size: u32, ca_cert_ids: *mut u32, count: u32, total_out: *mut u32) -> Result;

    /// GetCertificateBufSize
    ///
    /// # Arguments
    /// - `ca_cert_ids` (in): Input array of `SslCaCertificateId`.
    /// - `count` (in): Size of the ca_cert_ids array in entries.
    /// - `out` (out): Output size.
    pub fn sslGetCertificateBufSize(ca_cert_ids: *mut u32, count: u32, out: *mut u32) -> Result;

    /// FlushSessionCache
    ///
    /// # Arguments
    /// - `str` (in): Input string. Must be NULL with ::SslFlushSessionCacheOptionType_AllHosts.
    /// - `str_bufsize` (in): String buffer size, excluding NUL-terminator. Hence, this should be actual_bufsize-1. This must be 0 with ::SslFlushSessionCacheOptionType_AllHosts.
    /// - `type` (in): `SslFlushSessionCacheOptionType`
    /// - `out` (out): Output value.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn sslFlushSessionCache(str: *const c_char, str_bufsize: usize, type_: SslFlushSessionCacheOptionType, out: *mut u32) -> Result;

    /// SetDebugOption
    ///
    /// # Arguments
    /// - `buffer` (in): Input buffer, must not be NULL. The u8 from here is copied to state.
    /// - `size` (in): Buffer size, must not be 0.
    /// - `type` (in): `SslDebugOptionType`
    ///
    /// **Note:** Only available on [6.0.0+].
    ///
    /// **Note:** The official impl of this doesn't actually use the cmd.
    pub fn sslSetDebugOption(buffer: *const c_void, size: usize, type_: SslDebugOptionType) -> Result;

    /// GetDebugOption
    ///
    /// # Arguments
    /// - `buffer` (out): Output buffer, must not be NULL. An u8 is written here loaded from state.
    /// - `size` (in): Buffer size, must not be 0.
    /// - `type` (in): `SslDebugOptionType`
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn sslGetDebugOption(buffer: *mut c_void, size: usize, type_: SslDebugOptionType) -> Result;

    /// ClearTls12FallbackFlag
    ///
    /// **Note:** Only available on [14.0.0+].
    pub fn sslClearTls12FallbackFlag() -> Result;

    /// SetThreadCoreMask
    ///
    /// # Arguments
    /// - `mask` (in): CoreMask
    ///
    /// **Note:** Only available on [15.0.0+] with ::SslServiceType_System.
    pub fn sslSetThreadCoreMask(mask: u64) -> Result;

    /// GetThreadCoreMask
    ///
    /// # Arguments
    /// - `out` (out): Output CoreMask.
    ///
    /// **Note:** Only available on [15.0.0+] with ::SslServiceType_System.
    pub fn sslGetThreadCoreMask(out: *mut u64) -> Result;

    /// Closes a Context object.
    ///
    /// # Arguments
    /// - `c`: `SslContext`
    pub fn sslContextClose(c: *mut SslContext);

    /// SetOption
    ///
    /// # Arguments
    /// - `c`: `SslContext`
    /// - `option` (in): `SslContextOption`
    /// - `value` (in): Value to set. With ::SslContextOption_CrlImportDateCheckEnable, this must be 0 or 1.
    ///
    /// **Note:** Prior to 4.x this is stubbed.
    pub fn sslContextSetOption(c: *mut SslContext, option: SslContextOption, value: s32) -> Result;

    /// GetOption
    ///
    /// # Arguments
    /// - `c`: `SslContext`
    /// - `option` (in): `SslContextOption`
    /// - `out` (out): Output value.
    ///
    /// **Note:** Prior to 4.x this is stubbed.
    pub fn sslContextGetOption(c: *mut SslContext, option: SslContextOption, out: *mut s32) -> Result;

    /// CreateConnection
    ///
    /// # Arguments
    /// - `c`: `SslContext`
    /// - `conn` (out): Output `SslConnection`.
    pub fn sslContextCreateConnection(c: *mut SslContext, conn: *mut SslConnection) -> Result;

    /// GetConnectionCount
    ///
    /// # Arguments
    /// - `c`: `SslContext`
    /// - `out` (out): Output value.
    ///
    /// **Note:** Not exposed by official sw.
    pub fn sslContextGetConnectionCount(c: *mut SslContext, out: *mut u32) -> Result;

    /// ImportServerPki
    ///
    /// # Arguments
    /// - `c`: `SslContext`
    /// - `buffer` (in): Input buffer containing the cert data, must not be NULL. This can contain multiple certs. The certs can be CAs or server certs (no pubkeys).
    /// - `size` (in): Input buffer size.
    /// - `format` (in): `SslCertificateFormat`
    /// - `id` (out): Output Id. Optional, can be NULL.
    ///
    /// **Note:** A maximum of 71 ServerPki objects (associated with the output Id) can be imported.
    pub fn sslContextImportServerPki(c: *mut SslContext, buffer: *const c_void, size: u32, format: SslCertificateFormat, id: *mut u64) -> Result;

    /// ImportClientPki
    ///
    /// # Arguments
    /// - `c`: `SslContext`
    /// - `pkcs12` (in): PKCS#12 input buffer, must not be NULL.
    /// - `pkcs12_size` (in): pkcs12 buffer size.
    /// - `pw` (in): ASCII password string buffer, this can only be NULL if pw_size is 0. This will be internally copied to another buffer which was allocated with size=pw_size+1, for NUL-termination.
    /// - `pw_size` (in): Password buffer size, this can only be 0 if pw is NULL.
    /// - `id` (out): Output Id. Optional, can be NULL.
    ///
    /// **Note:** An error is thrown internally if this cmd or `sslContextRegisterInternalPki` was already used previously.
    pub fn sslContextImportClientPki(c: *mut SslContext, pkcs12: *const c_void, pkcs12_size: u32, pw: *const c_char, pw_size: u32, id: *mut u64) -> Result;

    /// Remove the specified *Pki, or on [3.0.0+] Crl.
    ///
    /// # Arguments
    /// - `c`: `SslContext`
    /// - `id` (in): Id
    pub fn sslContextRemovePki(c: *mut SslContext, id: u64) -> Result;

    /// RegisterInternalPki
    ///
    /// # Arguments
    /// - `c`: `SslContext`
    /// - `internal_pki` (in): `SslInternalPki`
    /// - `id` (out): Output Id. Optional, can be NULL.
    ///
    /// **Note:** An error is thrown internally if this cmd or `sslContextImportClientPki` was already used previously.
    pub fn sslContextRegisterInternalPki(c: *mut SslContext, internal_pki: SslInternalPki, id: *mut u64) -> Result;

    /// AddPolicyOid
    ///
    /// # Arguments
    /// - `c`: `SslContext`
    /// - `str` (in): Input string.
    /// - `str_bufsize` (in): String buffer size, excluding NUL-terminator (must not match the string length). Hence, this should be actual_bufsize-1. This must not be >0xff.
    pub fn sslContextAddPolicyOid(c: *mut SslContext, str: *const c_char, str_bufsize: u32) -> Result;

    /// ImportCrl
    ///
    /// # Arguments
    /// - `c`: `SslContext`
    /// - `buffer` (in): Input buffer, must not be NULL. This contains the DER CRL.
    /// - `size` (in): Input buffer size.
    /// - `id` (out): Output Id. Optional, can be NULL.
    ///
    /// **Note:** Only available on [3.0.0+].
    pub fn sslContextImportCrl(c: *mut SslContext, buffer: *const c_void, size: u32, id: *mut u64) -> Result;

    /// ImportClientCertKeyPki
    ///
    /// # Arguments
    /// - `c`: `SslContext`
    /// - `cert` (in): Input cert buffer,
    /// - `cert_size` (in): Size of the cert buffer.
    /// - `key` (in): Input key buffer.
    /// - `key_size` (in): Size of the key buffer.
    /// - `format` (in): `SslCertificateFormat` for the cert and key.
    /// - `id` (out): Output Id. Optional, can be NULL.
    ///
    /// **Note:** Only available on [16.0.0+].
    pub fn sslContextImportClientCertKeyPki(c: *mut SslContext, cert: *const c_void, cert_size: u32, key: *const c_void, key_size: u32, format: SslCertificateFormat, id: *mut u64) -> Result;

    /// GeneratePrivateKeyAndCert
    ///
    /// # Arguments
    /// - `c`: `SslContext`
    /// - `cert` (out): Output cert buffer,
    /// - `cert_size` (in): Size of the cert buffer.
    /// - `key` (out): Output key buffer.
    /// - `key_size` (in): Size of the key buffer.
    /// - `val` (in): Must be value 1.
    /// - `params` (in): `SslKeyAndCertParams`
    /// - `out_certsize` (out): Actual size of the generated cert data.
    /// - `out_keysize` (out): Actual size of the generated key data.
    ///
    /// **Note:** Only available on [16.0.0+].
    pub fn sslContextGeneratePrivateKeyAndCert(c: *mut SslContext, cert: *mut c_void, cert_size: u32, key: *mut c_void, key_size: u32, val: u32, params: *const SslKeyAndCertParams, out_certsize: *mut u32, out_keysize: *mut u32) -> Result;

    /// CreateConnectionForSystem
    ///
    /// # Arguments
    /// - `c`: `SslContext`
    /// - `conn` (out): Output `SslConnection`.
    ///
    /// **Note:** Only available on [15.0.0+] with ::SslServiceType_System.
    pub fn sslContextCreateConnectionForSystem(c: *mut SslContext, conn: *mut SslConnection) -> Result;

    /// Closes a Connection object.
    ///
    /// # Arguments
    /// - `c`: `SslConnection`
    pub fn sslConnectionClose(c: *mut SslConnection);

    /// SetSocketDescriptor. Do not use directly, use `socketSslConnectionSetSocketDescriptor` instead.
    ///
    /// # Arguments
    /// - `c`: `SslConnection`
    /// - `sockfd` (in): sockfd
    /// - `out_sockfd` (out): sockfd. Prior to using `sslConnectionClose`, this must be closed if it's not negative (it will be -1 if ::SslOptionType_DoNotCloseSocket is set).
    ///
    /// **Note:** An error is thrown if this was used previously.
    pub fn sslConnectionSetSocketDescriptor(c: *mut SslConnection, sockfd: c_int, out_sockfd: *mut c_int) -> Result;

    /// SetHostName
    ///
    /// # Arguments
    /// - `c`: `SslConnection`
    /// - `str` (in): Input string.
    /// - `str_bufsize` (in): String buffer size. This must not be >0xff.
    pub fn sslConnectionSetHostName(c: *mut SslConnection, str: *const c_char, str_bufsize: u32) -> Result;

    /// SetVerifyOption
    ///
    /// # Arguments
    /// - `c`: `SslConnection`
    /// - `verify_option` (in): Input bitmask of `SslVerifyOption`.
    pub fn sslConnectionSetVerifyOption(c: *mut SslConnection, verify_option: u32) -> Result;

    /// SetIoMode
    ///
    /// # Arguments
    /// - `c`: `SslConnection`
    /// - `mode` (in): `SslIoMode`
    ///
    /// **Note:** `sslConnectionSetSocketDescriptor` must have been used prior to this successfully.
    pub fn sslConnectionSetIoMode(c: *mut SslConnection, mode: SslIoMode) -> Result;

    /// GetSocketDescriptor. Do not use directly, use `socketSslConnectionGetSocketDescriptor` instead.
    ///
    /// # Arguments
    /// - `c`: `SslConnection`
    /// - `sockfd` (out): Output sockfd.
    ///
    /// **Note:** This gets the input sockfd which was previously saved in state by `sslConnectionSetSocketDescriptor`.
    ///
    /// **Note:** `sslConnectionSetSocketDescriptor` must have been used prior to this successfully.
    pub fn sslConnectionGetSocketDescriptor(c: *mut SslConnection, sockfd: *mut c_int) -> Result;

    /// GetHostName
    ///
    /// # Arguments
    /// - `c`: `SslConnection`
    /// - `str` (out): Output string buffer.
    /// - `str_bufsize` (in): String buffer size, must be large enough for the entire output string.
    /// - `out` (out): Output string length.
    pub fn sslConnectionGetHostName(c: *mut SslConnection, str: *mut c_char, str_bufsize: u32, out: *mut u32) -> Result;

    /// GetVerifyOption
    ///
    /// # Arguments
    /// - `c`: `SslConnection`
    /// - `out` (out): Output bitmask of `SslVerifyOption`.
    pub fn sslConnectionGetVerifyOption(c: *mut SslConnection, out: *mut u32) -> Result;

    /// GetIoMode
    ///
    /// # Arguments
    /// - `c`: `SslConnection`
    /// - `out` (out): `SslIoMode`
    pub fn sslConnectionGetIoMode(c: *mut SslConnection, out: *mut SslIoMode) -> Result;

    /// DoHandshake
    ///
    /// # Arguments
    /// - `c`: `SslConnection`
    /// - `out_size` (out): Total data size which was written to server_certbuf. Optional, can be NULL.
    /// - `total_certs` (out): Total certs which were written to server_certbuf, can be NULL.
    /// - `server_certbuf` (out): Optional output server cert buffer, can be NULL. Normally this just contains the server cert DER, however with ::SslOptionType_GetServerCertChain set this will contain the full chain (`sslConnectionGetServerCertDetail` can be used to parse that). With ::SslIoMode_NonBlocking this buffer will be only filled in once - when this cmd returns successfully the buffer will generally be empty.
    /// - `server_certbuf_size` (in): Optional output server cert buffer size, can be 0.
    ///
    /// **Note:** `sslConnectionSetSocketDescriptor` must have been used prior to this successfully.
    ///
    /// **Note:** `sslConnectionSetHostName` must have been used previously with a non-empty string when ::SslVerifyOption_HostName is set.
    ///
    /// **Note:** The DoHandshakeGetServerCert cmd is only used if both server_certbuf/server_certbuf_size are set, otherwise the DoHandshake cmd is used (in which case out_size/total_certs will be left at value 0).
    ///
    /// **Note:** No certs are returned when ::SslVerifyOption_PeerCa is not set.
    pub fn sslConnectionDoHandshake(c: *mut SslConnection, out_size: *mut u32, total_certs: *mut u32, server_certbuf: *mut c_void, server_certbuf_size: u32) -> Result;

    /// Parses the output server_certbuf from `sslConnectionDoHandshake` where ::SslOptionType_GetServerCertChain is set.
    ///
    /// # Arguments
    /// - `certbuf` (in): server_certbuf from `sslConnectionDoHandshake`, must not be NULL.
    /// - `certbuf_size` (in): out_size from `sslConnectionDoHandshake`.
    /// - `cert_index` (in): Cert index, must be within the range of certs stored in certbuf.
    /// - `cert` (out): Ptr for the ouput DER cert, must not be NULL.
    /// - `cert_size` (out): Size for the ouput cert, must not be NULL.
    pub fn sslConnectionGetServerCertDetail(certbuf: *const c_void, certbuf_size: u32, cert_index: u32, cert: *mut *mut c_void, cert_size: *mut u32) -> Result;

    /// Read
    ///
    /// # Arguments
    /// - `c`: `SslConnection`
    /// - `buffer` (out): Output buffer, must not be NULL.
    /// - `size` (in): Output buffer size, must not be 0.
    /// - `out_size` (out): Actual transferred size.
    ///
    /// **Note:** `sslConnectionSetSocketDescriptor` must have been used prior to this successfully.
    pub fn sslConnectionRead(c: *mut SslConnection, buffer: *mut c_void, size: u32, out_size: *mut u32) -> Result;

    /// Write
    ///
    /// # Arguments
    /// - `c`: `SslConnection`
    /// - `buffer` (in): Input buffer, must not be NULL.
    /// - `size` (in): Input buffer size, must not be 0.
    /// - `out_size` (out): Actual transferred size.
    ///
    /// **Note:** `sslConnectionSetSocketDescriptor` must have been used prior to this successfully.
    pub fn sslConnectionWrite(c: *mut SslConnection, buffer: *const c_void, size: u32, out_size: *mut u32) -> Result;

    /// Pending
    ///
    /// # Arguments
    /// - `c`: `SslConnection`
    /// - `out` (out): Output value.
    ///
    /// **Note:** `sslConnectionSetSocketDescriptor` must have been used prior to this successfully.
    pub fn sslConnectionPending(c: *mut SslConnection, out: *mut s32) -> Result;

    /// Peek
    ///
    /// # Arguments
    /// - `c`: `SslConnection`
    /// - `buffer` (out): Output buffer, must not be NULL.
    /// - `size` (in): Output buffer size, must not be 0.
    /// - `out_size` (out): Output size.
    ///
    /// **Note:** `sslConnectionSetSocketDescriptor` must have been used prior to this successfully.
    pub fn sslConnectionPeek(c: *mut SslConnection, buffer: *mut c_void, size: u32, out_size: *mut u32) -> Result;

    /// Poll
    ///
    /// # Arguments
    /// - `c`: `SslConnection`
    /// - `in_pollevent` (in): Input bitmask of `SslPollEvent`.
    /// - `out_pollevent` (out): Output bitmask of `SslPollEvent`.
    /// - `timeout` (in): Timeout in milliseconds.
    ///
    /// **Note:** `sslConnectionSetSocketDescriptor` must have been used prior to this successfully.
    pub fn sslConnectionPoll(c: *mut SslConnection, in_pollevent: u32, out_pollevent: *mut u32, timeout: u32) -> Result;

    /// GetVerifyCertError
    ///
    /// # Arguments
    /// - `c`: `SslConnection`
    ///
    /// **Note:** The value in state is cleared after loading it.
    pub fn sslConnectionGetVerifyCertError(c: *mut SslConnection) -> Result;

    /// GetNeededServerCertBufferSize
    ///
    /// # Arguments
    /// - `c`: `SslConnection`
    /// - `out` (out): Output value.
    pub fn sslConnectionGetNeededServerCertBufferSize(c: *mut SslConnection, out: *mut u32) -> Result;

    /// SetSessionCacheMode
    ///
    /// # Arguments
    /// - `c`: `SslConnection`
    /// - `mode` (in): `SslSessionCacheMode`
    ///
    /// **Note:** `sslConnectionSetSocketDescriptor` must have been used prior to this successfully.
    pub fn sslConnectionSetSessionCacheMode(c: *mut SslConnection, mode: SslSessionCacheMode) -> Result;

    /// GetSessionCacheMode
    ///
    /// # Arguments
    /// - `c`: `SslConnection`
    /// - `out` (out): `SslSessionCacheMode`
    ///
    /// **Note:** `sslConnectionSetSocketDescriptor` must have been used prior to this successfully.
    pub fn sslConnectionGetSessionCacheMode(c: *mut SslConnection, out: *mut SslSessionCacheMode) -> Result;

    /// GetSessionCacheMode
    ///
    /// # Arguments
    /// - `c`: `SslConnection`
    ///
    /// **Note:** `sslConnectionSetSocketDescriptor` must have been used prior to this successfully.
    pub fn sslConnectionFlushSessionCache(c: *mut SslConnection) -> Result;

    /// SetRenegotiationMode
    ///
    /// # Arguments
    /// - `c`: `SslConnection`
    /// - `mode` (in): `SslRenegotiationMode`
    ///
    /// **Note:** `sslConnectionSetSocketDescriptor` must have been used prior to this successfully.
    pub fn sslConnectionSetRenegotiationMode(c: *mut SslConnection, mode: SslRenegotiationMode) -> Result;

    /// GetRenegotiationMode
    ///
    /// # Arguments
    /// - `c`: `SslConnection`
    /// - `out` (out): `SslRenegotiationMode`
    ///
    /// **Note:** `sslConnectionSetSocketDescriptor` must have been used prior to this successfully.
    pub fn sslConnectionGetRenegotiationMode(c: *mut SslConnection, out: *mut SslRenegotiationMode) -> Result;

    /// SetOption
    ///
    /// # Arguments
    /// - `c`: `SslConnection`
    /// - `option` (in): `SslOptionType`
    /// - `flag` (in): Input flag value.
    pub fn sslConnectionSetOption(c: *mut SslConnection, option: SslOptionType, flag: bool) -> Result;

    /// GetOption
    ///
    /// # Arguments
    /// - `c`: `SslConnection`
    /// - `option` (in): `SslOptionType`
    /// - `out` (out): Output flag value.
    pub fn sslConnectionGetOption(c: *mut SslConnection, option: SslOptionType, out: *mut bool) -> Result;

    /// GetVerifyCertErrors
    ///
    /// # Arguments
    /// - `out0` (out): First output value, must not be NULL.
    /// - `out1` (out): Second output value.
    /// - `errors` (out): Output array of Result, must not be NULL.
    /// - `count` (in): Size of the errors array in entries.
    ///
    /// **Note:** An error is thrown when the cmd is successful, if the two output u32s match.
    pub fn sslConnectionGetVerifyCertErrors(c: *mut SslConnection, out0: *mut u32, out1: *mut u32, errors: *mut Result, count: u32) -> Result;

    /// GetCipherInfo
    ///
    /// # Arguments
    /// - `c`: `SslConnection`
    /// - `out` (out): `SslCipherInfo`
    ///
    /// **Note:** Only available on [4.0.0+].
    ///
    /// **Note:** `sslConnectionSetSocketDescriptor` must have been used prior to this successfully.
    pub fn sslConnectionGetCipherInfo(c: *mut SslConnection, out: *mut SslCipherInfo) -> Result;

    /// SetNextAlpnProto
    ///
    /// # Arguments
    /// - `c`: `SslConnection`
    /// - `buffer` (in): Input buffer, must not be NULL. This contains an array of {u8 size, {data with the specified size}}, which must be within the buffer-size bounds.
    /// - `size` (in): Input buffer size, must not be 0. Must be at least 0x2.
    ///
    /// **Note:** Only available on [9.0.0+].
    ///
    /// **Note:** `sslConnectionSetSocketDescriptor` must have been used prior to this successfully.
    ///
    /// **Note:** ::SslOptionType_EnableAlpn should be set at the time of using `sslConnectionDoHandshake`, otherwise using this cmd will have no affect.
    pub fn sslConnectionSetNextAlpnProto(c: *mut SslConnection, buffer: *const u8, size: u32) -> Result;

    /// GetNextAlpnProto
    ///
    /// # Arguments
    /// - `c`: `SslConnection`
    /// - `state` (out): `SslAlpnProtoState`
    /// - `out` (out): Output string length.
    /// - `buffer` (out): Output string buffer, must not be NULL.
    /// - `size` (in): Output buffer size, must not be 0.
    ///
    /// **Note:** Only available on [9.0.0+].
    ///
    /// **Note:** `sslConnectionSetSocketDescriptor` must have been used prior to this successfully.
    ///
    /// **Note:** The output will be all-zero/empty if not available - such as when this was used before `sslConnectionDoHandshake`.
    pub fn sslConnectionGetNextAlpnProto(c: *mut SslConnection, state: *mut SslAlpnProtoState, out: *mut u32, buffer: *mut u8, size: u32) -> Result;

    /// SetDtlsSocketDescriptor. Do not use directly, use `socketSslConnectionSetDtlsSocketDescriptor` instead.
    ///
    /// # Arguments
    /// - `c`: `SslConnection`
    /// - `sockfd` (in): sockfd
    /// - `Input` (in): sockaddr.
    /// - `size` (in): Input sockaddr size.
    /// - `out_sockfd` (out): sockfd. Prior to using `sslConnectionClose`, this must be closed if it's not negative (it will be -1 if ::SslOptionType_DoNotCloseSocket is set).
    ///
    /// **Note:** Only available on [16.0.0+].
    ///
    /// **Note:** An error is thrown if this was used previously.
    pub fn sslConnectionSetDtlsSocketDescriptor(c: *mut SslConnection, sockfd: c_int, buf: *const c_void, size: usize, out_sockfd: *mut c_int) -> Result;

    /// GetDtlsHandshakeTimeout
    ///
    /// # Arguments
    /// - `c`: `SslConnection`
    /// - `out` (out): Output nanoseconds value.
    ///
    /// **Note:** Only available on [16.0.0+].
    pub fn sslConnectionGetDtlsHandshakeTimeout(c: *mut SslConnection, out: *mut u64) -> Result;

    /// SetPrivateOption
    ///
    /// # Arguments
    /// - `c`: `SslConnection`
    /// - `option` (in): `SslPrivateOptionType`
    /// - `value` (in): Input value.
    ///
    /// **Note:** Only available on [16.0.0+].
    pub fn sslConnectionSetPrivateOption(c: *mut SslConnection, option: SslPrivateOptionType, value: u32) -> Result;

    /// SetSrtpCiphers
    ///
    /// # Arguments
    /// - `c`: `SslConnection`
    /// - `ciphers` (in): Input array of u16s. Each entry must be value 1-2, otherwise the entry is ignored.
    /// - `count` (in): Total entries in the ciphers array, the maximum is 4.
    ///
    /// **Note:** Only available on [16.0.0+].
    pub fn sslConnectionSetSrtpCiphers(c: *mut SslConnection, ciphers: *const u16, count: u32) -> Result;

    /// GetSrtpCipher
    ///
    /// # Arguments
    /// - `c`: `SslConnection`
    /// - `out` (out): Output value.
    ///
    /// **Note:** Only available on [16.0.0+].
    pub fn sslConnectionGetSrtpCipher(c: *mut SslConnection, out: *mut u16) -> Result;

    /// ExportKeyingMaterial
    ///
    /// # Arguments
    /// - `c`: `SslConnection`
    /// - `outbuf` (out): Output buffer.
    /// - `outbuf_size` (in): Output buffer size.
    /// - `label` (in): Input label string.
    /// - `label_size` (in): Size of the label buffer excluding NUL-terminator.
    /// - `context` (in): Optional input context buffer, can be NULL.
    /// - `context_size` (in): Size of context, if specified this must be &lt;0xFFFF.
    ///
    /// **Note:** Only available on [16.0.0+].
    pub fn sslConnectionExportKeyingMaterial(c: *mut SslConnection, outbuf: *mut u8, outbuf_size: u32, label: *const c_char, label_size: u32, context: *const c_void, context_size: u32) -> Result;

    /// SetIoTimeout
    ///
    /// # Arguments
    /// - `c`: `SslConnection`
    /// - `timeout` (in): Input timeout value.
    ///
    /// **Note:** Only available on [16.0.0+].
    pub fn sslConnectionSetIoTimeout(c: *mut SslConnection, timeout: u32) -> Result;

    /// GetIoTimeout
    ///
    /// # Arguments
    /// - `c`: `SslConnection`
    /// - `out` (out): Output timeout value.
    ///
    /// **Note:** Only available on [16.0.0+].
    pub fn sslConnectionGetIoTimeout(c: *mut SslConnection, out: *mut u32) -> Result;
}
