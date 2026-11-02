// inline: NV_EVENT_ID_CTRL__SYNCPT
#[inline(always)]
pub const fn NV_EVENT_ID_CTRL__SYNCPT(slot: u32, syncpt: u32) -> u32 {
    (1u32 << 28) | (syncpt << 16) | slot
}
