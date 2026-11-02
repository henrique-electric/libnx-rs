// inline: ringconGetErrorFlags
#[inline(always)]
pub fn ringconGetErrorFlags(c: &RingCon) -> u32 {
    c.error_flags
}
// inline: ringconGetErrorFlag
#[inline(always)]
pub fn ringconGetErrorFlag(c: &RingCon, flag: RingConErrorFlag) -> bool {
    (c.error_flags & BIT(flag)) != 0
}
// inline: ringconGetFwVersion
#[inline(always)]
pub fn ringconGetFwVersion(c: &RingCon) -> RingConFwVersion {
    c.fw_ver
}
// inline: ringconGetId
#[inline(always)]
pub fn ringconGetId(c: &RingCon, id_l: &mut u64, id_h: &mut u64) {
    *id_l = c.id_l;
    *id_h = c.id_h;
}
// inline: ringconGetUnkCal
#[inline(always)]
pub fn ringconGetUnkCal(c: &RingCon) -> s16 {
    c.unk_cal
}
// inline: ringconGetTotalPushCount
#[inline(always)]
pub fn ringconGetTotalPushCount(c: &RingCon) -> s32 {
    c.total_push_count
}
// inline: ringconGetManuCal
#[inline(always)]
pub fn ringconGetManuCal(c: &RingCon, out: &mut RingConManuCal) {
    *out = c.manu_cal;
}
// inline: ringconGetUserCal
#[inline(always)]
pub fn ringconGetUserCal(c: &RingCon, out: &mut RingConUserCal) {
    *out = c.user_cal;
}
