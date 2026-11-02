//! `sys/socket.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const SOCK_STREAM: u32 = 1;
pub const SOCK_DGRAM: u32 = 2;
pub const SOCK_RAW: u32 = 3;
pub const SOCK_RDM: u32 = 4;
pub const SOCK_SEQPACKET: u32 = 5;
pub const SOCK_CLOEXEC: u32 = 0x10000000;
pub const SOCK_NONBLOCK: u32 = 0x20000000;
pub const SO_DEBUG: u32 = 0x1;
pub const SO_ACCEPTCONN: u32 = 0x2;
pub const SO_REUSEADDR: u32 = 0x4;
pub const SO_KEEPALIVE: u32 = 0x8;
pub const SO_DONTROUTE: u32 = 0x10;
pub const SO_BROADCAST: u32 = 0x20;
pub const SO_USELOOPBACK: u32 = 0x40;
pub const SO_LINGER: u32 = 0x80;
pub const SO_OOBINLINE: u32 = 0x100;
pub const SO_REUSEPORT: u32 = 0x200;
pub const SO_TIMESTAMP: u32 = 0x400;
pub const SO_NOSIGPIPE: u32 = 0x800;
pub const SO_ACCEPTFILTER: u32 = 0x1000;
pub const SO_BINTIME: u32 = 0x2000;
pub const SO_NO_OFFLOAD: u32 = 0x4000;
pub const SO_NO_DDP: u32 = 0x8000;
pub const SO_SNDBUF: u32 = 0x1001;
pub const SO_RCVBUF: u32 = 0x1002;
pub const SO_SNDLOWAT: u32 = 0x1003;
pub const SO_RCVLOWAT: u32 = 0x1004;
pub const SO_SNDTIMEO: u32 = 0x1005;
pub const SO_RCVTIMEO: u32 = 0x1006;
pub const SO_ERROR: u32 = 0x1007;
pub const SO_TYPE: u32 = 0x1008;
pub const SO_LABEL: u32 = 0x1009;
pub const SO_PEERLABEL: u32 = 0x1010;
pub const SO_LISTENQLIMIT: u32 = 0x1011;
pub const SO_LISTENQLEN: u32 = 0x1012;
pub const SO_LISTENINCQLEN: u32 = 0x1013;
pub const SO_SETFIB: u32 = 0x1014;
pub const SO_USER_COOKIE: u32 = 0x1015;
pub const SO_PROTOCOL: u32 = 0x1016;
pub const SO_PROTOTYPE: u32 = 0x1016;
pub const SO_TS_CLOCK: u32 = 0x1017;
pub const SO_MAX_PACING_RATE: u32 = 0x1018;
pub const SO_TS_REALTIME_MICRO: u32 = 0;
pub const SO_TS_BINTIME: u32 = 1;
pub const SO_TS_REALTIME: u32 = 2;
pub const SO_TS_MONOTONIC: u32 = 3;
pub const SO_TS_DEFAULT: u32 = 0;
pub const SO_TS_CLOCK_MAX: u32 = 3;
pub const SO_VENDOR: u32 = 0x80000000;
pub const SOL_SOCKET: u32 = 0xffff;
pub const AF_UNSPEC: u32 = 0;
pub const AF_LOCAL: u32 = 1;
pub const AF_UNIX: u32 = 1;
pub const AF_INET: u32 = 2;
pub const AF_IMPLINK: u32 = 3;
pub const AF_PUP: u32 = 4;
pub const AF_CHAOS: u32 = 5;
pub const AF_NETBIOS: u32 = 6;
pub const AF_ISO: u32 = 7;
pub const AF_OSI: u32 = 7;
pub const AF_ECMA: u32 = 8;
pub const AF_DATAKIT: u32 = 9;
pub const AF_CCITT: u32 = 10;
pub const AF_SNA: u32 = 11;
pub const AF_DECnet: u32 = 12;
pub const AF_DLI: u32 = 13;
pub const AF_LAT: u32 = 14;
pub const AF_HYLINK: u32 = 15;
pub const AF_APPLETALK: u32 = 16;
pub const AF_ROUTE: u32 = 17;
pub const AF_LINK: u32 = 18;
pub const pseudo_AF_XTP: u32 = 19;
pub const AF_COIP: u32 = 20;
pub const AF_CNT: u32 = 21;
pub const pseudo_AF_RTIP: u32 = 22;
pub const AF_IPX: u32 = 23;
pub const AF_SIP: u32 = 24;
pub const pseudo_AF_PIP: u32 = 25;
pub const AF_ISDN: u32 = 26;
pub const AF_E164: u32 = 26;
pub const pseudo_AF_KEY: u32 = 27;
pub const AF_INET6: u32 = 28;
pub const AF_NATM: u32 = 29;
pub const AF_ATM: u32 = 30;
pub const pseudo_AF_HDRCMPLT: u32 = 31;
pub const AF_NETGRAPH: u32 = 32;
pub const AF_SLOW: u32 = 33;
pub const AF_SCLUSTER: u32 = 34;
pub const AF_ARP: u32 = 35;
pub const AF_BLUETOOTH: u32 = 36;
pub const AF_IEEE80211: u32 = 37;
pub const AF_INET_SDP: u32 = 40;
pub const AF_INET6_SDP: u32 = 42;
pub const AF_MAX: u32 = 42;
pub const AF_VENDOR00: u32 = 39;
pub const AF_VENDOR01: u32 = 41;
pub const AF_VENDOR02: u32 = 43;
pub const AF_VENDOR03: u32 = 45;
pub const AF_VENDOR04: u32 = 47;
pub const AF_VENDOR05: u32 = 49;
pub const AF_VENDOR06: u32 = 51;
pub const AF_VENDOR07: u32 = 53;
pub const AF_VENDOR08: u32 = 55;
pub const AF_VENDOR09: u32 = 57;
pub const AF_VENDOR10: u32 = 59;
pub const AF_VENDOR11: u32 = 61;
pub const AF_VENDOR12: u32 = 63;
pub const AF_VENDOR13: u32 = 65;
pub const AF_VENDOR14: u32 = 67;
pub const AF_VENDOR15: u32 = 69;
pub const AF_VENDOR16: u32 = 71;
pub const AF_VENDOR17: u32 = 73;
pub const AF_VENDOR18: u32 = 75;
pub const AF_VENDOR19: u32 = 77;
pub const AF_VENDOR20: u32 = 79;
pub const AF_VENDOR21: u32 = 81;
pub const AF_VENDOR22: u32 = 83;
pub const AF_VENDOR23: u32 = 85;
pub const AF_VENDOR24: u32 = 87;
pub const AF_VENDOR25: u32 = 89;
pub const AF_VENDOR26: u32 = 91;
pub const AF_VENDOR27: u32 = 93;
pub const AF_VENDOR28: u32 = 95;
pub const AF_VENDOR29: u32 = 97;
pub const AF_VENDOR30: u32 = 99;
pub const AF_VENDOR31: u32 = 101;
pub const AF_VENDOR32: u32 = 103;
pub const AF_VENDOR33: u32 = 105;
pub const AF_VENDOR34: u32 = 107;
pub const AF_VENDOR35: u32 = 109;
pub const AF_VENDOR36: u32 = 111;
pub const AF_VENDOR37: u32 = 113;
pub const AF_VENDOR38: u32 = 115;
pub const AF_VENDOR39: u32 = 117;
pub const AF_VENDOR40: u32 = 119;
pub const AF_VENDOR41: u32 = 121;
pub const AF_VENDOR42: u32 = 123;
pub const AF_VENDOR43: u32 = 125;
pub const AF_VENDOR44: u32 = 127;
pub const AF_VENDOR45: u32 = 129;
pub const AF_VENDOR46: u32 = 131;
pub const AF_VENDOR47: u32 = 133;
pub const SOCK_MAXADDRLEN: u32 = 255;
pub const PF_UNSPEC: u32 = 0;
pub const PF_LOCAL: u32 = 1;
pub const PF_UNIX: u32 = 1;
pub const PF_INET: u32 = 2;
pub const PF_IMPLINK: u32 = 3;
pub const PF_PUP: u32 = 4;
pub const PF_CHAOS: u32 = 5;
pub const PF_NETBIOS: u32 = 6;
pub const PF_ISO: u32 = 7;
pub const PF_OSI: u32 = 7;
pub const PF_ECMA: u32 = 8;
pub const PF_DATAKIT: u32 = 9;
pub const PF_CCITT: u32 = 10;
pub const PF_SNA: u32 = 11;
pub const PF_DECnet: u32 = 12;
pub const PF_DLI: u32 = 13;
pub const PF_LAT: u32 = 14;
pub const PF_HYLINK: u32 = 15;
pub const PF_APPLETALK: u32 = 16;
pub const PF_ROUTE: u32 = 17;
pub const PF_LINK: u32 = 18;
pub const PF_XTP: u32 = 19;
pub const PF_COIP: u32 = 20;
pub const PF_CNT: u32 = 21;
pub const PF_SIP: u32 = 24;
pub const PF_IPX: u32 = 23;
pub const PF_RTIP: u32 = 22;
pub const PF_PIP: u32 = 25;
pub const PF_ISDN: u32 = 26;
pub const PF_KEY: u32 = 27;
pub const PF_INET6: u32 = 28;
pub const PF_NATM: u32 = 29;
pub const PF_ATM: u32 = 30;
pub const PF_NETGRAPH: u32 = 32;
pub const PF_SLOW: u32 = 33;
pub const PF_SCLUSTER: u32 = 34;
pub const PF_ARP: u32 = 35;
pub const PF_BLUETOOTH: u32 = 36;
pub const PF_IEEE80211: u32 = 37;
pub const PF_INET_SDP: u32 = 40;
pub const PF_INET6_SDP: u32 = 42;
pub const PF_MAX: u32 = 42;
pub const NET_RT_DUMP: u32 = 1;
pub const NET_RT_FLAGS: u32 = 2;
pub const NET_RT_IFLIST: u32 = 3;
pub const NET_RT_IFMALIST: u32 = 4;
pub const NET_RT_IFLISTL: u32 = 5;
pub const SOMAXCONN: u32 = 128;
pub const MSG_OOB: u32 = 0x1;
pub const MSG_PEEK: u32 = 0x2;
pub const MSG_DONTROUTE: u32 = 0x4;
pub const MSG_EOR: u32 = 0x8;
pub const MSG_TRUNC: u32 = 0x10;
pub const MSG_CTRUNC: u32 = 0x20;
pub const MSG_WAITALL: u32 = 0x40;
pub const MSG_DONTWAIT: u32 = 0x80;
pub const MSG_EOF: u32 = 0x100;
pub const MSG_NOTIFICATION: u32 = 0x2000;
pub const MSG_NBIO: u32 = 0x4000;
pub const MSG_COMPAT: u32 = 0x8000;
pub const MSG_NOSIGNAL: u32 = 0x20000;
pub const MSG_CMSG_CLOEXEC: u32 = 0x40000;
pub const MSG_WAITFORONE: u32 = 0x80000;
pub const _ALIGNBYTES: u64 = 7;
pub const SCM_RIGHTS: u32 = 0x1;
pub const SCM_TIMESTAMP: u32 = 0x2;
pub const SCM_CREDS: u32 = 0x3;
pub const SCM_BINTIME: u32 = 0x4;
pub const SCM_REALTIME: u32 = 0x5;
pub const SCM_MONOTONIC: u32 = 0x6;
pub const SCM_TIME_INFO: u32 = 0x7;
pub const ST_INFO_HW: u32 = 0x1;
pub const ST_INFO_HW_HPREC: u32 = 0x2;
pub const SHUT_RD: u32 = 0;
pub const SHUT_WR: u32 = 1;
pub const SHUT_RDWR: u32 = 2;
pub const PRU_FLUSH_RD: u32 = 0;
pub const PRU_FLUSH_WR: u32 = 1;
pub const PRU_FLUSH_RDWR: u32 = 2;
pub const SF_NODISKIO: u32 = 0x1;
pub const SF_MNOWAIT: u32 = 0x2;
pub const SF_SYNC: u32 = 0x4;
pub const SF_USER_READAHEAD: u32 = 0x8;
pub const SF_NOCACHE: u32 = 0x10;

