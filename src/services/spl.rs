//! `switch/services/spl.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const SPL_RSA_BUFFER_SIZE: u32 = 0x100;

pub type SplConfigItem = u32;
pub const SplConfigItem_DisableProgramVerification: SplConfigItem = 1;
pub const SplConfigItem_DramId: SplConfigItem = 2;
pub const SplConfigItem_SecurityEngineIrqNumber: SplConfigItem = 3;
pub const SplConfigItem_Version: SplConfigItem = 4;
pub const SplConfigItem_HardwareType: SplConfigItem = 5;
pub const SplConfigItem_IsRetail: SplConfigItem = 6;
pub const SplConfigItem_IsRecoveryBoot: SplConfigItem = 7;
pub const SplConfigItem_DeviceId: SplConfigItem = 8;
pub const SplConfigItem_BootReason: SplConfigItem = 9;
pub const SplConfigItem_MemoryArrange: SplConfigItem = 10;
pub const SplConfigItem_IsDebugMode: SplConfigItem = 11;
pub const SplConfigItem_KernelMemoryConfiguration: SplConfigItem = 12;
pub const SplConfigItem_IsChargerHiZModeEnabled: SplConfigItem = 13;
pub const SplConfigItem_IsKiosk: SplConfigItem = 14;
pub const SplConfigItem_NewHardwareType: SplConfigItem = 15;
pub const SplConfigItem_NewKeyGeneration: SplConfigItem = 16;
pub const SplConfigItem_Package2Hash: SplConfigItem = 17;

pub type RsaKeyVersion = u32;
pub const RsaKeyVersion_Deprecated: RsaKeyVersion = 0;
pub const RsaKeyVersion_Extended: RsaKeyVersion = 1;

