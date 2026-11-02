//! `switch/services/bt.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

unsafe extern "C" {
    /// Initialize bt. Only available on [5.0.0+].
    pub fn btInitialize() -> Result;

    /// Exit bt.
    pub fn btExit();

    /// Gets the Service object for the actual bt service session.
    pub fn btGetServiceSession() -> *mut Service;

    /// LeClientReadCharacteristic
    ///
    /// # Arguments
    /// - `connection_handle` (in): ConnectionHandle
    /// - `is_primary` (in): Is a primary service or not
    /// - `serv_id` (in): Service GATT ID `BtdrvGattId`
    /// - `char_id` (in): Characteristic GATT ID `BtdrvGattId`
    /// - `auth_req` (in): `BtdrvGattAuthReqType`
    ///
    /// **Note:** This is essentially the same as `btdrvReadGattCharacteristic`.
    pub fn btLeClientReadCharacteristic(connection_handle: u32, is_primary: bool, serv_id: *const BtdrvGattId, char_id: *const BtdrvGattId, auth_req: u8) -> Result;

    /// LeClientReadDescriptor
    ///
    /// # Arguments
    /// - `connection_handle` (in): ConnectionHandle
    /// - `is_primary` (in): Is a primary service or not
    /// - `serv_id` (in): Service GATT ID `BtdrvGattId`
    /// - `char_id` (in): Characteristic GATT ID `BtdrvGattId`
    /// - `desc_id` (in): Descriptor GATT ID `BtdrvGattId`
    /// - `auth_req` (in): `BtdrvGattAuthReqType`
    ///
    /// **Note:** This is essentially the same as `btdrvReadGattDescriptor`.
    pub fn btLeClientReadDescriptor(connection_handle: u32, is_primary: bool, serv_id: *const BtdrvGattId, char_id: *const BtdrvGattId, desc_id: *const BtdrvGattId, auth_req: u8) -> Result;

    /// LeClientWriteCharacteristic
    ///
    /// # Arguments
    /// - `connection_handle` (in): ConnectionHandle
    /// - `is_primary` (in): Is a primary service or not
    /// - `serv_id` (in): Service GATT ID `BtdrvGattId`
    /// - `char_id` (in): Characteristic GATT ID `BtdrvGattId`
    /// - `buffer` (in): Input buffer.
    /// - `size` (in): Input buffer size, must be &lt;=0x258.
    /// - `auth_req` (in): `BtdrvGattAuthReqType`
    /// - `with_response` (in): Whether to use Write-With-Response write type or not
    ///
    /// **Note:** This is essentially the same as `btdrvWriteGattCharacteristic`.
    pub fn btLeClientWriteCharacteristic(connection_handle: u32, is_primary: bool, serv_id: *const BtdrvGattId, char_id: *const BtdrvGattId, buffer: *const c_void, size: usize, auth_req: u8, with_response: bool) -> Result;

    /// LeClientWriteDescriptor
    ///
    /// # Arguments
    /// - `connection_handle` (in): ConnectionHandle
    /// - `is_primary` (in): Is a primary service or not
    /// - `serv_id` (in): Service GATT ID `BtdrvGattId`
    /// - `char_id` (in): Characteristic GATT ID `BtdrvGattId`
    /// - `desc_id` (in): Descriptor GATT ID `BtdrvGattId`
    /// - `buffer` (in): Input buffer.
    /// - `size` (in): Input buffer size, must be &lt;=0x258.
    /// - `auth_req` (in): `BtdrvGattAuthReqType`
    ///
    /// **Note:** This is essentially the same as `btdrvWriteGattDescriptor`.
    pub fn btLeClientWriteDescriptor(connection_handle: u32, is_primary: bool, serv_id: *const BtdrvGattId, char_id: *const BtdrvGattId, desc_id: *const BtdrvGattId, buffer: *const c_void, size: usize, auth_req: u8) -> Result;

    /// LeClientRegisterNotification
    ///
    /// # Arguments
    /// - `connection_handle` (in): ConnectionHandle
    /// - `is_primary` (in): Is a primary service or not
    /// - `serv_id` (in): Service GATT ID `BtdrvGattId`
    /// - `char_id` (in): Characteristic GATT ID `BtdrvGattId`
    ///
    /// **Note:** This is essentially the same as `btdrvRegisterGattNotification`.
    pub fn btLeClientRegisterNotification(connection_handle: u32, is_primary: bool, serv_id: *const BtdrvGattId, char_id: *const BtdrvGattId) -> Result;

    /// LeClientDeregisterNotification
    ///
    /// # Arguments
    /// - `connection_handle` (in): ConnectionHandle
    /// - `is_primary` (in): Is a primary service or not
    /// - `serv_id` (in): Service GATT ID `BtdrvGattId`
    /// - `char_id` (in): Characteristic GATT ID `BtdrvGattId`
    ///
    /// **Note:** This is essentially the same as `btdrvUnregisterGattNotification`.
    pub fn btLeClientDeregisterNotification(connection_handle: u32, is_primary: bool, serv_id: *const BtdrvGattId, char_id: *const BtdrvGattId) -> Result;

    /// SetLeResponse
    ///
    /// # Arguments
    /// - `server_if` (in): Server interface ID
    /// - `serv_uuid` (in): Service UUID `BtdrvGattAttributeUuid`
    /// - `char_uuid` (in): Characteristic UUID `BtdrvGattAttributeUuid`
    /// - `buffer` (in): Input buffer.
    /// - `size` (in): Input buffer size, must be &lt;=0x258.
    pub fn btSetLeResponse(server_if: u8, serv_uuid: *const BtdrvGattAttributeUuid, char_uuid: *const BtdrvGattAttributeUuid, buffer: *const c_void, size: usize) -> Result;

    /// LeSendIndication
    ///
    /// # Arguments
    /// - `server_if` (in): Server interface ID
    /// - `serv_uuid` (in): Service UUID `BtdrvGattAttributeUuid`
    /// - `char_uuid` (in): Characteristic UUID `BtdrvGattAttributeUuid`
    /// - `buffer` (in): Input buffer.
    /// - `size` (in): Input buffer size, clamped to max size 0x258.
    /// - `noconfirm` (in): Whether no confirmation is required (notification) or not (indication)
    pub fn btLeSendIndication(server_if: u8, serv_uuid: *const BtdrvGattAttributeUuid, char_uuid: *const BtdrvGattAttributeUuid, buffer: *const c_void, size: usize, noconfirm: bool) -> Result;

    /// GetLeEventInfo
    ///
    /// # Arguments
    /// - `buffer` (in): Output buffer. 0x400-bytes from state is written here. See `BtdrvLeEventInfo`.
    /// - `size` (in): Output buffer size.
    /// - `type` (out): Output BtdrvBleEventType.
    ///
    /// **Note:** This is identical to `btdrvGetLeHidEventInfo` except different state is used.
    ///
    /// **Note:** The state used by this is reset after writing the data to output.
    pub fn btGetLeEventInfo(buffer: *mut c_void, size: usize, type_: *mut BtdrvBleEventType) -> Result;

    /// RegisterBleEvent
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=true.
    ///
    /// **Note:** This is identical to `btdrvRegisterBleHidEvent` except different state is used.
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn btRegisterBleEvent(out_event: *mut Event) -> Result;
}
