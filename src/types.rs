//! Various system types. (`switch/types.h`)

use core::ffi::c_void;

// `u8`..`u128` are Rust primitives already; only the signed `sN` names need aliases.

/// 8-bit signed integer.
pub type s8 = i8;
/// 16-bit signed integer.
pub type s16 = i16;
/// 32-bit signed integer.
pub type s32 = i32;
/// 64-bit signed integer.
pub type s64 = i64;
/// 128-bit signed integer.
pub type s128 = i128;

// Rust has no volatile types: the `v*` aliases only document intent.
// Accesses must go through `core::ptr::read_volatile` / `write_volatile`.

/// 8-bit volatile unsigned integer.
pub type vu8 = u8;
/// 16-bit volatile unsigned integer.
pub type vu16 = u16;
/// 32-bit volatile unsigned integer.
pub type vu32 = u32;
/// 64-bit volatile unsigned integer.
pub type vu64 = u64;
/// 128-bit volatile unsigned integer.
pub type vu128 = u128;

/// 8-bit volatile signed integer.
pub type vs8 = s8;
/// 16-bit volatile signed integer.
pub type vs16 = s16;
/// 32-bit volatile signed integer.
pub type vs32 = s32;
/// 64-bit volatile signed integer.
pub type vs64 = s64;
/// 128-bit volatile signed integer.
pub type vs128 = s128;

/// Kernel object handle.
pub type Handle = u32;
/// Function error code result type.
pub type Result = u32;
/// Thread entrypoint function.
pub type ThreadFunc = Option<unsafe extern "C" fn(*mut c_void)>;
/// Function without arguments nor return value.
pub type VoidFn = Option<unsafe extern "C" fn()>;

/// Unique identifier.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Uuid {
    pub uuid: [u8; 0x10],
}

/// 3 floats.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct UtilFloat3 {
    pub value: [f32; 3],
}

/// Creates a bitmask from a bit number.
#[inline(always)]
pub const fn BIT(n: u32) -> u32 {
    1u32 << n
}

/// Creates a 64-bit bitmask from a bit number.
#[inline(always)]
pub const fn BITL(n: u32) -> u64 {
    1u64 << n
}

/// Invalid handle.
pub const INVALID_HANDLE: Handle = 0;

/// Maximum value of `ssize_t`.
pub const SSIZE_MAX: isize = isize::MAX;

/// Implements `Default` as all-zeroes, matching C's `(T){}` for structs holding raw pointers.
macro_rules! impl_zeroed_default {
    ($($t:ty),* $(,)?) => {$(
        impl Default for $t {
            #[inline(always)]
            fn default() -> Self {
                // SAFETY: every field is an integer, bool, raw pointer, `Option<fn>` or
                // aggregate thereof, for which all-zeroes is a valid value.
                unsafe { core::mem::zeroed() }
            }
        }
    )*};
}
pub(crate) use impl_zeroed_default;

/// Conversion between a C bitfield's declared type and its raw bits.
pub trait BitfieldValue: Copy {
    fn from_bits(bits: u64, width: u32) -> Self;
    fn to_bits(self) -> u64;
}

macro_rules! impl_bitfield_unsigned {
    ($($t:ty),*) => {$(
        impl BitfieldValue for $t {
            #[inline(always)]
            fn from_bits(bits: u64, _width: u32) -> Self { bits as $t }
            #[inline(always)]
            fn to_bits(self) -> u64 { self as u64 }
        }
    )*};
}
macro_rules! impl_bitfield_signed {
    ($($t:ty),*) => {$(
        impl BitfieldValue for $t {
            /// Sign-extends from the field width, like reading a signed C bitfield.
            #[inline(always)]
            fn from_bits(bits: u64, width: u32) -> Self { (((bits << (64 - width)) as i64) >> (64 - width)) as $t }
            #[inline(always)]
            fn to_bits(self) -> u64 { self as i64 as u64 }
        }
    )*};
}
impl_bitfield_unsigned!(u8, u16, u32, u64);
impl_bitfield_signed!(i8, i16, i32, i64);

impl BitfieldValue for bool {
    #[inline(always)]
    fn from_bits(bits: u64, _width: u32) -> Self { bits != 0 }
    #[inline(always)]
    fn to_bits(self) -> u64 { self as u64 }
}

/// Storage that holds a run of C bitfields: an integer word, or (for generated code) the exact
/// bytes the run occupies, read little-endian.
pub trait BitStorage {
    fn load(&self) -> u128;
    fn store(&mut self, raw: u128);
}

macro_rules! impl_bit_storage_int {
    ($($t:ty),*) => {$(
        impl BitStorage for $t {
            #[inline(always)]
            fn load(&self) -> u128 { *self as u128 }
            #[inline(always)]
            fn store(&mut self, raw: u128) { *self = raw as $t; }
        }
    )*};
}
impl_bit_storage_int!(u8, u16, u32, u64);

impl<const N: usize> BitStorage for [u8; N] {
    #[inline(always)]
    fn load(&self) -> u128 {
        let mut b = [0u8; 16];
        b[..N].copy_from_slice(self);
        u128::from_le_bytes(b)
    }
    #[inline(always)]
    fn store(&mut self, raw: u128) {
        self.copy_from_slice(&raw.to_le_bytes()[..N]);
    }
}

/// Generates getters/setters for C bitfields packed into a storage field
/// (LSB-first, as GCC/Clang lay them out on AArch64).
///
/// `bitfields!(storage { getter / setter: shift, width => ValueType; ... })`
macro_rules! bitfields {
    ($word:ident { $($(#[$meta:meta])* $get:ident / $set:ident : $shift:literal, $width:literal => $vt:ty;)* }) => {$(
        $(#[$meta])*
        #[inline(always)]
        pub fn $get(&self) -> $vt {
            let raw = $crate::types::BitStorage::load(&self.$word);
            let bits = ((raw >> $shift) & ((1u128 << $width) - 1)) as u64;
            <$vt as $crate::types::BitfieldValue>::from_bits(bits, $width)
        }
        /// Sets the field, truncating `value` to the field width like a C bitfield assignment.
        $(#[$meta])*
        #[inline(always)]
        pub fn $set(&mut self, value: $vt) {
            let mask = ((1u128 << $width) - 1) << $shift;
            let bits = <$vt as $crate::types::BitfieldValue>::to_bits(value) as u128;
            let raw = $crate::types::BitStorage::load(&self.$word);
            $crate::types::BitStorage::store(&mut self.$word, (raw & !mask) | ((bits << $shift) & mask));
        }
    )*};
}
pub(crate) use bitfields;
