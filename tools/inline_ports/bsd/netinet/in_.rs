// inline: IN_CLASSA
#[inline(always)]
pub const fn IN_CLASSA(i: in_addr_t) -> bool { (i & 0x80000000) == 0 }
// inline: IN_CLASSB
#[inline(always)]
pub const fn IN_CLASSB(i: in_addr_t) -> bool { (i & 0xc0000000) == 0x80000000 }
// inline: IN_CLASSC
#[inline(always)]
pub const fn IN_CLASSC(i: in_addr_t) -> bool { (i & 0xe0000000) == 0xc0000000 }
// inline: IN_CLASSD
#[inline(always)]
pub const fn IN_CLASSD(i: in_addr_t) -> bool { (i & 0xf0000000) == 0xe0000000 }
// inline: IN_MULTICAST
#[inline(always)]
pub const fn IN_MULTICAST(i: in_addr_t) -> bool { IN_CLASSD(i) }
// inline: IN_EXPERIMENTAL
#[inline(always)]
pub const fn IN_EXPERIMENTAL(i: in_addr_t) -> bool { (i & 0xf0000000) == 0xf0000000 }
// inline: IN_BADCLASS
#[inline(always)]
pub const fn IN_BADCLASS(i: in_addr_t) -> bool { (i & 0xf0000000) == 0xf0000000 }
// inline: IN_LINKLOCAL
#[inline(always)]
pub const fn IN_LINKLOCAL(i: in_addr_t) -> bool { (i & 0xffff0000) == 0xa9fe0000 }
// inline: IN_LOOPBACK
#[inline(always)]
pub const fn IN_LOOPBACK(i: in_addr_t) -> bool { (i & 0xff000000) == 0x7f000000 }
// inline: IN_ZERONET
#[inline(always)]
pub const fn IN_ZERONET(i: in_addr_t) -> bool { (i & 0xff000000) == 0 }
// inline: IN_PRIVATE
#[inline(always)]
pub const fn IN_PRIVATE(i: in_addr_t) -> bool {
    (i & 0xff000000) == 0x0a000000 || (i & 0xfff00000) == 0xac100000 || (i & 0xffff0000) == 0xc0a80000
}
// inline: IN_LOCAL_GROUP
#[inline(always)]
pub const fn IN_LOCAL_GROUP(i: in_addr_t) -> bool { (i & 0xffffff00) == 0xe0000000 }
// inline: IN_ANY_LOCAL
#[inline(always)]
pub const fn IN_ANY_LOCAL(i: in_addr_t) -> bool { IN_LINKLOCAL(i) || IN_LOCAL_GROUP(i) }
// inline: in6_words
/// The address as four native-endian words, like the C macros' `__u6_addr32` view.
#[inline(always)]
const fn in6_words(a: &in6_addr) -> [u32; 4] {
    // SAFETY: every bit pattern of the address union is a valid `[u32; 4]`.
    unsafe { a.anon0.__u6_addr32 }
}
// inline: in6_bytes
#[inline(always)]
const fn in6_bytes(a: &in6_addr) -> [u8; 16] {
    // SAFETY: every bit pattern of the address union is a valid `[u8; 16]`.
    unsafe { a.anon0.s6_addr }
}
// inline: IN6_IS_ADDR_UNSPECIFIED
#[inline(always)]
pub const fn IN6_IS_ADDR_UNSPECIFIED(a: &in6_addr) -> bool {
    let w = in6_words(a);
    w[0] == 0 && w[1] == 0 && w[2] == 0 && w[3] == 0
}
// inline: IN6_IS_ADDR_LOOPBACK
#[inline(always)]
pub const fn IN6_IS_ADDR_LOOPBACK(a: &in6_addr) -> bool {
    let w = in6_words(a);
    w[0] == 0 && w[1] == 0 && w[2] == 0 && w[3] == ntohl(1)
}
// inline: IN6_IS_ADDR_V4COMPAT
#[inline(always)]
pub const fn IN6_IS_ADDR_V4COMPAT(a: &in6_addr) -> bool {
    let w = in6_words(a);
    w[0] == 0 && w[1] == 0 && w[2] == 0 && w[3] != 0 && w[3] != ntohl(1)
}
// inline: IN6_IS_ADDR_V4MAPPED
#[inline(always)]
pub const fn IN6_IS_ADDR_V4MAPPED(a: &in6_addr) -> bool {
    let w = in6_words(a);
    w[0] == 0 && w[1] == 0 && w[2] == ntohl(0x0000ffff)
}
// inline: IN6_IS_ADDR_LINKLOCAL
#[inline(always)]
pub const fn IN6_IS_ADDR_LINKLOCAL(a: &in6_addr) -> bool {
    let b = in6_bytes(a);
    b[0] == 0xfe && (b[1] & 0xc0) == 0x80
}
// inline: IN6_IS_ADDR_SITELOCAL
#[inline(always)]
pub const fn IN6_IS_ADDR_SITELOCAL(a: &in6_addr) -> bool {
    let b = in6_bytes(a);
    b[0] == 0xfe && (b[1] & 0xc0) == 0xc0
}
// inline: IN6_IS_ADDR_MULTICAST
#[inline(always)]
pub const fn IN6_IS_ADDR_MULTICAST(a: &in6_addr) -> bool {
    in6_bytes(a)[0] == 0xff
}
// inline: __IPV6_ADDR_MC_SCOPE
#[inline(always)]
pub const fn __IPV6_ADDR_MC_SCOPE(a: &in6_addr) -> u8 {
    in6_bytes(a)[1] & 0x0f
}
// inline: IN6_IS_ADDR_MC_NODELOCAL
#[inline(always)]
pub const fn IN6_IS_ADDR_MC_NODELOCAL(a: &in6_addr) -> bool {
    IN6_IS_ADDR_MULTICAST(a) && __IPV6_ADDR_MC_SCOPE(a) as u32 == __IPV6_ADDR_SCOPE_NODELOCAL
}
// inline: IN6_IS_ADDR_MC_LINKLOCAL
#[inline(always)]
pub const fn IN6_IS_ADDR_MC_LINKLOCAL(a: &in6_addr) -> bool {
    IN6_IS_ADDR_MULTICAST(a) && __IPV6_ADDR_MC_SCOPE(a) as u32 == __IPV6_ADDR_SCOPE_LINKLOCAL
}
// inline: IN6_IS_ADDR_MC_SITELOCAL
#[inline(always)]
pub const fn IN6_IS_ADDR_MC_SITELOCAL(a: &in6_addr) -> bool {
    IN6_IS_ADDR_MULTICAST(a) && __IPV6_ADDR_MC_SCOPE(a) as u32 == __IPV6_ADDR_SCOPE_SITELOCAL
}
// inline: IN6_IS_ADDR_MC_ORGLOCAL
#[inline(always)]
pub const fn IN6_IS_ADDR_MC_ORGLOCAL(a: &in6_addr) -> bool {
    IN6_IS_ADDR_MULTICAST(a) && __IPV6_ADDR_MC_SCOPE(a) as u32 == __IPV6_ADDR_SCOPE_ORGLOCAL
}
// inline: IN6_IS_ADDR_MC_GLOBAL
#[inline(always)]
pub const fn IN6_IS_ADDR_MC_GLOBAL(a: &in6_addr) -> bool {
    IN6_IS_ADDR_MULTICAST(a) && __IPV6_ADDR_MC_SCOPE(a) as u32 == __IPV6_ADDR_SCOPE_GLOBAL
}
