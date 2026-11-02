// inline: accountUidIsValid
#[inline(always)]
pub const fn accountUidIsValid(Uid: &AccountUid) -> bool {
    Uid.uid[0] != 0 || Uid.uid[1] != 0
}
