// inline: nvMapGetHandle
#[inline(always)]
pub fn nvMapGetHandle(m: &NvMap) -> u32 {
    m.handle
}
// inline: nvMapGetId
#[inline(always)]
pub fn nvMapGetId(m: &NvMap) -> u32 {
    m.id
}
// inline: nvMapGetSize
#[inline(always)]
pub fn nvMapGetSize(m: &NvMap) -> u32 {
    m.size
}
// inline: nvMapGetCpuAddr
#[inline(always)]
pub fn nvMapGetCpuAddr(m: &NvMap) -> *mut c_void {
    m.cpu_addr
}
// inline: nvMapIsRemote
#[inline(always)]
pub fn nvMapIsRemote(m: &NvMap) -> bool {
    m.cpu_addr.is_null()
}
// inline: nvMapGetKind
#[inline(always)]
pub fn nvMapGetKind(m: &NvMap) -> NvKind {
    m.kind
}
