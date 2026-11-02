// inline: viOpenDefaultDisplay
/// # Safety
/// The vi service must be initialized.
#[inline(always)]
pub unsafe fn viOpenDefaultDisplay(display: &mut ViDisplay) -> Result {
    unsafe { viOpenDisplay(c"Default".as_ptr(), display) }
}
