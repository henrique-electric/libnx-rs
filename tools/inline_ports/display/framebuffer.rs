// inline: RGBA8
/// Converts red, green, blue, and alpha components to packed RGBA8.
#[inline(always)]
pub const fn RGBA8(r: u32, g: u32, b: u32, a: u32) -> u32 {
    (r & 0xff) | ((g & 0xff) << 8) | ((b & 0xff) << 16) | ((a & 0xff) << 24)
}
// inline: RGBA8_MAXALPHA
/// Same as [`RGBA8`] except with alpha=0xff.
#[inline(always)]
pub const fn RGBA8_MAXALPHA(r: u32, g: u32, b: u32) -> u32 {
    RGBA8(r, g, b, 0xff)
}
// inline: RGBX8
/// Same as [`RGBA8`] with alpha=0 (for RGBX8).
#[inline(always)]
pub const fn RGBX8(r: u32, g: u32, b: u32) -> u32 {
    RGBA8(r, g, b, 0)
}
// inline: RGB565
/// Converts red (5 bits), green (6 bits), and blue (5 bits) components to packed RGB565.
#[inline(always)]
pub const fn RGB565(r: u32, g: u32, b: u32) -> u32 {
    (b & 0x1f) | ((g & 0x3f) << 5) | ((r & 0x1f) << 11)
}
// inline: RGB565_FROM_RGB8
/// Converts 8-bit red, green, and blue components to packed RGB565.
#[inline(always)]
pub const fn RGB565_FROM_RGB8(r: u32, g: u32, b: u32) -> u32 {
    RGB565(r >> 3, g >> 2, b >> 3)
}
// inline: BGRA8
/// Same as [`RGBA8`] except with the red and blue components swapped.
#[inline(always)]
pub const fn BGRA8(r: u32, g: u32, b: u32, a: u32) -> u32 {
    RGBA8(b, g, r, a)
}
// inline: BGRA8_MAXALPHA
/// Same as [`BGRA8`] except with alpha=0xff.
#[inline(always)]
pub const fn BGRA8_MAXALPHA(r: u32, g: u32, b: u32) -> u32 {
    RGBA8(b, g, r, 0xff)
}
// inline: RGBA4
/// Converts red, green, blue, and alpha components to packed RGBA4.
#[inline(always)]
pub const fn RGBA4(r: u32, g: u32, b: u32, a: u32) -> u32 {
    (r & 0xf) | ((g & 0xf) << 4) | ((b & 0xf) << 8) | ((a & 0xf) << 12)
}
// inline: RGBA4_MAXALPHA
/// Same as [`RGBA4`] except with alpha=0xf.
#[inline(always)]
pub const fn RGBA4_MAXALPHA(r: u32, g: u32, b: u32) -> u32 {
    RGBA4(r, g, b, 0xf)
}
// inline: RGBA4_FROM_RGBA8
/// Converts red, green, blue, and alpha components to packed RGBA4.
#[inline(always)]
pub const fn RGBA4_FROM_RGBA8(r: u32, g: u32, b: u32, a: u32) -> u32 {
    RGBA4(r >> 4, g >> 4, b >> 4, a >> 4)
}
// inline: RGBA4_FROM_RGBA8_MAXALPHA
/// Same as [`RGBA4_FROM_RGBA8`] except with alpha=0xf.
#[inline(always)]
pub const fn RGBA4_FROM_RGBA8_MAXALPHA(r: u32, g: u32, b: u32) -> u32 {
    RGBA4_MAXALPHA(r >> 4, g >> 4, b >> 4)
}
