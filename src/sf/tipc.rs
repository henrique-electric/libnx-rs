//! Tiny IPC protocol. (`switch/sf/tipc.h`)

use core::ffi::c_void;
use core::ptr;

use crate::arm::tls::armGetTls;
use crate::kernel::svc::{svcCloseHandle, svcSendSyncRequest};
use crate::result::*;
use crate::sf::hipc::*;
use crate::sf::service::*;
use crate::types::*;

pub type TipcCommandType = u32;
pub const TipcCommandType_Close: TipcCommandType = 15;

/// tipc Service object structure
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct TipcService {
    pub session: Handle,
}

/// Dispatch parameters; build with `TipcDispatchParams { .., ..Default::default() }`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TipcDispatchParams {
    pub buffer_attrs: SfBufferAttrs,
    pub buffers: [SfBuffer; 8],

    pub in_send_pid: bool,

    pub in_num_handles: u32,
    pub in_handles: [Handle; 8],

    pub out_num_objects: u32,
    pub out_objects: *mut TipcService,

    pub out_handle_attrs: SfOutHandleAttrs,
    pub out_handles: *mut Handle,
}

impl_zeroed_default!(TipcDispatchParams);

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct TipcRequestFormat {
    pub request_id: u32,
    pub data_size: u32,
    pub num_in_buffers: u32,
    pub num_out_buffers: u32,
    pub num_inout_buffers: u32,
    pub num_handles: u32,
    pub send_pid: u32,
}

/// Creates a tipc service object from an IPC session handle.
#[inline(always)]
pub const fn tipcCreate(s: &mut TipcService, h: Handle) {
    s.session = h;
}

/// Closes a tipc service.
///
/// # Safety
/// `s` must be a valid tipc service. Uses the calling thread's TLS message buffer.
#[inline(always)]
pub unsafe fn tipcClose(s: &mut TipcService) {
    unsafe {
        hipcMakeRequest(armGetTls(), HipcMetadata { type_: TipcCommandType_Close, ..Default::default() });
        svcSendSyncRequest(s.session);
        svcCloseHandle(s.session);
    }
    *s = TipcService::default();
}

/// # Safety
/// `req` must have a free `send_buffers` slot.
#[inline(always)]
pub unsafe fn tipcRequestInBuffer(req: &mut HipcRequest, buffer: *const c_void, size: usize, mode: HipcBufferMode) {
    unsafe {
        req.send_buffers.write(hipcMakeBuffer(buffer, size, mode));
        req.send_buffers = req.send_buffers.add(1);
    }
}

/// # Safety
/// `req` must have a free `recv_buffers` slot.
#[inline(always)]
pub unsafe fn tipcRequestOutBuffer(req: &mut HipcRequest, buffer: *mut c_void, size: usize, mode: HipcBufferMode) {
    unsafe {
        req.recv_buffers.write(hipcMakeBuffer(buffer, size, mode));
        req.recv_buffers = req.recv_buffers.add(1);
    }
}

/// # Safety
/// `req` must have a free `exch_buffers` slot.
#[inline(always)]
pub unsafe fn tipcRequestInOutBuffer(req: &mut HipcRequest, buffer: *mut c_void, size: usize, mode: HipcBufferMode) {
    unsafe {
        req.exch_buffers.write(hipcMakeBuffer(buffer, size, mode));
        req.exch_buffers = req.exch_buffers.add(1);
    }
}

/// # Safety
/// `req` must have a free `copy_handles` slot.
#[inline(always)]
pub unsafe fn tipcRequestHandle(req: &mut HipcRequest, handle: Handle) {
    unsafe {
        req.copy_handles.write(handle);
        req.copy_handles = req.copy_handles.add(1);
    }
}

#[inline(always)]
pub const fn _tipcRequestFormatProcessBuffer(fmt: &mut TipcRequestFormat, attr: u32) {
    if attr == 0 {
        return;
    }
    let is_in = (attr & SfBufferAttr_In) != 0;
    let is_out = (attr & SfBufferAttr_Out) != 0;

    if attr & SfBufferAttr_HipcMapAlias != 0 {
        if is_in && is_out {
            fmt.num_inout_buffers += 1;
        } else if is_in {
            fmt.num_in_buffers += 1;
        } else if is_out {
            fmt.num_out_buffers += 1;
        }
    }
}

