//! `switch/services/usb.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const USB_DT_INTERFACE_SIZE: u32 = 0x9;
pub const USB_DT_ENDPOINT_SIZE: u32 = 0x7;
pub const USB_DT_INTERFACE_ASSOCIATION_SIZE: u32 = 0x8;
pub const USB_DT_DEVICE_SIZE: u32 = 0x12;
pub const USB_DT_HID_SIZE: u32 = 0x9;
pub const USB_DT_SS_ENDPOINT_COMPANION_SIZE: u32 = 0x6;
pub const USB_ENDPOINT_ADDRESS_MASK: u32 = 0xf;
pub const USB_ENDPOINT_DIR_MASK: u32 = 0x80;
pub const USB_TRANSFER_TYPE_MASK: u32 = 0x3;

/// Imported from libusb, with some adjustments.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct usb_control_setup {
    pub bmRequestType: u8,
    pub bRequest: u8,
    pub wValue: u16,
    pub wIndex: u16,
    pub wLength: u16,
}

/// Imported from libusb, with some adjustments.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct usb_endpoint_descriptor {
    pub bLength: u8,
    /// Must match USB_DT_ENDPOINT.
    pub bDescriptorType: u8,
    /// Should be one of the usb_endpoint_direction values, the endpoint-number is automatically allocated.
    pub bEndpointAddress: u8,
    pub bmAttributes: u8,
    pub wMaxPacketSize: u16,
    pub bInterval: u8,
}

/// Imported from libusb, with some adjustments.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct usb_interface_descriptor {
    pub bLength: u8,
    /// Must match USB_DT_INTERFACE.
    pub bDescriptorType: u8,
    /// See also USBDS_DEFAULT_InterfaceNumber.
    pub bInterfaceNumber: u8,
    /// Must match 0.
    pub bAlternateSetting: u8,
    pub bNumEndpoints: u8,
    pub bInterfaceClass: u8,
    pub bInterfaceSubClass: u8,
    pub bInterfaceProtocol: u8,
    /// Ignored.
    pub iInterface: u8,
}

/// Imported from libusb, with some adjustments.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct usb_device_descriptor {
    pub bLength: u8,
    /// Must match USB_DT_Device.
    pub bDescriptorType: u8,
    pub bcdUSB: u16,
    pub bDeviceClass: u8,
    pub bDeviceSubClass: u8,
    pub bDeviceProtocol: u8,
    pub bMaxPacketSize0: u8,
    pub idVendor: u16,
    pub idProduct: u16,
    pub bcdDevice: u16,
    pub iManufacturer: u8,
    pub iProduct: u8,
    pub iSerialNumber: u8,
    pub bNumConfigurations: u8,
}

/// Imported from libusb, with some adjustments.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct usb_config_descriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub wTotalLength: u16,
    pub bNumInterfaces: u8,
    pub bConfigurationValue: u8,
    pub iConfiguration: u8,
    pub bmAttributes: u8,
    pub MaxPower: u8,
}

/// Imported from libusb, with some adjustments.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct usb_ss_endpoint_companion_descriptor {
    pub bLength: u8,
    /// Must match USB_DT_SS_ENDPOINT_COMPANION.
    pub bDescriptorType: u8,
    pub bMaxBurst: u8,
    pub bmAttributes: u8,
    pub wBytesPerInterval: u16,
}

/// Imported from libusb, with some adjustments.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct usb_string_descriptor {
    pub bLength: u8,
    /// Must match USB_DT_STRING.
    pub bDescriptorType: u8,
    pub wData: [u16; 64],
}

/// Imported from libusb, with some adjustments.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct usb_interface_association_descriptor {
    pub bLength: u8,
    /// Must match USB_DT_INTERFACE_ASSOCIATION.
    pub bDescriptorType: u8,
    pub bFirstInterface: u8,
    pub bInterfaceCount: u8,
    pub bFunctionClass: u8,
    pub bFunctionSubClass: u8,
    pub bFunctionProtocol: u8,
    pub iFunction: u8,
}

/// Imported from libusb, with some adjustments.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct usb_hid_descriptor {
    pub bLength: u8,
    /// Must match USB_DT_HID.
    pub bDescriptorType: u8,
    pub bcdHID: u16,
    pub bCountryCode: u8,
    pub bNumDescriptors: u8,
    pub bClassDescriptorType: u8,
    pub wClassDescriptorLength: u16,
}

