// inline: BPF_WORDALIGN
#[inline(always)]
pub const fn BPF_WORDALIGN(x: usize) -> usize {
    (x + (BPF_ALIGNMENT as usize - 1)) & !(BPF_ALIGNMENT as usize - 1)
}
// inline: BPF_T_FORMAT
#[inline(always)]
pub const fn BPF_T_FORMAT(t: u32) -> u32 { t & BPF_T_FORMAT_MASK }
// inline: BPF_T_FLAG
#[inline(always)]
pub const fn BPF_T_FLAG(t: u32) -> u32 { t & BPF_T_FLAG_MASK }
// inline: BPF_T_VALID
#[inline(always)]
pub const fn BPF_T_VALID(t: u32) -> bool {
    t == BPF_T_NONE
        || (BPF_T_FORMAT(t) != BPF_T_NONE && (t & !(BPF_T_FORMAT_MASK | BPF_T_FLAG_MASK)) == 0)
}
// inline: BPF_CLASS
#[inline(always)]
pub const fn BPF_CLASS(code: u32) -> u32 { code & 0x07 }
// inline: BPF_SIZE
#[inline(always)]
pub const fn BPF_SIZE(code: u32) -> u32 { code & 0x18 }
// inline: BPF_MODE
#[inline(always)]
pub const fn BPF_MODE(code: u32) -> u32 { code & 0xe0 }
// inline: BPF_OP
#[inline(always)]
pub const fn BPF_OP(code: u32) -> u32 { code & 0xf0 }
// inline: BPF_SRC
#[inline(always)]
pub const fn BPF_SRC(code: u32) -> u32 { code & 0x08 }
// inline: BPF_RVAL
#[inline(always)]
pub const fn BPF_RVAL(code: u32) -> u32 { code & 0x18 }
// inline: BPF_MISCOP
#[inline(always)]
pub const fn BPF_MISCOP(code: u32) -> u32 { code & 0xf8 }
// inline: BPF_STMT
/// Builds a BPF statement instruction.
#[inline(always)]
pub const fn BPF_STMT(code: u32, k: u32) -> bpf_insn {
    bpf_insn { code: code as u_short, jt: 0, jf: 0, k: k as _ }
}
// inline: BPF_JUMP
/// Builds a BPF jump instruction.
#[inline(always)]
pub const fn BPF_JUMP(code: u32, k: u32, jt: u8, jf: u8) -> bpf_insn {
    bpf_insn { code: code as u_short, jt, jf, k: k as _ }
}
