// inline: hidKeyboardStateGetKey
#[inline(always)]
pub const fn hidKeyboardStateGetKey(state: &HidKeyboardState, key: HidKeyboardKey) -> bool {
    (state.keys[(key / 64) as usize] & (1u64 << (key & 63))) != 0
}
