//! Horizon Inter-Process Communication protocol. (`switch/sf/hipc.h`)

use core::ffi::c_void;
use core::ptr;

use crate::types::*;

pub const HIPC_AUTO_RECV_STATIC: u32 = u8::MAX as u32;
pub const HIPC_RESPONSE_NO_PID: u64 = u32::MAX as u64;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct HipcMetadata {
    pub type_: u32,
    pub num_send_statics: u32,
    pub num_send_buffers: u32,
    pub num_recv_buffers: u32,
    pub num_exch_buffers: u32,
    pub num_data_words: u32,
    /// Also accepts [`HIPC_AUTO_RECV_STATIC`].
    pub num_recv_statics: u32,
    pub send_pid: u32,
    pub num_copy_handles: u32,
    pub num_move_handles: u32,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct HipcHeader {
    /// First raw bitfield word (`type`, `num_*_statics/buffers`).
    pub bits0: u32,
    /// Second raw bitfield word (`num_data_words`, `recv_static_mode`, ...).
    pub bits1: u32,
}

impl HipcHeader {
    bitfields!(bits0 {
        type_ / set_type_: 0, 16 => u32;
        num_send_statics / set_num_send_statics: 16, 4 => u32;
        num_send_buffers / set_num_send_buffers: 20, 4 => u32;
        num_recv_buffers / set_num_recv_buffers: 24, 4 => u32;
        num_exch_buffers / set_num_exch_buffers: 28, 4 => u32;
    });
    bitfields!(bits1 {
        num_data_words / set_num_data_words: 0, 10 => u32;
        recv_static_mode / set_recv_static_mode: 10, 4 => u32;
        padding / set_padding: 14, 6 => u32;
        /// Unused.
        recv_list_offset / set_recv_list_offset: 20, 11 => u32;
        has_special_header / set_has_special_header: 31, 1 => u32;
    });
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct HipcSpecialHeader {
    /// Raw bitfield word.
    pub bits: u32,
}

impl HipcSpecialHeader {
    bitfields!(bits {
        send_pid / set_send_pid: 0, 1 => u32;
        num_copy_handles / set_num_copy_handles: 1, 4 => u32;
        num_move_handles / set_num_move_handles: 5, 4 => u32;
        padding / set_padding: 9, 23 => u32;
    });
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct HipcStaticDescriptor {
    /// Raw bitfield word (`index`, `address_high`, `address_mid`, `size`).
    pub bits: u32,
    pub address_low: u32,
}

impl HipcStaticDescriptor {
    bitfields!(bits {
        index / set_index: 0, 6 => u32;
        address_high / set_address_high: 6, 6 => u32;
        address_mid / set_address_mid: 12, 4 => u32;
        size / set_size: 16, 16 => u32;
    });
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct HipcBufferDescriptor {
    pub size_low: u32,
    pub address_low: u32,
    /// Raw bitfield word (`mode`, `address_high`, `size_high`, `address_mid`).
    pub bits: u32,
}

impl HipcBufferDescriptor {
    bitfields!(bits {
        mode / set_mode: 0, 2 => u32;
        address_high / set_address_high: 2, 22 => u32;
        size_high / set_size_high: 24, 4 => u32;
        address_mid / set_address_mid: 28, 4 => u32;
    });
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct HipcRecvListEntry {
    pub address_low: u32,
    /// Raw bitfield word (`address_high`, `size`).
    pub bits: u32,
}

impl HipcRecvListEntry {
    bitfields!(bits {
        address_high / set_address_high: 0, 16 => u32;
        size / set_size: 16, 16 => u32;
    });
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HipcRequest {
    pub send_statics: *mut HipcStaticDescriptor,
    pub send_buffers: *mut HipcBufferDescriptor,
    pub recv_buffers: *mut HipcBufferDescriptor,
    pub exch_buffers: *mut HipcBufferDescriptor,
    pub data_words: *mut u32,
    pub recv_list: *mut HipcRecvListEntry,
    pub copy_handles: *mut Handle,
    pub move_handles: *mut Handle,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct HipcParsedRequest {
    pub meta: HipcMetadata,
    pub data: HipcRequest,
    pub pid: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HipcResponse {
    pub pid: u64,
    pub num_statics: u32,
    pub num_data_words: u32,
    pub num_copy_handles: u32,
    pub num_move_handles: u32,
    pub statics: *mut HipcStaticDescriptor,
    pub data_words: *mut u32,
    pub copy_handles: *mut Handle,
    pub move_handles: *mut Handle,
}

impl_zeroed_default!(HipcRequest, HipcResponse);

pub type HipcBufferMode = u32;
pub const HipcBufferMode_Normal: HipcBufferMode = 0;
pub const HipcBufferMode_NonSecure: HipcBufferMode = 1;
pub const HipcBufferMode_Invalid: HipcBufferMode = 2;
pub const HipcBufferMode_NonDevice: HipcBufferMode = 3;

#[inline(always)]
pub fn hipcMakeSendStatic(buffer: *const c_void, size: usize, index: u8) -> HipcStaticDescriptor {
    let addr = buffer as usize;
    let mut d = HipcStaticDescriptor { bits: 0, address_low: addr as u32 };
    d.set_index(index as u32);
    d.set_address_high((addr >> 36) as u32);
    d.set_address_mid((addr >> 32) as u32);
    d.set_size(size as u32);
    d
}

#[inline(always)]
pub fn hipcMakeBuffer(buffer: *const c_void, size: usize, mode: HipcBufferMode) -> HipcBufferDescriptor {
    let addr = buffer as usize;
    let mut d = HipcBufferDescriptor { size_low: size as u32, address_low: addr as u32, bits: 0 };
    d.set_mode(mode);
    d.set_address_high((addr >> 36) as u32);
    d.set_size_high((size as u64 >> 32) as u32);
    d.set_address_mid((addr >> 32) as u32);
    d
}

#[inline(always)]
pub fn hipcMakeRecvStatic(buffer: *mut c_void, size: usize) -> HipcRecvListEntry {
    let addr = buffer as usize;
    let mut e = HipcRecvListEntry { address_low: addr as u32, bits: 0 };
    e.set_address_high((addr >> 32) as u32);
    e.set_size(size as u32);
    e
}

#[inline(always)]
pub fn hipcGetStaticAddress(desc: &HipcStaticDescriptor) -> *mut c_void {
    (desc.address_low as usize | ((desc.address_mid() as usize) << 32) | ((desc.address_high() as usize) << 36)) as *mut c_void
}

#[inline(always)]
pub fn hipcGetStaticSize(desc: &HipcStaticDescriptor) -> usize {
    desc.size() as usize
}

#[inline(always)]
pub fn hipcGetBufferAddress(desc: &HipcBufferDescriptor) -> *mut c_void {
    (desc.address_low as usize | ((desc.address_mid() as usize) << 32) | ((desc.address_high() as usize) << 36)) as *mut c_void
}

#[inline(always)]
pub fn hipcGetBufferSize(desc: &HipcBufferDescriptor) -> usize {
    desc.size_low as usize | ((desc.size_high() as usize) << 32)
}

/// Lays out the request sections that follow the header(s), starting at `base`.
///
/// # Safety
/// `base` must point to a buffer large enough for every section described by `meta`.
#[inline(always)]
pub unsafe fn hipcCalcRequestLayout(meta: HipcMetadata, base: *mut c_void) -> HipcRequest {
    let mut base = base;

    // Takes `count` elements of `T` from the cursor, or returns null when there are none.
    macro_rules! take {
        ($t:ty, $count:expr) => {
            if $count != 0 {
                let p = base.cast::<$t>();
                base = unsafe { p.add($count as usize) }.cast();
                p
            } else {
                ptr::null_mut()
            }
        };
    }

    let copy_handles = take!(Handle, meta.num_copy_handles);
    let move_handles = take!(Handle, meta.num_move_handles);
    let send_statics = take!(HipcStaticDescriptor, meta.num_send_statics);
    let send_buffers = take!(HipcBufferDescriptor, meta.num_send_buffers);
    let recv_buffers = take!(HipcBufferDescriptor, meta.num_recv_buffers);
    let exch_buffers = take!(HipcBufferDescriptor, meta.num_exch_buffers);
    let data_words = take!(u32, meta.num_data_words);

    // Recv list
    let recv_list = if meta.num_recv_statics != 0 { base.cast::<HipcRecvListEntry>() } else { ptr::null_mut() };

    HipcRequest { send_statics, send_buffers, recv_buffers, exch_buffers, data_words, recv_list, copy_handles, move_handles }
}

/// Writes the HIPC header(s) for `meta` at `base` and returns the request layout.
///
/// This also replaces the C `hipcMakeRequestInline` macro: use
/// `hipcMakeRequest(base, HipcMetadata { type_: .., ..Default::default() })`.
///
/// # Safety
/// `base` must point to a 4-byte-aligned buffer (normally TLS) large enough for the message.
#[inline(always)]
pub unsafe fn hipcMakeRequest(base: *mut c_void, meta: HipcMetadata) -> HipcRequest {
    // Write message header
    let has_special_header = meta.send_pid != 0 || meta.num_copy_handles != 0 || meta.num_move_handles != 0;
    let hdr = base.cast::<HipcHeader>();
    let mut base: *mut c_void = unsafe { hdr.add(1) }.cast();
    let mut h = HipcHeader::default();
    h.set_type_(meta.type_);
    h.set_num_send_statics(meta.num_send_statics);
    h.set_num_send_buffers(meta.num_send_buffers);
    h.set_num_recv_buffers(meta.num_recv_buffers);
    h.set_num_exch_buffers(meta.num_exch_buffers);
    h.set_num_data_words(meta.num_data_words);
    h.set_recv_static_mode(if meta.num_recv_statics != 0 {
        if meta.num_recv_statics != HIPC_AUTO_RECV_STATIC { 2u32.wrapping_add(meta.num_recv_statics) } else { 2 }
    } else {
        0
    });
    h.set_has_special_header(has_special_header as u32);
    unsafe { hdr.write(h) };

    // Write special header
    if has_special_header {
        let sphdr = base.cast::<HipcSpecialHeader>();
        base = unsafe { sphdr.add(1) }.cast();
        let mut s = HipcSpecialHeader::default();
        s.set_send_pid(meta.send_pid);
        s.set_num_copy_handles(meta.num_copy_handles);
        s.set_num_move_handles(meta.num_move_handles);
        unsafe { sphdr.write(s) };
        if meta.send_pid != 0 {
            base = unsafe { base.byte_add(size_of::<u64>()) };
        }
    }

    // Calculate layout
    unsafe { hipcCalcRequestLayout(meta, base) }
}

/// Parses a request message received at `base`.
///
/// # Safety
/// `base` must point to a valid HIPC request message.
#[inline(always)]
pub unsafe fn hipcParseRequest(base: *mut c_void) -> HipcParsedRequest {
    // Parse message header
    let hdr = unsafe { base.cast::<HipcHeader>().read_unaligned() };
    let mut base = unsafe { base.byte_add(size_of::<HipcHeader>()) };
    let mut num_recv_statics = 0u32;
    let mut pid = 0u64;

    // Parse recv static mode
    if hdr.recv_static_mode() != 0 {
        if hdr.recv_static_mode() == 2 {
            num_recv_statics = HIPC_AUTO_RECV_STATIC;
        } else if hdr.recv_static_mode() > 2 {
            num_recv_statics = hdr.recv_static_mode() - 2;
        }
    }

    // Parse special header
    let mut sphdr = HipcSpecialHeader::default();
    if hdr.has_special_header() != 0 {
        sphdr = unsafe { base.cast::<HipcSpecialHeader>().read_unaligned() };
        base = unsafe { base.byte_add(size_of::<HipcSpecialHeader>()) };

        // Read PID descriptor
        if sphdr.send_pid() != 0 {
            pid = unsafe { base.cast::<u64>().read_unaligned() };
            base = unsafe { base.byte_add(size_of::<u64>()) };
        }
    }

    let meta = HipcMetadata {
        type_: hdr.type_(),
        num_send_statics: hdr.num_send_statics(),
        num_send_buffers: hdr.num_send_buffers(),
        num_recv_buffers: hdr.num_recv_buffers(),
        num_exch_buffers: hdr.num_exch_buffers(),
        num_data_words: hdr.num_data_words(),
        num_recv_statics,
        send_pid: sphdr.send_pid(),
        num_copy_handles: sphdr.num_copy_handles(),
        num_move_handles: sphdr.num_move_handles(),
    };

    HipcParsedRequest { meta, data: unsafe { hipcCalcRequestLayout(meta, base) }, pid }
}

/// Parses a response message received at `base`.
///
/// # Safety
/// `base` must point to a valid HIPC response message.
#[inline(always)]
pub unsafe fn hipcParseResponse(base: *mut c_void) -> HipcResponse {
    // Parse header
    let hdr = unsafe { base.cast::<HipcHeader>().read_unaligned() };
    let mut base = unsafe { base.byte_add(size_of::<HipcHeader>()) };

    // Initialize response
    let mut response = HipcResponse {
        num_statics: hdr.num_send_statics(),
        num_data_words: hdr.num_data_words(),
        pid: HIPC_RESPONSE_NO_PID,
        ..Default::default()
    };

    // Parse special header
    if hdr.has_special_header() != 0 {
        let sphdr = unsafe { base.cast::<HipcSpecialHeader>().read_unaligned() };
        base = unsafe { base.byte_add(size_of::<HipcSpecialHeader>()) };

        // Update response
        response.num_copy_handles = sphdr.num_copy_handles();
        response.num_move_handles = sphdr.num_move_handles();

        // Parse PID descriptor
        if sphdr.send_pid() != 0 {
            response.pid = unsafe { base.cast::<u64>().read_unaligned() };
            base = unsafe { base.byte_add(size_of::<u64>()) };
        }
    }

    // Copy handles
    response.copy_handles = base.cast();
    base = unsafe { response.copy_handles.add(response.num_copy_handles as usize) }.cast();

    // Move handles
    response.move_handles = base.cast();
    base = unsafe { response.move_handles.add(response.num_move_handles as usize) }.cast();

    // Send statics
    response.statics = base.cast();
    base = unsafe { response.statics.add(response.num_statics as usize) }.cast();

    // Data words
    response.data_words = base.cast();

    response
}
