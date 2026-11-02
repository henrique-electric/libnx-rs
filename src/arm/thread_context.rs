//! AArch64 register dump format and related definitions. (`switch/arm/thread_context.h`)

use crate::types::BIT;

/// Armv8 CPU register.
#[repr(C)]
#[derive(Clone, Copy)]
pub union CpuRegister {
    /// 64-bit AArch64 register view.
    pub x: u64,
    /// 32-bit AArch64 register view.
    pub w: u32,
    /// AArch32 register view.
    pub r: u32,
}

/// Armv8 NEON register.
#[repr(C)]
#[derive(Clone, Copy)]
pub union FpuRegister {
    /// 128-bit vector view.
    pub v: u128,
    /// 64-bit double-precision view.
    pub d: f64,
    /// 32-bit single-precision view.
    pub s: f32,
}

/// Armv8 register group. [`svcGetThreadContext3`](crate::svcGetThreadContext3) uses [`RegisterGroup_All`].
pub type RegisterGroup = u32;
/// General-purpose CPU registers (x0..x28 or r0..r10,r12).
pub const RegisterGroup_CpuGprs: RegisterGroup = BIT(0);
/// Special-purpose CPU registers (fp, lr, sp, pc, PSTATE or cpsr, TPIDR_EL0).
pub const RegisterGroup_CpuSprs: RegisterGroup = BIT(1);
/// General-purpose NEON registers.
pub const RegisterGroup_FpuGprs: RegisterGroup = BIT(2);
/// Special-purpose NEON registers.
pub const RegisterGroup_FpuSprs: RegisterGroup = BIT(3);
/// All CPU registers.
pub const RegisterGroup_CpuAll: RegisterGroup = RegisterGroup_CpuGprs | RegisterGroup_CpuSprs;
/// All NEON registers.
pub const RegisterGroup_FpuAll: RegisterGroup = RegisterGroup_FpuGprs | RegisterGroup_FpuSprs;
/// All registers.
pub const RegisterGroup_All: RegisterGroup = RegisterGroup_CpuAll | RegisterGroup_FpuAll;

/// This is for [`ThreadExceptionDump::error_desc`].
pub type ThreadExceptionDesc = u32;
/// Instruction abort
pub const ThreadExceptionDesc_InstructionAbort: ThreadExceptionDesc = 0x100;
/// Misaligned PC
pub const ThreadExceptionDesc_MisalignedPC: ThreadExceptionDesc = 0x102;
/// Misaligned SP
pub const ThreadExceptionDesc_MisalignedSP: ThreadExceptionDesc = 0x103;
/// SError [not in 1.0.0?]
pub const ThreadExceptionDesc_SError: ThreadExceptionDesc = 0x106;
/// Bad SVC
pub const ThreadExceptionDesc_BadSVC: ThreadExceptionDesc = 0x301;
/// Uncategorized, CP15RTTrap, CP15RRTTrap, CP14RTTrap, CP14RRTTrap, IllegalState, SystemRegisterTrap
pub const ThreadExceptionDesc_Trap: ThreadExceptionDesc = 0x104;
/// None of the above, EC <= 0x34 and not a breakpoint
pub const ThreadExceptionDesc_Other: ThreadExceptionDesc = 0x101;

/// Thread context structure (register dump)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ThreadContext {
    /// GPRs 0..28. Note: also contains AArch32 SPRs.
    pub cpu_gprs: [CpuRegister; 29],
    /// Frame pointer (x29) (AArch64). For AArch32, check r11.
    pub fp: u64,
    /// Link register (x30) (AArch64). For AArch32, check r14.
    pub lr: u64,
    /// Stack pointer (AArch64). For AArch32, check r13.
    pub sp: u64,
    /// Program counter.
    pub pc: CpuRegister,
    /// PSTATE or cpsr.
    pub psr: u32,

    /// 32 general-purpose NEON registers.
    pub fpu_gprs: [FpuRegister; 32],
    /// Floating-point control register.
    pub fpcr: u32,
    /// Floating-point status register.
    pub fpsr: u32,

    /// EL0 Read/Write Software Thread ID Register.
    pub tpidr: u64,
}

/// Thread exception dump structure.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ThreadExceptionDump {
    /// See [`ThreadExceptionDesc`].
    pub error_desc: u32,
    pub pad: [u32; 3],

    /// GPRs 0..28. Note: also contains AArch32 registers.
    pub cpu_gprs: [CpuRegister; 29],
    /// Frame pointer.
    pub fp: CpuRegister,
    /// Link register.
    pub lr: CpuRegister,
    /// Stack pointer.
    pub sp: CpuRegister,
    /// Program counter (elr_el1).
    pub pc: CpuRegister,

    pub padding: u64,

    /// 32 general-purpose NEON registers.
    pub fpu_gprs: [FpuRegister; 32],

    /// pstate & 0xFF0FFE20
    pub pstate: u32,
    pub afsr0: u32,
    pub afsr1: u32,
    pub esr: u32,

    /// Fault Address Register.
    pub far: CpuRegister,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct ThreadExceptionFrameA64 {
    /// GPRs 0..8.
    pub cpu_gprs: [u64; 9],
    pub lr: u64,
    pub sp: u64,
    pub elr_el1: u64,
    /// pstate & 0xFF0FFE20
    pub pstate: u32,
    pub afsr0: u32,
    pub afsr1: u32,
    pub esr: u32,
    pub far: u64,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct ThreadExceptionFrameA32 {
    /// GPRs 0..7.
    pub cpu_gprs: [u32; 8],
    pub sp: u32,
    pub lr: u32,
    pub elr_el1: u32,
    /// tpidr_el0 = 1
    pub tpidr_el0: u32,
    /// cpsr & 0xFF0FFE20
    pub cpsr: u32,
    pub afsr0: u32,
    pub afsr1: u32,
    pub esr: u32,
    pub far: u32,
}

/// Determines whether a thread context belong to an AArch64 process based on the PSR.
///
/// Returns true if and only if the thread context belongs to an AArch64 process.
#[inline(always)]
pub const fn threadContextIsAArch64(ctx: &ThreadContext) -> bool {
    (ctx.psr & 0x10) == 0
}

/// Determines whether a ThreadExceptionDump belongs to an AArch64 process based on the PSTATE.
///
/// Returns true if and only if the ThreadExceptionDump belongs to an AArch64 process.
#[inline(always)]
pub const fn threadExceptionIsAArch64(ctx: &ThreadExceptionDump) -> bool {
    (ctx.pstate & 0x10) == 0
}
