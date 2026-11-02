//! `net/route.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const RT_L2_ME_BIT: u32 = 2;
pub const RT_MAY_LOOP_BIT: u32 = 3;
pub const RT_HAS_HEADER_BIT: u32 = 4;
pub const RT_L2_ME: u32 = 4;
pub const RT_MAY_LOOP: u32 = 8;
pub const RT_HAS_HEADER: u32 = 16;
pub const RT_REJECT: u32 = 0x20;
pub const RT_BLACKHOLE: u32 = 0x40;
pub const RT_HAS_GW: u32 = 0x80;
pub const RT_LLE_CACHE: u32 = 0x100;
pub const RTM_RTTUNIT: u32 = 1000000;
pub const RT_DEFAULT_FIB: u32 = 0;
pub const RT_ALL_FIBS: i32 = -1;
pub const RTF_UP: u32 = 0x1;
pub const RTF_GATEWAY: u32 = 0x2;
pub const RTF_HOST: u32 = 0x4;
pub const RTF_REJECT: u32 = 0x8;
pub const RTF_DYNAMIC: u32 = 0x10;
pub const RTF_MODIFIED: u32 = 0x20;
pub const RTF_DONE: u32 = 0x40;
pub const RTF_XRESOLVE: u32 = 0x200;
pub const RTF_LLINFO: u32 = 0x400;
pub const RTF_LLDATA: u32 = 0x400;
pub const RTF_STATIC: u32 = 0x800;
pub const RTF_BLACKHOLE: u32 = 0x1000;
pub const RTF_PROTO2: u32 = 0x4000;
pub const RTF_PROTO1: u32 = 0x8000;
pub const RTF_PROTO3: u32 = 0x40000;
pub const RTF_FIXEDMTU: u32 = 0x80000;
pub const RTF_PINNED: u32 = 0x100000;
pub const RTF_LOCAL: u32 = 0x200000;
pub const RTF_BROADCAST: u32 = 0x400000;
pub const RTF_MULTICAST: u32 = 0x800000;
pub const RTF_STICKY: u32 = 0x10000000;
pub const RTF_RNH_LOCKED: u32 = 0x40000000;
pub const RTF_GWFLAG_COMPAT: u32 = 0x80000000;
pub const RTF_FMASK: u32 = 0x1004d808;
pub const NHF_REJECT: u32 = 0x10;
pub const NHF_BLACKHOLE: u32 = 0x20;
pub const NHF_REDIRECT: u32 = 0x40;
pub const NHF_DEFAULT: u32 = 0x80;
pub const NHF_BROADCAST: u32 = 0x100;
pub const NHF_GATEWAY: u32 = 0x200;
pub const NHR_IFAIF: u32 = 0x1;
pub const NHR_REF: u32 = 0x2;
pub const NHR_COPY: u32 = 0x100;
pub const RTM_VERSION: u32 = 5;
pub const RTM_ADD: u32 = 0x1;
pub const RTM_DELETE: u32 = 0x2;
pub const RTM_CHANGE: u32 = 0x3;
pub const RTM_GET: u32 = 0x4;
pub const RTM_LOSING: u32 = 0x5;
pub const RTM_REDIRECT: u32 = 0x6;
pub const RTM_MISS: u32 = 0x7;
pub const RTM_LOCK: u32 = 0x8;
pub const RTM_RESOLVE: u32 = 0xb;
pub const RTM_NEWADDR: u32 = 0xc;
pub const RTM_DELADDR: u32 = 0xd;
pub const RTM_IFINFO: u32 = 0xe;
pub const RTM_NEWMADDR: u32 = 0xf;
pub const RTM_DELMADDR: u32 = 0x10;
pub const RTM_IFANNOUNCE: u32 = 0x11;
pub const RTM_IEEE80211: u32 = 0x12;
pub const RTV_MTU: u32 = 0x1;
pub const RTV_HOPCOUNT: u32 = 0x2;
pub const RTV_EXPIRE: u32 = 0x4;
pub const RTV_RPIPE: u32 = 0x8;
pub const RTV_SPIPE: u32 = 0x10;
pub const RTV_SSTHRESH: u32 = 0x20;
pub const RTV_RTT: u32 = 0x40;
pub const RTV_RTTVAR: u32 = 0x80;
pub const RTV_WEIGHT: u32 = 0x100;
pub const RTA_DST: u32 = 0x1;
pub const RTA_GATEWAY: u32 = 0x2;
pub const RTA_NETMASK: u32 = 0x4;
pub const RTA_GENMASK: u32 = 0x8;
pub const RTA_IFP: u32 = 0x10;
pub const RTA_IFA: u32 = 0x20;
pub const RTA_AUTHOR: u32 = 0x40;
pub const RTA_BRD: u32 = 0x80;
pub const RTAX_DST: u32 = 0;
pub const RTAX_GATEWAY: u32 = 1;
pub const RTAX_NETMASK: u32 = 2;
pub const RTAX_GENMASK: u32 = 3;
pub const RTAX_IFP: u32 = 4;
pub const RTAX_IFA: u32 = 5;
pub const RTAX_AUTHOR: u32 = 6;
pub const RTAX_BRD: u32 = 7;
pub const RTAX_MAX: u32 = 8;

