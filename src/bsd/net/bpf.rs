//! `net/bpf.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const BPF_RELEASE: u32 = 199606;
pub const BPF_ALIGNMENT: u64 = 8;
pub const BPF_MAXINSNS: u32 = 512;
pub const BPF_MAXBUFSIZE: u32 = 0x80000;
pub const BPF_MINBUFSIZE: u32 = 32;
pub const BPF_MAJOR_VERSION: u32 = 1;
pub const BPF_MINOR_VERSION: u32 = 1;
pub const BPF_BUFMODE_BUFFER: u32 = 1;
pub const BPF_BUFMODE_ZBUF: u32 = 2;
pub const BIOCGBLEN: u32 = 0x40044266;
pub const BIOCSBLEN: i32 = -0x3ffbbd9a;
pub const BIOCSETF: i32 = -0x7fefbd99;
pub const BIOCFLUSH: u32 = 0x20004268;
pub const BIOCPROMISC: u32 = 0x20004269;
pub const BIOCGDLT: u32 = 0x4004426a;
pub const BIOCGETIF: u32 = 0x4020426b;
pub const BIOCSETIF: i32 = -0x7fdfbd94;
pub const BIOCSRTIMEOUT: i32 = -0x7fefbd93;
pub const BIOCGRTIMEOUT: u32 = 0x4010426e;
pub const BIOCGSTATS: u32 = 0x4008426f;
pub const BIOCIMMEDIATE: i32 = -0x7ffbbd90;
pub const BIOCVERSION: u32 = 0x40044271;
pub const BIOCGRSIG: u32 = 0x40044272;
pub const BIOCSRSIG: i32 = -0x7ffbbd8d;
pub const BIOCGHDRCMPLT: u32 = 0x40044274;
pub const BIOCSHDRCMPLT: i32 = -0x7ffbbd8b;
pub const BIOCGDIRECTION: u32 = 0x40044276;
pub const BIOCSDIRECTION: i32 = -0x7ffbbd89;
pub const BIOCSDLT: i32 = -0x7ffbbd88;
pub const BIOCGDLTLIST: i32 = -0x3fefbd87;
pub const BIOCLOCK: u32 = 0x2000427a;
pub const BIOCSETWF: i32 = -0x7fefbd85;
pub const BIOCFEEDBACK: i32 = -0x7ffbbd84;
pub const BIOCGETBUFMODE: u32 = 0x4004427d;
pub const BIOCSETBUFMODE: i32 = -0x7ffbbd82;
pub const BIOCGETZMAX: u32 = 0x4008427f;
pub const BIOCROTZBUF: u32 = 0x40184280;
pub const BIOCSETZBUF: i32 = -0x7fe7bd7f;
pub const BIOCSETFNR: i32 = -0x7fefbd7e;
pub const BIOCGTSTAMP: u32 = 0x40044283;
pub const BIOCSTSTAMP: i32 = -0x7ffbbd7c;
pub const BIOCSETF_SERIALIZED: i32 = -0x6ffbbd99;
pub const BIOCSETWF_SERIALIZED: i32 = -0x6ffbbd85;
pub const BIOCSETFNR_SERIALIZED: i32 = -0x6ffbbd7e;
pub const BIOCGSEESENT: u32 = 0x40044276;
pub const BIOCSSEESENT: i32 = -0x7ffbbd89;
pub const BPF_T_MICROTIME: u32 = 0x0;
pub const BPF_T_NANOTIME: u32 = 0x1;
pub const BPF_T_BINTIME: u32 = 0x2;
pub const BPF_T_NONE: u32 = 0x3;
pub const BPF_T_FORMAT_MASK: u32 = 0x3;
pub const BPF_T_NORMAL: u32 = 0x0;
pub const BPF_T_FAST: u32 = 0x100;
pub const BPF_T_MONOTONIC: u32 = 0x200;
pub const BPF_T_MONOTONIC_FAST: u32 = 768;
pub const BPF_T_FLAG_MASK: u32 = 0x300;
pub const BPF_T_MICROTIME_FAST: u32 = 256;
pub const BPF_T_NANOTIME_FAST: u32 = 257;
pub const BPF_T_BINTIME_FAST: u32 = 258;
pub const BPF_T_MICROTIME_MONOTONIC: u32 = 512;
pub const BPF_T_NANOTIME_MONOTONIC: u32 = 513;
pub const BPF_T_BINTIME_MONOTONIC: u32 = 514;
pub const BPF_T_MICROTIME_MONOTONIC_FAST: u32 = 768;
pub const BPF_T_NANOTIME_MONOTONIC_FAST: u32 = 769;
pub const BPF_T_BINTIME_MONOTONIC_FAST: u32 = 770;
pub const BPF_LD: u32 = 0x0;
pub const BPF_LDX: u32 = 0x1;
pub const BPF_ST: u32 = 0x2;
pub const BPF_STX: u32 = 0x3;
pub const BPF_ALU: u32 = 0x4;
pub const BPF_JMP: u32 = 0x5;
pub const BPF_RET: u32 = 0x6;
pub const BPF_MISC: u32 = 0x7;
pub const BPF_W: u32 = 0x0;
pub const BPF_H: u32 = 0x8;
pub const BPF_B: u32 = 0x10;
pub const BPF_IMM: u32 = 0x0;
pub const BPF_ABS: u32 = 0x20;
pub const BPF_IND: u32 = 0x40;
pub const BPF_MEM: u32 = 0x60;
pub const BPF_LEN: u32 = 0x80;
pub const BPF_MSH: u32 = 0xa0;
pub const BPF_ADD: u32 = 0x0;
pub const BPF_SUB: u32 = 0x10;
pub const BPF_MUL: u32 = 0x20;
pub const BPF_DIV: u32 = 0x30;
pub const BPF_OR: u32 = 0x40;
pub const BPF_AND: u32 = 0x50;
pub const BPF_LSH: u32 = 0x60;
pub const BPF_RSH: u32 = 0x70;
pub const BPF_NEG: u32 = 0x80;
pub const BPF_MOD: u32 = 0x90;
pub const BPF_XOR: u32 = 0xa0;
pub const BPF_JA: u32 = 0x0;
pub const BPF_JEQ: u32 = 0x10;
pub const BPF_JGT: u32 = 0x20;
pub const BPF_JGE: u32 = 0x30;
pub const BPF_JSET: u32 = 0x40;
pub const BPF_K: u32 = 0x0;
pub const BPF_X: u32 = 0x8;
pub const BPF_A: u32 = 0x10;
pub const BPF_TAX: u32 = 0x0;
pub const BPF_TXA: u32 = 0x80;
pub const BPF_MEMWORDS: u32 = 16;

