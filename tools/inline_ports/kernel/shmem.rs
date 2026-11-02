// inline: shmemGetAddr
#[inline(always)]
pub fn shmemGetAddr(s: &SharedMemory) -> *mut c_void {
    s.map_addr
}
