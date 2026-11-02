// inline: waiterForEvent
#[inline(always)]
pub fn waiterForEvent(t: &Event) -> Waiter {
    Waiter {
        type_: if t.autoclear { WaiterType_HandleWithClear } else { WaiterType_Handle },
        anon0: Waiter_anon0 { handle: t.revent },
    }
}
// inline: eventActive
#[inline(always)]
pub fn eventActive(t: &Event) -> bool {
    t.revent != INVALID_HANDLE
}
