// inline: romfsInit
/// # Safety
/// Requires the fs service to be initialized.
#[inline(always)]
pub unsafe fn romfsInit() -> Result {
    unsafe { romfsMountSelf(c"romfs".as_ptr()) }
}
// inline: romfsExit
/// # Safety
/// `romfs:` must have been mounted with [`romfsInit`].
#[inline(always)]
pub unsafe fn romfsExit() -> Result {
    unsafe { romfsUnmount(c"romfs".as_ptr()) }
}