/// Imported from libusb, with changed names.
pub type usb_class_code = u32;
pub const USB_CLASS_PER_INTERFACE: usb_class_code = 0;
pub const USB_CLASS_AUDIO: usb_class_code = 1;
pub const USB_CLASS_COMM: usb_class_code = 2;
pub const USB_CLASS_HID: usb_class_code = 3;
pub const USB_CLASS_PHYSICAL: usb_class_code = 5;
pub const USB_CLASS_PRINTER: usb_class_code = 7;
pub const USB_CLASS_PTP: usb_class_code = 6;
pub const USB_CLASS_IMAGE: usb_class_code = 6;
pub const USB_CLASS_MASS_STORAGE: usb_class_code = 8;
pub const USB_CLASS_HUB: usb_class_code = 9;
pub const USB_CLASS_DATA: usb_class_code = 10;
pub const USB_CLASS_SMART_CARD: usb_class_code = 0x0b;
pub const USB_CLASS_CONTENT_SECURITY: usb_class_code = 0x0d;
pub const USB_CLASS_VIDEO: usb_class_code = 0x0e;
pub const USB_CLASS_PERSONAL_HEALTHCARE: usb_class_code = 0x0f;
pub const USB_CLASS_DIAGNOSTIC_DEVICE: usb_class_code = 0xdc;
pub const USB_CLASS_WIRELESS: usb_class_code = 0xe0;
pub const USB_CLASS_APPLICATION: usb_class_code = 0xfe;
pub const USB_CLASS_VENDOR_SPEC: usb_class_code = 0xff;

/// Imported from libusb, with changed names.
pub type usb_descriptor_type = u32;
pub const USB_DT_DEVICE: usb_descriptor_type = 0x01;
pub const USB_DT_CONFIG: usb_descriptor_type = 0x02;
pub const USB_DT_STRING: usb_descriptor_type = 0x03;
pub const USB_DT_INTERFACE: usb_descriptor_type = 0x04;
pub const USB_DT_ENDPOINT: usb_descriptor_type = 0x05;
pub const USB_DT_INTERFACE_ASSOCIATION: usb_descriptor_type = 0x0b;
pub const USB_DT_BOS: usb_descriptor_type = 0x0f;
pub const USB_DT_DEVICE_CAPABILITY: usb_descriptor_type = 0x10;
pub const USB_DT_HID: usb_descriptor_type = 0x21;
pub const USB_DT_REPORT: usb_descriptor_type = 0x22;
pub const USB_DT_PHYSICAL: usb_descriptor_type = 0x23;
pub const USB_DT_HUB: usb_descriptor_type = 0x29;
pub const USB_DT_SUPERSPEED_HUB: usb_descriptor_type = 0x2a;
pub const USB_DT_SS_ENDPOINT_COMPANION: usb_descriptor_type = 0x30;

/// Imported from libusb, with changed names.
pub type usb_endpoint_direction = u32;
pub const USB_ENDPOINT_IN: usb_endpoint_direction = 0x80;
pub const USB_ENDPOINT_OUT: usb_endpoint_direction = 0x00;

/// Imported from libusb, with changed names.
pub type usb_transfer_type = u32;
pub const USB_TRANSFER_TYPE_CONTROL: usb_transfer_type = 0;
pub const USB_TRANSFER_TYPE_ISOCHRONOUS: usb_transfer_type = 1;
pub const USB_TRANSFER_TYPE_BULK: usb_transfer_type = 2;
pub const USB_TRANSFER_TYPE_INTERRUPT: usb_transfer_type = 3;
pub const USB_TRANSFER_TYPE_BULK_STREAM: usb_transfer_type = 4;

