// inline: audrvVoiceSetExtraParams
/// # Safety
/// `d` must be initialized and `id` a valid voice index.
#[inline(always)]
pub unsafe fn audrvVoiceSetExtraParams(d: &mut AudioDriver, id: c_int, params: *const c_void, params_size: usize) {
    let v = unsafe { &mut *d.in_voices.add(id as usize) };
    v.extra_params_ptr = params;
    v.extra_params_sz = params_size as _;
}
// inline: audrvVoiceSetDestinationMix
/// # Safety
/// `d` must be initialized and `id` a valid voice index.
#[inline(always)]
pub unsafe fn audrvVoiceSetDestinationMix(d: &mut AudioDriver, id: c_int, mix_id: c_int) {
    let v = unsafe { &mut *d.in_voices.add(id as usize) };
    v.dest_mix_id = mix_id as _;
    v.dest_splitter_id = AUDREN_UNUSED_SPLITTER_ID as _;
}
// inline: audrvVoiceSetMixFactor
/// # Safety
/// `d` must be initialized and `id` a valid voice index; the channel ids must be in range.
#[inline(always)]
pub unsafe fn audrvVoiceSetMixFactor(d: &mut AudioDriver, id: c_int, factor: f32, src_channel_id: c_int, dest_channel_id: c_int) {
    unsafe {
        let channel_id = (*d.in_voices.add(id as usize)).channel_ids[src_channel_id as usize];
        (*d.in_channels.add(channel_id as usize)).mix[dest_channel_id as usize] = factor;
    }
}
// inline: audrvVoiceSetVolume
/// # Safety
/// `d` must be initialized and `id` a valid voice index.
#[inline(always)]
pub unsafe fn audrvVoiceSetVolume(d: &mut AudioDriver, id: c_int, volume: f32) {
    unsafe { (*d.in_voices.add(id as usize)).volume = volume };
}
// inline: audrvVoiceSetPitch
/// # Safety
/// `d` must be initialized and `id` a valid voice index.
#[inline(always)]
pub unsafe fn audrvVoiceSetPitch(d: &mut AudioDriver, id: c_int, pitch: f32) {
    unsafe { (*d.in_voices.add(id as usize)).pitch = pitch };
}
// inline: audrvVoiceSetPriority
/// # Safety
/// `d` must be initialized and `id` a valid voice index.
#[inline(always)]
pub unsafe fn audrvVoiceSetPriority(d: &mut AudioDriver, id: c_int, priority: c_int) {
    unsafe { (*d.in_voices.add(id as usize)).priority = priority as _ };
}
// inline: audrvVoiceClearBiquadFilter
/// # Safety
/// `d` must be initialized and `id` a valid voice index.
#[inline(always)]
pub unsafe fn audrvVoiceClearBiquadFilter(d: &mut AudioDriver, id: c_int, biquad_id: c_int) {
    unsafe { (*d.in_voices.add(id as usize)).biquads[biquad_id as usize].enable = false };
}
// inline: audrvVoiceSetPaused
/// # Safety
/// `d` must be initialized and `id` a valid voice index.
#[inline(always)]
pub unsafe fn audrvVoiceSetPaused(d: &mut AudioDriver, id: c_int, paused: bool) {
    let state = if paused { AudioRendererVoicePlayState_Paused } else { AudioRendererVoicePlayState_Started };
    unsafe { (*d.in_voices.add(id as usize)).set_state(state) };
}
// inline: audrvVoiceStart
/// # Safety
/// `d` must be initialized and `id` a valid voice index.
#[inline(always)]
pub unsafe fn audrvVoiceStart(d: &mut AudioDriver, id: c_int) {
    unsafe { audrvVoiceSetPaused(d, id, false) }
}
// inline: audrvMixSetDestinationMix
/// # Safety
/// `d` must be initialized and `id` a valid mix index.
#[inline(always)]
pub unsafe fn audrvMixSetDestinationMix(d: &mut AudioDriver, id: c_int, mix_id: c_int) {
    let m = unsafe { &mut *d.in_mixes.add(id as usize) };
    m.dest_mix_id = mix_id as _;
    m.dest_splitter_id = AUDREN_UNUSED_SPLITTER_ID as _;
}
// inline: audrvMixSetMixFactor
/// # Safety
/// `d` must be initialized and `id` a valid mix index; the channel ids must be in range.
#[inline(always)]
pub unsafe fn audrvMixSetMixFactor(d: &mut AudioDriver, id: c_int, factor: f32, src_channel_id: c_int, dest_channel_id: c_int) {
    unsafe { (*d.in_mixes.add(id as usize)).mix[src_channel_id as usize][dest_channel_id as usize] = factor };
}
// inline: audrvMixSetVolume
/// # Safety
/// `d` must be initialized and `id` a valid mix index.
#[inline(always)]
pub unsafe fn audrvMixSetVolume(d: &mut AudioDriver, id: c_int, volume: f32) {
    unsafe { (*d.in_mixes.add(id as usize)).volume = volume };
}
