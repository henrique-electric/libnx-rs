// inline: IF_Kbps
/// Kilobits/sec.
#[inline(always)]
pub const fn IF_Kbps(x: u64) -> u64 { x * 1000 }
// inline: IF_Mbps
/// Megabits/sec.
#[inline(always)]
pub const fn IF_Mbps(x: u64) -> u64 { IF_Kbps(x * 1000) }
// inline: IF_Gbps
/// Gigabits/sec.
#[inline(always)]
pub const fn IF_Gbps(x: u64) -> u64 { IF_Mbps(x * 1000) }