#[repr(C)]
#[derive(Debug)]
pub struct rtentry {
    _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug)]
pub struct llentry {
    _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct route {
    pub ro_rt: *mut rtentry,
    pub ro_lle: *mut llentry,
    pub ro_prepend: *mut c_char,
    pub ro_plen: u16,
    pub ro_flags: u16,
    pub ro_mtu: u16,
    pub spare: u16,
    pub ro_dst: sockaddr,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct rt_metrics {
    pub rmx_locks: u_long,
    pub rmx_mtu: u_long,
    pub rmx_hopcount: u_long,
    pub rmx_expire: u_long,
    pub rmx_recvpipe: u_long,
    pub rmx_sendpipe: u_long,
    pub rmx_ssthresh: u_long,
    pub rmx_rtt: u_long,
    pub rmx_rttvar: u_long,
    pub rmx_pksent: u_long,
    pub rmx_weight: u_long,
    pub rmx_filler: [u_long; 3],
}

pub type rt_gen_t = u_int;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct rtstat {
    pub rts_badredirect: c_short,
    pub rts_dynamic: c_short,
    pub rts_newgateway: c_short,
    pub rts_unreach: c_short,
    pub rts_wildcard: c_short,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct rt_msghdr {
    pub rtm_msglen: u_short,
    pub rtm_version: u_char,
    pub rtm_type: u_char,
    pub rtm_index: u_short,
    pub rtm_flags: c_int,
    pub rtm_addrs: c_int,
    pub rtm_pid: pid_t,
    pub rtm_seq: c_int,
    pub rtm_errno: c_int,
    pub rtm_fmask: c_int,
    pub rtm_inits: u_long,
    pub rtm_rmx: rt_metrics,
}

pub type rt_filter_f_t = Option<unsafe extern "C" fn(*const rtentry, *mut c_void) -> c_int>;

#[repr(C)]
#[derive(Debug)]
pub struct ifaddr {
    _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct rt_addrinfo {
    pub rti_addrs: c_int,
    pub rti_flags: c_int,
    pub rti_info: [*mut sockaddr; 8],
    pub rti_ifa: *mut ifaddr,
    pub rti_ifp: *mut ifnet,
    pub rti_filter: rt_filter_f_t,
    pub rti_filterdata: *mut c_void,
    pub rti_mflags: u_long,
    pub rti_spare: u_long,
    pub rti_rmx: *mut rt_metrics,
}

impl_zeroed_default!(route, rt_metrics, rtstat, rt_msghdr, rt_addrinfo);
