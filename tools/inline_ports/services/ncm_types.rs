// inline: ncmContentInfoSizeToU64
#[inline(always)]
pub fn ncmContentInfoSizeToU64(info: &NcmContentInfo, out: &mut u64) {
    *out = ((info.size_high as u64) << 32) | info.size_low as u64;
}
// inline: ncmU64ToContentInfoSize
#[inline(always)]
pub fn ncmU64ToContentInfoSize(size: u64, info: &mut NcmContentInfo) {
    info.size_low = (size & 0xFFFFFFFF) as _;
    info.size_high = (size >> 32) as u8;
}
