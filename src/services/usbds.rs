//! `switch/services/usbds.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const USBDS_DEFAULT_InterfaceNumber: u32 = 0x4;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UsbDsInterface {
    pub initialized: bool,
    pub interface_index: u8,
    pub s: Service,
    pub SetupEvent: Event,
    pub CtrlInCompletionEvent: Event,
    pub CtrlOutCompletionEvent: Event,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UsbDsEndpoint {
    pub initialized: bool,
    pub s: Service,
    pub CompletionEvent: Event,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UsbDsDeviceInfo {
    /// VID
    pub idVendor: u16,
    /// PID
    pub idProduct: u16,
    pub bcdDevice: u16,
    pub Manufacturer: [c_char; 32],
    pub Product: [c_char; 32],
    pub SerialNumber: [c_char; 32],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UsbDsReportEntry {
    /// urbId from post-buffer cmds
    pub id: u32,
    pub requestedSize: u32,
    pub transferredSize: u32,
    pub urb_status: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UsbDsReportData {
    pub report: [UsbDsReportEntry; 8],
    pub report_count: u32,
}

pub type UsbComplexId = u32;
pub const UsbComplexId_Default: UsbComplexId = 0x2;

pub type UsbDeviceSpeed = u32;
pub const UsbDeviceSpeed_None: UsbDeviceSpeed = 0x0;
/// USB 1.0 Low Speed
pub const UsbDeviceSpeed_Low: UsbDeviceSpeed = 0x1;
/// USB 1.1 Full Speed
pub const UsbDeviceSpeed_Full: UsbDeviceSpeed = 0x2;
/// USB 2.0 High Speed
pub const UsbDeviceSpeed_High: UsbDeviceSpeed = 0x3;
/// USB 3.0 Super Speed
pub const UsbDeviceSpeed_Super: UsbDeviceSpeed = 0x4;

impl_zeroed_default!(UsbDsInterface, UsbDsEndpoint, UsbDsDeviceInfo, UsbDsReportEntry, UsbDsReportData);

unsafe extern "C" {
    /// Opens a session with usb:ds.
    pub fn usbDsInitialize() -> Result;

    /// Closes the usb:ds session. Any interfaces/endpoints which are left open are automatically closed, since otherwise usb-sysmodule won't fully reset usb:ds to defaults.
    pub fn usbDsExit();

    /// Gets the Service object for the actual usb:ds service session.
    pub fn usbDsGetServiceSession() -> *mut Service;

    /// Helper func.
    pub fn usbDsWaitReady(timeout: u64) -> Result;

    /// Helper func.
    pub fn usbDsParseReportData(reportdata: *mut UsbDsReportData, urbId: u32, requestedSize: *mut u32, transferredSize: *mut u32) -> Result;

    pub fn usbDsGetStateChangeEvent() -> *mut Event;

    /// Gets the device state. See `UsbState`.
    pub fn usbDsGetState(out: *mut UsbState) -> Result;

    /// Removed in [5.0.0+].
    pub fn usbDsGetDsInterface(out: *mut *mut UsbDsInterface, descriptor: *mut usb_interface_descriptor, interface_name: *const c_char) -> Result;

    /// Removed in [5.0.0+].
    pub fn usbDsSetVidPidBcd(deviceinfo: *const UsbDsDeviceInfo) -> Result;

    /// Only available on [5.0.0+].
    pub fn usbDsRegisterInterface(out: *mut *mut UsbDsInterface) -> Result;

    /// Only available on [5.0.0+].
    pub fn usbDsRegisterInterfaceEx(out: *mut *mut UsbDsInterface, intf_num: u8) -> Result;

    /// Only available on [5.0.0+].
    pub fn usbDsClearDeviceData() -> Result;

    /// Only available on [5.0.0+].
    pub fn usbDsAddUsbStringDescriptor(out_index: *mut u8, string: *const c_char) -> Result;

    /// Only available on [5.0.0+].
    pub fn usbDsAddUsbLanguageStringDescriptor(out_index: *mut u8, lang_ids: *const u16, num_langs: u16) -> Result;

    /// Only available on [5.0.0+].
    pub fn usbDsDeleteUsbStringDescriptor(index: u8) -> Result;

    /// Only available on [5.0.0+].
    pub fn usbDsSetUsbDeviceDescriptor(speed: UsbDeviceSpeed, descriptor: *mut usb_device_descriptor) -> Result;

    /// Only available on [5.0.0+].
    pub fn usbDsSetBinaryObjectStore(bos: *const c_void, bos_size: usize) -> Result;

    /// Only available on [5.0.0+].
    pub fn usbDsEnable() -> Result;

    /// Only available on [5.0.0+].
    pub fn usbDsDisable() -> Result;

    /// Only available on [8.0.0+].
    pub fn usbDsGetSpeed(out: *mut UsbDeviceSpeed) -> Result;

    pub fn usbDsInterface_Close(interface: *mut UsbDsInterface);

    pub fn usbDsInterface_GetSetupPacket(interface: *mut UsbDsInterface, buffer: *mut c_void, size: usize) -> Result;

    pub fn usbDsInterface_EnableInterface(interface: *mut UsbDsInterface) -> Result;

    pub fn usbDsInterface_DisableInterface(interface: *mut UsbDsInterface) -> Result;

    pub fn usbDsInterface_CtrlInPostBufferAsync(interface: *mut UsbDsInterface, buffer: *mut c_void, size: usize, urbId: *mut u32) -> Result;

    pub fn usbDsInterface_CtrlOutPostBufferAsync(interface: *mut UsbDsInterface, buffer: *mut c_void, size: usize, urbId: *mut u32) -> Result;

    pub fn usbDsInterface_GetCtrlInReportData(interface: *mut UsbDsInterface, out: *mut UsbDsReportData) -> Result;

    pub fn usbDsInterface_GetCtrlOutReportData(interface: *mut UsbDsInterface, out: *mut UsbDsReportData) -> Result;

    pub fn usbDsInterface_StallCtrl(interface: *mut UsbDsInterface) -> Result;

    /// Removed in [5.0.0+].
    pub fn usbDsInterface_GetDsEndpoint(interface: *mut UsbDsInterface, endpoint: *mut *mut UsbDsEndpoint, descriptor: *mut usb_endpoint_descriptor) -> Result;

    /// Only available on [5.0.0+].
    pub fn usbDsInterface_RegisterEndpoint(interface: *mut UsbDsInterface, endpoint: *mut *mut UsbDsEndpoint, endpoint_address: u8) -> Result;

    /// Only available on [5.0.0+].
    pub fn usbDsInterface_AppendConfigurationData(interface: *mut UsbDsInterface, speed: UsbDeviceSpeed, buffer: *const c_void, size: usize) -> Result;

    pub fn usbDsEndpoint_Close(endpoint: *mut UsbDsEndpoint);

    pub fn usbDsEndpoint_Cancel(endpoint: *mut UsbDsEndpoint) -> Result;

    pub fn usbDsEndpoint_PostBufferAsync(endpoint: *mut UsbDsEndpoint, buffer: *mut c_void, size: usize, urbId: *mut u32) -> Result;

    pub fn usbDsEndpoint_GetReportData(endpoint: *mut UsbDsEndpoint, out: *mut UsbDsReportData) -> Result;

    pub fn usbDsEndpoint_Stall(endpoint: *mut UsbDsEndpoint) -> Result;

    pub fn usbDsEndpoint_SetZlt(endpoint: *mut UsbDsEndpoint, zlt: bool) -> Result;
}
