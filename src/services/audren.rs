//! `switch/services/audren.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const AUDREN_TIMER_FREQ_HZ: f32 = 200.0;
pub const AUDREN_TIMER_PERIOD_MS: f32 = 5.0;
pub const AUDREN_SAMPLES_PER_FRAME_32KHZ: u32 = 160;
pub const AUDREN_SAMPLES_PER_FRAME_48KHZ: u32 = 240;
pub const AUDREN_INPUT_PARAM_ALIGNMENT: u32 = 0x1000;
pub const AUDREN_OUTPUT_PARAM_ALIGNMENT: u32 = 0x10;
pub const AUDREN_MEMPOOL_ALIGNMENT: u32 = 0x1000;
pub const AUDREN_BUFFER_ALIGNMENT: u32 = 0x40;
pub const AUDREN_REVISION_1: u32 = 0x31564552;
pub const AUDREN_REVISION_2: u32 = 0x32564552;
pub const AUDREN_REVISION_3: u32 = 0x33564552;
pub const AUDREN_REVISION_4: u32 = 0x34564552;
pub const AUDREN_REVISION_5: u32 = 0x35564552;
pub const AUDREN_REVISION_6: u32 = 0x36564552;
pub const AUDREN_FINAL_MIX_ID: u32 = 0;
pub const AUDREN_UNUSED_MIX_ID: u32 = 0x7fffffff;
pub const AUDREN_UNUSED_SPLITTER_ID: u32 = 0xffffffff;
pub const AUDREN_DEFAULT_DEVICE_NAME: &CStr = c"MainAudioOut";

