// inline: htonl
#[inline(always)]
pub const fn htonl(hostlong: u32) -> u32 {
    hostlong.swap_bytes()
}
// inline: htons
#[inline(always)]
pub const fn htons(hostshort: u16) -> u16 {
    hostshort.swap_bytes()
}
// inline: ntohl
#[inline(always)]
pub const fn ntohl(netlong: u32) -> u32 {
    netlong.swap_bytes()
}
// inline: ntohs
#[inline(always)]
pub const fn ntohs(netshort: u16) -> u16 {
    netshort.swap_bytes()
}