pub type sa_family_t = __sa_family_t;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct linger {
    pub l_onoff: c_int,
    pub l_linger: c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct accept_filter_arg {
    pub af_name: [c_char; 16],
    pub af_arg: [c_char; 240],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct sockaddr {
    pub sa_len: c_uchar,
    pub sa_family: sa_family_t,
    pub sa_data: [c_char; 14],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct sockproto {
    pub sp_family: c_ushort,
    pub sp_protocol: c_ushort,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct msghdr {
    pub msg_name: *mut c_void,
    pub msg_namelen: socklen_t,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: c_int,
    pub msg_control: *mut c_void,
    pub msg_controllen: socklen_t,
    pub msg_flags: c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cmsghdr {
    pub cmsg_len: socklen_t,
    pub cmsg_level: c_int,
    pub cmsg_type: c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct sock_timestamp_info {
    pub st_info_flags: u32,
    pub st_info_pad0: u32,
    pub st_info_rsv: [u64; 7],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct osockaddr {
    pub sa_family: c_ushort,
    pub sa_data: [c_char; 14],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct omsghdr {
    pub msg_name: *mut c_char,
    pub msg_namelen: c_int,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: c_int,
    pub msg_accrights: *mut c_char,
    pub msg_accrightslen: c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct sf_hdtr {
    pub headers: *mut iovec,
    pub hdr_cnt: c_int,
    pub trailers: *mut iovec,
    pub trl_cnt: c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct mmsghdr {
    pub msg_hdr: msghdr,
    pub msg_len: isize,
}

impl_zeroed_default!(linger, accept_filter_arg, sockaddr, sockproto, msghdr, cmsghdr, sock_timestamp_info, osockaddr, omsghdr, sf_hdtr, mmsghdr);

#[inline(always)]
pub const fn _ALIGN(p: usize) -> usize {
    (p + _ALIGNBYTES as usize) & !(_ALIGNBYTES as usize)
}

/// Given a pointer to a `cmsghdr`, returns a pointer to its data.
#[inline(always)]
pub fn CMSG_DATA(cmsg: *mut cmsghdr) -> *mut c_uchar {
    cmsg.cast::<c_uchar>().wrapping_add(_ALIGN(size_of::<cmsghdr>()))
}

/// Given a pointer to a `cmsghdr`, returns a pointer to the next one (or null).
///
/// # Safety
/// `mhdr` and `cmsg` must describe a valid control-message buffer.
#[inline(always)]
pub unsafe fn CMSG_NXTHDR(mhdr: *const msghdr, cmsg: *const cmsghdr) -> *mut cmsghdr {
    unsafe {
        let next = cmsg as usize + _ALIGN((*cmsg).cmsg_len as usize);
        let end = (*mhdr).msg_control as usize + (*mhdr).msg_controllen as usize;
        if next + _ALIGN(size_of::<cmsghdr>()) > end { core::ptr::null_mut() } else { next as *mut cmsghdr }
    }
}

/// Returns the first `cmsghdr` of a message (or null).
///
/// # Safety
/// `mhdr` must point to a valid `msghdr`.
#[inline(always)]
pub unsafe fn CMSG_FIRSTHDR(mhdr: *const msghdr) -> *mut cmsghdr {
    unsafe {
        if (*mhdr).msg_controllen as usize >= size_of::<cmsghdr>() { (*mhdr).msg_control.cast() } else { core::ptr::null_mut() }
    }
}

#[inline(always)]
pub const fn CMSG_LEN(len: usize) -> usize {
    _ALIGN(size_of::<cmsghdr>()) + len
}

#[inline(always)]
pub const fn CMSG_SPACE(len: usize) -> usize {
    _ALIGN(size_of::<cmsghdr>()) + _ALIGN(len)
}

#[inline(always)]
pub const fn SF_FLAGS(rh: c_int, flags: c_int) -> c_int {
    (rh << 16) | flags
}

unsafe extern "C" {
    pub fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;

    pub fn recv(sockfd: c_int, buf: *mut c_void, len: usize, flags: c_int) -> isize;

    pub fn recvfrom(sockfd: c_int, buf: *mut c_void, len: usize, flags: c_int, src_addr: *mut sockaddr, addrlen: *mut socklen_t) -> isize;

    pub fn send(sockfd: c_int, buf: *const c_void, len: usize, flags: c_int) -> isize;

    pub fn sendto(sockfd: c_int, buf: *const c_void, len: usize, flags: c_int, dest_addr: *const sockaddr, addrlen: socklen_t) -> isize;

    pub fn accept(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;

    pub fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;

    pub fn connect(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;

    pub fn getpeername(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;

    pub fn getsockname(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;

    pub fn getsockopt(sockfd: c_int, level: c_int, optname: c_int, optval: *mut c_void, optlen: *mut socklen_t) -> c_int;

    pub fn listen(sockfd: c_int, backlog: c_int) -> c_int;

    pub fn setsockopt(sockfd: c_int, level: c_int, optname: c_int, optval: *const c_void, optlen: socklen_t) -> c_int;

    pub fn shutdown(sockfd: c_int, how: c_int) -> c_int;

    pub fn sockatmark(sockfd: c_int) -> c_int;

    pub fn socketpair(domain: c_int, type_: c_int, protocol: c_int, sv: *mut c_int) -> c_int;

    pub fn recvmsg(sockfd: c_int, msg: *mut msghdr, flags: c_int) -> isize;

    pub fn sendmsg(sockfd: c_int, msg: *const msghdr, flags: c_int) -> isize;

    pub fn sendmmsg(sockfd: c_int, msgvec: *mut mmsghdr, vlen: c_uint, flags: c_int) -> c_int;

    pub fn recvmmsg(sockfd: c_int, msgvec: *mut mmsghdr, vlen: c_uint, flags: c_int, timeout: *mut timespec) -> c_int;
}
