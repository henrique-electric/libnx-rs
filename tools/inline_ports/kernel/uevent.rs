// inline: waiterForUEvent
#[inline(always)]
pub fn waiterForUEvent(e: &mut UEvent) -> Waiter {
    Waiter { type_: WaiterType_Waitable, anon0: Waiter_anon0 { waitable: &raw mut e.waitable } }
}
