//! `switch/services/usbhs.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub type UsbHsInterfaceFilterFlags = u32;
pub const UsbHsInterfaceFilterFlags_idVendor: UsbHsInterfaceFilterFlags = BIT(0);
pub const UsbHsInterfaceFilterFlags_idProduct: UsbHsInterfaceFilterFlags = BIT(1);
pub const UsbHsInterfaceFilterFlags_bcdDevice_Min: UsbHsInterfaceFilterFlags = BIT(2);
pub const UsbHsInterfaceFilterFlags_bcdDevice_Max: UsbHsInterfaceFilterFlags = BIT(3);
pub const UsbHsInterfaceFilterFlags_bDeviceClass: UsbHsInterfaceFilterFlags = BIT(4);
pub const UsbHsInterfaceFilterFlags_bDeviceSubClass: UsbHsInterfaceFilterFlags = BIT(5);
pub const UsbHsInterfaceFilterFlags_bDeviceProtocol: UsbHsInterfaceFilterFlags = BIT(6);
pub const UsbHsInterfaceFilterFlags_bInterfaceClass: UsbHsInterfaceFilterFlags = BIT(7);
pub const UsbHsInterfaceFilterFlags_bInterfaceSubClass: UsbHsInterfaceFilterFlags = BIT(8);
pub const UsbHsInterfaceFilterFlags_bInterfaceProtocol: UsbHsInterfaceFilterFlags = BIT(9);

/// Interface filtering struct. When the associated flag bit is set, the associated descriptor field and struct field are compared, on mismatch the interface is filtered out.
/// [7.0.0+]: The filter struct has to be unique, it can't be used by anything else (including other processes). Hence, Flags has to be non-zero. When initialized with usb:hs:a and VID and/or PID filtering is enabled, the VID/PID will be checked against a blacklist.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UsbHsInterfaceFilter {
    /// See `UsbHsInterfaceFilterFlags`. Setting this to 0 is equivalent to disabling filtering.
    pub Flags: u16,
    pub idVendor: u16,
    pub idProduct: u16,
    /// Descriptor value must be >= bcdDevice_Min.
    pub bcdDevice_Min: u16,
    /// Descriptor value must be &lt;= bcdDevice_Max.
    pub bcdDevice_Max: u16,
    pub bDeviceClass: u8,
    pub bDeviceSubClass: u8,
    pub bDeviceProtocol: u8,
    pub bInterfaceClass: u8,
    pub bInterfaceSubClass: u8,
    pub bInterfaceProtocol: u8,
}

/// Descriptors which are not available are set to all-zero.
/// The INPUT/OUTPUT endpoint descriptors were swapped with [8.0.0+], libnx converts this struct to the newer layout when running on pre-8.0.0.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UsbHsInterfaceInfo {
    pub ID: s32,
    pub deviceID_2: u32,
    pub unk_x8: u32,
    pub interface_desc: usb_interface_descriptor,
    pub pad_x15: [u8; 7],
    pub input_endpoint_descs: [usb_endpoint_descriptor; 15],
    pub pad_x85: [u8; 7],
    pub output_endpoint_descs: [usb_endpoint_descriptor; 15],
    pub pad_xf5: [u8; 6],
    /// ?
    pub input_ss_endpoint_companion_descs: [usb_ss_endpoint_companion_descriptor; 15],
    pub pad_x155: [u8; 6],
    /// ?
    pub output_ss_endpoint_companion_descs: [usb_ss_endpoint_companion_descriptor; 15],
    pub pad_x1b5: [u8; 3],
}

