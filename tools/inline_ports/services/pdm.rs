// inline: pdmPlayTimestampToPosix
#[inline(always)]
pub const fn pdmPlayTimestampToPosix(timestamp: u32) -> u64 {
    (timestamp as u64) * 60 + 946598400
}
