//! Service wrapper object. (`switch/sf/service.h`)

use core::ffi::c_void;
use core::ptr;

use crate::arm::tls::armGetTls;
use crate::kernel::svc::{svcCloseHandle, svcSendSyncRequest};
use crate::result::*;
use crate::sf::cmif::*;
use crate::sf::hipc::*;
use crate::types::*;

/// Service object structure
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Service {
    pub session: Handle,
    pub own_handle: u32,
    pub object_id: u32,
    pub pointer_buffer_size: u16,
}

pub const SfBufferAttr_In: u32 = BIT(0);
pub const SfBufferAttr_Out: u32 = BIT(1);
pub const SfBufferAttr_HipcMapAlias: u32 = BIT(2);
pub const SfBufferAttr_HipcPointer: u32 = BIT(3);
pub const SfBufferAttr_FixedSize: u32 = BIT(4);
pub const SfBufferAttr_HipcAutoSelect: u32 = BIT(5);
pub const SfBufferAttr_HipcMapTransferAllowsNonSecure: u32 = BIT(6);
pub const SfBufferAttr_HipcMapTransferAllowsNonDevice: u32 = BIT(7);

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SfBufferAttrs {
    pub attr0: u32,
    pub attr1: u32,
    pub attr2: u32,
    pub attr3: u32,
    pub attr4: u32,
    pub attr5: u32,
    pub attr6: u32,
    pub attr7: u32,
}

