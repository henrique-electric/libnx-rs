// inline: lp2pGroupInfoSetMemberCountMax
#[inline(always)]
pub fn lp2pGroupInfoSetMemberCountMax(info: &mut Lp2pGroupInfo, count: usize) {
    info.member_count_max = count as _;
}
// inline: lp2pGroupInfoSetFrequencyChannel
#[inline(always)]
pub fn lp2pGroupInfoSetFrequencyChannel(info: &mut Lp2pGroupInfo, frequency: u16, channel: s16) {
    info.frequency = frequency;
    info.channel = channel;
}
// inline: lp2pGroupInfoSetStealthEnabled
#[inline(always)]
pub fn lp2pGroupInfoSetStealthEnabled(info: &mut Lp2pGroupInfo, flag: bool) {
    info.stealth_enabled = flag as _;
}
