//! `netinet/in.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const IPPROTO_IP: u32 = 0;
pub const IPPROTO_ICMP: u32 = 1;
pub const IPPROTO_TCP: u32 = 6;
pub const IPPROTO_UDP: u32 = 17;
pub const INADDR_ANY: u32 = 0;
pub const INADDR_BROADCAST: u32 = 0xffffffff;
pub const IPPROTO_IPV6: u32 = 41;
pub const IPPROTO_RAW: u32 = 255;
pub const INET_ADDRSTRLEN: u32 = 16;
pub const IPPROTO_HOPOPTS: u32 = 0;
pub const IPPROTO_IGMP: u32 = 2;
pub const IPPROTO_GGP: u32 = 3;
pub const IPPROTO_IPV4: u32 = 4;
pub const IPPROTO_IPIP: u32 = 4;
pub const IPPROTO_ST: u32 = 7;
pub const IPPROTO_EGP: u32 = 8;
pub const IPPROTO_PIGP: u32 = 9;
pub const IPPROTO_RCCMON: u32 = 10;
pub const IPPROTO_NVPII: u32 = 11;
pub const IPPROTO_PUP: u32 = 12;
pub const IPPROTO_ARGUS: u32 = 13;
pub const IPPROTO_EMCON: u32 = 14;
pub const IPPROTO_XNET: u32 = 15;
pub const IPPROTO_CHAOS: u32 = 16;
pub const IPPROTO_MUX: u32 = 18;
pub const IPPROTO_MEAS: u32 = 19;
pub const IPPROTO_HMP: u32 = 20;
pub const IPPROTO_PRM: u32 = 21;
pub const IPPROTO_IDP: u32 = 22;
pub const IPPROTO_TRUNK1: u32 = 23;
pub const IPPROTO_TRUNK2: u32 = 24;
pub const IPPROTO_LEAF1: u32 = 25;
pub const IPPROTO_LEAF2: u32 = 26;
pub const IPPROTO_RDP: u32 = 27;
pub const IPPROTO_IRTP: u32 = 28;
pub const IPPROTO_TP: u32 = 29;
pub const IPPROTO_BLT: u32 = 30;
pub const IPPROTO_NSP: u32 = 31;
pub const IPPROTO_INP: u32 = 32;
pub const IPPROTO_SEP: u32 = 33;
pub const IPPROTO_3PC: u32 = 34;
pub const IPPROTO_IDPR: u32 = 35;
pub const IPPROTO_XTP: u32 = 36;
pub const IPPROTO_DDP: u32 = 37;
pub const IPPROTO_CMTP: u32 = 38;
pub const IPPROTO_TPXX: u32 = 39;
pub const IPPROTO_IL: u32 = 40;
pub const IPPROTO_SDRP: u32 = 42;
pub const IPPROTO_ROUTING: u32 = 43;
pub const IPPROTO_FRAGMENT: u32 = 44;
pub const IPPROTO_IDRP: u32 = 45;
pub const IPPROTO_RSVP: u32 = 46;
pub const IPPROTO_GRE: u32 = 47;
pub const IPPROTO_MHRP: u32 = 48;
pub const IPPROTO_BHA: u32 = 49;
pub const IPPROTO_ESP: u32 = 50;
pub const IPPROTO_AH: u32 = 51;
pub const IPPROTO_INLSP: u32 = 52;
pub const IPPROTO_SWIPE: u32 = 53;
pub const IPPROTO_NHRP: u32 = 54;
pub const IPPROTO_MOBILE: u32 = 55;
pub const IPPROTO_TLSP: u32 = 56;
pub const IPPROTO_SKIP: u32 = 57;
pub const IPPROTO_ICMPV6: u32 = 58;
pub const IPPROTO_NONE: u32 = 59;
pub const IPPROTO_DSTOPTS: u32 = 60;
pub const IPPROTO_AHIP: u32 = 61;
pub const IPPROTO_CFTP: u32 = 62;
pub const IPPROTO_HELLO: u32 = 63;
pub const IPPROTO_SATEXPAK: u32 = 64;
pub const IPPROTO_KRYPTOLAN: u32 = 65;
pub const IPPROTO_RVD: u32 = 66;
pub const IPPROTO_IPPC: u32 = 67;
pub const IPPROTO_ADFS: u32 = 68;
pub const IPPROTO_SATMON: u32 = 69;
pub const IPPROTO_VISA: u32 = 70;
pub const IPPROTO_IPCV: u32 = 71;
pub const IPPROTO_CPNX: u32 = 72;
pub const IPPROTO_CPHB: u32 = 73;
pub const IPPROTO_WSN: u32 = 74;
pub const IPPROTO_PVP: u32 = 75;
pub const IPPROTO_BRSATMON: u32 = 76;
pub const IPPROTO_ND: u32 = 77;
pub const IPPROTO_WBMON: u32 = 78;
pub const IPPROTO_WBEXPAK: u32 = 79;
pub const IPPROTO_EON: u32 = 80;
pub const IPPROTO_VMTP: u32 = 81;
pub const IPPROTO_SVMTP: u32 = 82;
pub const IPPROTO_VINES: u32 = 83;
pub const IPPROTO_TTP: u32 = 84;
pub const IPPROTO_IGP: u32 = 85;
pub const IPPROTO_DGP: u32 = 86;
pub const IPPROTO_TCF: u32 = 87;
pub const IPPROTO_IGRP: u32 = 88;
pub const IPPROTO_OSPFIGP: u32 = 89;
pub const IPPROTO_SRPC: u32 = 90;
pub const IPPROTO_LARP: u32 = 91;
pub const IPPROTO_MTP: u32 = 92;
pub const IPPROTO_AX25: u32 = 93;
pub const IPPROTO_IPEIP: u32 = 94;
pub const IPPROTO_MICP: u32 = 95;
pub const IPPROTO_SCCSP: u32 = 96;
pub const IPPROTO_ETHERIP: u32 = 97;
pub const IPPROTO_ENCAP: u32 = 98;
pub const IPPROTO_APES: u32 = 99;
pub const IPPROTO_GMTP: u32 = 100;
pub const IPPROTO_IPCOMP: u32 = 108;
pub const IPPROTO_SCTP: u32 = 132;
pub const IPPROTO_MH: u32 = 135;
pub const IPPROTO_UDPLITE: u32 = 136;
pub const IPPROTO_HIP: u32 = 139;
pub const IPPROTO_SHIM6: u32 = 140;
pub const IPPROTO_PIM: u32 = 103;
pub const IPPROTO_CARP: u32 = 112;
pub const IPPROTO_PGM: u32 = 113;
pub const IPPROTO_MPLS: u32 = 137;
pub const IPPROTO_PFSYNC: u32 = 240;
pub const IPPROTO_RESERVED_253: u32 = 253;
pub const IPPROTO_RESERVED_254: u32 = 254;
pub const IPPROTO_OLD_DIVERT: u32 = 254;
pub const IPPROTO_MAX: u32 = 256;
pub const IPPROTO_DONE: u32 = 257;
pub const IPPROTO_DIVERT: u32 = 258;
pub const IPPROTO_SEND: u32 = 259;
pub const IPPROTO_SPACER: u32 = 32767;
pub const IPPORT_EPHEMERALFIRST: u32 = 10000;
pub const IPPORT_EPHEMERALLAST: u32 = 65535;
pub const IPPORT_HIFIRSTAUTO: u32 = 49152;
pub const IPPORT_HILASTAUTO: u32 = 65535;
pub const IPPORT_RESERVEDSTART: u32 = 600;
pub const IPPORT_MAX: u32 = 65535;
pub const IN_CLASSA_NET: u32 = 0xff000000;
pub const IN_CLASSA_NSHIFT: u32 = 24;
pub const IN_CLASSA_HOST: u32 = 0xffffff;
pub const IN_CLASSA_MAX: u32 = 128;
pub const IN_CLASSB_NET: u32 = 0xffff0000;
pub const IN_CLASSB_NSHIFT: u32 = 16;
pub const IN_CLASSB_HOST: u32 = 0xffff;
pub const IN_CLASSB_MAX: u32 = 65536;
pub const IN_CLASSC_NET: u32 = 0xffffff00;
pub const IN_CLASSC_NSHIFT: u32 = 8;
pub const IN_CLASSC_HOST: u32 = 0xff;
pub const IN_CLASSD_NET: u32 = 0xf0000000;
pub const IN_CLASSD_NSHIFT: u32 = 28;
pub const IN_CLASSD_HOST: u32 = 0xfffffff;
pub const INADDR_LOOPBACK: u32 = 0x7f000001;
pub const INADDR_NONE: u32 = 0xffffffff;
pub const INADDR_UNSPEC_GROUP: u32 = 0xe0000000;
pub const INADDR_ALLHOSTS_GROUP: u32 = 0xe0000001;
pub const INADDR_ALLRTRS_GROUP: u32 = 0xe0000002;
pub const INADDR_ALLRPTS_GROUP: u32 = 0xe0000016;
pub const INADDR_CARP_GROUP: u32 = 0xe0000012;
pub const INADDR_PFSYNC_GROUP: u32 = 0xe00000f0;
pub const INADDR_ALLMDNS_GROUP: u32 = 0xe00000fb;
pub const INADDR_MAX_LOCAL_GROUP: u32 = 0xe00000ff;
pub const IN_LOOPBACKNET: u32 = 127;
pub const IN_RFC3021_MASK: u32 = 0xfffffffe;
pub const IP_OPTIONS: u32 = 1;
pub const IP_HDRINCL: u32 = 2;
pub const IP_TOS: u32 = 3;
pub const IP_TTL: u32 = 4;
pub const IP_RECVOPTS: u32 = 5;
pub const IP_RECVRETOPTS: u32 = 6;
pub const IP_RECVDSTADDR: u32 = 7;
pub const IP_SENDSRCADDR: u32 = 7;
pub const IP_RETOPTS: u32 = 8;
pub const IP_MULTICAST_IF: u32 = 9;
pub const IP_MULTICAST_TTL: u32 = 10;
pub const IP_MULTICAST_LOOP: u32 = 11;
pub const IP_ADD_MEMBERSHIP: u32 = 12;
pub const IP_DROP_MEMBERSHIP: u32 = 13;
pub const IP_MULTICAST_VIF: u32 = 14;
pub const IP_RSVP_ON: u32 = 15;
pub const IP_RSVP_OFF: u32 = 16;
pub const IP_RSVP_VIF_ON: u32 = 17;
pub const IP_RSVP_VIF_OFF: u32 = 18;
pub const IP_PORTRANGE: u32 = 19;
pub const IP_RECVIF: u32 = 20;
pub const IP_IPSEC_POLICY: u32 = 21;
pub const IP_ONESBCAST: u32 = 23;
pub const IP_BINDANY: u32 = 24;
pub const IP_BINDMULTI: u32 = 25;
pub const IP_RSS_LISTEN_BUCKET: u32 = 26;
pub const IP_ORIGDSTADDR: u32 = 27;
pub const IP_RECVORIGDSTADDR: u32 = 27;
pub const IP_FW_TABLE_ADD: u32 = 40;
pub const IP_FW_TABLE_DEL: u32 = 41;
pub const IP_FW_TABLE_FLUSH: u32 = 42;
pub const IP_FW_TABLE_GETSIZE: u32 = 43;
pub const IP_FW_TABLE_LIST: u32 = 44;
pub const IP_FW3: u32 = 48;
pub const IP_DUMMYNET3: u32 = 49;
pub const IP_FW_ADD: u32 = 50;
pub const IP_FW_DEL: u32 = 51;
pub const IP_FW_FLUSH: u32 = 52;
pub const IP_FW_ZERO: u32 = 53;
pub const IP_FW_GET: u32 = 54;
pub const IP_FW_RESETLOG: u32 = 55;
pub const IP_FW_NAT_CFG: u32 = 56;
pub const IP_FW_NAT_DEL: u32 = 57;
pub const IP_FW_NAT_GET_CONFIG: u32 = 58;
pub const IP_FW_NAT_GET_LOG: u32 = 59;
pub const IP_DUMMYNET_CONFIGURE: u32 = 60;
pub const IP_DUMMYNET_DEL: u32 = 61;
pub const IP_DUMMYNET_FLUSH: u32 = 62;
pub const IP_DUMMYNET_GET: u32 = 64;
pub const IP_RECVTTL: u32 = 65;
pub const IP_MINTTL: u32 = 66;
pub const IP_DONTFRAG: u32 = 67;
pub const IP_RECVTOS: u32 = 68;
pub const IP_ADD_SOURCE_MEMBERSHIP: u32 = 70;
pub const IP_DROP_SOURCE_MEMBERSHIP: u32 = 71;
pub const IP_BLOCK_SOURCE: u32 = 72;
pub const IP_UNBLOCK_SOURCE: u32 = 73;
pub const IP_MSFILTER: u32 = 74;
pub const MCAST_JOIN_GROUP: u32 = 80;
pub const MCAST_LEAVE_GROUP: u32 = 81;
pub const MCAST_JOIN_SOURCE_GROUP: u32 = 82;
pub const MCAST_LEAVE_SOURCE_GROUP: u32 = 83;
pub const MCAST_BLOCK_SOURCE: u32 = 84;
pub const MCAST_UNBLOCK_SOURCE: u32 = 85;
pub const IP_FLOWID: u32 = 90;
pub const IP_FLOWTYPE: u32 = 91;
pub const IP_RSSBUCKETID: u32 = 92;
pub const IP_RECVFLOWID: u32 = 93;
pub const IP_RECVRSSBUCKETID: u32 = 94;
pub const IP_DEFAULT_MULTICAST_TTL: u32 = 1;
pub const IP_DEFAULT_MULTICAST_LOOP: u32 = 1;
pub const IP_MIN_MEMBERSHIPS: u32 = 31;
pub const IP_MAX_MEMBERSHIPS: u32 = 4095;
pub const IP_MAX_SOURCE_FILTER: u32 = 1024;
pub const IP_MAX_GROUP_SRC_FILTER: u32 = 512;
pub const IP_MAX_SOCK_SRC_FILTER: u32 = 128;
pub const IP_MAX_SOCK_MUTE_FILTER: u32 = 128;
pub const MCAST_UNDEFINED: u32 = 0;
pub const MCAST_INCLUDE: u32 = 1;
pub const MCAST_EXCLUDE: u32 = 2;
pub const IP_PORTRANGE_DEFAULT: u32 = 0;
pub const IP_PORTRANGE_HIGH: u32 = 1;
pub const IP_PORTRANGE_LOW: u32 = 2;
pub const IPCTL_FORWARDING: u32 = 1;
pub const IPCTL_SENDREDIRECTS: u32 = 2;
pub const IPCTL_DEFTTL: u32 = 3;
pub const IPCTL_SOURCEROUTE: u32 = 8;
pub const IPCTL_DIRECTEDBROADCAST: u32 = 9;
pub const IPCTL_INTRQMAXLEN: u32 = 10;
pub const IPCTL_INTRQDROPS: u32 = 11;
pub const IPCTL_STATS: u32 = 12;
pub const IPCTL_ACCEPTSOURCEROUTE: u32 = 13;
pub const IPCTL_FASTFORWARDING: u32 = 14;
pub const IPCTL_GIF_TTL: u32 = 16;
pub const IPCTL_INTRDQMAXLEN: u32 = 17;
pub const IPCTL_INTRDQDROPS: u32 = 18;
pub const INET6_ADDRSTRLEN: u32 = 46;
pub const IPV6_UNICAST_HOPS: u32 = 4;
pub const IPV6_MULTICAST_IF: u32 = 9;
pub const IPV6_MULTICAST_HOPS: u32 = 10;
pub const IPV6_MULTICAST_LOOP: u32 = 11;
pub const IPV6_JOIN_GROUP: u32 = 12;
pub const IPV6_LEAVE_GROUP: u32 = 13;
pub const IPV6_PORTRANGE: u32 = 14;
pub const IPV6_V6ONLY: u32 = 27;
pub const __IPV6_ADDR_SCOPE_NODELOCAL: u32 = 0x1;
pub const __IPV6_ADDR_SCOPE_INTFACELOCAL: u32 = 0x1;
pub const __IPV6_ADDR_SCOPE_LINKLOCAL: u32 = 0x2;
pub const __IPV6_ADDR_SCOPE_SITELOCAL: u32 = 0x5;
pub const __IPV6_ADDR_SCOPE_ORGLOCAL: u32 = 0x8;
pub const __IPV6_ADDR_SCOPE_GLOBAL: u32 = 0xe;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct sockaddr_in {
    pub sin_len: u8,
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [c_char; 8],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ip_mreq {
    pub imr_multiaddr: in_addr,
    pub imr_interface: in_addr,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ip_mreqn {
    pub imr_multiaddr: in_addr,
    pub imr_address: in_addr,
    pub imr_ifindex: c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ip_mreq_source {
    pub imr_multiaddr: in_addr,
    pub imr_sourceaddr: in_addr,
    pub imr_interface: in_addr,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct group_req {
    pub gr_interface: u32,
    pub gr_group: sockaddr_storage,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct group_source_req {
    pub gsr_interface: u32,
    pub gsr_group: sockaddr_storage,
    pub gsr_source: sockaddr_storage,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union in6_addr_anon0 {
    pub __u6_addr16: [u16; 8],
    pub __u6_addr32: [u32; 4],
    pub s6_addr: [c_uchar; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct in6_addr {
    pub anon0: in6_addr_anon0,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: u32,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: u32,
}

impl_zeroed_default!(in_addr, sockaddr_in, ip_mreq, ip_mreqn, ip_mreq_source, group_req, group_source_req, in6_addr_anon0, in6_addr, sockaddr_in6);

#[inline(always)]
pub const fn IN_CLASSA(i: in_addr_t) -> bool { (i & 0x80000000) == 0 }

#[inline(always)]
pub const fn IN_CLASSB(i: in_addr_t) -> bool { (i & 0xc0000000) == 0x80000000 }

#[inline(always)]
pub const fn IN_CLASSC(i: in_addr_t) -> bool { (i & 0xe0000000) == 0xc0000000 }

#[inline(always)]
pub const fn IN_CLASSD(i: in_addr_t) -> bool { (i & 0xf0000000) == 0xe0000000 }

#[inline(always)]
pub const fn IN_MULTICAST(i: in_addr_t) -> bool { IN_CLASSD(i) }

#[inline(always)]
pub const fn IN_EXPERIMENTAL(i: in_addr_t) -> bool { (i & 0xf0000000) == 0xf0000000 }

#[inline(always)]
pub const fn IN_BADCLASS(i: in_addr_t) -> bool { (i & 0xf0000000) == 0xf0000000 }

#[inline(always)]
pub const fn IN_LINKLOCAL(i: in_addr_t) -> bool { (i & 0xffff0000) == 0xa9fe0000 }

#[inline(always)]
pub const fn IN_LOOPBACK(i: in_addr_t) -> bool { (i & 0xff000000) == 0x7f000000 }

#[inline(always)]
pub const fn IN_ZERONET(i: in_addr_t) -> bool { (i & 0xff000000) == 0 }

#[inline(always)]
pub const fn IN_PRIVATE(i: in_addr_t) -> bool {
    (i & 0xff000000) == 0x0a000000 || (i & 0xfff00000) == 0xac100000 || (i & 0xffff0000) == 0xc0a80000
}

#[inline(always)]
pub const fn IN_LOCAL_GROUP(i: in_addr_t) -> bool { (i & 0xffffff00) == 0xe0000000 }

#[inline(always)]
pub const fn IN_ANY_LOCAL(i: in_addr_t) -> bool { IN_LINKLOCAL(i) || IN_LOCAL_GROUP(i) }

/// The address as four native-endian words, like the C macros' `__u6_addr32` view.
#[inline(always)]
const fn in6_words(a: &in6_addr) -> [u32; 4] {
    // SAFETY: every bit pattern of the address union is a valid `[u32; 4]`.
    unsafe { a.anon0.__u6_addr32 }
}

#[inline(always)]
const fn in6_bytes(a: &in6_addr) -> [u8; 16] {
    // SAFETY: every bit pattern of the address union is a valid `[u8; 16]`.
    unsafe { a.anon0.s6_addr }
}

#[inline(always)]
pub const fn IN6_IS_ADDR_UNSPECIFIED(a: &in6_addr) -> bool {
    let w = in6_words(a);
    w[0] == 0 && w[1] == 0 && w[2] == 0 && w[3] == 0
}

#[inline(always)]
pub const fn IN6_IS_ADDR_LOOPBACK(a: &in6_addr) -> bool {
    let w = in6_words(a);
    w[0] == 0 && w[1] == 0 && w[2] == 0 && w[3] == ntohl(1)
}

#[inline(always)]
pub const fn IN6_IS_ADDR_V4COMPAT(a: &in6_addr) -> bool {
    let w = in6_words(a);
    w[0] == 0 && w[1] == 0 && w[2] == 0 && w[3] != 0 && w[3] != ntohl(1)
}

#[inline(always)]
pub const fn IN6_IS_ADDR_V4MAPPED(a: &in6_addr) -> bool {
    let w = in6_words(a);
    w[0] == 0 && w[1] == 0 && w[2] == ntohl(0x0000ffff)
}

#[inline(always)]
pub const fn IN6_IS_ADDR_LINKLOCAL(a: &in6_addr) -> bool {
    let b = in6_bytes(a);
    b[0] == 0xfe && (b[1] & 0xc0) == 0x80
}

#[inline(always)]
pub const fn IN6_IS_ADDR_SITELOCAL(a: &in6_addr) -> bool {
    let b = in6_bytes(a);
    b[0] == 0xfe && (b[1] & 0xc0) == 0xc0
}

#[inline(always)]
pub const fn IN6_IS_ADDR_MULTICAST(a: &in6_addr) -> bool {
    in6_bytes(a)[0] == 0xff
}

#[inline(always)]
pub const fn __IPV6_ADDR_MC_SCOPE(a: &in6_addr) -> u8 {
    in6_bytes(a)[1] & 0x0f
}

#[inline(always)]
pub const fn IN6_IS_ADDR_MC_NODELOCAL(a: &in6_addr) -> bool {
    IN6_IS_ADDR_MULTICAST(a) && __IPV6_ADDR_MC_SCOPE(a) as u32 == __IPV6_ADDR_SCOPE_NODELOCAL
}

#[inline(always)]
pub const fn IN6_IS_ADDR_MC_LINKLOCAL(a: &in6_addr) -> bool {
    IN6_IS_ADDR_MULTICAST(a) && __IPV6_ADDR_MC_SCOPE(a) as u32 == __IPV6_ADDR_SCOPE_LINKLOCAL
}

#[inline(always)]
pub const fn IN6_IS_ADDR_MC_SITELOCAL(a: &in6_addr) -> bool {
    IN6_IS_ADDR_MULTICAST(a) && __IPV6_ADDR_MC_SCOPE(a) as u32 == __IPV6_ADDR_SCOPE_SITELOCAL
}

#[inline(always)]
pub const fn IN6_IS_ADDR_MC_ORGLOCAL(a: &in6_addr) -> bool {
    IN6_IS_ADDR_MULTICAST(a) && __IPV6_ADDR_MC_SCOPE(a) as u32 == __IPV6_ADDR_SCOPE_ORGLOCAL
}

#[inline(always)]
pub const fn IN6_IS_ADDR_MC_GLOBAL(a: &in6_addr) -> bool {
    IN6_IS_ADDR_MULTICAST(a) && __IPV6_ADDR_MC_SCOPE(a) as u32 == __IPV6_ADDR_SCOPE_GLOBAL
}

unsafe extern "C" {
    pub static mut in6addr_any: in6_addr;

    pub static mut in6addr_loopback: in6_addr;
}
