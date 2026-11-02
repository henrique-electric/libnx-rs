// inline: mutexInit
#[inline(always)]
pub fn mutexInit(m: &mut Mutex) {
    *m = INVALID_HANDLE;
}
// inline: rmutexInit
#[inline(always)]
pub fn rmutexInit(m: &mut RMutex) {
    m.lock = 0;
    m.counter = 0;
}
