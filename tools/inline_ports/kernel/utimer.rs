// inline: waiterForUTimer
#[inline(always)]
pub fn waiterForUTimer(t: &mut UTimer) -> Waiter {
    Waiter { type_: WaiterType_Waitable, anon0: Waiter_anon0 { waitable: &raw mut t.waitable } }
}
