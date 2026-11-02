// inline: binderIncreaseWeakRef
/// # Safety
/// `b` must be an initialized binder.
#[inline(always)]
pub unsafe fn binderIncreaseWeakRef(b: &mut Binder) -> Result {
    unsafe { binderAdjustRefcount(b, 1, 0) }
}
// inline: binderDecreaseWeakRef
/// # Safety
/// `b` must be an initialized binder.
#[inline(always)]
pub unsafe fn binderDecreaseWeakRef(b: &mut Binder) -> Result {
    unsafe { binderAdjustRefcount(b, -1, 0) }
}
// inline: binderIncreaseStrongRef
/// # Safety
/// `b` must be an initialized binder.
#[inline(always)]
pub unsafe fn binderIncreaseStrongRef(b: &mut Binder) -> Result {
    unsafe { binderAdjustRefcount(b, 1, 1) }
}
// inline: binderDecreaseStrongRef
/// # Safety
/// `b` must be an initialized binder.
#[inline(always)]
pub unsafe fn binderDecreaseStrongRef(b: &mut Binder) -> Result {
    unsafe { binderAdjustRefcount(b, -1, 1) }
}