unsafe extern "C" {
    /// Initialize 'spl:'.
    pub fn splInitialize() -> Result;

    /// Exit 'spl:'.
    pub fn splExit();

    /// Gets the Service object for the IGeneralInterface usable with spl*().
    pub fn splGetServiceSession() -> *mut Service;

    /// Initialize spl:mig. On pre-4.0.0 this just calls `splInitialize`.
    pub fn splCryptoInitialize() -> Result;

    /// Exit spl:mig. On pre-4.0.0 this just calls `splExit`.
    pub fn splCryptoExit();

    /// Gets the Service object for the IGeneralInterface usable with splCrypto*().
    pub fn splCryptoGetServiceSession() -> *mut Service;

    /// Initialize spl:ssl. On pre-4.0.0 this just calls `splInitialize`.
    pub fn splSslInitialize() -> Result;

    /// Exit spl:ssl. On pre-4.0.0 this just calls `splExit`.
    pub fn splSslExit();

    /// Gets the Service object for the IGeneralInterface usable with splSsl*().
    pub fn splSslGetServiceSession() -> *mut Service;

    /// Initialize spl:es. On pre-4.0.0 this just calls `splInitialize`.
    pub fn splEsInitialize() -> Result;

    /// Exit spl:es. On pre-4.0.0 this just calls `splExit`.
    pub fn splEsExit();

    /// Gets the Service object for the IGeneralInterface usable with splEs*().
    pub fn splEsGetServiceSession() -> *mut Service;

    /// Initialize spl:fs. On pre-4.0.0 this just calls `splInitialize`.
    pub fn splFsInitialize() -> Result;

    /// Exit spl:fs. On pre-4.0.0 this just calls `splExit`.
    pub fn splFsExit();

    /// Gets the Service object for the IGeneralInterface usable with splFs*().
    pub fn splFsGetServiceSession() -> *mut Service;

    /// Initialize spl:manu. On pre-4.0.0 this just calls `splInitialize`.
    pub fn splManuInitialize() -> Result;

    /// Exit spl:manu. On pre-4.0.0 this just calls `splExit`.
    pub fn splManuExit();

    /// Gets the Service object for the IGeneralInterface usable with splManu*().
    pub fn splManuGetServiceSession() -> *mut Service;

    pub fn splGetConfig(config_item: SplConfigItem, out_config: *mut u64) -> Result;

    pub fn splUserExpMod(input: *const c_void, modulus: *const c_void, exp: *const c_void, exp_size: usize, dst: *mut c_void) -> Result;

    pub fn splSetConfig(config_item: SplConfigItem, value: u64) -> Result;

    pub fn splGetRandomBytes(out: *mut c_void, out_size: usize) -> Result;

    pub fn splIsDevelopment(out_is_development: *mut bool) -> Result;

    pub fn splSetBootReason(value: u32) -> Result;

    pub fn splGetBootReason(out_value: *mut u32) -> Result;

    pub fn splCryptoGenerateAesKek(wrapped_kek: *const c_void, key_generation: u32, option: u32, out_sealed_kek: *mut c_void) -> Result;

    pub fn splCryptoLoadAesKey(sealed_kek: *const c_void, wrapped_key: *const c_void, keyslot: u32) -> Result;

    pub fn splCryptoGenerateAesKey(sealed_kek: *const c_void, wrapped_key: *const c_void, out_sealed_key: *mut c_void) -> Result;

    pub fn splCryptoDecryptAesKey(wrapped_key: *const c_void, key_generation: u32, option: u32, out_sealed_key: *mut c_void) -> Result;

    pub fn splCryptoCryptAesCtr(input: *const c_void, output: *mut c_void, size: usize, keyslot: u32, ctr: *const c_void) -> Result;

    pub fn splCryptoComputeCmac(input: *const c_void, size: usize, keyslot: u32, out_cmac: *mut c_void) -> Result;

    pub fn splCryptoLockAesEngine(out_keyslot: *mut u32) -> Result;

    pub fn splCryptoUnlockAesEngine(keyslot: u32) -> Result;

    pub fn splCryptoGetSecurityEngineEvent(out_event: *mut Event) -> Result;

    pub fn splRsaDecryptPrivateKey(sealed_kek: *const c_void, wrapped_key: *const c_void, wrapped_rsa_key: *const c_void, wrapped_rsa_key_size: usize, version: RsaKeyVersion, dst: *mut c_void, dst_size: usize) -> Result;

    pub fn splSslLoadSecureExpModKey(sealed_kek: *const c_void, wrapped_key: *const c_void, wrapped_rsa_key: *const c_void, wrapped_rsa_key_size: usize) -> Result;

    pub fn splSslSecureExpMod(input: *const c_void, modulus: *const c_void, dst: *mut c_void) -> Result;

    pub fn splEsLoadRsaOaepKey(sealed_kek: *const c_void, wrapped_key: *const c_void, wrapped_rsa_key: *const c_void, wrapped_rsa_key_size: usize, version: RsaKeyVersion) -> Result;

    pub fn splEsUnwrapRsaOaepWrappedTitlekey(rsa_wrapped_titlekey: *const c_void, modulus: *const c_void, label_hash: *const c_void, label_hash_size: usize, key_generation: u32, out_sealed_titlekey: *mut c_void) -> Result;

    pub fn splEsUnwrapAesWrappedTitlekey(aes_wrapped_titlekey: *const c_void, key_generation: u32, out_sealed_titlekey: *mut c_void) -> Result;

    pub fn splEsLoadSecureExpModKey(sealed_kek: *const c_void, wrapped_key: *const c_void, wrapped_rsa_key: *const c_void, wrapped_rsa_key_size: usize) -> Result;

    pub fn splEsSecureExpMod(input: *const c_void, modulus: *const c_void, dst: *mut c_void) -> Result;

    pub fn splEsUnwrapElicenseKey(rsa_wrapped_elicense_key: *const c_void, modulus: *const c_void, label_hash: *const c_void, label_hash_size: usize, key_generation: u32, out_sealed_elicense_key: *mut c_void) -> Result;

    pub fn splEsLoadElicenseKey(sealed_elicense_key: *const c_void, keyslot: u32) -> Result;

    pub fn splFsLoadSecureExpModKey(sealed_kek: *const c_void, wrapped_key: *const c_void, wrapped_rsa_key: *const c_void, wrapped_rsa_key_size: usize, version: RsaKeyVersion) -> Result;

    pub fn splFsSecureExpMod(input: *const c_void, modulus: *const c_void, dst: *mut c_void) -> Result;

    pub fn splFsGenerateSpecificAesKey(wrapped_key: *const c_void, key_generation: u32, option: u32, out_sealed_key: *mut c_void) -> Result;

    pub fn splFsLoadTitlekey(sealed_titlekey: *const c_void, keyslot: u32) -> Result;

    pub fn splFsGetPackage2Hash(out_hash: *mut c_void) -> Result;

    pub fn splManuEncryptRsaKeyForImport(sealed_kek_pre: *const c_void, wrapped_key_pre: *const c_void, sealed_kek_post: *const c_void, wrapped_kek_post: *const c_void, option: u32, wrapped_rsa_key: *const c_void, out_wrapped_rsa_key: *mut c_void, rsa_key_size: usize) -> Result;
}
