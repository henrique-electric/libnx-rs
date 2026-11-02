// inline: waiterForThread
#[inline(always)]
pub fn waiterForThread(t: &Thread) -> Waiter {
    waiterForHandle(t.handle)
}
