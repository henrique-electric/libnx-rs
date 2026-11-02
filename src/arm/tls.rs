//! AArch64 thread local storage. (`switch/arm/tls.h`)

use core::ffi::c_void;

/// Gets the thread local storage buffer.
#[inline(always)]
pub fn armGetTls() -> *mut c_void {
    #[cfg(target_arch = "aarch64")]
    {
        let ret: *mut c_void;
        // SAFETY: reading tpidrro_el0 has no side effects.
        unsafe { core::arch::asm!("mrs {}, tpidrro_el0", out(reg) ret, options(pure, nomem, nostack, preserves_flags)) };
        ret
    }
    #[cfg(not(target_arch = "aarch64"))]
    unimplemented!("armGetTls is only available on AArch64")
}
