// inline: sessionmgrGetClientSession
#[inline(always)]
pub fn sessionmgrGetClientSession(mgr: &SessionMgr, slot: c_int) -> Handle {
    mgr.sessions[slot as usize]
}