/// Interface struct. Note that devices have a seperate `UsbHsInterface` for each interface.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UsbHsInterface {
    pub inf: UsbHsInterfaceInfo,
    pub pathstr: [c_char; 64],
    pub busID: u32,
    pub deviceID: u32,
    pub device_desc: usb_device_descriptor,
    pub config_desc: usb_config_descriptor,
    pub pad_x21b: [u8; 5],
    /// Unknown u64 timestamp for when the device was inserted?
    pub timestamp: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UsbHsXferReport {
    pub xferId: u32,
    pub res: Result,
    pub requestedSize: u32,
    pub transferredSize: u32,
    /// id from `usbHsEpPostBufferAsync`.
    pub id: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UsbHsRingHeader {
    pub write_index: vu64,
    pub read_index: vu64,
}

/// The interface service object. These Events have autoclear=false.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UsbHsClientIfSession {
    pub s: Service,
    /// Unknown.
    pub event0: Event,
    /// [2.0.0+] Signaled when CtrlXferAsync finishes.
    pub eventCtrlXfer: Event,
    pub ID: s32,
    /// Initialized with the input interface from `usbHsAcquireUsbIf`, then overwritten with the cmd output. Pre-3.0.0 this only overwrites the first 0x1B8-bytes (data before pathstr).
    pub inf: UsbHsInterface,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UsbHsClientEpSession {
    pub s: Service,
    /// [2.0.0+] Signaled when PostBufferAsync finishes.
    pub eventXfer: Event,
    pub maxUrbCount: u32,
    pub max_reports: u64,
    pub ringbuf: *mut c_void,
    pub desc: usb_endpoint_descriptor,
}

/// Returns whether the specified interface session was initialized.
#[inline(always)]
pub fn usbHsIfIsActive(s: &UsbHsClientIfSession) -> bool {
    serviceIsActive(&s.s)
}

/// Returns the ID which can be used for comparing with the ID in the output interfaces from `usbHsQueryAcquiredInterfaces`.
#[inline(always)]
pub fn usbHsIfGetID(s: &UsbHsClientIfSession) -> s32 {
    s.ID
}

/// Gets the Xfer Event which is signaled when PostBufferAsync finishes. This is only valid for [2.0.0+]. If using `eventWait` with this, then `eventClear` should be used if the event was signaled (since the autoclear is false).
#[inline(always)]
pub fn usbHsEpGetXferEvent(s: &mut UsbHsClientEpSession) -> &mut Event {
    &mut s.eventXfer
}

/// Gets the buffer size to use with `usbHsEpShareReportRing`.
#[inline(always)]
pub const fn usbHsEpGetReportRingSize(s: &UsbHsClientEpSession) -> u32 {
    let max_reports = s.maxUrbCount as u64 * 0x21;
    let size = (size_of::<UsbHsRingHeader>() as u64 + max_reports * size_of::<UsbHsXferReport>() as u64) as u32;
    (size.wrapping_add(0xFFF)) & !0xFFF
}

impl_zeroed_default!(UsbHsInterfaceFilter, UsbHsInterfaceInfo, UsbHsInterface, UsbHsXferReport, UsbHsRingHeader, UsbHsClientIfSession, UsbHsClientEpSession);

unsafe extern "C" {
    /// Initialize usb:hs.
    pub fn usbHsInitialize() -> Result;

    /// Exit usb:hs.
    pub fn usbHsExit();

    /// Gets the Service object for the actual usb:hs service session.
    pub fn usbHsGetServiceSession() -> *mut Service;

    /// Returns the Event loaded during init with autoclear=false.
    /// Signaled when a device was removed.
    /// When signaled, the user should use `usbHsQueryAcquiredInterfaces` and cleanup state for all interfaces which are not listed in the output interfaces (none of the IDs match `usbHsIfGetID` output).
    pub fn usbHsGetInterfaceStateChangeEvent() -> *mut Event;

    /// Returns an array of all `UsbHsInterface`. Internally this loads the same interfaces as `usbHsQueryAvailableInterfaces`, followed by `usbHsQueryAcquiredInterfaces`. However, ID in `UsbHsInterface` is set to -1, hence the output from this should not be used with `usbHsAcquireUsbIf`.
    ///
    /// # Arguments
    /// - `filter` (in): `UsbHsInterfaceFilter`.
    /// - `interfaces` (out): Array of output interfaces.
    /// - `interfaces_maxsize` (in): Max byte-size of the interfaces buffer.
    /// - `total_entries` (out): Total number of output interfaces.
    pub fn usbHsQueryAllInterfaces(filter: *const UsbHsInterfaceFilter, interfaces: *mut UsbHsInterface, interfaces_maxsize: usize, total_entries: *mut s32) -> Result;

    /// Returns an array of `UsbHsInterface` which are available.
    ///
    /// # Arguments
    /// - `filter` (in): `UsbHsInterfaceFilter`.
    /// - `interfaces` (out): Array of output interfaces.
    /// - `interfaces_maxsize` (in): Max byte-size of the interfaces buffer.
    /// - `total_entries` (out): Total number of output interfaces.
    pub fn usbHsQueryAvailableInterfaces(filter: *const UsbHsInterfaceFilter, interfaces: *mut UsbHsInterface, interfaces_maxsize: usize, total_entries: *mut s32) -> Result;

    /// Returns an array of `UsbHsInterface` which were previously acquired.
    ///
    /// # Arguments
    /// - `interfaces` (out): Array of output interfaces.
    /// - `interfaces_maxsize` (in): Max byte-size of the interfaces buffer.
    /// - `total_entries` (out): Total number of output interfaces.
    pub fn usbHsQueryAcquiredInterfaces(interfaces: *mut UsbHsInterface, interfaces_maxsize: usize, total_entries: *mut s32) -> Result;

    /// Creates an event which is signaled when an interface is available which passes the filtering checks.
    ///
    /// # Arguments
    /// - `out_event` (out): Event object.
    /// - `autoclear` (in): Event autoclear.
    /// - `index` (in): Event index, must be 0..2.
    /// - `filter` (in): `UsbHsInterfaceFilter`.
    pub fn usbHsCreateInterfaceAvailableEvent(out_event: *mut Event, autoclear: bool, index: u8, filter: *const UsbHsInterfaceFilter) -> Result;

    /// Destroys an event setup by `usbHsCreateInterfaceAvailableEvent`. This *must* be used at some point during cleanup.
    ///
    /// # Arguments
    /// - `event` (in): Event object to close.
    /// - `index` (in): Event index, must be 0..2.
    pub fn usbHsDestroyInterfaceAvailableEvent(event: *mut Event, index: u8) -> Result;

    /// Acquires/opens the specified interface. This returns an error if the interface was already acquired by another process.
    ///
    /// # Arguments
    /// - `s` (in): The service object.
    /// - `interface` (in): Interface to use.
    pub fn usbHsAcquireUsbIf(s: *mut UsbHsClientIfSession, interface: *mut UsbHsInterface) -> Result;

    /// Closes the specified interface session.
    pub fn usbHsIfClose(s: *mut UsbHsClientIfSession);

    /// Selects an interface.
    ///
    /// # Arguments
    /// - `s` (in): The service object.
    /// - `inf` (out): The output interface info. If NULL, the output is stored within s instead.
    /// - `id` (in): ID
    pub fn usbHsIfSetInterface(s: *mut UsbHsClientIfSession, inf: *mut UsbHsInterfaceInfo, id: u8) -> Result;

    /// Gets an interface.
    ///
    /// # Arguments
    /// - `s` (in): The service object.
    /// - `inf` (out): The output interface info. If NULL, the output is stored within s instead.
    pub fn usbHsIfGetInterface(s: *mut UsbHsClientIfSession, inf: *mut UsbHsInterfaceInfo) -> Result;

    /// Gets an alternate interface.
    ///
    /// # Arguments
    /// - `s` (in): The service object.
    /// - `inf` (out): The output interface info. If NULL, the output is stored within s instead.
    /// - `id` (in): ID
    pub fn usbHsIfGetAlternateInterface(s: *mut UsbHsClientIfSession, inf: *mut UsbHsInterfaceInfo, id: u8) -> Result;

    /// On [1.0.0] this is stubbed, just returns 0 with out=0.
    pub fn usbHsIfGetCurrentFrame(s: *mut UsbHsClientIfSession, out: *mut u32) -> Result;

    /// Uses a control transfer, this will block until the transfer finishes. The buffer address and size should be aligned to 0x1000-bytes, where wLength is the original size.
    pub fn usbHsIfCtrlXfer(s: *mut UsbHsClientIfSession, bmRequestType: u8, bRequest: u8, wValue: u16, wIndex: u16, wLength: u16, buffer: *mut c_void, transferredSize: *mut u32) -> Result;

    /// Opens an endpoint. maxUrbCount*maxXferSize must be non-zero.
    ///
    /// # Arguments
    /// - `s` (in): The interface object.
    /// - `ep` (out): The endpoint object.
    /// - `maxUrbCount` (in): maxUrbCount, must be &lt;0x11.
    /// - `maxXferSize` (in): Max transfer size for a packet. This can be desc->wMaxPacketSize. Must be &lt;=0xFF0000.
    /// - `desc` (in): Endpoint descriptor.
    pub fn usbHsIfOpenUsbEp(s: *mut UsbHsClientIfSession, ep: *mut UsbHsClientEpSession, maxUrbCount: u16, maxXferSize: u32, desc: *mut usb_endpoint_descriptor) -> Result;

    /// Resets the device: has the same affect as unplugging the device and plugging it back in.
    pub fn usbHsIfResetDevice(s: *mut UsbHsClientIfSession) -> Result;

    /// Closes the specified endpoint session.
    pub fn usbHsEpClose(s: *mut UsbHsClientEpSession);

    /// Starts an async data transfer with the specified endpoint. The Event from `usbHsEpGetXferEvent` can be used to determine when the transfer finished. If you don't need async, `usbHsEpPostBuffer` can be used instead.
    ///
    /// # Arguments
    /// - `s` (in): The endpoint object.
    /// - `buffer`: Data buffer. The buffer address and size should be aligned to 0x1000-bytes.
    /// - `size` (in): The actual data size.
    /// - `id` (in): This is an arbitrary value which will be later returned in `UsbHsXferReport`. For example a value starting at 0 can be used, then if sending multiple requests at once this value can be incremented each time (with 0 for the first request in this set of requests).
    /// - `xferId` (out): Output xferId.
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn usbHsEpPostBufferAsync(s: *mut UsbHsClientEpSession, buffer: *mut c_void, size: u32, id: u64, xferId: *mut u32) -> Result;

    /// Gets an array of `UsbHsXferReport` for the specified endpoint. This should be used after waiting on the Event from `usbHsEpGetXferEvent`.
    ///
    /// # Arguments
    /// - `s` (in): The endpoint object.
    /// - `reports` (out): Output array of `UsbHsXferReport`.
    /// - `max_reports` (in): Size of the reports array in entries.
    /// - `count` (out): Number of entries written to the array.
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn usbHsEpGetXferReport(s: *mut UsbHsClientEpSession, reports: *mut UsbHsXferReport, max_reports: u32, count: *mut u32) -> Result;

    /// Uses a data transfer with the specified endpoint, this will block until the transfer finishes. This wraps `usbHsEpPostBufferAsync` and `usbHsEpGetXferReport`, and also handles the Event (on pre-2.0.0 this handles using the relevant cmds instead). If async is needed, use `usbHsEpPostBufferAsync` instead.
    ///
    /// # Arguments
    /// - `s` (in): The endpoint object.
    /// - `buffer`: Data buffer. The buffer address and size should be aligned to 0x1000-bytes.
    /// - `size` (in): The actual data size.
    /// - `transferredSize` (out): Output transferred size.
    pub fn usbHsEpPostBuffer(s: *mut UsbHsClientEpSession, buffer: *mut c_void, size: u32, transferredSize: *mut u32) -> Result;

    /// This uses the same functionality internally as `usbHsEpPostBufferAsync` except the urbs array and unk1/unk2 are specified by the user instead.
    ///
    /// # Arguments
    /// - `s` (in): The endpoint object.
    /// - `buffer`: Data buffer. The buffer address and size should be aligned to 0x1000-bytes.
    /// - `urbs` (in): Input array of u32s for the size of each urb.
    /// - `urbCount` (in): Total entries in the urbs array.
    /// - `id` (in): Same as `usbHsEpPostBufferAsync`.
    /// - `unk1` (in): `usbHsEpPostBufferAsync` would internally pass value 0 here.
    /// - `unk2` (in): `usbHsEpPostBufferAsync` would internally pass value 0 here.
    /// - `xferId` (out): Output xferId.
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn usbHsEpBatchBufferAsync(s: *mut UsbHsClientEpSession, buffer: *mut c_void, urbs: *mut u32, urbCount: u32, id: u64, unk1: u32, unk2: u32, xferId: *mut u32) -> Result;

    /// This can be used to map the specified buffer as devicemem, which can then be used with `usbHsEpPostBufferAsync` / `usbHsEpPostBuffer` / `usbHsEpBatchBufferAsync`. If the buffer address passed to those funcs is within this SmmuSpace, the specified buffer must be within the bounds of the SmmuSpace buffer.
    ///
    /// # Arguments
    /// - `s` (in): The endpoint object.
    /// - `buffer`: Buffer address, this must be aligned to 0x1000-bytes.
    /// - `size` (in): Buffer size, this must be aligned to 0x1000-bytes.
    ///
    /// **Note:** Only available on [4.0.0+].
    ///
    /// **Note:** A buffer from usbHsEpCreateSmmuSpace can't be reused by another endpoint with the aforementioned funcs.
    ///
    /// **Note:** This can only be used once per UsbHsClientEpSession object.
    pub fn usbHsEpCreateSmmuSpace(s: *mut UsbHsClientEpSession, buffer: *mut c_void, size: u32) -> Result;

    /// This creates TransferMemory which is used to read `UsbHsXferReport` when `usbHsEpGetXferReport` is used, instead of using the service cmd.
    ///
    /// # Arguments
    /// - `buffer`: Buffer, must be 0x1000-byte aligned.
    /// - `size` (in): Buffer size, `usbHsEpGetReportRingSize` can be used to calculate this.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn usbHsEpShareReportRing(s: *mut UsbHsClientEpSession, buffer: *mut c_void, size: usize) -> Result;
}
