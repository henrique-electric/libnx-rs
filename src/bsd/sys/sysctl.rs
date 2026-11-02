//! `sys/sysctl.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const CTL_MAXNAME: u32 = 24;
pub const CTLTYPE: u32 = 0xf;
pub const CTLTYPE_NODE: u32 = 1;
pub const CTLTYPE_INT: u32 = 2;
pub const CTLTYPE_STRING: u32 = 3;
pub const CTLTYPE_S64: u32 = 4;
pub const CTLTYPE_OPAQUE: u32 = 5;
pub const CTLTYPE_STRUCT: u32 = 5;
pub const CTLTYPE_UINT: u32 = 6;
pub const CTLTYPE_LONG: u32 = 7;
pub const CTLTYPE_ULONG: u32 = 8;
pub const CTLTYPE_U64: u32 = 9;
pub const CTLTYPE_U8: u32 = 0xa;
pub const CTLTYPE_U16: u32 = 0xb;
pub const CTLTYPE_S8: u32 = 0xc;
pub const CTLTYPE_S16: u32 = 0xd;
pub const CTLTYPE_S32: u32 = 0xe;
pub const CTLTYPE_U32: u32 = 0xf;
pub const CTLFLAG_RD: u32 = 0x80000000;
pub const CTLFLAG_WR: u32 = 0x40000000;
pub const CTLFLAG_RW: u32 = 0xc0000000;
pub const CTLFLAG_DORMANT: u32 = 0x20000000;
pub const CTLFLAG_ANYBODY: u32 = 0x10000000;
pub const CTLFLAG_SECURE: u32 = 0x8000000;
pub const CTLFLAG_PRISON: u32 = 0x4000000;
pub const CTLFLAG_DYN: u32 = 0x2000000;
pub const CTLFLAG_SKIP: u32 = 0x1000000;
pub const CTLMASK_SECURE: u32 = 0xf00000;
pub const CTLFLAG_TUN: u32 = 0x80000;
pub const CTLFLAG_RDTUN: u32 = 0x80080000;
pub const CTLFLAG_RWTUN: u32 = 0xc0080000;
pub const CTLFLAG_MPSAFE: u32 = 0x40000;
pub const CTLFLAG_VNET: u32 = 0x20000;
pub const CTLFLAG_DYING: u32 = 0x10000;
pub const CTLFLAG_CAPRD: u32 = 0x8000;
pub const CTLFLAG_CAPWR: u32 = 0x4000;
pub const CTLFLAG_STATS: u32 = 0x2000;
pub const CTLFLAG_NOFETCH: u32 = 0x1000;
pub const CTLFLAG_CAPRW: u32 = 0xc000;
pub const CTLSHIFT_SECURE: u32 = 20;
pub const CTLFLAG_SECURE1: u32 = 0x8000000;
pub const CTLFLAG_SECURE2: u32 = 0x8100000;
pub const CTLFLAG_SECURE3: u32 = 0x8200000;
pub const OID_AUTO: i32 = -1;
pub const CTL_AUTO_START: u32 = 0x100;
pub const CTL_UNSPEC: u32 = 0;
pub const CTL_KERN: u32 = 1;
pub const CTL_VM: u32 = 2;
pub const CTL_VFS: u32 = 3;
pub const CTL_NET: u32 = 4;
pub const CTL_DEBUG: u32 = 5;
pub const CTL_HW: u32 = 6;
pub const CTL_MACHDEP: u32 = 7;
pub const CTL_USER: u32 = 8;
pub const CTL_P1003_1B: u32 = 9;
pub const KERN_OSTYPE: u32 = 1;
pub const KERN_OSRELEASE: u32 = 2;
pub const KERN_OSREV: u32 = 3;
pub const KERN_VERSION: u32 = 4;
pub const KERN_MAXVNODES: u32 = 5;
pub const KERN_MAXPROC: u32 = 6;
pub const KERN_MAXFILES: u32 = 7;
pub const KERN_ARGMAX: u32 = 8;
pub const KERN_SECURELVL: u32 = 9;
pub const KERN_HOSTNAME: u32 = 10;
pub const KERN_HOSTID: u32 = 11;
pub const KERN_CLOCKRATE: u32 = 12;
pub const KERN_VNODE: u32 = 13;
pub const KERN_PROC: u32 = 14;
pub const KERN_FILE: u32 = 15;
pub const KERN_PROF: u32 = 16;
pub const KERN_POSIX1: u32 = 17;
pub const KERN_NGROUPS: u32 = 18;
pub const KERN_JOB_CONTROL: u32 = 19;
pub const KERN_SAVED_IDS: u32 = 20;
pub const KERN_BOOTTIME: u32 = 21;
pub const KERN_NISDOMAINNAME: u32 = 22;
pub const KERN_UPDATEINTERVAL: u32 = 23;
pub const KERN_OSRELDATE: u32 = 24;
pub const KERN_NTP_PLL: u32 = 25;
pub const KERN_BOOTFILE: u32 = 26;
pub const KERN_MAXFILESPERPROC: u32 = 27;
pub const KERN_MAXPROCPERUID: u32 = 28;
pub const KERN_DUMPDEV: u32 = 29;
pub const KERN_IPC: u32 = 30;
pub const KERN_DUMMY: u32 = 31;
pub const KERN_PS_STRINGS: u32 = 32;
pub const KERN_USRSTACK: u32 = 33;
pub const KERN_LOGSIGEXIT: u32 = 34;
pub const KERN_IOV_MAX: u32 = 35;
pub const KERN_HOSTUUID: u32 = 36;
pub const KERN_ARND: u32 = 37;
pub const KERN_MAXPHYS: u32 = 38;
pub const KERN_PROC_ALL: u32 = 0;
pub const KERN_PROC_PID: u32 = 1;
pub const KERN_PROC_PGRP: u32 = 2;
pub const KERN_PROC_SESSION: u32 = 3;
pub const KERN_PROC_TTY: u32 = 4;
pub const KERN_PROC_UID: u32 = 5;
pub const KERN_PROC_RUID: u32 = 6;
pub const KERN_PROC_ARGS: u32 = 7;
pub const KERN_PROC_PROC: u32 = 8;
pub const KERN_PROC_SV_NAME: u32 = 9;
pub const KERN_PROC_RGID: u32 = 10;
pub const KERN_PROC_GID: u32 = 11;
pub const KERN_PROC_PATHNAME: u32 = 12;
pub const KERN_PROC_OVMMAP: u32 = 13;
pub const KERN_PROC_OFILEDESC: u32 = 14;
pub const KERN_PROC_KSTACK: u32 = 15;
pub const KERN_PROC_INC_THREAD: u32 = 0x10;
pub const KERN_PROC_VMMAP: u32 = 32;
pub const KERN_PROC_FILEDESC: u32 = 33;
pub const KERN_PROC_GROUPS: u32 = 34;
pub const KERN_PROC_ENV: u32 = 35;
pub const KERN_PROC_AUXV: u32 = 36;
pub const KERN_PROC_RLIMIT: u32 = 37;
pub const KERN_PROC_PS_STRINGS: u32 = 38;
pub const KERN_PROC_UMASK: u32 = 39;
pub const KERN_PROC_OSREL: u32 = 40;
pub const KERN_PROC_SIGTRAMP: u32 = 41;
pub const KERN_PROC_CWD: u32 = 42;
pub const KERN_PROC_NFDS: u32 = 43;
pub const KIPC_MAXSOCKBUF: u32 = 1;
pub const KIPC_SOCKBUF_WASTE: u32 = 2;
pub const KIPC_SOMAXCONN: u32 = 3;
pub const KIPC_MAX_LINKHDR: u32 = 4;
pub const KIPC_MAX_PROTOHDR: u32 = 5;
pub const KIPC_MAX_HDR: u32 = 6;
pub const KIPC_MAX_DATALEN: u32 = 7;
pub const HW_MACHINE: u32 = 1;
pub const HW_MODEL: u32 = 2;
pub const HW_NCPU: u32 = 3;
pub const HW_BYTEORDER: u32 = 4;
pub const HW_PHYSMEM: u32 = 5;
pub const HW_USERMEM: u32 = 6;
pub const HW_PAGESIZE: u32 = 7;
pub const HW_DISKNAMES: u32 = 8;
pub const HW_DISKSTATS: u32 = 9;
pub const HW_FLOATINGPT: u32 = 10;
pub const HW_MACHINE_ARCH: u32 = 11;
pub const HW_REALMEM: u32 = 12;
pub const USER_CS_PATH: u32 = 1;
pub const USER_BC_BASE_MAX: u32 = 2;
pub const USER_BC_DIM_MAX: u32 = 3;
pub const USER_BC_SCALE_MAX: u32 = 4;
pub const USER_BC_STRING_MAX: u32 = 5;
pub const USER_COLL_WEIGHTS_MAX: u32 = 6;
pub const USER_EXPR_NEST_MAX: u32 = 7;
pub const USER_LINE_MAX: u32 = 8;
pub const USER_RE_DUP_MAX: u32 = 9;
pub const USER_POSIX2_VERSION: u32 = 10;
pub const USER_POSIX2_C_BIND: u32 = 11;
pub const USER_POSIX2_C_DEV: u32 = 12;
pub const USER_POSIX2_CHAR_TERM: u32 = 13;
pub const USER_POSIX2_FORT_DEV: u32 = 14;
pub const USER_POSIX2_FORT_RUN: u32 = 15;
pub const USER_POSIX2_LOCALEDEF: u32 = 16;
pub const USER_POSIX2_SW_DEV: u32 = 17;
pub const USER_POSIX2_UPE: u32 = 18;
pub const USER_STREAM_MAX: u32 = 19;
pub const USER_TZNAME_MAX: u32 = 20;
pub const CTL_P1003_1B_ASYNCHRONOUS_IO: u32 = 1;
pub const CTL_P1003_1B_MAPPED_FILES: u32 = 2;
pub const CTL_P1003_1B_MEMLOCK: u32 = 3;
pub const CTL_P1003_1B_MEMLOCK_RANGE: u32 = 4;
pub const CTL_P1003_1B_MEMORY_PROTECTION: u32 = 5;
pub const CTL_P1003_1B_MESSAGE_PASSING: u32 = 6;
pub const CTL_P1003_1B_PRIORITIZED_IO: u32 = 7;
pub const CTL_P1003_1B_PRIORITY_SCHEDULING: u32 = 8;
pub const CTL_P1003_1B_REALTIME_SIGNALS: u32 = 9;
pub const CTL_P1003_1B_SEMAPHORES: u32 = 10;
pub const CTL_P1003_1B_FSYNC: u32 = 11;
pub const CTL_P1003_1B_SHARED_MEMORY_OBJECTS: u32 = 12;
pub const CTL_P1003_1B_SYNCHRONIZED_IO: u32 = 13;
pub const CTL_P1003_1B_TIMERS: u32 = 14;
pub const CTL_P1003_1B_AIO_LISTIO_MAX: u32 = 15;
pub const CTL_P1003_1B_AIO_MAX: u32 = 16;
pub const CTL_P1003_1B_AIO_PRIO_DELTA_MAX: u32 = 17;
pub const CTL_P1003_1B_DELAYTIMER_MAX: u32 = 18;
pub const CTL_P1003_1B_MQ_OPEN_MAX: u32 = 19;
pub const CTL_P1003_1B_PAGESIZE: u32 = 20;
pub const CTL_P1003_1B_RTSIG_MAX: u32 = 21;
pub const CTL_P1003_1B_SEM_NSEMS_MAX: u32 = 22;
pub const CTL_P1003_1B_SEM_VALUE_MAX: u32 = 23;
pub const CTL_P1003_1B_SIGQUEUE_MAX: u32 = 24;
pub const CTL_P1003_1B_TIMER_MAX: u32 = 25;
pub const CTL_P1003_1B_MAXID: u32 = 26;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ctlname {
    pub ctl_name: *mut c_char,
    pub ctl_type: c_int,
}

impl_zeroed_default!(ctlname);

unsafe extern "C" {
    pub fn sysctl(name: *const c_int, namelen: c_uint, oldp: *mut c_void, oldlenp: *mut usize, newp: *const c_void, newlen: usize) -> c_int;

    pub fn sysctlbyname(name: *const c_char, oldp: *mut c_void, oldlenp: *mut usize, newp: *const c_void, newlen: usize) -> c_int;

    pub fn sysctlnametomib(name: *const c_char, mibp: *mut c_int, sizep: *mut usize) -> c_int;
}
