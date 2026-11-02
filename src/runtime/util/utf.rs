//! `switch/runtime/util/utf.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

unsafe extern "C" {
    /// Convert a UTF-8 sequence into a UTF-32 codepoint
    ///
    /// # Arguments
    /// - `out` (out): Output codepoint
    /// - `in` (in): Input sequence
    ///
    /// **Returns:** number of input code units consumed
    ///
    /// **Returns:** -1 for error
    pub fn decode_utf8(out: *mut u32, in_: *const u8) -> isize;

    /// Convert a UTF-16 sequence into a UTF-32 codepoint
    ///
    /// # Arguments
    /// - `out` (out): Output codepoint
    /// - `in` (in): Input sequence
    ///
    /// **Returns:** number of input code units consumed
    ///
    /// **Returns:** -1 for error
    pub fn decode_utf16(out: *mut u32, in_: *const u16) -> isize;

    /// Convert a UTF-32 codepoint into a UTF-8 sequence
    ///
    /// # Arguments
    /// - `out` (out): Output sequence
    /// - `in` (in): Input codepoint
    ///
    /// **Returns:** number of output code units produced
    ///
    /// **Returns:** -1 for error
    ///
    /// **Note:** `out` must be able to store 4 code units
    pub fn encode_utf8(out: *mut u8, in_: u32) -> isize;

    /// Convert a UTF-32 codepoint into a UTF-16 sequence
    ///
    /// # Arguments
    /// - `out` (out): Output sequence
    /// - `in` (in): Input codepoint
    ///
    /// **Returns:** number of output code units produced
    ///
    /// **Returns:** -1 for error
    ///
    /// **Note:** `out` must be able to store 2 code units
    pub fn encode_utf16(out: *mut u16, in_: u32) -> isize;

    /// Convert a UTF-8 sequence into a UTF-16 sequence
    ///
    /// Fills the output buffer up to `len` code units.
    /// Returns the number of code units that the input would produce;
    /// if it returns greater than `len`, the output has been
    /// truncated.
    ///
    /// # Arguments
    /// - `out` (out): Output sequence
    /// - `in` (in): Input sequence (null-terminated)
    /// - `len` (in): Output length
    ///
    /// **Returns:** number of output code units produced
    ///
    /// **Returns:** -1 for error
    ///
    /// **Note:** `out` is not null-terminated
    pub fn utf8_to_utf16(out: *mut u16, in_: *const u8, len: usize) -> isize;

    /// Convert a UTF-8 sequence into a UTF-32 sequence
    ///
    /// Fills the output buffer up to `len` code units.
    /// Returns the number of code units that the input would produce;
    /// if it returns greater than `len`, the output has been
    /// truncated.
    ///
    /// # Arguments
    /// - `out` (out): Output sequence
    /// - `in` (in): Input sequence (null-terminated)
    /// - `len` (in): Output length
    ///
    /// **Returns:** number of output code units produced
    ///
    /// **Returns:** -1 for error
    ///
    /// **Note:** `out` is not null-terminated
    pub fn utf8_to_utf32(out: *mut u32, in_: *const u8, len: usize) -> isize;

    /// Convert a UTF-16 sequence into a UTF-8 sequence
    ///
    /// Fills the output buffer up to `len` code units.
    /// Returns the number of code units that the input would produce;
    /// if it returns greater than `len`, the output has been
    /// truncated.
    ///
    /// # Arguments
    /// - `out` (out): Output sequence
    /// - `in` (in): Input sequence (null-terminated)
    /// - `len` (in): Output length
    ///
    /// **Returns:** number of output code units produced
    ///
    /// **Returns:** -1 for error
    ///
    /// **Note:** `out` is not null-terminated
    pub fn utf16_to_utf8(out: *mut u8, in_: *const u16, len: usize) -> isize;

    /// Convert a UTF-16 sequence into a UTF-32 sequence
    ///
    /// Fills the output buffer up to `len` code units.
    /// Returns the number of code units that the input would produce;
    /// if it returns greater than `len`, the output has been
    /// truncated.
    ///
    /// # Arguments
    /// - `out` (out): Output sequence
    /// - `in` (in): Input sequence (null-terminated)
    /// - `len` (in): Output length
    ///
    /// **Returns:** number of output code units produced
    ///
    /// **Returns:** -1 for error
    ///
    /// **Note:** `out` is not null-terminated
    pub fn utf16_to_utf32(out: *mut u32, in_: *const u16, len: usize) -> isize;

    /// Convert a UTF-32 sequence into a UTF-8 sequence
    ///
    /// Fills the output buffer up to `len` code units.
    /// Returns the number of code units that the input would produce;
    /// if it returns greater than `len`, the output has been
    /// truncated.
    ///
    /// # Arguments
    /// - `out` (out): Output sequence
    /// - `in` (in): Input sequence (null-terminated)
    /// - `len` (in): Output length
    ///
    /// **Returns:** number of output code units produced
    ///
    /// **Returns:** -1 for error
    ///
    /// **Note:** `out` is not null-terminated
    pub fn utf32_to_utf8(out: *mut u8, in_: *const u32, len: usize) -> isize;

    /// Convert a UTF-32 sequence into a UTF-16 sequence
    ///
    /// # Arguments
    /// - `out` (out): Output sequence
    /// - `in` (in): Input sequence (null-terminated)
    /// - `len` (in): Output length
    ///
    /// **Returns:** number of output code units produced
    ///
    /// **Returns:** -1 for error
    ///
    /// **Note:** `out` is not null-terminated
    pub fn utf32_to_utf16(out: *mut u16, in_: *const u32, len: usize) -> isize;
}
