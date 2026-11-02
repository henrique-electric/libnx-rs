//! `net/if_media.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const IFM_ETHER: u32 = 0x20;
pub const IFM_10_T: u32 = 3;
pub const IFM_10_2: u32 = 4;
pub const IFM_10_5: u32 = 5;
pub const IFM_100_TX: u32 = 6;
pub const IFM_100_FX: u32 = 7;
pub const IFM_100_T4: u32 = 8;
pub const IFM_100_VG: u32 = 9;
pub const IFM_100_T2: u32 = 10;
pub const IFM_1000_SX: u32 = 11;
pub const IFM_10_STP: u32 = 12;
pub const IFM_10_FL: u32 = 13;
pub const IFM_1000_LX: u32 = 14;
pub const IFM_1000_CX: u32 = 15;
pub const IFM_1000_T: u32 = 16;
pub const IFM_HPNA_1: u32 = 17;
pub const IFM_10G_LR: u32 = 18;
pub const IFM_10G_SR: u32 = 19;
pub const IFM_10G_CX4: u32 = 20;
pub const IFM_2500_SX: u32 = 21;
pub const IFM_10G_TWINAX: u32 = 22;
pub const IFM_10G_TWINAX_LONG: u32 = 23;
pub const IFM_10G_LRM: u32 = 24;
pub const IFM_UNKNOWN: u32 = 25;
pub const IFM_10G_T: u32 = 26;
pub const IFM_40G_CR4: u32 = 27;
pub const IFM_40G_SR4: u32 = 28;
pub const IFM_40G_LR4: u32 = 29;
pub const IFM_1000_KX: u32 = 30;
pub const IFM_OTHER: u32 = 31;
pub const IFM_10G_KX4: u32 = 2048;
pub const IFM_10G_KR: u32 = 2049;
pub const IFM_10G_CR1: u32 = 2050;
pub const IFM_20G_KR2: u32 = 2051;
pub const IFM_2500_KX: u32 = 2052;
pub const IFM_2500_T: u32 = 2053;
pub const IFM_5000_T: u32 = 2054;
pub const IFM_50G_PCIE: u32 = 2055;
pub const IFM_25G_PCIE: u32 = 2056;
pub const IFM_1000_SGMII: u32 = 2057;
pub const IFM_10G_SFI: u32 = 2058;
pub const IFM_40G_XLPPI: u32 = 2059;
pub const IFM_1000_CX_SGMII: u32 = 2060;
pub const IFM_40G_KR4: u32 = 2061;
pub const IFM_10G_ER: u32 = 2062;
pub const IFM_100G_CR4: u32 = 2063;
pub const IFM_100G_SR4: u32 = 2064;
pub const IFM_100G_KR4: u32 = 2065;
pub const IFM_100G_LR4: u32 = 2066;
pub const IFM_56G_R4: u32 = 2067;
pub const IFM_100_T: u32 = 2068;
pub const IFM_25G_CR: u32 = 2069;
pub const IFM_25G_KR: u32 = 2070;
pub const IFM_25G_SR: u32 = 2071;
pub const IFM_50G_CR2: u32 = 2072;
pub const IFM_50G_KR2: u32 = 2073;
pub const IFM_25G_LR: u32 = 2074;
pub const IFM_10G_AOC: u32 = 2075;
pub const IFM_25G_ACC: u32 = 2076;
pub const IFM_25G_AOC: u32 = 2077;
pub const IFM_ETH_MASTER: u32 = 0x100;
pub const IFM_ETH_RXPAUSE: u32 = 0x200;
pub const IFM_ETH_TXPAUSE: u32 = 0x400;
pub const IFM_ETH_XTYPE: u32 = 0x7800;
pub const IFM_ETH_XSHIFT: u32 = 6;
pub const IFM_TOKEN: u32 = 0x40;
pub const IFM_TOK_STP4: u32 = 3;
pub const IFM_TOK_STP16: u32 = 4;
pub const IFM_TOK_UTP4: u32 = 5;
pub const IFM_TOK_UTP16: u32 = 6;
pub const IFM_TOK_STP100: u32 = 7;
pub const IFM_TOK_UTP100: u32 = 8;
pub const IFM_TOK_ETR: u32 = 0x200;
pub const IFM_TOK_SRCRT: u32 = 0x400;
pub const IFM_TOK_ALLR: u32 = 0x800;
pub const IFM_TOK_DTR: u32 = 0x2000;
pub const IFM_TOK_CLASSIC: u32 = 0x4000;
pub const IFM_TOK_AUTO: u32 = 0x8000;
pub const IFM_FDDI: u32 = 0x60;
pub const IFM_FDDI_SMF: u32 = 3;
pub const IFM_FDDI_MMF: u32 = 4;
pub const IFM_FDDI_UTP: u32 = 5;
pub const IFM_FDDI_DA: u32 = 0x100;
pub const IFM_IEEE80211: u32 = 0x80;
pub const IFM_IEEE80211_FH1: u32 = 3;
pub const IFM_IEEE80211_FH2: u32 = 4;
pub const IFM_IEEE80211_DS1: u32 = 5;
pub const IFM_IEEE80211_DS2: u32 = 6;
pub const IFM_IEEE80211_DS5: u32 = 7;
pub const IFM_IEEE80211_DS11: u32 = 8;
pub const IFM_IEEE80211_DS22: u32 = 9;
pub const IFM_IEEE80211_OFDM6: u32 = 10;
pub const IFM_IEEE80211_OFDM9: u32 = 11;
pub const IFM_IEEE80211_OFDM12: u32 = 12;
pub const IFM_IEEE80211_OFDM18: u32 = 13;
pub const IFM_IEEE80211_OFDM24: u32 = 14;
pub const IFM_IEEE80211_OFDM36: u32 = 15;
pub const IFM_IEEE80211_OFDM48: u32 = 16;
pub const IFM_IEEE80211_OFDM54: u32 = 17;
pub const IFM_IEEE80211_OFDM72: u32 = 18;
pub const IFM_IEEE80211_DS354k: u32 = 19;
pub const IFM_IEEE80211_DS512k: u32 = 20;
pub const IFM_IEEE80211_OFDM3: u32 = 21;
pub const IFM_IEEE80211_OFDM4: u32 = 22;
pub const IFM_IEEE80211_OFDM27: u32 = 23;
pub const IFM_IEEE80211_MCS: u32 = 24;
pub const IFM_IEEE80211_VHT: u32 = 25;
pub const IFM_IEEE80211_ADHOC: u32 = 0x100;
pub const IFM_IEEE80211_HOSTAP: u32 = 0x200;
pub const IFM_IEEE80211_IBSS: u32 = 0x400;
pub const IFM_IEEE80211_WDS: u32 = 0x800;
pub const IFM_IEEE80211_TURBO: u32 = 0x1000;
pub const IFM_IEEE80211_MONITOR: u32 = 0x2000;
pub const IFM_IEEE80211_MBSS: u32 = 0x4000;
pub const IFM_IEEE80211_11A: u32 = 0x10000;
pub const IFM_IEEE80211_11B: u32 = 0x20000;
pub const IFM_IEEE80211_11G: u32 = 0x30000;
pub const IFM_IEEE80211_FH: u32 = 0x40000;
pub const IFM_IEEE80211_11NA: u32 = 0x50000;
pub const IFM_IEEE80211_11NG: u32 = 0x60000;
pub const IFM_IEEE80211_VHT5G: u32 = 0x70000;
pub const IFM_IEEE80211_VHT2G: u32 = 0x80000;
pub const IFM_ATM: u32 = 0xa0;
pub const IFM_ATM_UNKNOWN: u32 = 3;
pub const IFM_ATM_UTP_25: u32 = 4;
pub const IFM_ATM_TAXI_100: u32 = 5;
pub const IFM_ATM_TAXI_140: u32 = 6;
pub const IFM_ATM_MM_155: u32 = 7;
pub const IFM_ATM_SM_155: u32 = 8;
pub const IFM_ATM_UTP_155: u32 = 9;
pub const IFM_ATM_MM_622: u32 = 10;
pub const IFM_ATM_SM_622: u32 = 11;
pub const IFM_ATM_VIRTUAL: u32 = 12;
pub const IFM_ATM_SDH: u32 = 0x100;
pub const IFM_ATM_NOSCRAMB: u32 = 0x200;
pub const IFM_ATM_UNASSIGNED: u32 = 0x400;
pub const IFM_AUTO: u32 = 0;
pub const IFM_MANUAL: u32 = 1;
pub const IFM_NONE: u32 = 2;
pub const IFM_FDX: u32 = 0x100000;
pub const IFM_HDX: u32 = 0x200000;
pub const IFM_FLOW: u32 = 0x400000;
pub const IFM_FLAG0: u32 = 0x1000000;
pub const IFM_FLAG1: u32 = 0x2000000;
pub const IFM_FLAG2: u32 = 0x4000000;
pub const IFM_LOOP: u32 = 0x8000000;
pub const IFM_NMASK: u32 = 0xe0;
pub const IFM_TMASK: u32 = 0x1f;
pub const IFM_IMASK: u32 = 0xf0000000;
pub const IFM_ISHIFT: u32 = 28;
pub const IFM_OMASK: u32 = 0xff00;
pub const IFM_MMASK: u32 = 0x70000;
pub const IFM_MSHIFT: u32 = 16;
pub const IFM_GMASK: u32 = 0xff00000;
pub const IFM_ETH_FMASK: u32 = 0x400600;
pub const IFM_AVALID: u32 = 0x1;
pub const IFM_ACTIVE: u32 = 0x2;
pub const IFM_STATUS_VALID: u32 = 1;
pub const IFM_INST_MAX: u32 = 15;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ifmedia_description {
    pub ifmt_word: c_int,
    pub ifmt_string: *const c_char,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ifmedia_baudrate {
    pub ifmb_word: c_int,
    pub ifmb_baudrate: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ifmedia_status_description {
    pub ifms_type: c_int,
    pub ifms_valid: c_int,
    pub ifms_bit: c_int,
    pub ifms_string: [*const c_char; 2],
}

impl_zeroed_default!(ifmedia_description, ifmedia_baudrate, ifmedia_status_description);

#[inline(always)]
pub const fn IFM_ETHER_SUBTYPE(x: u32) -> u32 {
    (x & IFM_TMASK) | ((x & (IFM_ETH_XTYPE >> IFM_ETH_XSHIFT)) << IFM_ETH_XSHIFT)
}

/// Internal shorthand for [`IFM_ETHER_SUBTYPE`].
#[inline(always)]
pub const fn IFM_X(x: u32) -> u32 { IFM_ETHER_SUBTYPE(x) }

#[inline(always)]
pub const fn IFM_ETHER_SUBTYPE_SET(x: u32) -> u32 { IFM_ETHER_SUBTYPE(x) | IFM_ETHER }

#[inline(always)]
pub const fn IFM_ETHER_SUBTYPE_GET(x: u32) -> u32 { x & (IFM_TMASK | IFM_ETH_XTYPE) }

#[inline(always)]
pub const fn IFM_ETHER_IS_EXTENDED(x: u32) -> u32 { x & IFM_ETH_XTYPE }

#[inline(always)]
pub const fn IFM_TYPE(x: u32) -> u32 { x & IFM_NMASK }

#[inline(always)]
pub const fn IFM_SUBTYPE(x: u32) -> u32 {
    if IFM_TYPE(x) == IFM_ETHER { IFM_ETHER_SUBTYPE_GET(x) } else { x & IFM_TMASK }
}

#[inline(always)]
pub const fn IFM_TYPE_MATCH(x: u32, y: u32) -> bool {
    IFM_TYPE(x) == IFM_TYPE(y) && IFM_SUBTYPE(x) == IFM_SUBTYPE(y)
}

#[inline(always)]
pub const fn IFM_TYPE_OPTIONS(x: u32) -> u32 { x & IFM_OMASK }

#[inline(always)]
pub const fn IFM_INST(x: u32) -> u32 { (x & IFM_IMASK) >> IFM_ISHIFT }

#[inline(always)]
pub const fn IFM_OPTIONS(x: u32) -> u32 { x & (IFM_OMASK | IFM_GMASK) }

#[inline(always)]
pub const fn IFM_MODE(x: u32) -> u32 { x & IFM_MMASK }

#[inline(always)]
pub const fn IFM_MAKEWORD(type_: u32, subtype: u32, options: u32, instance: u32) -> u32 {
    type_ | subtype | options | (instance << IFM_ISHIFT)
}

#[inline(always)]
pub const fn IFM_MAKEMODE(mode: u32) -> u32 { (mode << IFM_MSHIFT) & IFM_MMASK }
