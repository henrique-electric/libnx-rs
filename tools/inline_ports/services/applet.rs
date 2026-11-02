// inline: appletHolderGetExitEvent
#[inline(always)]
pub fn appletHolderGetExitEvent(h: &mut AppletHolder) -> &mut Event {
    &mut h.StateChangedEvent
}
