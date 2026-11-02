//! Common Message Interface Framework protocol. (`switch/sf/cmif.h`)

use core::ffi::c_void;
use core::ptr;

use crate::arm::tls::armGetTls;
use crate::kernel::svc::svcSendSyncRequest;
use crate::result::*;
use crate::sf::hipc::*;
use crate::types::*;

/// "SFCI"
pub const CMIF_IN_HEADER_MAGIC: u32 = 0x49434653;
/// "SFCO"
pub const CMIF_OUT_HEADER_MAGIC: u32 = 0x4F434653;

pub type CmifCommandType = u32;
pub const CmifCommandType_Invalid: CmifCommandType = 0;
pub const CmifCommandType_LegacyRequest: CmifCommandType = 1;
pub const CmifCommandType_Close: CmifCommandType = 2;
pub const CmifCommandType_LegacyControl: CmifCommandType = 3;
pub const CmifCommandType_Request: CmifCommandType = 4;
pub const CmifCommandType_Control: CmifCommandType = 5;
pub const CmifCommandType_RequestWithContext: CmifCommandType = 6;
pub const CmifCommandType_ControlWithContext: CmifCommandType = 7;

pub type CmifDomainRequestType = u32;
pub const CmifDomainRequestType_Invalid: CmifDomainRequestType = 0;
pub const CmifDomainRequestType_SendMessage: CmifDomainRequestType = 1;
pub const CmifDomainRequestType_Close: CmifDomainRequestType = 2;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CmifInHeader {
    pub magic: u32,
    pub version: u32,
    pub command_id: u32,
    pub token: u32,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CmifOutHeader {
    pub magic: u32,
    pub version: u32,
    pub result: Result,
    pub token: u32,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CmifDomainInHeader {
    pub type_: u8,
    pub num_in_objects: u8,
    pub data_size: u16,
    pub object_id: u32,
    pub padding: u32,
    pub token: u32,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CmifDomainOutHeader {
    pub num_out_objects: u32,
    pub padding: [u32; 3],
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CmifRequestFormat {
    pub object_id: u32,
    pub request_id: u32,
    pub context: u32,
    pub data_size: u32,
    pub server_pointer_size: u32,
    pub num_in_auto_buffers: u32,
    pub num_out_auto_buffers: u32,
    pub num_in_buffers: u32,
    pub num_out_buffers: u32,
    pub num_inout_buffers: u32,
    pub num_in_pointers: u32,
    pub num_out_pointers: u32,
    pub num_out_fixed_pointers: u32,
    pub num_objects: u32,
    pub num_handles: u32,
    pub send_pid: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CmifRequest {
    pub hipc: HipcRequest,
    pub data: *mut c_void,
    pub out_pointer_sizes: *mut u16,
    pub objects: *mut u32,
    pub server_pointer_size: u32,
    pub cur_in_ptr_id: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CmifResponse {
    pub data: *mut c_void,
    pub objects: *mut u32,
    pub copy_handles: *mut Handle,
    pub move_handles: *mut Handle,
}

impl_zeroed_default!(CmifRequest, CmifResponse);

/// Returns the first 16-byte-aligned address (relative to `base`) at or after `data_words`.
///
/// # Safety
/// `data_words` and `base` must point into the same message buffer.
#[inline(always)]
pub unsafe fn cmifGetAlignedDataStart(data_words: *mut u32, base: *mut c_void) -> *mut c_void {
    let data_start = ((data_words as isize).wrapping_sub(base as isize) + 15) & !15;
    unsafe { base.byte_offset(data_start) }
}

/// Builds a CMIF request in the message buffer at `base`.
///
/// # Safety
/// `base` must point to a 16-byte-aligned message buffer (normally TLS) large enough for the request.
#[inline(always)]
pub unsafe fn cmifMakeRequest(base: *mut c_void, fmt: CmifRequestFormat) -> CmifRequest {
    // First of all, we need to figure out what size we need.
    let mut actual_size: u32 = 16;
    if fmt.object_id != 0 {
        actual_size += size_of::<CmifDomainInHeader>() as u32 + fmt.num_objects * size_of::<u32>() as u32;
    }
    actual_size += size_of::<CmifInHeader>() as u32 + fmt.data_size;
    actual_size = (actual_size + 1) & !1; // hword-align
    let out_pointer_size_table_offset = actual_size;
    let out_pointer_size_table_size = fmt.num_out_auto_buffers + fmt.num_out_pointers;
    actual_size += size_of::<u16>() as u32 * out_pointer_size_table_size;
    let num_data_words = actual_size.div_ceil(4);

    let hipc = unsafe {
        hipcMakeRequest(base, HipcMetadata {
            type_: if fmt.context != 0 { CmifCommandType_RequestWithContext } else { CmifCommandType_Request },
            num_send_statics: fmt.num_in_auto_buffers + fmt.num_in_pointers,
            num_send_buffers: fmt.num_in_auto_buffers + fmt.num_in_buffers,
            num_recv_buffers: fmt.num_out_auto_buffers + fmt.num_out_buffers,
            num_exch_buffers: fmt.num_inout_buffers,
            num_data_words,
            num_recv_statics: out_pointer_size_table_size + fmt.num_out_fixed_pointers,
            send_pid: fmt.send_pid,
            num_copy_handles: fmt.num_handles,
            num_move_handles: 0,
        })
    };
    let mut req = CmifRequest { hipc, ..Default::default() };

    let hdr: *mut CmifInHeader;
    let start = unsafe { cmifGetAlignedDataStart(req.hipc.data_words, base) };
    if fmt.object_id != 0 {
        let domain_hdr = start.cast::<CmifDomainInHeader>();
        let payload_size = size_of::<CmifInHeader>() as u32 + fmt.data_size;
        unsafe {
            domain_hdr.write(CmifDomainInHeader {
                type_: CmifDomainRequestType_SendMessage as u8,
                num_in_objects: fmt.num_objects as u8,
                data_size: payload_size as u16,
                object_id: fmt.object_id,
                padding: 0,
                token: fmt.context,
            });
            hdr = domain_hdr.add(1).cast();
            req.objects = hdr.byte_add(payload_size as usize).cast();
        }
    } else {
        hdr = start.cast();
    }

    unsafe {
        hdr.write(CmifInHeader {
            magic: CMIF_IN_HEADER_MAGIC,
            version: if fmt.context != 0 { 1 } else { 0 },
            command_id: fmt.request_id,
            token: if fmt.object_id != 0 { 0 } else { fmt.context },
        });

        req.data = hdr.add(1).cast();
        req.out_pointer_sizes = req.hipc.data_words.byte_add(out_pointer_size_table_offset as usize).cast();
    }
    req.server_pointer_size = fmt.server_pointer_size;

    req
}

/// Builds a CMIF control request and returns a pointer to its `size`-byte payload.
///
/// # Safety
/// `base` must point to a 16-byte-aligned message buffer (normally TLS).
#[inline(always)]
pub unsafe fn cmifMakeControlRequest(base: *mut c_void, request_id: u32, size: u32) -> *mut c_void {
    let actual_size = 16 + size_of::<CmifInHeader>() as u32 + size;
    let hipc = unsafe {
        hipcMakeRequest(base, HipcMetadata {
            type_: CmifCommandType_Control,
            num_data_words: actual_size.div_ceil(4),
            ..Default::default()
        })
    };
    unsafe {
        let hdr = cmifGetAlignedDataStart(hipc.data_words, base).cast::<CmifInHeader>();
        hdr.write(CmifInHeader { magic: CMIF_IN_HEADER_MAGIC, version: 0, command_id: request_id, token: 0 });
        hdr.add(1).cast()
    }
}

/// Builds a CMIF close request (a domain close when `object_id` is non-zero).
///
/// # Safety
/// `base` must point to a 16-byte-aligned message buffer (normally TLS).
#[inline(always)]
pub unsafe fn cmifMakeCloseRequest(base: *mut c_void, object_id: u32) {
    if object_id != 0 {
        unsafe {
            let hipc = hipcMakeRequest(base, HipcMetadata {
                type_: CmifCommandType_Request,
                num_data_words: (16 + size_of::<CmifDomainInHeader>() as u32) / 4,
                ..Default::default()
            });
            let domain_hdr = cmifGetAlignedDataStart(hipc.data_words, base).cast::<CmifDomainInHeader>();
            domain_hdr.write(CmifDomainInHeader {
                type_: CmifDomainRequestType_Close as u8,
                object_id,
                ..Default::default()
            });
        }
    } else {
        unsafe { hipcMakeRequest(base, HipcMetadata { type_: CmifCommandType_Close, ..Default::default() }) };
    }
}

/// # Safety
/// `req` must come from [`cmifMakeRequest`] with a free `send_buffers` slot.
#[inline(always)]
pub unsafe fn cmifRequestInBuffer(req: &mut CmifRequest, buffer: *const c_void, size: usize, mode: HipcBufferMode) {
    unsafe {
        req.hipc.send_buffers.write(hipcMakeBuffer(buffer, size, mode));
        req.hipc.send_buffers = req.hipc.send_buffers.add(1);
    }
}

/// # Safety
/// `req` must come from [`cmifMakeRequest`] with a free `recv_buffers` slot.
#[inline(always)]
pub unsafe fn cmifRequestOutBuffer(req: &mut CmifRequest, buffer: *mut c_void, size: usize, mode: HipcBufferMode) {
    unsafe {
        req.hipc.recv_buffers.write(hipcMakeBuffer(buffer, size, mode));
        req.hipc.recv_buffers = req.hipc.recv_buffers.add(1);
    }
}

/// # Safety
/// `req` must come from [`cmifMakeRequest`] with a free `exch_buffers` slot.
#[inline(always)]
pub unsafe fn cmifRequestInOutBuffer(req: &mut CmifRequest, buffer: *mut c_void, size: usize, mode: HipcBufferMode) {
    unsafe {
        req.hipc.exch_buffers.write(hipcMakeBuffer(buffer, size, mode));
        req.hipc.exch_buffers = req.hipc.exch_buffers.add(1);
    }
}

/// # Safety
/// `req` must come from [`cmifMakeRequest`] with a free `send_statics` slot.
#[inline(always)]
pub unsafe fn cmifRequestInPointer(req: &mut CmifRequest, buffer: *const c_void, size: usize) {
    unsafe {
        req.hipc.send_statics.write(hipcMakeSendStatic(buffer, size, req.cur_in_ptr_id as u8));
        req.hipc.send_statics = req.hipc.send_statics.add(1);
    }
    req.cur_in_ptr_id = req.cur_in_ptr_id.wrapping_add(1);
    req.server_pointer_size = req.server_pointer_size.wrapping_sub(size as u32);
}

/// # Safety
/// `req` must come from [`cmifMakeRequest`] with a free `recv_list` slot.
#[inline(always)]
pub unsafe fn cmifRequestOutFixedPointer(req: &mut CmifRequest, buffer: *mut c_void, size: usize) {
    unsafe {
        req.hipc.recv_list.write(hipcMakeRecvStatic(buffer, size));
        req.hipc.recv_list = req.hipc.recv_list.add(1);
    }
    req.server_pointer_size = req.server_pointer_size.wrapping_sub(size as u32);
}

/// # Safety
/// `req` must come from [`cmifMakeRequest`] with free `recv_list` and `out_pointer_sizes` slots.
#[inline(always)]
pub unsafe fn cmifRequestOutPointer(req: &mut CmifRequest, buffer: *mut c_void, size: usize) {
    unsafe {
        cmifRequestOutFixedPointer(req, buffer, size);
        req.out_pointer_sizes.write(size as u16);
        req.out_pointer_sizes = req.out_pointer_sizes.add(1);
    }
}

/// # Safety
/// `req` must come from [`cmifMakeRequest`] with free `send_statics` and `send_buffers` slots.
#[inline(always)]
pub unsafe fn cmifRequestInAutoBuffer(req: &mut CmifRequest, buffer: *const c_void, size: usize, mode: HipcBufferMode) {
    unsafe {
        if req.server_pointer_size != 0 && size <= req.server_pointer_size as usize {
            cmifRequestInPointer(req, buffer, size);
            cmifRequestInBuffer(req, ptr::null(), 0, mode);
        } else {
            cmifRequestInPointer(req, ptr::null(), 0);
            cmifRequestInBuffer(req, buffer, size, mode);
        }
    }
}

/// # Safety
/// `req` must come from [`cmifMakeRequest`] with free `recv_list`, `out_pointer_sizes` and `recv_buffers` slots.
#[inline(always)]
pub unsafe fn cmifRequestOutAutoBuffer(req: &mut CmifRequest, buffer: *mut c_void, size: usize, mode: HipcBufferMode) {
    unsafe {
        if req.server_pointer_size != 0 && size <= req.server_pointer_size as usize {
            cmifRequestOutPointer(req, buffer, size);
            cmifRequestOutBuffer(req, ptr::null_mut(), 0, mode);
        } else {
            cmifRequestOutPointer(req, ptr::null_mut(), 0);
            cmifRequestOutBuffer(req, buffer, size, mode);
        }
    }
}

/// # Safety
/// `req` must come from a domain [`cmifMakeRequest`] with a free `objects` slot.
#[inline(always)]
pub unsafe fn cmifRequestObject(req: &mut CmifRequest, object_id: u32) {
    unsafe {
        req.objects.write(object_id);
        req.objects = req.objects.add(1);
    }
}

/// # Safety
/// `req` must come from [`cmifMakeRequest`] with a free `copy_handles` slot.
#[inline(always)]
pub unsafe fn cmifRequestHandle(req: &mut CmifRequest, handle: Handle) {
    unsafe {
        req.hipc.copy_handles.write(handle);
        req.hipc.copy_handles = req.hipc.copy_handles.add(1);
    }
}

/// Parses a CMIF response at `base`, filling `res` on success.
///
/// # Safety
/// `base` must point to a 16-byte-aligned buffer holding a response to a request of the given shape.
#[inline(always)]
pub unsafe fn cmifParseResponse(res: &mut CmifResponse, base: *mut c_void, is_domain: bool, size: u32) -> Result {
    let hipc = unsafe { hipcParseResponse(base) };
    let start = unsafe { cmifGetAlignedDataStart(hipc.data_words, base) };

    let hdr: *mut CmifOutHeader;
    let mut objects: *mut u32 = ptr::null_mut();
    if is_domain {
        let domain_hdr = start.cast::<CmifDomainOutHeader>();
        unsafe {
            hdr = domain_hdr.add(1).cast();
            objects = hdr.byte_add(size_of::<CmifOutHeader>() + size as usize).cast();
        }
    } else {
        hdr = start.cast();
    }

    let out = unsafe { hdr.read() };
    if out.magic != CMIF_OUT_HEADER_MAGIC {
        return MAKERESULT(Module_Libnx, LibnxError_InvalidCmifOutHeader);
    }
    if R_FAILED(out.result) {
        return out.result;
    }

    *res = CmifResponse {
        data: unsafe { hdr.add(1) }.cast(),
        objects,
        copy_handles: hipc.copy_handles,
        move_handles: hipc.move_handles,
    };

    0
}

/// # Safety
/// `res` must have an unread output object.
#[inline(always)]
pub unsafe fn cmifResponseGetObject(res: &mut CmifResponse) -> u32 {
    unsafe {
        let v = res.objects.read();
        res.objects = res.objects.add(1);
        v
    }
}

/// # Safety
/// `res` must have an unread copy handle.
#[inline(always)]
pub unsafe fn cmifResponseGetCopyHandle(res: &mut CmifResponse) -> Handle {
    unsafe {
        let v = res.copy_handles.read();
        res.copy_handles = res.copy_handles.add(1);
        v
    }
}

/// # Safety
/// `res` must have an unread move handle.
#[inline(always)]
pub unsafe fn cmifResponseGetMoveHandle(res: &mut CmifResponse) -> Handle {
    unsafe {
        let v = res.move_handles.read();
        res.move_handles = res.move_handles.add(1);
        v
    }
}

/// # Safety
/// `h` must be a CMIF session handle. Uses the calling thread's TLS message buffer.
#[inline(always)]
pub unsafe fn cmifConvertCurrentObjectToDomain(h: Handle, out_object_id: Option<&mut u32>) -> Result {
    unsafe {
        cmifMakeControlRequest(armGetTls(), 0, 0);
        let mut rc = svcSendSyncRequest(h);
        if R_SUCCEEDED(rc) {
            let mut resp = CmifResponse::default();
            rc = cmifParseResponse(&mut resp, armGetTls(), false, size_of::<u32>() as u32);
            if R_SUCCEEDED(rc)
                && let Some(out) = out_object_id
            {
                *out = resp.data.cast::<u32>().read();
            }
        }
        rc
    }
}

/// # Safety
/// `h` must be a CMIF domain session handle. Uses the calling thread's TLS message buffer.
#[inline(always)]
pub unsafe fn cmifCopyFromCurrentDomain(h: Handle, object_id: u32, out_h: Option<&mut Handle>) -> Result {
    unsafe {
        let raw = cmifMakeControlRequest(armGetTls(), 1, size_of::<u32>() as u32);
        raw.cast::<u32>().write(object_id);
        let mut rc = svcSendSyncRequest(h);
        if R_SUCCEEDED(rc) {
            let mut resp = CmifResponse::default();
            rc = cmifParseResponse(&mut resp, armGetTls(), false, 0);
            if R_SUCCEEDED(rc)
                && let Some(out) = out_h
            {
                *out = resp.move_handles.read();
            }
        }
        rc
    }
}

/// # Safety
/// `h` must be a CMIF session handle. Uses the calling thread's TLS message buffer.
#[inline(always)]
pub unsafe fn cmifCloneCurrentObject(h: Handle, out_h: Option<&mut Handle>) -> Result {
    unsafe {
        cmifMakeControlRequest(armGetTls(), 2, 0);
        let mut rc = svcSendSyncRequest(h);
        if R_SUCCEEDED(rc) {
            let mut resp = CmifResponse::default();
            rc = cmifParseResponse(&mut resp, armGetTls(), false, 0);
            if R_SUCCEEDED(rc)
                && let Some(out) = out_h
            {
                *out = resp.move_handles.read();
            }
        }
        rc
    }
}

/// # Safety
/// `h` must be a CMIF session handle. Uses the calling thread's TLS message buffer.
#[inline(always)]
pub unsafe fn cmifQueryPointerBufferSize(h: Handle, out_size: Option<&mut u16>) -> Result {
    unsafe {
        cmifMakeControlRequest(armGetTls(), 3, 0);
        let mut rc = svcSendSyncRequest(h);
        if R_SUCCEEDED(rc) {
            let mut resp = CmifResponse::default();
            rc = cmifParseResponse(&mut resp, armGetTls(), false, size_of::<u16>() as u32);
            if R_SUCCEEDED(rc)
                && let Some(out) = out_size
            {
                *out = resp.data.cast::<u16>().read();
            }
        }
        rc
    }
}

/// # Safety
/// `h` must be a CMIF session handle. Uses the calling thread's TLS message buffer.
#[inline(always)]
pub unsafe fn cmifCloneCurrentObjectEx(h: Handle, tag: u32, out_h: Option<&mut Handle>) -> Result {
    unsafe {
        let raw = cmifMakeControlRequest(armGetTls(), 4, size_of::<u32>() as u32);
        raw.cast::<u32>().write(tag);
        let mut rc = svcSendSyncRequest(h);
        if R_SUCCEEDED(rc) {
            let mut resp = CmifResponse::default();
            rc = cmifParseResponse(&mut resp, armGetTls(), false, 0);
            if R_SUCCEEDED(rc)
                && let Some(out) = out_h
            {
                *out = resp.move_handles.read();
            }
        }
        rc
    }
}