/// # Safety
/// `req` must have been built from a format counted with [`_tipcRequestFormatProcessBuffer`] for `attr`.
#[inline(always)]
pub unsafe fn _tipcRequestProcessBuffer(req: &mut HipcRequest, buf: &SfBuffer, attr: u32) {
    if attr == 0 {
        return;
    }
    let is_in = (attr & SfBufferAttr_In) != 0;
    let is_out = (attr & SfBufferAttr_Out) != 0;

    if attr & SfBufferAttr_HipcMapAlias != 0 {
        let mode = sf_buffer_mode(attr);
        unsafe {
            if is_in && is_out {
                tipcRequestInOutBuffer(req, buf.ptr.cast_mut(), buf.size, mode);
            } else if is_in {
                tipcRequestInBuffer(req, buf.ptr, buf.size, mode);
            } else if is_out {
                tipcRequestOutBuffer(req, buf.ptr.cast_mut(), buf.size, mode);
            }
        }
    }
}

/// Builds a tipc request in TLS and returns a pointer to its payload.
///
/// # Safety
/// `buffers` must have 8 entries and `handles` `num_handles` entries. Uses the calling thread's TLS message buffer.
#[inline(always)]
pub unsafe fn tipcMakeRequest(
    request_id: u32, data_size: u32, send_pid: bool,
    buffer_attrs: SfBufferAttrs, buffers: *const SfBuffer,
    num_handles: u32, handles: *const Handle,
) -> *mut c_void {
    let mut fmt = TipcRequestFormat {
        request_id: request_id + 16,
        data_size,
        num_handles,
        send_pid: send_pid as u32,
        ..Default::default()
    };

    let attrs = buffer_attrs.to_array();
    for &attr in &attrs {
        _tipcRequestFormatProcessBuffer(&mut fmt, attr);
    }

    unsafe {
        let mut req = hipcMakeRequest(armGetTls(), HipcMetadata {
            type_: fmt.request_id,
            num_send_statics: 0,
            num_send_buffers: fmt.num_in_buffers,
            num_recv_buffers: fmt.num_out_buffers,
            num_exch_buffers: fmt.num_inout_buffers,
            num_data_words: data_size.div_ceil(4),
            num_recv_statics: 0,
            send_pid: fmt.send_pid,
            num_copy_handles: fmt.num_handles,
            num_move_handles: 0,
        });

        for i in 0..num_handles as usize {
            tipcRequestHandle(&mut req, *handles.add(i));
        }

        for (i, &attr) in attrs.iter().enumerate() {
            _tipcRequestProcessBuffer(&mut req, &*buffers.add(i), attr);
        }

        req.data_words.cast()
    }
}

/// # Safety
/// `res` must have an unread copy handle.
#[inline(always)]
pub unsafe fn tipcResponseGetCopyHandle(res: &mut HipcResponse) -> Handle {
    unsafe {
        let v = res.copy_handles.read();
        res.copy_handles = res.copy_handles.add(1);
        v
    }
}

/// # Safety
/// `res` must have an unread move handle.
#[inline(always)]
pub unsafe fn tipcResponseGetMoveHandle(res: &mut HipcResponse) -> Handle {
    unsafe {
        let v = res.move_handles.read();
        res.move_handles = res.move_handles.add(1);
        v
    }
}

/// # Safety
/// When `type_` is not [`SfOutHandleAttr_None`], `out` must be writable and `res` must have a matching unread handle.
#[inline(always)]
pub unsafe fn _tipcResponseGetHandle(res: &mut HipcResponse, type_: SfOutHandleAttr, out: *mut Handle) {
    unsafe {
        match type_ {
            SfOutHandleAttr_HipcCopy => *out = tipcResponseGetCopyHandle(res),
            SfOutHandleAttr_HipcMove => *out = tipcResponseGetMoveHandle(res),
            _ => {}
        }
    }
}

