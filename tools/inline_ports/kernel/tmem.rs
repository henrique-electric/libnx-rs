// inline: tmemGetAddr
#[inline(always)]
pub fn tmemGetAddr(t: &TransferMemory) -> *mut c_void {
    t.map_addr
}
