//! `net/if.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const IF_NAMESIZE: u32 = 16;
pub const IFNAMSIZ: u32 = 16;
pub const IF_MAXUNIT: u32 = 0x7fff;
pub const IFF_UP: u32 = 0x1;
pub const IFF_BROADCAST: u32 = 0x2;
pub const IFF_DEBUG: u32 = 0x4;
pub const IFF_LOOPBACK: u32 = 0x8;
pub const IFF_POINTOPOINT: u32 = 0x10;
pub const IFF_DRV_RUNNING: u32 = 0x40;
pub const IFF_NOARP: u32 = 0x80;
pub const IFF_PROMISC: u32 = 0x100;
pub const IFF_ALLMULTI: u32 = 0x200;
pub const IFF_DRV_OACTIVE: u32 = 0x400;
pub const IFF_SIMPLEX: u32 = 0x800;
pub const IFF_LINK0: u32 = 0x1000;
pub const IFF_LINK1: u32 = 0x2000;
pub const IFF_LINK2: u32 = 0x4000;
pub const IFF_ALTPHYS: u32 = 0x4000;
pub const IFF_MULTICAST: u32 = 0x8000;
pub const IFF_CANTCONFIG: u32 = 0x10000;
pub const IFF_PPROMISC: u32 = 0x20000;
pub const IFF_MONITOR: u32 = 0x40000;
pub const IFF_STATICARP: u32 = 0x80000;
pub const IFF_DYING: u32 = 0x200000;
pub const IFF_RENAMING: u32 = 0x400000;
pub const IFF_RUNNING: u32 = 64;
pub const IFF_OACTIVE: u32 = 1024;
pub const IFF_CANTCHANGE: u32 = 0x218f52;
pub const LINK_STATE_UNKNOWN: u32 = 0;
pub const LINK_STATE_DOWN: u32 = 1;
pub const LINK_STATE_UP: u32 = 2;
pub const IFCAP_RXCSUM: u32 = 0x1;
pub const IFCAP_TXCSUM: u32 = 0x2;
pub const IFCAP_NETCONS: u32 = 0x4;
pub const IFCAP_VLAN_MTU: u32 = 0x8;
pub const IFCAP_VLAN_HWTAGGING: u32 = 0x10;
pub const IFCAP_JUMBO_MTU: u32 = 0x20;
pub const IFCAP_POLLING: u32 = 0x40;
pub const IFCAP_VLAN_HWCSUM: u32 = 0x80;
pub const IFCAP_TSO4: u32 = 0x100;
pub const IFCAP_TSO6: u32 = 0x200;
pub const IFCAP_LRO: u32 = 0x400;
pub const IFCAP_WOL_UCAST: u32 = 0x800;
pub const IFCAP_WOL_MCAST: u32 = 0x1000;
pub const IFCAP_WOL_MAGIC: u32 = 0x2000;
pub const IFCAP_TOE4: u32 = 0x4000;
pub const IFCAP_TOE6: u32 = 0x8000;
pub const IFCAP_VLAN_HWFILTER: u32 = 0x10000;
pub const IFCAP_VLAN_HWTSO: u32 = 0x40000;
pub const IFCAP_LINKSTATE: u32 = 0x80000;
pub const IFCAP_NETMAP: u32 = 0x100000;
pub const IFCAP_RXCSUM_IPV6: u32 = 0x200000;
pub const IFCAP_TXCSUM_IPV6: u32 = 0x400000;
pub const IFCAP_HWSTATS: u32 = 0x800000;
pub const IFCAP_TXRTLMT: u32 = 0x1000000;
pub const IFCAP_HWRXTSTMP: u32 = 0x2000000;
pub const IFCAP_HWCSUM_IPV6: u32 = 0x600000;
pub const IFCAP_HWCSUM: u32 = 3;
pub const IFCAP_TSO: u32 = 768;
pub const IFCAP_WOL: u32 = 0x3800;
pub const IFCAP_TOE: u32 = 0xc000;
pub const IFCAP_CANTCHANGE: u32 = 0x100000;
pub const IFQ_MAXLEN: u32 = 50;
pub const IFNET_SLOWHZ: u32 = 1;
pub const IFAN_ARRIVAL: u32 = 0;
pub const IFAN_DEPARTURE: u32 = 1;
pub const IFSTATMAX: u32 = 800;
pub const IFG_ALL: &CStr = c"all";
pub const IFG_EGRESS: &CStr = c"egress";
pub const RSS_FUNC_NONE: u32 = 0;
pub const RSS_FUNC_PRIVATE: u32 = 1;
pub const RSS_FUNC_TOEPLITZ: u32 = 2;
pub const RSS_TYPE_IPV4: u32 = 0x1;
pub const RSS_TYPE_TCP_IPV4: u32 = 0x2;
pub const RSS_TYPE_IPV6: u32 = 0x4;
pub const RSS_TYPE_IPV6_EX: u32 = 0x8;
pub const RSS_TYPE_TCP_IPV6: u32 = 0x10;
pub const RSS_TYPE_TCP_IPV6_EX: u32 = 0x20;
pub const RSS_TYPE_UDP_IPV4: u32 = 0x40;
pub const RSS_TYPE_UDP_IPV6: u32 = 0x80;
pub const RSS_TYPE_UDP_IPV6_EX: u32 = 0x100;
pub const RSS_KEYLEN: u32 = 128;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct if_clonereq {
    pub ifcr_total: c_int,
    pub ifcr_count: c_int,
    pub ifcr_buffer: *mut c_char,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct if_data {
    pub ifi_type: u8,
    pub ifi_physical: u8,
    pub ifi_addrlen: u8,
    pub ifi_hdrlen: u8,
    pub ifi_link_state: u8,
    pub ifi_vhid: u8,
    pub ifi_datalen: u16,
    pub ifi_mtu: u32,
    pub ifi_metric: u32,
    pub ifi_baudrate: u64,
    pub ifi_ipackets: u64,
    pub ifi_ierrors: u64,
    pub ifi_opackets: u64,
    pub ifi_oerrors: u64,
    pub ifi_collisions: u64,
    pub ifi_ibytes: u64,
    pub ifi_obytes: u64,
    pub ifi_imcasts: u64,
    pub ifi_omcasts: u64,
    pub ifi_iqdrops: u64,
    pub ifi_oqdrops: u64,
    pub ifi_noproto: u64,
    pub ifi_hwassist: u64,
    pub ifi_epoch: time_t,
    pub ifi_lastchange: timeval,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct if_msghdr {
    pub ifm_msglen: u_short,
    pub ifm_version: u_char,
    pub ifm_type: u_char,
    pub ifm_addrs: c_int,
    pub ifm_flags: c_int,
    pub ifm_index: u_short,
    pub ifm_data: if_data,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct if_msghdrl {
    pub ifm_msglen: u_short,
    pub ifm_version: u_char,
    pub ifm_type: u_char,
    pub ifm_addrs: c_int,
    pub ifm_flags: c_int,
    pub ifm_index: u_short,
    pub _ifm_spare1: u_short,
    pub ifm_len: u_short,
    pub ifm_data_off: u_short,
    pub ifm_data: if_data,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ifa_msghdr {
    pub ifam_msglen: u_short,
    pub ifam_version: u_char,
    pub ifam_type: u_char,
    pub ifam_addrs: c_int,
    pub ifam_flags: c_int,
    pub ifam_index: u_short,
    pub ifam_metric: c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ifa_msghdrl {
    pub ifam_msglen: u_short,
    pub ifam_version: u_char,
    pub ifam_type: u_char,
    pub ifam_addrs: c_int,
    pub ifam_flags: c_int,
    pub ifam_index: u_short,
    pub _ifam_spare1: u_short,
    pub ifam_len: u_short,
    pub ifam_data_off: u_short,
    pub ifam_metric: c_int,
    pub ifam_data: if_data,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ifma_msghdr {
    pub ifmam_msglen: u_short,
    pub ifmam_version: u_char,
    pub ifmam_type: u_char,
    pub ifmam_addrs: c_int,
    pub ifmam_flags: c_int,
    pub ifmam_index: u_short,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct if_announcemsghdr {
    pub ifan_msglen: u_short,
    pub ifan_version: u_char,
    pub ifan_type: u_char,
    pub ifan_index: u_short,
    pub ifan_name: [c_char; 16],
    pub ifan_what: u_short,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ifreq_buffer {
    pub length: usize,
    pub buffer: *mut c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union ifreq_anon0 {
    pub ifr_addr: sockaddr,
    pub ifr_dstaddr: sockaddr,
    pub ifr_broadaddr: sockaddr,
    pub ifr_buffer: ifreq_buffer,
    pub ifr_flags: c_short,
    pub ifr_flagshigh: c_short,
    pub ifr_index: c_short,
    pub ifr_jid: c_int,
    pub ifr_metric: c_int,
    pub ifr_mtu: c_int,
    pub ifr_phys: c_int,
    pub ifr_media: c_int,
    pub ifr_data: caddr_t,
    pub ifr_cap: [c_int; 2],
    pub ifr_fib: u_int,
    pub ifr_vlan_pcp: u_char,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ifreq {
    pub ifr_name: [c_char; 16],
    pub anon0: ifreq_anon0,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ifaliasreq {
    pub ifra_name: [c_char; 16],
    pub ifra_addr: sockaddr,
    pub ifra_broadaddr: sockaddr,
    pub ifra_mask: sockaddr,
    pub ifra_vhid: c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct oifaliasreq {
    pub ifra_name: [c_char; 16],
    pub ifra_addr: sockaddr,
    pub ifra_broadaddr: sockaddr,
    pub ifra_mask: sockaddr,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ifmediareq {
    pub ifm_name: [c_char; 16],
    pub ifm_current: c_int,
    pub ifm_mask: c_int,
    pub ifm_status: c_int,
    pub ifm_active: c_int,
    pub ifm_count: c_int,
    pub ifm_ulist: *mut c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ifdrv {
    pub ifd_name: [c_char; 16],
    pub ifd_cmd: c_ulong,
    pub ifd_len: usize,
    pub ifd_data: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ifstat {
    pub ifs_name: [c_char; 16],
    pub ascii: [c_char; 801],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union ifconf_anon0 {
    pub ifc_buf: caddr_t,
    pub ifc_req: *mut ifreq,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ifconf {
    pub ifc_len: c_int,
    pub anon0: ifconf_anon0,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union ifg_req_anon0 {
    pub ifgrq_group: [c_char; 16],
    pub ifgrq_member: [c_char; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ifg_req {
    pub anon0: ifg_req_anon0,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union ifgroupreq_anon0 {
    pub ifgr_group: [c_char; 16],
    pub ifgr_groups: *mut ifg_req,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ifgroupreq {
    pub ifgr_name: [c_char; 16],
    pub ifgr_len: u_int,
    pub anon0: ifgroupreq_anon0,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ifi2creq {
    pub dev_addr: u8,
    pub offset: u8,
    pub len: u8,
    pub spare0: u8,
    pub spare1: u32,
    pub data: [u8; 8],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ifrsskey {
    pub ifrk_name: [c_char; 16],
    pub ifrk_func: u8,
    pub ifrk_spare0: u8,
    pub ifrk_keylen: u16,
    pub ifrk_key: [u8; 128],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ifrsshash {
    pub ifrh_name: [c_char; 16],
    pub ifrh_func: u8,
    pub ifrh_spare0: u8,
    pub ifrh_spare1: u16,
    pub ifrh_types: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct if_nameindex {
    pub if_index: c_uint,
    pub if_name: *mut c_char,
}

impl_zeroed_default!(if_clonereq, if_data, if_msghdr, if_msghdrl, ifa_msghdr, ifa_msghdrl, ifma_msghdr, if_announcemsghdr, ifreq_buffer, ifreq_anon0, ifreq, ifaliasreq, oifaliasreq, ifmediareq, ifdrv, ifstat, ifconf_anon0, ifconf, ifg_req_anon0, ifg_req, ifgroupreq_anon0, ifgroupreq, ifi2creq, ifrsskey, ifrsshash, if_nameindex);

/// Kilobits/sec.
#[inline(always)]
pub const fn IF_Kbps(x: u64) -> u64 { x * 1000 }

/// Megabits/sec.
#[inline(always)]
pub const fn IF_Mbps(x: u64) -> u64 { IF_Kbps(x * 1000) }

/// Gigabits/sec.
#[inline(always)]
pub const fn IF_Gbps(x: u64) -> u64 { IF_Mbps(x * 1000) }
