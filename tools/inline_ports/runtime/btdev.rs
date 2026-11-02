// inline: btdevGattAttributeGetType
#[inline(always)]
pub fn btdevGattAttributeGetType(a: &BtdevGattAttribute) -> u8 {
    a.type_
}
// inline: btdevGattAttributeGetUuid
#[inline(always)]
pub fn btdevGattAttributeGetUuid(a: &BtdevGattAttribute, out: &mut BtdrvGattAttributeUuid) {
    *out = a.uuid;
}
// inline: btdevGattAttributeGetHandle
#[inline(always)]
pub fn btdevGattAttributeGetHandle(a: &BtdevGattAttribute) -> u16 {
    a.handle
}
// inline: btdevGattAttributeGetConnectionHandle
#[inline(always)]
pub fn btdevGattAttributeGetConnectionHandle(a: &BtdevGattAttribute) -> u32 {
    a.connection_handle
}
// inline: btdevGattServiceGetInstanceId
#[inline(always)]
pub fn btdevGattServiceGetInstanceId(s: &BtdevGattService) -> u16 {
    s.instance_id
}
// inline: btdevGattServiceGetEndGroupHandle
#[inline(always)]
pub fn btdevGattServiceGetEndGroupHandle(s: &BtdevGattService) -> u16 {
    s.end_group_handle
}
// inline: btdevGattServiceIsPrimaryService
#[inline(always)]
pub fn btdevGattServiceIsPrimaryService(s: &BtdevGattService) -> u16 {
    s.primary_service as u16
}
// inline: btdevGattCharacteristicGetInstanceId
#[inline(always)]
pub fn btdevGattCharacteristicGetInstanceId(c: &BtdevGattCharacteristic) -> u16 {
    c.instance_id
}
// inline: btdevGattCharacteristicGetProperties
#[inline(always)]
pub fn btdevGattCharacteristicGetProperties(c: &BtdevGattCharacteristic) -> u8 {
    c.properties
}