/// Imported from libusb, with changed names.
pub type usb_standard_request = u32;
/// Request status of the specific recipient
pub const USB_REQUEST_GET_STATUS: usb_standard_request = 0x00;
/// Clear or disable a specific feature
pub const USB_REQUEST_CLEAR_FEATURE: usb_standard_request = 0x01;
/// Set or enable a specific feature
pub const USB_REQUEST_SET_FEATURE: usb_standard_request = 0x03;
/// Set device address for all future accesses
pub const USB_REQUEST_SET_ADDRESS: usb_standard_request = 0x05;
/// Get the specified descriptor
pub const USB_REQUEST_GET_DESCRIPTOR: usb_standard_request = 0x06;
/// Used to update existing descriptors or add new descriptors
pub const USB_REQUEST_SET_DESCRIPTOR: usb_standard_request = 0x07;
/// Get the current device configuration value
pub const USB_REQUEST_GET_CONFIGURATION: usb_standard_request = 0x08;
/// Set device configuration
pub const USB_REQUEST_SET_CONFIGURATION: usb_standard_request = 0x09;
/// Return the selected alternate setting for the specified interface
pub const USB_REQUEST_GET_INTERFACE: usb_standard_request = 0x0A;
/// Select an alternate interface for the specified interface
pub const USB_REQUEST_SET_INTERFACE: usb_standard_request = 0x0B;
/// Set then report an endpoint's synchronization frame
pub const USB_REQUEST_SYNCH_FRAME: usb_standard_request = 0x0C;
/// Sets both the U1 and U2 Exit Latency
pub const USB_REQUEST_SET_SEL: usb_standard_request = 0x30;
/// Delay from the time a host transmits a packet to the time it is
/// received by the device.
pub const USB_SET_ISOCH_DELAY: usb_standard_request = 0x31;

/// Imported from libusb, with changed names.
pub type usb_request_type = u32;
pub const USB_REQUEST_TYPE_STANDARD: usb_request_type = (0x00 << 5);
pub const USB_REQUEST_TYPE_CLASS: usb_request_type = (0x01 << 5);
pub const USB_REQUEST_TYPE_VENDOR: usb_request_type = (0x02 << 5);
pub const USB_REQUEST_TYPE_RESERVED: usb_request_type = (0x03 << 5);

/// Imported from libusb, with changed names.
pub type usb_request_recipient = u32;
pub const USB_RECIPIENT_DEVICE: usb_request_recipient = 0x00;
pub const USB_RECIPIENT_INTERFACE: usb_request_recipient = 0x01;
pub const USB_RECIPIENT_ENDPOINT: usb_request_recipient = 0x02;
pub const USB_RECIPIENT_OTHER: usb_request_recipient = 0x03;

/// Imported from libusb, with changed names.
pub type usb_iso_sync_type = u32;
pub const USB_ISO_SYNC_TYPE_NONE: usb_iso_sync_type = 0;
pub const USB_ISO_SYNC_TYPE_ASYNC: usb_iso_sync_type = 1;
pub const USB_ISO_SYNC_TYPE_ADAPTIVE: usb_iso_sync_type = 2;
pub const USB_ISO_SYNC_TYPE_SYNC: usb_iso_sync_type = 3;

/// Imported from libusb, with changed names.
pub type usb_iso_usage_type = u32;
pub const USB_ISO_USAGE_TYPE_DATA: usb_iso_usage_type = 0;
pub const USB_ISO_USAGE_TYPE_FEEDBACK: usb_iso_usage_type = 1;
pub const USB_ISO_USAGE_TYPE_IMPLICIT: usb_iso_usage_type = 2;

/// USB Device States, per USB 2.0 spec
pub type UsbState = u32;
/// Device is not attached to USB.
pub const UsbState_Detached: UsbState = 0;
/// Device is attached, but is not powered.
pub const UsbState_Attached: UsbState = 1;
/// Device is attached and powered, but has not been reset.
pub const UsbState_Powered: UsbState = 2;
/// Device is attached, powered, and has been reset, but does not have an address.
pub const UsbState_Default: UsbState = 3;
/// Device is attached, powered, has been reset, has an address, but is not configured.
pub const UsbState_Address: UsbState = 4;
/// Device is attached, powered, has been reset, has an address, configured, and may be used.
pub const UsbState_Configured: UsbState = 5;
/// Device is attached and powered, but has not seen bus activity for 3ms. Device is suspended and cannot be used.
pub const UsbState_Suspended: UsbState = 6;

impl_zeroed_default!(usb_control_setup, usb_endpoint_descriptor, usb_interface_descriptor, usb_device_descriptor, usb_config_descriptor, usb_ss_endpoint_companion_descriptor, usb_string_descriptor, usb_interface_association_descriptor, usb_hid_descriptor);
