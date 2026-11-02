//! `switch/audio/driver.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

#[repr(C)]
#[derive(Debug)]
pub struct AudioDriverEtc {
    _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioDriver {
    pub etc: *mut AudioDriverEtc,
    pub config: AudioRendererConfig,
    pub in_mempools: *mut AudioRendererMemPoolInfoIn,
    pub in_channels: *mut AudioRendererChannelInfoIn,
    pub in_voices: *mut AudioRendererVoiceInfoIn,
    pub in_mixes: *mut AudioRendererMixInfoIn,
    pub in_sinks: *mut AudioRendererSinkInfoIn,
}

pub type AudioDriverWaveBufState = u32;
pub const AudioDriverWaveBufState_Free: AudioDriverWaveBufState = 0;
pub const AudioDriverWaveBufState_Waiting: AudioDriverWaveBufState = 1;
pub const AudioDriverWaveBufState_Queued: AudioDriverWaveBufState = 2;
pub const AudioDriverWaveBufState_Playing: AudioDriverWaveBufState = 3;
pub const AudioDriverWaveBufState_Done: AudioDriverWaveBufState = 4;

#[repr(C)]
#[derive(Clone, Copy)]
pub union AudioDriverWaveBuf_anon0 {
    pub data_pcm16: *mut s16,
    pub data_adpcm: *mut u8,
    pub data_raw: *const c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AudioDriverWaveBuf {
    pub anon0: AudioDriverWaveBuf_anon0,
    pub size: u64,
    pub start_sample_offset: s32,
    pub end_sample_offset: s32,
    pub context_addr: *const c_void,
    pub context_sz: u64,
    pub _bitfield_0: [u8; 1],
    pub is_looping: bool,
    pub sequence_id: u32,
    pub next: *mut AudioDriverWaveBuf,
}

impl AudioDriverWaveBuf {
    bitfields!(_bitfield_0 {
        state / set_state: 0, 8 => AudioDriverWaveBufState;
    });
}

/// # Safety
/// `d` must be initialized and `id` a valid voice index.
#[inline(always)]
pub unsafe fn audrvVoiceSetExtraParams(d: &mut AudioDriver, id: c_int, params: *const c_void, params_size: usize) {
    let v = unsafe { &mut *d.in_voices.add(id as usize) };
    v.extra_params_ptr = params;
    v.extra_params_sz = params_size as _;
}

/// # Safety
/// `d` must be initialized and `id` a valid voice index.
#[inline(always)]
pub unsafe fn audrvVoiceSetDestinationMix(d: &mut AudioDriver, id: c_int, mix_id: c_int) {
    let v = unsafe { &mut *d.in_voices.add(id as usize) };
    v.dest_mix_id = mix_id as _;
    v.dest_splitter_id = AUDREN_UNUSED_SPLITTER_ID as _;
}

/// # Safety
/// `d` must be initialized and `id` a valid voice index; the channel ids must be in range.
#[inline(always)]
pub unsafe fn audrvVoiceSetMixFactor(d: &mut AudioDriver, id: c_int, factor: f32, src_channel_id: c_int, dest_channel_id: c_int) {
    unsafe {
        let channel_id = (*d.in_voices.add(id as usize)).channel_ids[src_channel_id as usize];
        (*d.in_channels.add(channel_id as usize)).mix[dest_channel_id as usize] = factor;
    }
}

/// # Safety
/// `d` must be initialized and `id` a valid voice index.
#[inline(always)]
pub unsafe fn audrvVoiceSetVolume(d: &mut AudioDriver, id: c_int, volume: f32) {
    unsafe { (*d.in_voices.add(id as usize)).volume = volume };
}

/// # Safety
/// `d` must be initialized and `id` a valid voice index.
#[inline(always)]
pub unsafe fn audrvVoiceSetPitch(d: &mut AudioDriver, id: c_int, pitch: f32) {
    unsafe { (*d.in_voices.add(id as usize)).pitch = pitch };
}

/// # Safety
/// `d` must be initialized and `id` a valid voice index.
#[inline(always)]
pub unsafe fn audrvVoiceSetPriority(d: &mut AudioDriver, id: c_int, priority: c_int) {
    unsafe { (*d.in_voices.add(id as usize)).priority = priority as _ };
}

/// # Safety
/// `d` must be initialized and `id` a valid voice index.
#[inline(always)]
pub unsafe fn audrvVoiceClearBiquadFilter(d: &mut AudioDriver, id: c_int, biquad_id: c_int) {
    unsafe { (*d.in_voices.add(id as usize)).biquads[biquad_id as usize].enable = false };
}

/// # Safety
/// `d` must be initialized and `id` a valid voice index.
#[inline(always)]
pub unsafe fn audrvVoiceSetPaused(d: &mut AudioDriver, id: c_int, paused: bool) {
    let state = if paused { AudioRendererVoicePlayState_Paused } else { AudioRendererVoicePlayState_Started };
    unsafe { (*d.in_voices.add(id as usize)).set_state(state) };
}

/// # Safety
/// `d` must be initialized and `id` a valid voice index.
#[inline(always)]
pub unsafe fn audrvVoiceStart(d: &mut AudioDriver, id: c_int) {
    unsafe { audrvVoiceSetPaused(d, id, false) }
}

/// # Safety
/// `d` must be initialized and `id` a valid mix index.
#[inline(always)]
pub unsafe fn audrvMixSetDestinationMix(d: &mut AudioDriver, id: c_int, mix_id: c_int) {
    let m = unsafe { &mut *d.in_mixes.add(id as usize) };
    m.dest_mix_id = mix_id as _;
    m.dest_splitter_id = AUDREN_UNUSED_SPLITTER_ID as _;
}

/// # Safety
/// `d` must be initialized and `id` a valid mix index; the channel ids must be in range.
#[inline(always)]
pub unsafe fn audrvMixSetMixFactor(d: &mut AudioDriver, id: c_int, factor: f32, src_channel_id: c_int, dest_channel_id: c_int) {
    unsafe { (*d.in_mixes.add(id as usize)).mix[src_channel_id as usize][dest_channel_id as usize] = factor };
}

/// # Safety
/// `d` must be initialized and `id` a valid mix index.
#[inline(always)]
pub unsafe fn audrvMixSetVolume(d: &mut AudioDriver, id: c_int, volume: f32) {
    unsafe { (*d.in_mixes.add(id as usize)).volume = volume };
}

impl_zeroed_default!(AudioDriver, AudioDriverWaveBuf_anon0, AudioDriverWaveBuf);

unsafe extern "C" {
    pub fn audrvCreate(d: *mut AudioDriver, config: *const AudioRendererConfig, num_final_mix_channels: c_int) -> Result;

    pub fn audrvUpdate(d: *mut AudioDriver) -> Result;

    pub fn audrvClose(d: *mut AudioDriver);

    pub fn audrvMemPoolAdd(d: *mut AudioDriver, buffer: *mut c_void, size: usize) -> c_int;

    pub fn audrvMemPoolRemove(d: *mut AudioDriver, id: c_int) -> bool;

    pub fn audrvMemPoolAttach(d: *mut AudioDriver, id: c_int) -> bool;

    pub fn audrvMemPoolDetach(d: *mut AudioDriver, id: c_int) -> bool;

    pub fn audrvVoiceInit(d: *mut AudioDriver, id: c_int, num_channels: c_int, format: PcmFormat, sample_rate: c_int) -> bool;

    pub fn audrvVoiceDrop(d: *mut AudioDriver, id: c_int);

    pub fn audrvVoiceStop(d: *mut AudioDriver, id: c_int);

    pub fn audrvVoiceIsPaused(d: *mut AudioDriver, id: c_int) -> bool;

    pub fn audrvVoiceIsPlaying(d: *mut AudioDriver, id: c_int) -> bool;

    pub fn audrvVoiceAddWaveBuf(d: *mut AudioDriver, id: c_int, wavebuf: *mut AudioDriverWaveBuf) -> bool;

    pub fn audrvVoiceGetWaveBufSeq(d: *mut AudioDriver, id: c_int) -> u32;

    pub fn audrvVoiceGetPlayedSampleCount(d: *mut AudioDriver, id: c_int) -> u32;

    pub fn audrvVoiceGetVoiceDropsCount(d: *mut AudioDriver, id: c_int) -> u32;

    pub fn audrvVoiceSetBiquadFilter(d: *mut AudioDriver, id: c_int, biquad_id: c_int, a0: f32, a1: f32, a2: f32, b0: f32, b1: f32, b2: f32);

    pub fn audrvMixAdd(d: *mut AudioDriver, sample_rate: c_int, num_channels: c_int) -> c_int;

    pub fn audrvMixRemove(d: *mut AudioDriver, id: c_int);

    pub fn audrvDeviceSinkAdd(d: *mut AudioDriver, device_name: *const c_char, num_channels: c_int, channel_ids: *const u8) -> c_int;

    pub fn audrvSinkRemove(d: *mut AudioDriver, id: c_int);
}