impl SfBufferAttrs {
    #[inline(always)]
    pub(crate) const fn to_array(self) -> [u32; 8] {
        [self.attr0, self.attr1, self.attr2, self.attr3, self.attr4, self.attr5, self.attr6, self.attr7]
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SfBuffer {
    pub ptr: *const c_void,
    pub size: usize,
}

pub type SfOutHandleAttr = u32;
pub const SfOutHandleAttr_None: SfOutHandleAttr = 0;
pub const SfOutHandleAttr_HipcCopy: SfOutHandleAttr = 1;
pub const SfOutHandleAttr_HipcMove: SfOutHandleAttr = 2;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SfOutHandleAttrs {
    pub attr0: SfOutHandleAttr,
    pub attr1: SfOutHandleAttr,
    pub attr2: SfOutHandleAttr,
    pub attr3: SfOutHandleAttr,
    pub attr4: SfOutHandleAttr,
    pub attr5: SfOutHandleAttr,
    pub attr6: SfOutHandleAttr,
    pub attr7: SfOutHandleAttr,
}

impl SfOutHandleAttrs {
    #[inline(always)]
    pub(crate) const fn to_array(self) -> [SfOutHandleAttr; 8] {
        [self.attr0, self.attr1, self.attr2, self.attr3, self.attr4, self.attr5, self.attr6, self.attr7]
    }
}

/// Dispatch parameters. Build with struct-update syntax, the Rust equivalent of the C
/// designated initializers: `SfDispatchParams { in_send_pid: true, ..Default::default() }`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SfDispatchParams {
    pub target_session: Handle,
    pub context: u32,

    pub buffer_attrs: SfBufferAttrs,
    pub buffers: [SfBuffer; 8],

    pub in_send_pid: bool,

    pub in_num_objects: u32,
    pub in_objects: [*const Service; 8],

    pub in_num_handles: u32,
    pub in_handles: [Handle; 8],

    pub out_num_objects: u32,
    pub out_objects: *mut Service,

    pub out_handle_attrs: SfOutHandleAttrs,
    pub out_handles: *mut Handle,
}

impl_zeroed_default!(SfBuffer, SfDispatchParams);

/// Mirrors the `NX_SERVICE_ASSUME_NON_DOMAIN` guard at the top of several libnx inlines.
#[inline(always)]
fn assume_non_domain(_s: &Service) {
    #[cfg(feature = "service-assume-non-domain")]
    if _s.object_id != 0 {
        // SAFETY: the feature is an explicit promise that no service is a domain.
        unsafe { core::hint::unreachable_unchecked() }
    }
}

/// Returns whether a service has been initialized.
#[inline(always)]
pub const fn serviceIsActive(s: &Service) -> bool {
    s.session != INVALID_HANDLE
}

/// Returns whether a service is overriden in the homebrew environment.
#[inline(always)]
pub const fn serviceIsOverride(s: &Service) -> bool {
    serviceIsActive(s) && s.own_handle == 0 && s.object_id == 0
}

/// Returns whether a service is a domain.
#[inline(always)]
pub const fn serviceIsDomain(s: &Service) -> bool {
    serviceIsActive(s) && s.own_handle != 0 && s.object_id != 0
}

/// Returns whether a service is a domain subservice.
#[inline(always)]
pub const fn serviceIsDomainSubservice(s: &Service) -> bool {
    serviceIsActive(s) && s.own_handle == 0 && s.object_id != 0
}

/// For a domain/domain subservice, return the associated object ID.
#[inline(always)]
pub const fn serviceGetObjectId(s: &Service) -> u32 {
    s.object_id
}

/// Creates a service object from an IPC session handle.
///
/// # Safety
/// `h` must be a CMIF session handle. Uses the calling thread's TLS message buffer.
#[inline(always)]
pub unsafe fn serviceCreate(s: &mut Service, h: Handle) {
    s.session = h;
    s.own_handle = 1;
    s.object_id = 0;
    s.pointer_buffer_size = 0;
    unsafe { cmifQueryPointerBufferSize(h, Some(&mut s.pointer_buffer_size)) };
}

/// Creates a non-domain subservice object from a parent service.
#[inline(always)]
pub fn serviceCreateNonDomainSubservice(s: &mut Service, parent: &Service, h: Handle) {
    if h != INVALID_HANDLE {
        s.session = h;
        s.own_handle = 1;
        s.object_id = 0;
        s.pointer_buffer_size = parent.pointer_buffer_size;
    } else {
        *s = Service::default();
    }
}

/// Creates a domain subservice object from a parent service.
#[inline(always)]
pub fn serviceCreateDomainSubservice(s: &mut Service, parent: &Service, object_id: u32) {
    if object_id != 0 {
        s.session = parent.session;
        s.own_handle = 0;
        s.object_id = object_id;
        s.pointer_buffer_size = parent.pointer_buffer_size;
    } else {
        *s = Service::default();
    }
}

/// Hints the compiler that a service will always contain a domain object.
///
/// # Safety
/// `s.object_id` must be non-zero.
#[inline(always)]
pub unsafe fn serviceAssumeDomain(s: &Service) {
    if s.object_id == 0 {
        unsafe { core::hint::unreachable_unchecked() }
    }
}

/// Closes a service.
///
/// # Safety
/// `s` must be a valid service. Uses the calling thread's TLS message buffer.
#[inline(always)]
pub unsafe fn serviceClose(s: &mut Service) {
    assume_non_domain(s);

    if s.own_handle != 0 || s.object_id != 0 {
        unsafe {
            cmifMakeCloseRequest(armGetTls(), if s.own_handle != 0 { 0 } else { s.object_id });
            svcSendSyncRequest(s.session);
            if s.own_handle != 0 {
                svcCloseHandle(s.session);
            }
        }
    }
    *s = Service::default();
}

/// Clones a service.
///
/// # Safety
/// `s` must be a valid service. Uses the calling thread's TLS message buffer.
#[inline(always)]
pub unsafe fn serviceClone(s: &Service, out_s: &mut Service) -> Result {
    assume_non_domain(s);

    out_s.session = 0;
    out_s.own_handle = 1;
    out_s.object_id = s.object_id;
    out_s.pointer_buffer_size = s.pointer_buffer_size;
    unsafe { cmifCloneCurrentObject(s.session, Some(&mut out_s.session)) }
}

/// Clones a service with a session manager tag (unused in current official server code).
///
/// # Safety
/// `s` must be a valid service. Uses the calling thread's TLS message buffer.
#[inline(always)]
pub unsafe fn serviceCloneEx(s: &Service, tag: u32, out_s: &mut Service) -> Result {
    assume_non_domain(s);

    out_s.session = 0;
    out_s.own_handle = 1;
    out_s.object_id = s.object_id;
    out_s.pointer_buffer_size = s.pointer_buffer_size;
    unsafe { cmifCloneCurrentObjectEx(s.session, tag, Some(&mut out_s.session)) }
}

/// Converts a regular service to a domain.
///
/// # Safety
/// `s` must be a valid service. Uses the calling thread's TLS message buffer.
#[inline(always)]
pub unsafe fn serviceConvertToDomain(s: &mut Service) -> Result {
    if s.own_handle == 0 {
        // For overridden services, create a clone first.
        let session = s.session;
        let rc = unsafe { cmifCloneCurrentObjectEx(session, 0, Some(&mut s.session)) };
        if R_FAILED(rc) {
            return rc;
        }
        s.own_handle = 1;
    }

    unsafe { cmifConvertCurrentObjectToDomain(s.session, Some(&mut s.object_id)) }
}

#[inline(always)]
pub const fn _serviceRequestFormatProcessBuffer(fmt: &mut CmifRequestFormat, attr: u32) {
    if attr == 0 {
        return;
    }
    let is_in = (attr & SfBufferAttr_In) != 0;
    let is_out = (attr & SfBufferAttr_Out) != 0;

    if attr & SfBufferAttr_HipcAutoSelect != 0 {
        if is_in {
            fmt.num_in_auto_buffers += 1;
        }
        if is_out {
            fmt.num_out_auto_buffers += 1;
        }
    } else if attr & SfBufferAttr_HipcPointer != 0 {
        if is_in {
            fmt.num_in_pointers += 1;
        }
        if is_out {
            if attr & SfBufferAttr_FixedSize != 0 {
                fmt.num_out_fixed_pointers += 1;
            } else {
                fmt.num_out_pointers += 1;
            }
        }
    } else if attr & SfBufferAttr_HipcMapAlias != 0 {
        if is_in && is_out {
            fmt.num_inout_buffers += 1;
        } else if is_in {
            fmt.num_in_buffers += 1;
        } else if is_out {
            fmt.num_out_buffers += 1;
        }
    }
}

/// Picks the HIPC buffer mode encoded in a buffer attribute.
#[inline(always)]
pub(crate) const fn sf_buffer_mode(attr: u32) -> HipcBufferMode {
    let mut mode = HipcBufferMode_Normal;
    if attr & SfBufferAttr_HipcMapTransferAllowsNonSecure != 0 {
        mode = HipcBufferMode_NonSecure;
    }
    if attr & SfBufferAttr_HipcMapTransferAllowsNonDevice != 0 {
        mode = HipcBufferMode_NonDevice;
    }
    mode
}

/// # Safety
/// `req` must have been built from a format counted with [`_serviceRequestFormatProcessBuffer`] for `attr`.
#[inline(always)]
pub unsafe fn _serviceRequestProcessBuffer(req: &mut CmifRequest, buf: &SfBuffer, attr: u32) {
    if attr == 0 {
        return;
    }
    let is_in = (attr & SfBufferAttr_In) != 0;
    let is_out = (attr & SfBufferAttr_Out) != 0;

    unsafe {
        if attr & SfBufferAttr_HipcAutoSelect != 0 {
            let mode = sf_buffer_mode(attr);
            if is_in {
                cmifRequestInAutoBuffer(req, buf.ptr, buf.size, mode);
            }
            if is_out {
                cmifRequestOutAutoBuffer(req, buf.ptr.cast_mut(), buf.size, mode);
            }
        } else if attr & SfBufferAttr_HipcPointer != 0 {
            if is_in {
                cmifRequestInPointer(req, buf.ptr, buf.size);
            }
            if is_out {
                if attr & SfBufferAttr_FixedSize != 0 {
                    cmifRequestOutFixedPointer(req, buf.ptr.cast_mut(), buf.size);
                } else {
                    cmifRequestOutPointer(req, buf.ptr.cast_mut(), buf.size);
                }
            }
        } else if attr & SfBufferAttr_HipcMapAlias != 0 {
            let mode = sf_buffer_mode(attr);
            if is_in && is_out {
                cmifRequestInOutBuffer(req, buf.ptr.cast_mut(), buf.size, mode);
            } else if is_in {
                cmifRequestInBuffer(req, buf.ptr, buf.size, mode);
            } else if is_out {
                cmifRequestOutBuffer(req, buf.ptr.cast_mut(), buf.size, mode);
            }
        }
    }
}

/// Builds a request in TLS and returns a pointer to its `data_size`-byte payload.
///
/// # Safety
/// `buffers` must have 8 entries; `objects`/`handles` must hold `num_objects`/`num_handles` entries.
/// Uses the calling thread's TLS message buffer.
#[allow(clippy::too_many_arguments)]
#[inline(always)]
pub unsafe fn serviceMakeRequest(
    s: &Service, request_id: u32, context: u32, data_size: u32, send_pid: bool,
    buffer_attrs: SfBufferAttrs, buffers: *const SfBuffer,
    num_objects: u32, objects: *const *const Service,
    num_handles: u32, handles: *const Handle,
) -> *mut c_void {
    assume_non_domain(s);

    let mut fmt = CmifRequestFormat {
        object_id: s.object_id,
        request_id,
        context,
        data_size,
        server_pointer_size: s.pointer_buffer_size as u32,
        num_objects,
        num_handles,
        send_pid: send_pid as u32,
        ..Default::default()
    };

    let attrs = buffer_attrs.to_array();
    for &attr in &attrs {
        _serviceRequestFormatProcessBuffer(&mut fmt, attr);
    }

    unsafe {
        let mut req = cmifMakeRequest(armGetTls(), fmt);

        if s.object_id != 0 {
            // TODO (upstream): Check behavior of input objects in non-domain sessions
            for i in 0..num_objects as usize {
                cmifRequestObject(&mut req, (**objects.add(i)).object_id);
            }
        }

        for i in 0..num_handles as usize {
            cmifRequestHandle(&mut req, *handles.add(i));
        }

        for (i, &attr) in attrs.iter().enumerate() {
            _serviceRequestProcessBuffer(&mut req, &*buffers.add(i), attr);
        }

        req.data
    }
}

/// # Safety
/// When `type_` is not [`SfOutHandleAttr_None`], `out` must be writable and `res` must have a matching unread handle.
#[inline(always)]
pub unsafe fn _serviceResponseGetHandle(res: &mut CmifResponse, type_: SfOutHandleAttr, out: *mut Handle) {
    unsafe {
        match type_ {
            SfOutHandleAttr_HipcCopy => *out = cmifResponseGetCopyHandle(res),
            SfOutHandleAttr_HipcMove => *out = cmifResponseGetMoveHandle(res),
            _ => {}
        }
    }
}

/// Parses the response in TLS.
///
/// # Safety
/// `out_objects` must hold `num_out_objects` entries; `out_handles` must be writable at each index whose
/// attribute is not [`SfOutHandleAttr_None`]. Uses the calling thread's TLS message buffer.
#[inline(always)]
pub unsafe fn serviceParseResponse(
    s: &Service, out_size: u32, out_data: *mut *mut c_void,
    num_out_objects: u32, out_objects: *mut Service,
    out_handle_attrs: SfOutHandleAttrs, out_handles: *mut Handle,
) -> Result {
    assume_non_domain(s);

    let mut res = CmifResponse::default();
    let is_domain = s.object_id != 0;
    let rc = unsafe { cmifParseResponse(&mut res, armGetTls(), is_domain, out_size) };
    if R_FAILED(rc) {
        return rc;
    }

    unsafe {
        if out_size != 0 {
            *out_data = res.data;
        }

        for i in 0..num_out_objects as usize {
            let out = &mut *out_objects.add(i);
            if is_domain {
                serviceCreateDomainSubservice(out, s, cmifResponseGetObject(&mut res));
            } else {
                // Output objects are marshalled as move handles at the beginning of the list.
                serviceCreateNonDomainSubservice(out, s, cmifResponseGetMoveHandle(&mut res));
            }
        }

        for (i, attr) in out_handle_attrs.to_array().into_iter().enumerate() {
            _serviceResponseGetHandle(&mut res, attr, out_handles.wrapping_add(i));
        }
    }

    0
}

/// Performs a full request/response round trip on a service.
///
/// # Safety
/// `in_data`/`out_data` must be valid for `in_data_size`/`out_data_size` bytes, and `disp` must
/// satisfy the requirements of [`serviceMakeRequest`] and [`serviceParseResponse`].
#[inline(always)]
pub unsafe fn serviceDispatchImpl(
    s: &Service, request_id: u32,
    in_data: *const c_void, in_data_size: u32,
    out_data: *mut c_void, out_data_size: u32,
    disp: SfDispatchParams,
) -> Result {
    // Make a copy of the service struct, so that the compiler can assume that it won't be modified by function calls.
    let srv = *s;

    unsafe {
        let in_ = serviceMakeRequest(&srv, request_id, disp.context,
            in_data_size, disp.in_send_pid,
            disp.buffer_attrs, disp.buffers.as_ptr(),
            disp.in_num_objects, disp.in_objects.as_ptr(),
            disp.in_num_handles, disp.in_handles.as_ptr());

        if in_data_size != 0 {
            ptr::copy_nonoverlapping(in_data.cast::<u8>(), in_.cast::<u8>(), in_data_size as usize);
        }

        let mut rc = svcSendSyncRequest(if disp.target_session == INVALID_HANDLE { s.session } else { disp.target_session });
        if R_SUCCEEDED(rc) {
            let mut out: *mut c_void = ptr::null_mut();
            rc = serviceParseResponse(&srv,
                out_data_size, &mut out,
                disp.out_num_objects, disp.out_objects,
                disp.out_handle_attrs, disp.out_handles);

            if R_SUCCEEDED(rc) && !out_data.is_null() && out_data_size != 0 {
                ptr::copy_nonoverlapping(out.cast::<u8>(), out_data.cast::<u8>(), out_data_size as usize);
            }
        }

        rc
    }
}

// The C `serviceDispatch*` macros. Generic references replace the C `static_assert(!is_pointer)` checks.

/// # Safety
/// See [`serviceDispatchImpl`].
#[inline(always)]
pub unsafe fn serviceDispatch(s: &Service, rid: u32, disp: SfDispatchParams) -> Result {
    unsafe { serviceDispatchImpl(s, rid, ptr::null(), 0, ptr::null_mut(), 0, disp) }
}

/// # Safety
/// See [`serviceDispatchImpl`]. `I` must be plain old data.
#[inline(always)]
pub unsafe fn serviceDispatchIn<I: Copy>(s: &Service, rid: u32, in_: &I, disp: SfDispatchParams) -> Result {
    unsafe { serviceDispatchImpl(s, rid, ptr::from_ref(in_).cast(), size_of::<I>() as u32, ptr::null_mut(), 0, disp) }
}

/// # Safety
/// See [`serviceDispatchImpl`]. `O` must be valid for any bit pattern.
#[inline(always)]
pub unsafe fn serviceDispatchOut<O: Copy>(s: &Service, rid: u32, out: &mut O, disp: SfDispatchParams) -> Result {
    unsafe { serviceDispatchImpl(s, rid, ptr::null(), 0, ptr::from_mut(out).cast(), size_of::<O>() as u32, disp) }
}

/// # Safety
/// See [`serviceDispatchImpl`]. `I` must be plain old data; `O` must be valid for any bit pattern.
#[inline(always)]
pub unsafe fn serviceDispatchInOut<I: Copy, O: Copy>(s: &Service, rid: u32, in_: &I, out: &mut O, disp: SfDispatchParams) -> Result {
    unsafe {
        serviceDispatchImpl(s, rid, ptr::from_ref(in_).cast(), size_of::<I>() as u32,
            ptr::from_mut(out).cast(), size_of::<O>() as u32, disp)
    }
}
