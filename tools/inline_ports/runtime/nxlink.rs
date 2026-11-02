// inline: nxlinkStdio
/// # Safety
/// Requires the socket service to be initialized.
#[inline(always)]
pub unsafe fn nxlinkStdio() -> c_int {
    unsafe { nxlinkConnectToHost(true, true) }
}
// inline: nxlinkStdioForDebug
/// # Safety
/// Requires the socket service to be initialized.
#[inline(always)]
pub unsafe fn nxlinkStdioForDebug() -> c_int {
    unsafe { nxlinkConnectToHost(false, true) }
}
