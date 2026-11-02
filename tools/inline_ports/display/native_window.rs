// inline: nwindowIsConsumerRunningBehind
#[inline(always)]
pub fn nwindowIsConsumerRunningBehind(nw: &NWindow) -> bool {
    nw.consumer_running_behind
}