pub type AudioRendererOutputRate = u32;
pub const AudioRendererOutputRate_32kHz: AudioRendererOutputRate = 0;
pub const AudioRendererOutputRate_48kHz: AudioRendererOutputRate = 1;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioRendererConfig {
    pub output_rate: AudioRendererOutputRate,
    pub num_voices: c_int,
    pub num_effects: c_int,
    pub num_sinks: c_int,
    pub num_mix_objs: c_int,
    pub num_mix_buffers: c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioRendererUpdateDataHeader {
    pub revision: u32,
    pub behavior_sz: u32,
    pub mempools_sz: u32,
    pub voices_sz: u32,
    pub channels_sz: u32,
    pub effects_sz: u32,
    pub mixes_sz: u32,
    pub sinks_sz: u32,
    pub perfmgr_sz: u32,
    pub _padding: [u32; 6],
    pub total_sz: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioRendererBehaviorInfoIn {
    pub revision: u32,
    pub _padding1: u32,
    pub flags: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioRendererBehaviorInfoOut {
    pub unknown: [u64; 20],
    pub _padding1: [u64; 2],
}

pub type AudioRendererMemPoolState = u32;
pub const AudioRendererMemPoolState_Invalid: AudioRendererMemPoolState = 0;
pub const AudioRendererMemPoolState_New: AudioRendererMemPoolState = 1;
pub const AudioRendererMemPoolState_RequestDetach: AudioRendererMemPoolState = 2;
pub const AudioRendererMemPoolState_Detached: AudioRendererMemPoolState = 3;
pub const AudioRendererMemPoolState_RequestAttach: AudioRendererMemPoolState = 4;
pub const AudioRendererMemPoolState_Attached: AudioRendererMemPoolState = 5;
pub const AudioRendererMemPoolState_Released: AudioRendererMemPoolState = 6;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioRendererMemPoolInfoIn {
    pub address: *const c_void,
    pub size: u64,
    pub state: AudioRendererMemPoolState,
    pub _padding2: [u32; 3],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioRendererMemPoolInfoOut {
    pub new_state: AudioRendererMemPoolState,
    pub _padding2: [u32; 3],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioRendererChannelInfoIn {
    pub id: u32,
    pub mix: [f32; 24],
    pub is_used: bool,
    pub _padding1: [u8; 11],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioRendererBiquadFilter {
    pub enable: bool,
    pub _padding: u8,
    pub numerator: [s16; 3],
    pub denominator: [s16; 2],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioRendererAdpcmParameters {
    pub coefficients: [u16; 16],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioRendererAdpcmContext {
    pub index: u16,
    pub history0: s16,
    pub history1: s16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioRendererWaveBuf {
    pub address: *const c_void,
    pub size: u64,
    pub start_sample_offset: s32,
    pub end_sample_offset: s32,
    pub is_looping: bool,
    pub end_of_stream: bool,
    pub sent_to_server: bool,
    pub _padding1: [u8; 5],
    pub context_addr: *const c_void,
    pub context_sz: u64,
    pub _padding2: u64,
}

pub type AudioRendererVoicePlayState = u32;
pub const AudioRendererVoicePlayState_Started: AudioRendererVoicePlayState = 0;
pub const AudioRendererVoicePlayState_Stopped: AudioRendererVoicePlayState = 1;
pub const AudioRendererVoicePlayState_Paused: AudioRendererVoicePlayState = 2;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioRendererVoiceInfoIn {
    pub id: u32,
    pub node_id: u32,
    pub is_new: bool,
    pub is_used: bool,
    pub _bitfield_0: [u8; 2],
    pub sample_rate: u32,
    pub priority: u32,
    pub sorting_order: u32,
    pub channel_count: u32,
    pub pitch: f32,
    pub volume: f32,
    pub biquads: [AudioRendererBiquadFilter; 2],
    pub wavebuf_count: u32,
    pub wavebuf_head: s16,
    pub _padding1: u16,
    pub _padding2: u32,
    pub extra_params_ptr: *const c_void,
    pub extra_params_sz: u64,
    pub dest_mix_id: u32,
    pub dest_splitter_id: u32,
    pub wavebufs: [AudioRendererWaveBuf; 4],
    pub channel_ids: [u32; 6],
    pub _padding3: [u8; 24],
}

impl AudioRendererVoiceInfoIn {
    bitfields!(_bitfield_0 {
        state / set_state: 0, 8 => AudioRendererVoicePlayState;
        sample_format / set_sample_format: 8, 8 => PcmFormat;
    });
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioRendererVoiceInfoOut {
    pub played_sample_count: u64,
    pub num_wavebufs_consumed: u32,
    pub voice_drops_count: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioRendererMixInfoIn {
    pub volume: f32,
    pub sample_rate: u32,
    pub buffer_count: u32,
    pub is_used: bool,
    pub _padding1: [u8; 3],
    pub mix_id: u32,
    pub _padding2: u32,
    pub node_id: u32,
    pub _padding3: [u32; 2],
    pub mix: [[f32; 24]; 24],
    pub dest_mix_id: u32,
    pub dest_splitter_id: u32,
    pub _padding4: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioRendererDownMixParameters {
    pub coefficients: [u8; 16],
}

pub type AudioRendererSinkType = u32;
pub const AudioRendererSinkType_Invalid: AudioRendererSinkType = 0;
pub const AudioRendererSinkType_Device: AudioRendererSinkType = 1;
pub const AudioRendererSinkType_CircularBuffer: AudioRendererSinkType = 2;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioRendererDeviceSinkInfoIn {
    pub name: [c_char; 255],
    pub _padding1: u8,
    pub input_count: u32,
    pub inputs: [u8; 6],
    pub _padding2: u8,
    pub downmix_params_enabled: bool,
    pub downmix_params: AudioRendererDownMixParameters,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioRendererCircularBufferSinkInfoIn {
    pub buffer_ptr: *mut c_void,
    pub buffer_sz: u32,
    pub input_count: u32,
    pub sample_count: u32,
    pub last_read_offset: u32,
    pub sample_format: PcmFormat,
    pub inputs: [u8; 6],
    pub _padding2: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union AudioRendererSinkInfoIn_anon0 {
    pub device_sink: AudioRendererDeviceSinkInfoIn,
    pub circular_buffer_sink: AudioRendererCircularBufferSinkInfoIn,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AudioRendererSinkInfoIn {
    pub _bitfield_0: [u8; 1],
    pub is_used: bool,
    pub _padding1: [u8; 2],
    pub node_id: u32,
    pub _padding2: [u64; 3],
    pub anon0: AudioRendererSinkInfoIn_anon0,
}

impl AudioRendererSinkInfoIn {
    bitfields!(_bitfield_0 {
        type_ / set_type_: 0, 8 => AudioRendererSinkType;
    });
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioRendererSinkInfoOut {
    pub last_written_offset: u32,
    pub unk1: u32,
    pub unk2: u64,
    pub _padding1: [u64; 2],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioRendererPerformanceBufferInfoIn {
    pub detail_target: u32,
    pub _padding1: [u32; 3],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioRendererPerformanceBufferInfoOut {
    pub written_sz: u32,
    pub _padding1: [u32; 3],
}

/// # Safety
/// The audren service must be initialized.
#[inline(always)]
pub unsafe fn audrenGetRevision() -> u32 {
    unsafe extern "C" {
        static g_audrenRevision: u32;
    }
    unsafe { g_audrenRevision }
}

#[inline(always)]
pub const fn audrenGetMemPoolCount(config: &AudioRendererConfig) -> c_int {
    (config.num_effects + 4 * config.num_voices) as c_int
}

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

impl_zeroed_default!(AudioRendererConfig, AudioRendererUpdateDataHeader, AudioRendererBehaviorInfoIn, AudioRendererBehaviorInfoOut, AudioRendererMemPoolInfoIn, AudioRendererMemPoolInfoOut, AudioRendererChannelInfoIn, AudioRendererBiquadFilter, AudioRendererAdpcmParameters, AudioRendererAdpcmContext, AudioRendererWaveBuf, AudioRendererVoiceInfoIn, AudioRendererVoiceInfoOut, AudioRendererMixInfoIn, AudioRendererDownMixParameters, AudioRendererDeviceSinkInfoIn, AudioRendererCircularBufferSinkInfoIn, AudioRendererSinkInfoIn_anon0, AudioRendererSinkInfoIn, AudioRendererSinkInfoOut, AudioRendererPerformanceBufferInfoIn, AudioRendererPerformanceBufferInfoOut);

/// Builds an audio renderer node id.
#[inline(always)]
pub const fn AUDREN_NODEID(a: u32, b: u32, c: u32) -> u32 {
    ((a & 0xF) << 28) | ((b & 0xFFF) << 16) | (c & 0xFFFF)
}

unsafe extern "C" {
    /// Initialize audren.
    pub fn audrenInitialize(config: *const AudioRendererConfig) -> Result;

    /// Exit audren.
    pub fn audrenExit();

    /// Gets the Service object for IAudioRenderer.
    pub fn audrenGetServiceSession_AudioRenderer() -> *mut Service;

    /// Returns event that is signalled on new frame (autoclear=true).
    pub fn audrenGetFrameEvent() -> *mut Event;

    pub fn audrenWaitFrame();

    pub fn audrenGetState(out_state: *mut u32) -> Result;

    pub fn audrenRequestUpdateAudioRenderer(in_param_buf: *const c_void, in_param_buf_size: usize, out_param_buf: *mut c_void, out_param_buf_size: usize, perf_buf: *mut c_void, perf_buf_size: usize) -> Result;

    pub fn audrenStartAudioRenderer() -> Result;

    pub fn audrenStopAudioRenderer() -> Result;

    pub fn audrenSetAudioRendererRenderingTimeLimit(percent: c_int) -> Result;
}
