// inline: nvMultiFenceCreate
#[inline(always)]
pub fn nvMultiFenceCreate(mf: &mut NvMultiFence, fence: &NvFence) {
    mf.num_fences = 1;
    mf.fences[0] = *fence;
}
