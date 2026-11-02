// inline: nvGpuChannelGetSyncpointId
#[inline(always)]
pub fn nvGpuChannelGetSyncpointId(c: &NvGpuChannel) -> u32 {
    c.fence.id
}
// inline: nvGpuChannelGetFence
#[inline(always)]
pub fn nvGpuChannelGetFence(c: &NvGpuChannel, fence_out: &mut NvFence) {
    fence_out.id = c.fence.id;
    fence_out.value = c.fence.value.wrapping_add(c.fence_incr);
}
// inline: nvGpuChannelIncrFence
#[inline(always)]
pub fn nvGpuChannelIncrFence(c: &mut NvGpuChannel) {
    c.fence_incr = c.fence_incr.wrapping_add(1);
}
