//! `switch/nvidia/ioctl.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const _NV_IOC_NRBITS: u32 = 8;
pub const _NV_IOC_TYPEBITS: u32 = 8;
pub const _NV_IOC_SIZEBITS: u32 = 14;
pub const _NV_IOC_DIRBITS: u32 = 2;
pub const _NV_IOC_NRMASK: u32 = 255;
pub const _NV_IOC_TYPEMASK: u32 = 255;
pub const _NV_IOC_SIZEMASK: u32 = 0x3fff;
pub const _NV_IOC_DIRMASK: u32 = 3;
pub const _NV_IOC_NRSHIFT: u32 = 0;
pub const _NV_IOC_TYPESHIFT: u32 = 8;
pub const _NV_IOC_SIZESHIFT: u32 = 16;
pub const _NV_IOC_DIRSHIFT: u32 = 30;
pub const _NV_IOC_NONE: u32 = 0;
pub const _NV_IOC_WRITE: u32 = 1;
pub const _NV_IOC_READ: u32 = 2;
pub const NVGPU_ZBC_TYPE_INVALID: u32 = 0;
pub const NVGPU_ZBC_TYPE_COLOR: u32 = 1;
pub const NVGPU_ZBC_TYPE_DEPTH: u32 = 2;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct nvioctl_zcull_info {
    pub width_align_pixels: u32,
    pub height_align_pixels: u32,
    pub pixel_squares_by_aliquots: u32,
    pub aliquot_total: u32,
    pub region_byte_multiplier: u32,
    pub region_header_size: u32,
    pub subregion_header_size: u32,
    pub subregion_width_align_pixels: u32,
    pub subregion_height_align_pixels: u32,
    pub subregion_count: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct nvioctl_zbc_entry {
    pub color_ds: [u32; 4],
    pub color_l2: [u32; 4],
    pub depth: u32,
    pub ref_cnt: u32,
    pub format: u32,
    pub type_: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct nvioctl_gpu_characteristics {
    pub arch: u32,
    pub impl_: u32,
    pub rev: u32,
    pub num_gpc: u32,
    pub L2_cache_size: u64,
    pub on_board_video_memory_size: u64,
    pub num_tpc_per_gpc: u32,
    pub bus_type: u32,
    pub big_page_size: u32,
    pub compression_page_size: u32,
    pub pde_coverage_bit_count: u32,
    pub available_big_page_sizes: u32,
    pub gpc_mask: u32,
    pub sm_arch_sm_version: u32,
    pub sm_arch_spa_version: u32,
    pub sm_arch_warp_count: u32,
    pub gpu_va_bit_count: u32,
    pub reserved: u32,
    pub flags: u64,
    pub twod_class: u32,
    pub threed_class: u32,
    pub compute_class: u32,
    pub gpfifo_class: u32,
    pub inline_to_memory_class: u32,
    pub dma_copy_class: u32,
    pub max_fbps_count: u32,
    pub fbp_en_mask: u32,
    pub max_ltc_per_fbp: u32,
    pub max_lts_per_ltc: u32,
    pub max_tex_per_tpc: u32,
    pub max_gpc_count: u32,
    pub rop_l2_en_mask_0: u32,
    pub rop_l2_en_mask_1: u32,
    pub chipname: u64,
    pub gr_compbit_store_base_hw: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct nvioctl_va_region {
    pub offset: u64,
    pub page_size: u32,
    pub pad: u32,
    pub pages: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct nvioctl_zbc_slot_mask {
    pub slot: u32,
    pub mask: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct nvioctl_gpu_time {
    pub timestamp: u64,
    pub reserved: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct nvioctl_fence {
    pub id: u32,
    pub value: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union nvioctl_gpfifo_entry_anon0 {
    pub desc: u64,
    pub desc32: [u32; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct nvioctl_gpfifo_entry {
    pub anon0: nvioctl_gpfifo_entry_anon0,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct nvioctl_cmdbuf {
    pub mem: u32,
    pub offset: u32,
    pub words: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct nvioctl_reloc {
    pub cmdbuf_mem: u32,
    pub cmdbuf_offset: u32,
    pub target: u32,
    pub target_offset: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct nvioctl_reloc_shift {
    pub shift: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct nvioctl_syncpt_incr {
    pub syncpt_id: u32,
    pub syncpt_incrs: u32,
    pub waitbase_id: u32,
    pub next: u32,
    pub prev: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct nvioctl_command_buffer_map {
    pub handle: u32,
    pub iova: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct nvioctl_clk_rate {
    pub rate: u32,
    pub moduleid: u32,
}

pub type nvioctl_map_param = u32;
pub const NvMapParam_Size: nvioctl_map_param = 1;
pub const NvMapParam_Alignment: nvioctl_map_param = 2;
pub const NvMapParam_Base: nvioctl_map_param = 3;
pub const NvMapParam_Heap: nvioctl_map_param = 4;
pub const NvMapParam_Kind: nvioctl_map_param = 5;

pub type NvMapParam = nvioctl_map_param;

pub type nvioctl_channel_obj_classnum = u32;
pub const NvClassNumber_2D: nvioctl_channel_obj_classnum = 0x902D;
pub const NvClassNumber_3D: nvioctl_channel_obj_classnum = 0xB197;
pub const NvClassNumber_Compute: nvioctl_channel_obj_classnum = 0xB1C0;
pub const NvClassNumber_Kepler: nvioctl_channel_obj_classnum = 0xA140;
pub const NvClassNumber_DMA: nvioctl_channel_obj_classnum = 0xB0B5;
pub const NvClassNumber_ChannelGpfifo: nvioctl_channel_obj_classnum = 0xB06F;

pub type NvClassNumber = nvioctl_channel_obj_classnum;

pub type nvioctl_channel_priority = u32;
pub const NvChannelPriority_Low: nvioctl_channel_priority = 50;
pub const NvChannelPriority_Medium: nvioctl_channel_priority = 100;
pub const NvChannelPriority_High: nvioctl_channel_priority = 150;

pub type NvChannelPriority = nvioctl_channel_priority;

pub type NvZcullConfig = u32;
pub const NvZcullConfig_Global: NvZcullConfig = 0;
pub const NvZcullConfig_NoCtxSwitch: NvZcullConfig = 1;
pub const NvZcullConfig_SeparateBuffer: NvZcullConfig = 2;
pub const NvZcullConfig_PartOfRegularBuffer: NvZcullConfig = 3;

pub type NvAllocSpaceFlags = u32;
pub const NvAllocSpaceFlags_FixedOffset: NvAllocSpaceFlags = 1;
pub const NvAllocSpaceFlags_Sparse: NvAllocSpaceFlags = 2;

pub type NvMapBufferFlags = u32;
pub const NvMapBufferFlags_FixedOffset: NvMapBufferFlags = 1;
pub const NvMapBufferFlags_IsCacheable: NvMapBufferFlags = 4;
pub const NvMapBufferFlags_Modify: NvMapBufferFlags = 0x100;

pub type NvNotificationType = u32;
pub const NvNotificationType_FifoErrorIdleTimeout: NvNotificationType = 8;
pub const NvNotificationType_GrErrorSwNotify: NvNotificationType = 13;
pub const NvNotificationType_GrSemaphoreTimeout: NvNotificationType = 24;
pub const NvNotificationType_GrIllegalNotify: NvNotificationType = 25;
pub const NvNotificationType_FifoErrorMmuErrFlt: NvNotificationType = 31;
pub const NvNotificationType_PbdmaError: NvNotificationType = 32;
pub const NvNotificationType_ResetChannelVerifError: NvNotificationType = 43;
pub const NvNotificationType_PbdmaPushbufferCrcMismatch: NvNotificationType = 80;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NvNotification {
    pub timestamp: u64,
    pub info32: u32,
    pub info16: u16,
    pub status: u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NvError {
    pub type_: u32,
    pub info: [u32; 31],
}

impl_zeroed_default!(nvioctl_zcull_info, nvioctl_zbc_entry, nvioctl_gpu_characteristics, nvioctl_va_region, nvioctl_zbc_slot_mask, nvioctl_gpu_time, nvioctl_fence, nvioctl_gpfifo_entry_anon0, nvioctl_gpfifo_entry, nvioctl_cmdbuf, nvioctl_reloc, nvioctl_reloc_shift, nvioctl_syncpt_incr, nvioctl_command_buffer_map, nvioctl_clk_rate, NvNotification, NvError);

/// Builds an nvidia ioctl number.
#[inline(always)]
pub const fn _NV_IOC(dir: u32, type_: u32, nr: u32, size: u32) -> u32 {
    (dir << _NV_IOC_DIRSHIFT) | (type_ << _NV_IOC_TYPESHIFT) | (nr << _NV_IOC_NRSHIFT) | (size << _NV_IOC_SIZESHIFT)
}

#[inline(always)]
pub const fn _NV_IO(type_: u32, nr: u32) -> u32 {
    _NV_IOC(_NV_IOC_NONE, type_, nr, 0)
}

/// `size` is the C macro's `sizeof(size)` argument.
#[inline(always)]
pub const fn _NV_IOR(type_: u32, nr: u32, size: usize) -> u32 {
    _NV_IOC(_NV_IOC_READ, type_, nr, size as u32)
}

/// `size` is the C macro's `sizeof(size)` argument.
#[inline(always)]
pub const fn _NV_IOW(type_: u32, nr: u32, size: usize) -> u32 {
    _NV_IOC(_NV_IOC_WRITE, type_, nr, size as u32)
}

/// `size` is the C macro's `sizeof(size)` argument.
#[inline(always)]
pub const fn _NV_IOWR(type_: u32, nr: u32, size: usize) -> u32 {
    _NV_IOC(_NV_IOC_READ | _NV_IOC_WRITE, type_, nr, size as u32)
}

#[inline(always)]
pub const fn _NV_IOC_DIR(nr: u32) -> u32 {
    (nr >> _NV_IOC_DIRSHIFT) & _NV_IOC_DIRMASK
}

#[inline(always)]
pub const fn _NV_IOC_TYPE(nr: u32) -> u32 {
    (nr >> _NV_IOC_TYPESHIFT) & _NV_IOC_TYPEMASK
}

#[inline(always)]
pub const fn _NV_IOC_NR(nr: u32) -> u32 {
    (nr >> _NV_IOC_NRSHIFT) & _NV_IOC_NRMASK
}

#[inline(always)]
pub const fn _NV_IOC_SIZE(nr: u32) -> u32 {
    (nr >> _NV_IOC_SIZESHIFT) & _NV_IOC_SIZEMASK
}

unsafe extern "C" {
    pub fn nvioctlNvhostCtrl_SyncptRead(fd: u32, id: u32, out: *mut u32) -> Result;

    pub fn nvioctlNvhostCtrl_SyncptIncr(fd: u32, id: u32) -> Result;

    pub fn nvioctlNvhostCtrl_SyncptWait(fd: u32, id: u32, threshold: u32, timeout: u32) -> Result;

    pub fn nvioctlNvhostCtrl_EventSignal(fd: u32, event_id: u32) -> Result;

    pub fn nvioctlNvhostCtrl_EventWait(fd: u32, syncpt_id: u32, threshold: u32, timeout: s32, event_id: u32, out: *mut u32) -> Result;

    pub fn nvioctlNvhostCtrl_EventWaitAsync(fd: u32, syncpt_id: u32, threshold: u32, timeout: s32, event_id: u32) -> Result;

    pub fn nvioctlNvhostCtrl_EventRegister(fd: u32, event_id: u32) -> Result;

    pub fn nvioctlNvhostCtrl_EventUnregister(fd: u32, event_id: u32) -> Result;

    pub fn nvioctlNvhostCtrlGpu_ZCullGetCtxSize(fd: u32, out: *mut u32) -> Result;

    pub fn nvioctlNvhostCtrlGpu_ZCullGetInfo(fd: u32, out: *mut nvioctl_zcull_info) -> Result;

    pub fn nvioctlNvhostCtrlGpu_ZbcSetTable(fd: u32, color_ds: *const u32, color_l2: *const u32, depth: u32, format: u32, type_: u32) -> Result;

    pub fn nvioctlNvhostCtrlGpu_ZbcQueryTable(fd: u32, index: u32, out: *mut nvioctl_zbc_entry) -> Result;

    pub fn nvioctlNvhostCtrlGpu_GetCharacteristics(fd: u32, out: *mut nvioctl_gpu_characteristics) -> Result;

    pub fn nvioctlNvhostCtrlGpu_GetTpcMasks(fd: u32, buffer: *mut c_void, size: usize) -> Result;

    pub fn nvioctlNvhostCtrlGpu_ZbcGetActiveSlotMask(fd: u32, out: *mut nvioctl_zbc_slot_mask) -> Result;

    pub fn nvioctlNvhostCtrlGpu_GetGpuTime(fd: u32, out: *mut nvioctl_gpu_time) -> Result;

    pub fn nvioctlNvhostAsGpu_BindChannel(fd: u32, channel_fd: u32) -> Result;

    pub fn nvioctlNvhostAsGpu_AllocSpace(fd: u32, pages: u32, page_size: u32, flags: u32, align_or_offset: u64, offset: *mut u64) -> Result;

    pub fn nvioctlNvhostAsGpu_FreeSpace(fd: u32, offset: u64, pages: u32, page_size: u32) -> Result;

    pub fn nvioctlNvhostAsGpu_MapBufferEx(fd: u32, flags: u32, kind: u32, nvmap_handle: u32, page_size: u32, buffer_offset: u64, mapping_size: u64, input_offset: u64, offset: *mut u64) -> Result;

    pub fn nvioctlNvhostAsGpu_UnmapBuffer(fd: u32, offset: u64) -> Result;

    pub fn nvioctlNvhostAsGpu_GetVARegions(fd: u32, regions: *mut nvioctl_va_region) -> Result;

    pub fn nvioctlNvhostAsGpu_InitializeEx(fd: u32, flags: u32, big_page_size: u32) -> Result;

    pub fn nvioctlNvmap_Create(fd: u32, size: u32, nvmap_handle: *mut u32) -> Result;

    pub fn nvioctlNvmap_FromId(fd: u32, id: u32, nvmap_handle: *mut u32) -> Result;

    pub fn nvioctlNvmap_Alloc(fd: u32, nvmap_handle: u32, heapmask: u32, flags: u32, align: u32, kind: u8, addr: *mut c_void) -> Result;

    pub fn nvioctlNvmap_Free(fd: u32, nvmap_handle: u32) -> Result;

    pub fn nvioctlNvmap_Param(fd: u32, nvmap_handle: u32, param: NvMapParam, result: *mut u32) -> Result;

    pub fn nvioctlNvmap_GetId(fd: u32, nvmap_handle: u32, id: *mut u32) -> Result;

    pub fn nvioctlChannel_SetNvmapFd(fd: u32, nvmap_fd: u32) -> Result;

    pub fn nvioctlChannel_SubmitGpfifo(fd: u32, entries: *mut nvioctl_gpfifo_entry, num_entries: u32, flags: u32, fence_inout: *mut nvioctl_fence) -> Result;

    pub fn nvioctlChannel_KickoffPb(fd: u32, entries: *mut nvioctl_gpfifo_entry, num_entries: u32, flags: u32, fence_inout: *mut nvioctl_fence) -> Result;

    pub fn nvioctlChannel_AllocObjCtx(fd: u32, class_num: u32, flags: u32, id_out: *mut u64) -> Result;

    pub fn nvioctlChannel_ZCullBind(fd: u32, gpu_va: u64, mode: u32) -> Result;

    pub fn nvioctlChannel_SetErrorNotifier(fd: u32, enable: u32) -> Result;

    pub fn nvioctlChannel_GetErrorInfo(fd: u32, out: *mut NvError) -> Result;

    pub fn nvioctlChannel_GetErrorNotification(fd: u32, out: *mut NvNotification) -> Result;

    pub fn nvioctlChannel_SetPriority(fd: u32, priority: u32) -> Result;

    pub fn nvioctlChannel_SetTimeout(fd: u32, timeout: u32) -> Result;

    pub fn nvioctlChannel_AllocGpfifoEx2(fd: u32, num_entries: u32, flags: u32, unk0: u32, unk1: u32, unk2: u32, unk3: u32, fence_out: *mut nvioctl_fence) -> Result;

    pub fn nvioctlChannel_SetUserData(fd: u32, addr: *mut c_void) -> Result;

    pub fn nvioctlChannel_Submit(fd: u32, cmdbufs: *const nvioctl_cmdbuf, num_cmdbufs: u32, relocs: *const nvioctl_reloc, reloc_shifts: *const nvioctl_reloc_shift, num_relocs: u32, syncpt_incrs: *const nvioctl_syncpt_incr, num_syncpt_incrs: u32, fences: *mut nvioctl_fence, num_fences: u32) -> Result;

    pub fn nvioctlChannel_GetSyncpt(fd: u32, module_id: u32, syncpt: *mut u32) -> Result;

    pub fn nvioctlChannel_GetModuleClockRate(fd: u32, module_id: u32, freq: *mut u32) -> Result;

    pub fn nvioctlChannel_SetModuleClockRate(fd: u32, module_id: u32, freq: u32) -> Result;

    pub fn nvioctlChannel_MapCommandBuffer(fd: u32, maps: *mut nvioctl_command_buffer_map, num_maps: u32, compressed: bool) -> Result;

    pub fn nvioctlChannel_UnmapCommandBuffer(fd: u32, maps: *const nvioctl_command_buffer_map, num_maps: u32, compressed: bool) -> Result;

    pub fn nvioctlChannel_SetSubmitTimeout(fd: u32, timeout: u32) -> Result;
}
