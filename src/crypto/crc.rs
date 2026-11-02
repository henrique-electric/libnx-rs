//! `switch/crypto/crc.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Calculate a CRC32 over data using a seed.
/// Can be used to calculate a CRC32 in chunks using an initial seed of zero for the first chunk.
#[inline(always)]
pub fn crc32CalculateWithSeed(seed: u32, src: &[u8]) -> u32 {
    !crc_impl::crc32(!seed, src)
}

/// Calculate a CRC32 over data.
#[inline(always)]
pub fn crc32Calculate(src: &[u8]) -> u32 {
    crc32CalculateWithSeed(0, src)
}

/// Calculate a CRC32C over data using a seed.
/// Can be used to calculate a CRC32C in chunks using an initial seed of zero for the first chunk.
#[inline(always)]
pub fn crc32cCalculateWithSeed(seed: u32, src: &[u8]) -> u32 {
    !crc_impl::crc32c(!seed, src)
}

/// Calculate a CRC32C over data.
#[inline(always)]
pub fn crc32cCalculate(src: &[u8]) -> u32 {
    crc32cCalculateWithSeed(0, src)
}

/// CRC32 (reflected 0xEDB88320) and CRC32C (reflected 0x82F63B78) on the raw register value.
/// On AArch64 this uses the CRC32 instructions like libnx (the Switch's Cortex-A57 has them);
/// elsewhere it falls back to a bitwise implementation with identical results.
mod crc_impl {
    #[cfg(target_arch = "aarch64")]
    macro_rules! hw_crc {
        ($name:ident, $b:ident, $d:ident) => {
            #[target_feature(enable = "crc")]
            unsafe fn $name(mut crc: u32, src: &[u8]) -> u32 {
                use core::arch::aarch64::*;
                // SAFETY: align_to only reinterprets the aligned middle part as u64 words.
                let (head, words, tail) = unsafe { src.align_to::<u64>() };
                for &b in head {
                    crc = $b(crc, b);
                }
                for &w in words {
                    crc = $d(crc, w);
                }
                for &b in tail {
                    crc = $b(crc, b);
                }
                crc
            }
        };
    }
    #[cfg(target_arch = "aarch64")]
    hw_crc!(crc32_hw, __crc32b, __crc32d);
    #[cfg(target_arch = "aarch64")]
    hw_crc!(crc32c_hw, __crc32cb, __crc32cd);

    #[cfg(not(target_arch = "aarch64"))]
    fn soft(mut crc: u32, src: &[u8], poly: u32) -> u32 {
        for &b in src {
            crc ^= b as u32;
            for _ in 0..8 {
                crc = (crc >> 1) ^ (poly & 0u32.wrapping_sub(crc & 1));
            }
        }
        crc
    }

    #[inline(always)]
    pub fn crc32(crc: u32, src: &[u8]) -> u32 {
        #[cfg(target_arch = "aarch64")]
        // SAFETY: every AArch64 CPU the Switch runs on implements the CRC32 extension.
        return unsafe { crc32_hw(crc, src) };
        #[cfg(not(target_arch = "aarch64"))]
        return soft(crc, src, 0xEDB88320);
    }

    #[inline(always)]
    pub fn crc32c(crc: u32, src: &[u8]) -> u32 {
        #[cfg(target_arch = "aarch64")]
        // SAFETY: as above.
        return unsafe { crc32c_hw(crc, src) };
        #[cfg(not(target_arch = "aarch64"))]
        return soft(crc, src, 0x82F63B78);
    }
}
