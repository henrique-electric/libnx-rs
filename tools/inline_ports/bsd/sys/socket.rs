// inline: _ALIGN
#[inline(always)]
pub const fn _ALIGN(p: usize) -> usize {
    (p + _ALIGNBYTES as usize) & !(_ALIGNBYTES as usize)
}
// inline: CMSG_DATA
/// Given a pointer to a `cmsghdr`, returns a pointer to its data.
#[inline(always)]
pub fn CMSG_DATA(cmsg: *mut cmsghdr) -> *mut c_uchar {
    cmsg.cast::<c_uchar>().wrapping_add(_ALIGN(size_of::<cmsghdr>()))
}
// inline: CMSG_NXTHDR
/// Given a pointer to a `cmsghdr`, returns a pointer to the next one (or null).
///
/// # Safety
/// `mhdr` and `cmsg` must describe a valid control-message buffer.
#[inline(always)]
pub unsafe fn CMSG_NXTHDR(mhdr: *const msghdr, cmsg: *const cmsghdr) -> *mut cmsghdr {
    unsafe {
        let next = cmsg as usize + _ALIGN((*cmsg).cmsg_len as usize);
        let end = (*mhdr).msg_control as usize + (*mhdr).msg_controllen as usize;
        if next + _ALIGN(size_of::<cmsghdr>()) > end { core::ptr::null_mut() } else { next as *mut cmsghdr }
    }
}
// inline: CMSG_FIRSTHDR
/// Returns the first `cmsghdr` of a message (or null).
///
/// # Safety
/// `mhdr` must point to a valid `msghdr`.
#[inline(always)]
pub unsafe fn CMSG_FIRSTHDR(mhdr: *const msghdr) -> *mut cmsghdr {
    unsafe {
        if (*mhdr).msg_controllen as usize >= size_of::<cmsghdr>() { (*mhdr).msg_control.cast() } else { core::ptr::null_mut() }
    }
}
// inline: CMSG_LEN
#[inline(always)]
pub const fn CMSG_LEN(len: usize) -> usize {
    _ALIGN(size_of::<cmsghdr>()) + len
}
// inline: CMSG_SPACE
#[inline(always)]
pub const fn CMSG_SPACE(len: usize) -> usize {
    _ALIGN(size_of::<cmsghdr>()) + _ALIGN(len)
}
// inline: SF_FLAGS
#[inline(always)]
pub const fn SF_FLAGS(rh: c_int, flags: c_int) -> c_int {
    (rh << 16) | flags
}
