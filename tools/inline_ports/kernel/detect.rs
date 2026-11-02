// inline: detectDebugger
#[inline(always)]
pub fn detectDebugger() -> bool {
    let mut tmp: u64 = 0;
    // SAFETY: svcGetInfo only writes to `tmp`.
    let rc = unsafe { svcGetInfo(&mut tmp, InfoType_DebuggerAttached, INVALID_HANDLE, 0) };
    R_SUCCEEDED(rc) && tmp != 0
}
// inline: detectMesosphere
#[inline(always)]
pub fn detectMesosphere() -> bool {
    let mut dummy: u64 = 0;
    // SAFETY: svcGetInfo only writes to `dummy`. 65000 = InfoType_MesosphereMeta.
    let rc = unsafe { svcGetInfo(&mut dummy, 65000, INVALID_HANDLE, 0) };
    R_SUCCEEDED(rc)
}
