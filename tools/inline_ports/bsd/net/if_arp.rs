// inline: ar_sha
/// Sender hardware address following an `arphdr`.
#[inline(always)]
pub fn ar_sha(ap: *mut arphdr) -> caddr_t {
    ap.wrapping_add(1).cast::<c_char>()
}
// inline: ar_spa
/// Sender protocol address following an `arphdr`.
///
/// # Safety
/// `ap` must point to a valid `arphdr`.
#[inline(always)]
pub unsafe fn ar_spa(ap: *mut arphdr) -> caddr_t {
    unsafe { ar_sha(ap).wrapping_add((*ap).ar_hln as usize) }
}
// inline: ar_tha
/// Target hardware address following an `arphdr`.
///
/// # Safety
/// `ap` must point to a valid `arphdr`.
#[inline(always)]
pub unsafe fn ar_tha(ap: *mut arphdr) -> caddr_t {
    unsafe { ar_sha(ap).wrapping_add((*ap).ar_hln as usize + (*ap).ar_pln as usize) }
}
// inline: ar_tpa
/// Target protocol address following an `arphdr`.
///
/// # Safety
/// `ap` must point to a valid `arphdr`.
#[inline(always)]
pub unsafe fn ar_tpa(ap: *mut arphdr) -> caddr_t {
    unsafe { ar_sha(ap).wrapping_add(2 * (*ap).ar_hln as usize + (*ap).ar_pln as usize) }
}
// inline: arphdr_len2
#[inline(always)]
pub const fn arphdr_len2(ar_hln: usize, ar_pln: usize) -> usize {
    size_of::<arphdr>() + 2 * ar_hln + 2 * ar_pln
}
// inline: arphdr_len
#[inline(always)]
pub const fn arphdr_len(ap: &arphdr) -> usize {
    arphdr_len2(ap.ar_hln as usize, ap.ar_pln as usize)
}
