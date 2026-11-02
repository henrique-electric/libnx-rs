// inline: IOCPARM_LEN
#[inline(always)]
pub const fn IOCPARM_LEN(x: u32) -> u32 { (x >> 16) & IOCPARM_MASK }
// inline: IOCBASECMD
#[inline(always)]
pub const fn IOCBASECMD(x: u32) -> u32 { x & !(IOCPARM_MASK << 16) }
// inline: IOCGROUP
#[inline(always)]
pub const fn IOCGROUP(x: u32) -> u32 { (x >> 8) & 0xff }
// inline: _IOC
/// Builds an ioctl number (an `int`, as in libnx's `ioccom.h`).
#[inline(always)]
pub const fn _IOC(inout: u32, group: u32, num: u32, len: usize) -> c_int {
    (inout | (((len as u32) & IOCPARM_MASK) << 16) | (group << 8) | num) as c_int
}
// inline: _IO
#[inline(always)]
pub const fn _IO(g: u32, n: u32) -> c_int { _IOC(IOC_VOID, g, n, 0) }
// inline: _IOWINT
#[inline(always)]
pub const fn _IOWINT(g: u32, n: u32) -> c_int { _IOC(IOC_VOID, g, n, size_of::<c_int>()) }
// inline: _IOR
/// `len` is the C macro's `sizeof(t)` argument.
#[inline(always)]
pub const fn _IOR(g: u32, n: u32, len: usize) -> c_int { _IOC(IOC_OUT, g, n, len) }
// inline: _IOW
/// `len` is the C macro's `sizeof(t)` argument.
#[inline(always)]
pub const fn _IOW(g: u32, n: u32, len: usize) -> c_int { _IOC(IOC_IN, g, n, len) }
// inline: _IOWR
/// `len` is the C macro's `sizeof(t)` argument.
#[inline(always)]
pub const fn _IOWR(g: u32, n: u32, len: usize) -> c_int { _IOC(IOC_INOUT, g, n, len) }
