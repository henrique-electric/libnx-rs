// inline: hosversionAtLeast
#[inline(always)]
pub fn hosversionAtLeast(major: u8, minor: u8, micro: u8) -> bool {
    // SAFETY: reads a value cached by the libnx runtime.
    unsafe { hosversionGet() >= MAKEHOSVERSION(major as u32, minor as u32, micro as u32) }
}
// inline: hosversionBefore
#[inline(always)]
pub fn hosversionBefore(major: u8, minor: u8, micro: u8) -> bool {
    !hosversionAtLeast(major, minor, micro)
}
// inline: hosversionBetween
#[inline(always)]
pub fn hosversionBetween(major1: u8, major2: u8) -> bool {
    // SAFETY: reads a value cached by the libnx runtime.
    let ver = unsafe { hosversionGet() };
    ver >= MAKEHOSVERSION(major1 as u32, 0, 0) && ver < MAKEHOSVERSION(major2 as u32, 0, 0)
}
// inline: MAKEHOSVERSION
/// Builds a HOS version value.
#[inline(always)]
pub const fn MAKEHOSVERSION(major: u32, minor: u32, micro: u32) -> u32 {
    (major << 16) | (minor << 8) | micro
}
// inline: HOSVER_MAJOR
/// Extracts the major number from a HOS version value.
#[inline(always)]
pub const fn HOSVER_MAJOR(version: u32) -> u32 {
    (version >> 16) & 0xFF
}
// inline: HOSVER_MINOR
/// Extracts the minor number from a HOS version value.
#[inline(always)]
pub const fn HOSVER_MINOR(version: u32) -> u32 {
    (version >> 8) & 0xFF
}
// inline: HOSVER_MICRO
/// Extracts the micro number from a HOS version value.
#[inline(always)]
pub const fn HOSVER_MICRO(version: u32) -> u32 {
    version & 0xFF
}
