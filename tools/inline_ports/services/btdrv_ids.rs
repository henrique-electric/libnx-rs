// inline: APPEARANCE_CATEGORY
/// Extracts the category from a Bluetooth appearance value.
#[inline(always)]
pub const fn APPEARANCE_CATEGORY(x: u32) -> u32 {
    (x & 0xFFC0) >> 6
}