pub type bpf_int32 = i32;

pub type bpf_u_int32 = u_int32_t;

pub type bpf_int64 = i64;

pub type bpf_u_int64 = u_int64_t;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct bpf_program {
    pub bf_len: u_int,
    pub bf_insns: *mut bpf_insn,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct bpf_stat {
    pub bs_recv: u_int,
    pub bs_drop: u_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct bpf_version {
    pub bv_major: u_short,
    pub bv_minor: u_short,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct bpf_zbuf {
    pub bz_bufa: *mut c_void,
    pub bz_bufb: *mut c_void,
    pub bz_buflen: usize,
}

pub type bpf_direction = u32;
pub const BPF_D_IN: bpf_direction = 0;
pub const BPF_D_INOUT: bpf_direction = 1;
pub const BPF_D_OUT: bpf_direction = 2;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct bpf_ts {
    pub bt_sec: bpf_int64,
    pub bt_frac: bpf_u_int64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct bpf_xhdr {
    pub bh_tstamp: bpf_ts,
    pub bh_caplen: bpf_u_int32,
    pub bh_datalen: bpf_u_int32,
    pub bh_hdrlen: u_short,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct bpf_hdr {
    pub bh_tstamp: timeval,
    pub bh_caplen: bpf_u_int32,
    pub bh_datalen: bpf_u_int32,
    pub bh_hdrlen: u_short,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct bpf_zbuf_header {
    pub bzh_kernel_gen: u_int,
    pub bzh_kernel_len: u_int,
    pub bzh_user_gen: u_int,
    pub _bzh_pad: [u_int; 5],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct bpf_insn {
    pub code: u_short,
    pub jt: u_char,
    pub jf: u_char,
    pub k: bpf_u_int32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct bpf_program_serialized {
    pub bf_len: u_int,
    pub bf_insns: [bpf_insn; 512],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct bpf_dltlist {
    pub bfl_len: u_int,
    pub bfl_list: *mut u_int,
}

#[repr(C)]
#[derive(Debug)]
pub struct bpf_if {
    _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug)]
pub struct bpf_if_ext {
    _opaque: [u8; 0],
}

impl_zeroed_default!(bpf_program, bpf_stat, bpf_version, bpf_zbuf, bpf_ts, bpf_xhdr, bpf_hdr, bpf_zbuf_header, bpf_insn, bpf_program_serialized, bpf_dltlist);

#[inline(always)]
pub const fn BPF_WORDALIGN(x: usize) -> usize {
    (x + (BPF_ALIGNMENT as usize - 1)) & !(BPF_ALIGNMENT as usize - 1)
}

#[inline(always)]
pub const fn BPF_T_FORMAT(t: u32) -> u32 { t & BPF_T_FORMAT_MASK }

#[inline(always)]
pub const fn BPF_T_FLAG(t: u32) -> u32 { t & BPF_T_FLAG_MASK }

#[inline(always)]
pub const fn BPF_T_VALID(t: u32) -> bool {
    t == BPF_T_NONE
        || (BPF_T_FORMAT(t) != BPF_T_NONE && (t & !(BPF_T_FORMAT_MASK | BPF_T_FLAG_MASK)) == 0)
}

#[inline(always)]
pub const fn BPF_CLASS(code: u32) -> u32 { code & 0x07 }

#[inline(always)]
pub const fn BPF_SIZE(code: u32) -> u32 { code & 0x18 }

#[inline(always)]
pub const fn BPF_MODE(code: u32) -> u32 { code & 0xe0 }

#[inline(always)]
pub const fn BPF_OP(code: u32) -> u32 { code & 0xf0 }

#[inline(always)]
pub const fn BPF_SRC(code: u32) -> u32 { code & 0x08 }

#[inline(always)]
pub const fn BPF_RVAL(code: u32) -> u32 { code & 0x18 }

#[inline(always)]
pub const fn BPF_MISCOP(code: u32) -> u32 { code & 0xf8 }

/// Builds a BPF statement instruction.
#[inline(always)]
pub const fn BPF_STMT(code: u32, k: u32) -> bpf_insn {
    bpf_insn { code: code as u_short, jt: 0, jf: 0, k: k as _ }
}

/// Builds a BPF jump instruction.
#[inline(always)]
pub const fn BPF_JUMP(code: u32, k: u32, jt: u8, jf: u8) -> bpf_insn {
    bpf_insn { code: code as u_short, jt, jf, k: k as _ }
}
