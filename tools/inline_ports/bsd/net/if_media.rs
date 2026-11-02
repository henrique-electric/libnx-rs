// inline: IFM_ETHER_SUBTYPE
#[inline(always)]
pub const fn IFM_ETHER_SUBTYPE(x: u32) -> u32 {
    (x & IFM_TMASK) | ((x & (IFM_ETH_XTYPE >> IFM_ETH_XSHIFT)) << IFM_ETH_XSHIFT)
}
// inline: IFM_X
/// Internal shorthand for [`IFM_ETHER_SUBTYPE`].
#[inline(always)]
pub const fn IFM_X(x: u32) -> u32 { IFM_ETHER_SUBTYPE(x) }
// inline: IFM_ETHER_SUBTYPE_SET
#[inline(always)]
pub const fn IFM_ETHER_SUBTYPE_SET(x: u32) -> u32 { IFM_ETHER_SUBTYPE(x) | IFM_ETHER }
// inline: IFM_ETHER_SUBTYPE_GET
#[inline(always)]
pub const fn IFM_ETHER_SUBTYPE_GET(x: u32) -> u32 { x & (IFM_TMASK | IFM_ETH_XTYPE) }
// inline: IFM_ETHER_IS_EXTENDED
#[inline(always)]
pub const fn IFM_ETHER_IS_EXTENDED(x: u32) -> u32 { x & IFM_ETH_XTYPE }
// inline: IFM_TYPE
#[inline(always)]
pub const fn IFM_TYPE(x: u32) -> u32 { x & IFM_NMASK }
// inline: IFM_SUBTYPE
#[inline(always)]
pub const fn IFM_SUBTYPE(x: u32) -> u32 {
    if IFM_TYPE(x) == IFM_ETHER { IFM_ETHER_SUBTYPE_GET(x) } else { x & IFM_TMASK }
}
// inline: IFM_TYPE_MATCH
#[inline(always)]
pub const fn IFM_TYPE_MATCH(x: u32, y: u32) -> bool {
    IFM_TYPE(x) == IFM_TYPE(y) && IFM_SUBTYPE(x) == IFM_SUBTYPE(y)
}
// inline: IFM_TYPE_OPTIONS
#[inline(always)]
pub const fn IFM_TYPE_OPTIONS(x: u32) -> u32 { x & IFM_OMASK }
// inline: IFM_INST
#[inline(always)]
pub const fn IFM_INST(x: u32) -> u32 { (x & IFM_IMASK) >> IFM_ISHIFT }
// inline: IFM_OPTIONS
#[inline(always)]
pub const fn IFM_OPTIONS(x: u32) -> u32 { x & (IFM_OMASK | IFM_GMASK) }
// inline: IFM_MODE
#[inline(always)]
pub const fn IFM_MODE(x: u32) -> u32 { x & IFM_MMASK }
// inline: IFM_MAKEWORD
#[inline(always)]
pub const fn IFM_MAKEWORD(type_: u32, subtype: u32, options: u32, instance: u32) -> u32 {
    type_ | subtype | options | (instance << IFM_ISHIFT)
}
// inline: IFM_MAKEMODE
#[inline(always)]
pub const fn IFM_MAKEMODE(mode: u32) -> u32 { (mode << IFM_MSHIFT) & IFM_MMASK }
