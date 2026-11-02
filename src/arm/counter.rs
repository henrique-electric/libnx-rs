//! AArch64 system counter-timer. (`switch/arm/counter.h`)

/// Gets the current system tick.
#[inline(always)]
pub fn armGetSystemTick() -> u64 {
    #[cfg(target_arch = "aarch64")]
    {
        let ret: u64;
        // SAFETY: reading cntpct_el0 has no side effects. Not `pure`: the value changes between reads.
        unsafe { core::arch::asm!("mrs {}, cntpct_el0", out(reg) ret, options(nomem, nostack, preserves_flags)) };
        ret
    }
    #[cfg(not(target_arch = "aarch64"))]
    unimplemented!("armGetSystemTick is only available on AArch64")
}

/// Gets the system counter-timer frequency, in Hz.
#[inline(always)]
pub fn armGetSystemTickFreq() -> u64 {
    #[cfg(target_arch = "aarch64")]
    {
        let ret: u64;
        // SAFETY: reading cntfrq_el0 has no side effects.
        unsafe { core::arch::asm!("mrs {}, cntfrq_el0", out(reg) ret, options(pure, nomem, nostack, preserves_flags)) };
        ret
    }
    #[cfg(not(target_arch = "aarch64"))]
    unimplemented!("armGetSystemTickFreq is only available on AArch64")
}

/// Converts from nanoseconds to CPU ticks unit.
#[inline(always)]
pub const fn armNsToTicks(ns: u64) -> u64 {
    ns.wrapping_mul(12) / 625
}

/// Converts from CPU ticks unit to nanoseconds.
#[inline(always)]
pub const fn armTicksToNs(tick: u64) -> u64 {
    tick.wrapping_mul(625) / 12
}
