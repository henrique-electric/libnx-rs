// inline: DLT_CLASS
#[inline(always)]
pub const fn DLT_CLASS(x: u32) -> u32 { x & 0x03ff0000 }
// inline: DLT_NETBSD_RAWAF
#[inline(always)]
pub const fn DLT_NETBSD_RAWAF(af: u32) -> u32 { DLT_CLASS_NETBSD_RAWAF | af }
// inline: DLT_NETBSD_RAWAF_AF
#[inline(always)]
pub const fn DLT_NETBSD_RAWAF_AF(x: u32) -> u32 { x & 0x0000ffff }
// inline: DLT_IS_NETBSD_RAWAF
#[inline(always)]
pub const fn DLT_IS_NETBSD_RAWAF(x: u32) -> bool { DLT_CLASS(x) == DLT_CLASS_NETBSD_RAWAF }
