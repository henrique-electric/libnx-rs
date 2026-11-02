// inline: socketInitializeDefault
/// # Safety
/// Must not be called while the socket service is already initialized.
#[inline(always)]
pub unsafe fn socketInitializeDefault() -> Result {
    unsafe { socketInitialize(core::ptr::null()) }
}
