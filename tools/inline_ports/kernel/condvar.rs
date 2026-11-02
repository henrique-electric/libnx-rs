// inline: condvarInit
#[inline(always)]
pub fn condvarInit(c: &mut CondVar) {
    *c = 0;
}
// inline: condvarWait
/// # Safety
/// `m` must be locked by the calling thread.
#[inline(always)]
pub unsafe fn condvarWait(c: &mut CondVar, m: &mut Mutex) -> Result {
    unsafe { condvarWaitTimeout(c, m, u64::MAX) }
}
// inline: condvarWake
/// # Safety
/// `c` must be a valid condition variable.
#[inline(always)]
pub unsafe fn condvarWake(c: &mut CondVar, num: c_int) -> Result {
    unsafe { svcSignalProcessWideKey(c, num) };
    0
}
// inline: condvarWakeOne
/// # Safety
/// See [`condvarWake`].
#[inline(always)]
pub unsafe fn condvarWakeOne(c: &mut CondVar) -> Result {
    unsafe { condvarWake(c, 1) }
}
// inline: condvarWakeAll
/// # Safety
/// See [`condvarWake`].
#[inline(always)]
pub unsafe fn condvarWakeAll(c: &mut CondVar) -> Result {
    unsafe { condvarWake(c, -1) }
}
