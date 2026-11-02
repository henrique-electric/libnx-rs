// inline: jitGetRwAddr
#[inline(always)]
pub fn jitGetRwAddr(j: &Jit) -> *mut c_void {
    j.rw_addr
}
// inline: jitGetRxAddr
#[inline(always)]
pub fn jitGetRxAddr(j: &Jit) -> *mut c_void {
    j.rx_addr
}
