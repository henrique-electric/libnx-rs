// inline: audrenGetRevision
/// # Safety
/// The audren service must be initialized.
#[inline(always)]
pub unsafe fn audrenGetRevision() -> u32 {
    unsafe extern "C" {
        static g_audrenRevision: u32;
    }
    unsafe { g_audrenRevision }
}
// inline: audrenGetMemPoolCount
#[inline(always)]
pub const fn audrenGetMemPoolCount(config: &AudioRendererConfig) -> c_int {
    (config.num_effects + 4 * config.num_voices) as c_int
}
// inline: audrenGetInputParamSize
#[inline(always)]
pub const fn audrenGetInputParamSize(config: &AudioRendererConfig) -> usize {
    let mut size = 0usize;
    size += size_of::<AudioRendererUpdateDataHeader>();
    size += size_of::<AudioRendererBehaviorInfoIn>();
    size += size_of::<AudioRendererMemPoolInfoIn>() * audrenGetMemPoolCount(config) as usize;
    size += size_of::<AudioRendererChannelInfoIn>() * config.num_voices as usize;
    size += size_of::<AudioRendererVoiceInfoIn>() * config.num_voices as usize;
    // todo (upstream): effects, splitters
    size += size_of::<AudioRendererMixInfoIn>() * config.num_mix_objs as usize;
    size += size_of::<AudioRendererSinkInfoIn>() * config.num_sinks as usize;
    size += size_of::<AudioRendererPerformanceBufferInfoIn>();
    size
}
// inline: audrenGetOutputParamSize
#[inline(always)]
pub const fn audrenGetOutputParamSize(config: &AudioRendererConfig) -> usize {
    let mut size = 0usize;
    size += size_of::<AudioRendererUpdateDataHeader>();
    size += size_of::<AudioRendererMemPoolInfoOut>() * audrenGetMemPoolCount(config) as usize;
    size += size_of::<AudioRendererVoiceInfoOut>() * config.num_voices as usize;
    // todo (upstream): effects
    size += size_of::<AudioRendererSinkInfoOut>() * config.num_sinks as usize;
    size += size_of::<AudioRendererPerformanceBufferInfoOut>();
    size += size_of::<AudioRendererBehaviorInfoOut>();
    size
}
// inline: AUDREN_NODEID
/// Builds an audio renderer node id.
#[inline(always)]
pub const fn AUDREN_NODEID(a: u32, b: u32, c: u32) -> u32 {
    ((a & 0xF) << 28) | ((b & 0xFFF) << 16) | (c & 0xFFFF)
}
