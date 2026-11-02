//! AArch64 cache operations. (`switch/arm/cache.h`)

use core::ffi::c_void;

unsafe extern "C" {
    /// Performs a data cache flush on the specified buffer.
    ///
    /// # Arguments
    /// - `addr`: Address of the buffer.
    /// - `size`: Size of the buffer, in bytes.
    ///
    /// **Remarks:** Cache flush is defined as Clean + Invalidate.
    ///
    /// **Note:** The start and end addresses of the buffer are forcibly rounded to cache line boundaries (read from CTR_EL0 system register).
    pub fn armDCacheFlush(addr: *mut c_void, size: usize);

    /// Performs a data cache clean on the specified buffer.
    ///
    /// # Arguments
    /// - `addr`: Address of the buffer.
    /// - `size`: Size of the buffer, in bytes.
    ///
    /// **Note:** The start and end addresses of the buffer are forcibly rounded to cache line boundaries (read from CTR_EL0 system register).
    pub fn armDCacheClean(addr: *mut c_void, size: usize);

    /// Performs an instruction cache invalidation clean on the specified buffer.
    ///
    /// # Arguments
    /// - `addr`: Address of the buffer.
    /// - `size`: Size of the buffer, in bytes.
    ///
    /// **Note:** The start and end addresses of the buffer are forcibly rounded to cache line boundaries (read from CTR_EL0 system register).
    pub fn armICacheInvalidate(addr: *mut c_void, size: usize);

    /// Performs a data cache zeroing operation on the specified buffer.
    ///
    /// # Arguments
    /// - `addr`: Address of the buffer.
    /// - `size`: Size of the buffer, in bytes.
    ///
    /// **Note:** The start and end addresses of the buffer are forcibly rounded to cache line boundaries (read from CTR_EL0 system register).
    pub fn armDCacheZero(addr: *mut c_void, size: usize);
}
