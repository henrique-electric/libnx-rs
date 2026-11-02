// inline: usbHsIfIsActive
#[inline(always)]
pub fn usbHsIfIsActive(s: &UsbHsClientIfSession) -> bool {
    serviceIsActive(&s.s)
}
// inline: usbHsIfGetID
#[inline(always)]
pub fn usbHsIfGetID(s: &UsbHsClientIfSession) -> s32 {
    s.ID
}
// inline: usbHsEpGetXferEvent
#[inline(always)]
pub fn usbHsEpGetXferEvent(s: &mut UsbHsClientEpSession) -> &mut Event {
    &mut s.eventXfer
}
// inline: usbHsEpGetReportRingSize
#[inline(always)]
pub const fn usbHsEpGetReportRingSize(s: &UsbHsClientEpSession) -> u32 {
    let max_reports = s.maxUrbCount as u64 * 0x21;
    let size = (size_of::<UsbHsRingHeader>() as u64 + max_reports * size_of::<UsbHsXferReport>() as u64) as u32;
    (size.wrapping_add(0xFFF)) & !0xFFF
}
