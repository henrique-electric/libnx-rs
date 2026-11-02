// inline: waiterForHandle
#[inline(always)]
pub fn waiterForHandle(h: Handle) -> Waiter {
    Waiter { type_: WaiterType_Handle, anon0: Waiter_anon0 { handle: h } }
}
// inline: waitSingle
/// # Safety
/// `w` must refer to a valid handle or waitable object.
#[inline(always)]
pub unsafe fn waitSingle(w: Waiter, timeout: u64) -> Result {
    let mut idx: s32 = 0;
    unsafe { waitObjects(&mut idx, &w, 1, timeout) }
}
// inline: waitSingleHandle
/// # Safety
/// `h` must be a valid synchronization handle.
#[inline(always)]
pub unsafe fn waitSingleHandle(h: Handle, timeout: u64) -> Result {
    let mut idx: s32 = 0;
    unsafe { waitHandles(&mut idx, &h, 1, timeout) }
}
// inline: waitMulti
/// Helper for [`waitObjects`] that takes the waiters as a slice (replaces the variadic C macro).
///
/// **Note:** The number of objects must not be greater than `MAX_WAIT_OBJECTS`. This is a Horizon kernel limitation.
///
/// # Safety
/// Every waiter must refer to a valid handle or waitable object.
#[inline(always)]
pub unsafe fn waitMulti(idx_out: &mut s32, timeout: u64, objects: &[Waiter]) -> Result {
    unsafe { waitObjects(idx_out, objects.as_ptr(), objects.len() as s32, timeout) }
}
// inline: waitMultiHandle
/// Helper for [`waitHandles`] that takes the handles as a slice (replaces the variadic C macro).
///
/// **Note:** The number of objects must not be greater than `MAX_WAIT_OBJECTS`. This is a Horizon kernel limitation.
///
/// # Safety
/// Every handle must be a valid synchronization handle.
#[inline(always)]
pub unsafe fn waitMultiHandle(idx_out: &mut s32, timeout: u64, handles: &[Handle]) -> Result {
    unsafe { waitHandles(idx_out, handles.as_ptr(), handles.len() as s32, timeout) }
}
