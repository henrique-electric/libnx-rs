//! `sys/sockio.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const SIOCSHIWAT: i32 = -0x7ffb8d00;
pub const SIOCGHIWAT: u32 = 0x40047301;
pub const SIOCSLOWAT: i32 = -0x7ffb8cfe;
pub const SIOCGLOWAT: u32 = 0x40047303;
pub const SIOCATMARK: u32 = 0x40047307;
pub const SIOCSPGRP: i32 = -0x7ffb8cf8;
pub const SIOCGPGRP: u32 = 0x40047309;
pub const SIOCSIFADDR: i32 = -0x7fdf96f4;
pub const SIOCGIFADDR: i32 = -0x3fdf96df;
pub const SIOCSIFDSTADDR: i32 = -0x7fdf96f2;
pub const SIOCGIFDSTADDR: i32 = -0x3fdf96de;
pub const SIOCSIFFLAGS: i32 = -0x7fdf96f0;
pub const SIOCGIFFLAGS: i32 = -0x3fdf96ef;
pub const SIOCGIFBRDADDR: i32 = -0x3fdf96dd;
pub const SIOCSIFBRDADDR: i32 = -0x7fdf96ed;
pub const SIOCGIFCONF: i32 = -0x3fef96dc;
pub const SIOCGIFNETMASK: i32 = -0x3fdf96db;
pub const SIOCSIFNETMASK: i32 = -0x7fdf96ea;
pub const SIOCGIFMETRIC: i32 = -0x3fdf96e9;
pub const SIOCSIFMETRIC: i32 = -0x7fdf96e8;
pub const SIOCDIFADDR: i32 = -0x7fdf96e7;
pub const OSIOCAIFADDR: i32 = -0x7fbf96e6;
pub const SIOCSIFCAP: i32 = -0x7fdf96e2;
pub const SIOCGIFCAP: i32 = -0x3fdf96e1;
pub const SIOCGIFINDEX: i32 = -0x3fdf96e0;
pub const SIOCGIFMAC: i32 = -0x3fdf96da;
pub const SIOCSIFMAC: i32 = -0x7fdf96d9;
pub const SIOCSIFNAME: i32 = -0x7fdf96d8;
pub const SIOCSIFDESCR: i32 = -0x7fdf96d7;
pub const SIOCGIFDESCR: i32 = -0x3fdf96d6;
pub const SIOCAIFADDR: i32 = -0x7fbb96d5;
pub const SIOCADDMULTI: i32 = -0x7fdf96cf;
pub const SIOCDELMULTI: i32 = -0x7fdf96ce;
pub const SIOCGIFMTU: i32 = -0x3fdf96cd;
pub const SIOCSIFMTU: i32 = -0x7fdf96cc;
pub const SIOCGIFPHYS: i32 = -0x3fdf96cb;
pub const SIOCSIFPHYS: i32 = -0x7fdf96ca;
pub const SIOCSIFMEDIA: i32 = -0x3fdf96c9;
pub const SIOCGIFMEDIA: i32 = -0x3fcf96c8;
pub const SIOCSIFGENERIC: i32 = -0x7fdf96c7;
pub const SIOCGIFGENERIC: i32 = -0x3fdf96c6;
pub const SIOCGIFSTATUS: i32 = -0x3cce96c5;
pub const SIOCSIFLLADDR: i32 = -0x7fdf96c4;
pub const SIOCGI2C: i32 = -0x3fdf96c3;
pub const SIOCGHWADDR: i32 = -0x3fdf96c2;
pub const SIOCSIFPHYADDR: i32 = -0x7fbb96ba;
pub const SIOCGIFPSRCADDR: i32 = -0x3fdf96b9;
pub const SIOCGIFPDSTADDR: i32 = -0x3fdf96b8;
pub const SIOCDIFPHYADDR: i32 = -0x7fdf96b7;
pub const SIOCGPRIVATE_0: i32 = -0x3fdf96b0;
pub const SIOCGPRIVATE_1: i32 = -0x3fdf96af;
pub const SIOCSIFVNET: i32 = -0x3fdf96a6;
pub const SIOCSIFRVNET: i32 = -0x3fdf96a5;
pub const SIOCGIFFIB: i32 = -0x3fdf96a4;
pub const SIOCSIFFIB: i32 = -0x7fdf96a3;
pub const SIOCGTUNFIB: i32 = -0x3fdf96a2;
pub const SIOCSTUNFIB: i32 = -0x7fdf96a1;
pub const SIOCSDRVSPEC: i32 = -0x7fd79685;
pub const SIOCGDRVSPEC: i32 = -0x3fd79685;
pub const SIOCIFCREATE: i32 = -0x3fdf9686;
pub const SIOCIFCREATE2: i32 = -0x3fdf9684;
pub const SIOCIFDESTROY: i32 = -0x7fdf9687;
pub const SIOCIFGCLONERS: i32 = -0x3fef9688;
pub const SIOCAIFGROUP: i32 = -0x7fd79679;
pub const SIOCGIFGROUP: i32 = -0x3fd79678;
pub const SIOCDIFGROUP: i32 = -0x7fd79677;
pub const SIOCGIFGMEMB: i32 = -0x3fd79676;
pub const SIOCGIFXMEDIA: i32 = -0x3fcf9675;
pub const SIOCGIFRSSKEY: i32 = -0x3f6b966a;
pub const SIOCGIFRSSHASH: i32 = -0x3fe79669;
