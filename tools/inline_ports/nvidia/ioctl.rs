// inline: _NV_IOC
/// Builds an nvidia ioctl number.
#[inline(always)]
pub const fn _NV_IOC(dir: u32, type_: u32, nr: u32, size: u32) -> u32 {
    (dir << _NV_IOC_DIRSHIFT) | (type_ << _NV_IOC_TYPESHIFT) | (nr << _NV_IOC_NRSHIFT) | (size << _NV_IOC_SIZESHIFT)
}
// inline: _NV_IO
#[inline(always)]
pub const fn _NV_IO(type_: u32, nr: u32) -> u32 {
    _NV_IOC(_NV_IOC_NONE, type_, nr, 0)
}
// inline: _NV_IOR
/// `size` is the C macro's `sizeof(size)` argument.
#[inline(always)]
pub const fn _NV_IOR(type_: u32, nr: u32, size: usize) -> u32 {
    _NV_IOC(_NV_IOC_READ, type_, nr, size as u32)
}
// inline: _NV_IOW
/// `size` is the C macro's `sizeof(size)` argument.
#[inline(always)]
pub const fn _NV_IOW(type_: u32, nr: u32, size: usize) -> u32 {
    _NV_IOC(_NV_IOC_WRITE, type_, nr, size as u32)
}
// inline: _NV_IOWR
/// `size` is the C macro's `sizeof(size)` argument.
#[inline(always)]
pub const fn _NV_IOWR(type_: u32, nr: u32, size: usize) -> u32 {
    _NV_IOC(_NV_IOC_READ | _NV_IOC_WRITE, type_, nr, size as u32)
}
// inline: _NV_IOC_DIR
#[inline(always)]
pub const fn _NV_IOC_DIR(nr: u32) -> u32 {
    (nr >> _NV_IOC_DIRSHIFT) & _NV_IOC_DIRMASK
}
// inline: _NV_IOC_TYPE
#[inline(always)]
pub const fn _NV_IOC_TYPE(nr: u32) -> u32 {
    (nr >> _NV_IOC_TYPESHIFT) & _NV_IOC_TYPEMASK
}
// inline: _NV_IOC_NR
#[inline(always)]
pub const fn _NV_IOC_NR(nr: u32) -> u32 {
    (nr >> _NV_IOC_NRSHIFT) & _NV_IOC_NRMASK
}
// inline: _NV_IOC_SIZE
#[inline(always)]
pub const fn _NV_IOC_SIZE(nr: u32) -> u32 {
    (nr >> _NV_IOC_SIZESHIFT) & _NV_IOC_SIZEMASK
}