/// Parses the tipc response in TLS.
///
/// # Safety
/// `out_objects` must hold `num_out_objects` entries; `out_handles` must be writable at each index whose
/// attribute is not [`SfOutHandleAttr_None`]. Uses the calling thread's TLS message buffer.
#[inline(always)]
pub unsafe fn tipcParseResponse(
    out_size: u32, out_data: *mut *mut c_void,
    num_out_objects: u32, out_objects: *mut TipcService,
    out_handle_attrs: SfOutHandleAttrs, out_handles: *mut Handle,
) -> Result {
    unsafe {
        let mut res = hipcParseResponse(armGetTls());

        let rc: Result = res.data_words.read();
        res.data_words = res.data_words.add(1);
        if R_FAILED(rc) {
            return rc;
        }

        if out_size != 0 {
            *out_data = res.data_words.cast();
        }

        for i in 0..num_out_objects as usize {
            tipcCreate(&mut *out_objects.add(i), tipcResponseGetMoveHandle(&mut res));
        }

        for (i, attr) in out_handle_attrs.to_array().into_iter().enumerate() {
            _tipcResponseGetHandle(&mut res, attr, out_handles.wrapping_add(i));
        }
    }

    0
}

/// Performs a full request/response round trip on a tipc service.
///
/// # Safety
/// `in_data`/`out_data` must be valid for `in_data_size`/`out_data_size` bytes, and `disp` must
/// satisfy the requirements of [`tipcMakeRequest`] and [`tipcParseResponse`].
#[inline(always)]
pub unsafe fn tipcDispatchImpl(
    s: &TipcService, request_id: u32,
    in_data: *const c_void, in_data_size: u32,
    out_data: *mut c_void, out_data_size: u32,
    disp: TipcDispatchParams,
) -> Result {
    unsafe {
        let in_ = tipcMakeRequest(request_id,
            in_data_size, disp.in_send_pid,
            disp.buffer_attrs, disp.buffers.as_ptr(),
            disp.in_num_handles, disp.in_handles.as_ptr());

        if in_data_size != 0 {
            ptr::copy_nonoverlapping(in_data.cast::<u8>(), in_.cast::<u8>(), in_data_size as usize);
        }

        let mut rc = svcSendSyncRequest(s.session);
        if R_SUCCEEDED(rc) {
            let mut out: *mut c_void = ptr::null_mut();
            rc = tipcParseResponse(out_data_size, &mut out,
                disp.out_num_objects, disp.out_objects,
                disp.out_handle_attrs, disp.out_handles);

            if R_SUCCEEDED(rc) && !out_data.is_null() && out_data_size != 0 {
                ptr::copy_nonoverlapping(out.cast::<u8>(), out_data.cast::<u8>(), out_data_size as usize);
            }
        }

        rc
    }
}

// The C `tipcDispatch*` macros.

/// # Safety
/// See [`tipcDispatchImpl`].
#[inline(always)]
pub unsafe fn tipcDispatch(s: &TipcService, rid: u32, disp: TipcDispatchParams) -> Result {
    unsafe { tipcDispatchImpl(s, rid, ptr::null(), 0, ptr::null_mut(), 0, disp) }
}

/// # Safety
/// See [`tipcDispatchImpl`]. `I` must be plain old data.
#[inline(always)]
pub unsafe fn tipcDispatchIn<I: Copy>(s: &TipcService, rid: u32, in_: &I, disp: TipcDispatchParams) -> Result {
    unsafe { tipcDispatchImpl(s, rid, ptr::from_ref(in_).cast(), size_of::<I>() as u32, ptr::null_mut(), 0, disp) }
}

/// # Safety
/// See [`tipcDispatchImpl`]. `O` must be valid for any bit pattern.
#[inline(always)]
pub unsafe fn tipcDispatchOut<O: Copy>(s: &TipcService, rid: u32, out: &mut O, disp: TipcDispatchParams) -> Result {
    unsafe { tipcDispatchImpl(s, rid, ptr::null(), 0, ptr::from_mut(out).cast(), size_of::<O>() as u32, disp) }
}

/// # Safety
/// See [`tipcDispatchImpl`]. `I` must be plain old data; `O` must be valid for any bit pattern.
#[inline(always)]
pub unsafe fn tipcDispatchInOut<I: Copy, O: Copy>(s: &TipcService, rid: u32, in_: &I, out: &mut O, disp: TipcDispatchParams) -> Result {
    unsafe {
        tipcDispatchImpl(s, rid, ptr::from_ref(in_).cast(), size_of::<I>() as u32,
            ptr::from_mut(out).cast(), size_of::<O>() as u32, disp)
    }
}
