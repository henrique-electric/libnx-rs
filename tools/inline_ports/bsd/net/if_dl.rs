// inline: LLADDR
/// Pointer to the link-level address inside a `sockaddr_dl`.
///
/// # Safety
/// `s` must point to a valid `sockaddr_dl`.
#[inline(always)]
pub unsafe fn LLADDR(s: *mut sockaddr_dl) -> caddr_t {
    unsafe { (&raw mut (*s).sdl_data).cast::<c_char>().wrapping_add((*s).sdl_nlen as usize) }
}
// inline: CLLADDR
/// Const pointer to the link-level address inside a `sockaddr_dl`.
///
/// # Safety
/// `s` must point to a valid `sockaddr_dl`.
#[inline(always)]
pub unsafe fn CLLADDR(s: *const sockaddr_dl) -> *const c_char {
    unsafe { (&raw const (*s).sdl_data).cast::<c_char>().wrapping_add((*s).sdl_nlen as usize) }
}
// inline: LLINDEX
#[inline(always)]
pub const fn LLINDEX(s: &sockaddr_dl) -> u_short {
    s.sdl_index
}
