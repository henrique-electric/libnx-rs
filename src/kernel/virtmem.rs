//! `switch/kernel/virtmem.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Address space reservation type (see `virtmemAddReservation`)
#[repr(C)]
#[derive(Debug)]
pub struct VirtmemReservation {
    _opaque: [u8; 0],
}

unsafe extern "C" {
    /// Locks the virtual memory manager mutex.
    pub fn virtmemLock();

    /// Unlocks the virtual memory manager mutex.
    pub fn virtmemUnlock();

    /// Finds a random slice of free general purpose address space.
    ///
    /// # Arguments
    /// - `size`: Desired size of the slice (rounded up to page alignment).
    /// - `guard_size`: Desired size of the unmapped guard areas surrounding the slice  (rounded up to page alignment).
    ///
    /// **Returns:** Pointer to the slice of address space, or NULL on failure.
    ///
    /// **Note:** The virtual memory manager mutex must be held during the find-and-map process (see `virtmemLock` and `virtmemUnlock`).
    pub fn virtmemFindAslr(size: usize, guard_size: usize) -> *mut c_void;

    /// Finds a random slice of free stack address space.
    ///
    /// # Arguments
    /// - `size`: Desired size of the slice (rounded up to page alignment).
    /// - `guard_size`: Desired size of the unmapped guard areas surrounding the slice  (rounded up to page alignment).
    ///
    /// **Returns:** Pointer to the slice of address space, or NULL on failure.
    ///
    /// **Note:** The virtual memory manager mutex must be held during the find-and-map process (see `virtmemLock` and `virtmemUnlock`).
    pub fn virtmemFindStack(size: usize, guard_size: usize) -> *mut c_void;

    /// Finds a random slice of free code memory address space.
    ///
    /// # Arguments
    /// - `size`: Desired size of the slice (rounded up to page alignment).
    /// - `guard_size`: Desired size of the unmapped guard areas surrounding the slice  (rounded up to page alignment).
    ///
    /// **Returns:** Pointer to the slice of address space, or NULL on failure.
    ///
    /// **Note:** The virtual memory manager mutex must be held during the find-and-map process (see `virtmemLock` and `virtmemUnlock`).
    pub fn virtmemFindCodeMemory(size: usize, guard_size: usize) -> *mut c_void;

    /// Reserves a range of memory address space.
    ///
    /// # Arguments
    /// - `mem`: Pointer to the address space slice.
    /// - `size`: Size of the slice.
    ///
    /// **Returns:** Pointer to a reservation object, or NULL on failure.
    ///
    /// **Remark:** This function is intended to be used in lieu of a memory map operation when the memory won't be mapped straight away.
    ///
    /// **Note:** The virtual memory manager mutex must be held during the find-and-reserve process (see `virtmemLock` and `virtmemUnlock`).
    pub fn virtmemAddReservation(mem: *mut c_void, size: usize) -> *mut VirtmemReservation;

    /// Releases a memory address space reservation.
    ///
    /// # Arguments
    /// - `rv`: Reservation to release.
    ///
    /// **Note:** The virtual memory manager mutex must be held before calling this function (see `virtmemLock` and `virtmemUnlock`).
    pub fn virtmemRemoveReservation(rv: *mut VirtmemReservation);
}
