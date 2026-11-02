//! `switch/runtime/devices/usb_comms.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UsbCommsInterfaceInfo {
    pub bInterfaceClass: u8,
    pub bInterfaceSubClass: u8,
    pub bInterfaceProtocol: u8,
}

impl_zeroed_default!(UsbCommsInterfaceInfo);

unsafe extern "C" {
    /// Initializes usbComms with the default number of interfaces (1)
    pub fn usbCommsInitialize() -> Result;

    /// Initializes usbComms with a specific number of interfaces.
    pub fn usbCommsInitializeEx(num_interfaces: u32, infos: *const UsbCommsInterfaceInfo, idVendor: u16, idProduct: u16) -> Result;

    /// Exits usbComms.
    pub fn usbCommsExit();

    /// Sets whether to throw a fatal error in usbComms{Read/Write}* on failure, or just return the transferred size. By default (false) the latter is used.
    pub fn usbCommsSetErrorHandling(flag: bool);

    /// Read data with the default interface.
    pub fn usbCommsRead(buffer: *mut c_void, size: usize) -> usize;

    /// Write data with the default interface.
    pub fn usbCommsWrite(buffer: *const c_void, size: usize) -> usize;

    /// Same as usbCommsRead except with the specified interface.
    pub fn usbCommsReadEx(buffer: *mut c_void, size: usize, interface: u32) -> usize;

    /// Same as usbCommsWrite except with the specified interface.
    pub fn usbCommsWriteEx(buffer: *const c_void, size: usize, interface: u32) -> usize;

    /// Retrieve event used for read completion with the given interface.
    pub fn usbCommsGetReadCompletionEvent(interface: u32) -> *mut Event;

    /// Start an asynchronous read. The completion event will be signaled when the read completes.
    /// The buffer must be page-aligned and no larger than one page.
    pub fn usbCommsReadAsync(buffer: *mut c_void, size: usize, urbId: *mut u32, interface: u32) -> Result;

    /// Complete an asynchronous read, clearing the completion event, and return the amount of data which was read.
    pub fn usbCommsGetReadResult(urbId: u32, transferredSize: *mut u32, interface: u32) -> Result;

    /// Retrieve event used for write completion with the given interface.
    pub fn usbCommsGetWriteCompletionEvent(interface: u32) -> *mut Event;

    /// Start an asynchronous write. The completion event will be signaled when the write completes.
    /// The buffer must be page-aligned and no larger than one page.
    pub fn usbCommsWriteAsync(buffer: *mut c_void, size: usize, urbId: *mut u32, interface: u32) -> Result;

    /// Complete an asynchronous write, clearing the completion event, and return the amount of data which was written.
    pub fn usbCommsGetWriteResult(urbId: u32, transferredSize: *mut u32, interface: u32) -> Result;
}
