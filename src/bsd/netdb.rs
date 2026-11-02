//! `netdb.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const IPPORT_RESERVED: u32 = 1024;
pub const NETDB_INTERNAL: i32 = -1;
pub const NETDB_SUCCESS: u32 = 0;
pub const HOST_NOT_FOUND: u32 = 1;
pub const TRY_AGAIN: u32 = 2;
pub const NO_RECOVERY: u32 = 3;
pub const NO_DATA: u32 = 4;
pub const NO_ADDRESS: u32 = 4;
pub const EAI_ADDRFAMILY: u32 = 1;
pub const EAI_AGAIN: u32 = 2;
pub const EAI_BADFLAGS: u32 = 3;
pub const EAI_FAIL: u32 = 4;
pub const EAI_FAMILY: u32 = 5;
pub const EAI_MEMORY: u32 = 6;
pub const EAI_NODATA: u32 = 7;
pub const EAI_NONAME: u32 = 8;
pub const EAI_SERVICE: u32 = 9;
pub const EAI_SOCKTYPE: u32 = 10;
pub const EAI_SYSTEM: u32 = 11;
pub const EAI_BADHINTS: u32 = 12;
pub const EAI_PROTOCOL: u32 = 13;
pub const EAI_OVERFLOW: u32 = 14;
pub const EAI_MAX: u32 = 15;
pub const AI_PASSIVE: u32 = 0x1;
pub const AI_CANONNAME: u32 = 0x2;
pub const AI_NUMERICHOST: u32 = 0x4;
pub const AI_NUMERICSERV: u32 = 0x8;
pub const AI_MASK: u32 = 3343;
pub const AI_ALL: u32 = 0x100;
pub const AI_V4MAPPED_CFG: u32 = 0x200;
pub const AI_ADDRCONFIG: u32 = 0x400;
pub const AI_V4MAPPED: u32 = 0x800;
pub const AI_DEFAULT: u32 = 1536;
pub const NI_MAXHOST: u32 = 1025;
pub const NI_MAXSERV: u32 = 32;
pub const NI_NOFQDN: u32 = 0x1;
pub const NI_NUMERICHOST: u32 = 0x2;
pub const NI_NAMEREQD: u32 = 0x4;
pub const NI_NUMERICSERV: u32 = 0x8;
pub const NI_DGRAM: u32 = 0x10;
pub const NI_NUMERICSCOPE: u32 = 0x20;
pub const SCOPE_DELIMITER: u32 = 37;

pub type socklen_t = __socklen_t;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct hostent {
    pub h_name: *mut c_char,
    pub h_aliases: *mut *mut c_char,
    pub h_addrtype: c_int,
    pub h_length: c_int,
    pub h_addr_list: *mut *mut c_char,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct netent {
    pub n_name: *mut c_char,
    pub n_aliases: *mut *mut c_char,
    pub n_addrtype: c_int,
    pub n_net: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct servent {
    pub s_name: *mut c_char,
    pub s_aliases: *mut *mut c_char,
    pub s_port: c_int,
    pub s_proto: *mut c_char,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct protoent {
    pub p_name: *mut c_char,
    pub p_aliases: *mut *mut c_char,
    pub p_proto: c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct addrinfo {
    pub ai_flags: c_int,
    pub ai_family: c_int,
    pub ai_socktype: c_int,
    pub ai_protocol: c_int,
    pub ai_addrlen: socklen_t,
    pub ai_canonname: *mut c_char,
    pub ai_addr: *mut sockaddr,
    pub ai_next: *mut addrinfo,
}

impl_zeroed_default!(hostent, netent, servent, protoent, addrinfo);

unsafe extern "C" {
    pub static mut h_errno: c_int;

    pub fn endhostent();

    pub fn endnetent();

    pub fn endprotoent();

    pub fn endservent();

    pub fn gethostbyname(name: *const c_char) -> *mut hostent;

    pub fn gethostbyaddr(addr: *const c_void, len: socklen_t, type_: c_int) -> *mut hostent;

    pub fn gethostent() -> *mut hostent;

    pub fn getnetbyaddr(arg0: u32, arg1: c_int) -> *mut netent;

    pub fn getnetbyname(arg0: *const c_char) -> *mut netent;

    pub fn getnetent() -> *mut netent;

    pub fn getprotobyname(arg0: *const c_char) -> *mut protoent;

    pub fn getprotobynumber(arg0: c_int) -> *mut protoent;

    pub fn getprotoent() -> *mut protoent;

    pub fn getservbyname(arg0: *const c_char, arg1: *const c_char) -> *mut servent;

    pub fn getservbyport(arg0: c_int, arg1: *const c_char) -> *mut servent;

    pub fn getservent() -> *mut servent;

    pub fn sethostent(arg0: c_int);

    pub fn setnetent(arg0: c_int);

    pub fn setprotoent(arg0: c_int);

    pub fn getnameinfo(sa: *const sockaddr, salen: socklen_t, host: *mut c_char, hostlen: socklen_t, serv: *mut c_char, servlen: socklen_t, flags: c_int) -> c_int;

    pub fn getaddrinfo(node: *const c_char, service: *const c_char, hints: *const addrinfo, res: *mut *mut addrinfo) -> c_int;

    pub fn freeaddrinfo(ai: *mut addrinfo);

    pub fn gai_strerror(err: c_int) -> *const c_char;

    pub fn setservent(arg0: c_int);

    pub fn freehostent(he: *mut hostent);

    pub fn herror(s: *const c_char);

    pub fn hstrerror(err: c_int) -> *const c_char;
}
