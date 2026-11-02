//! `switch/services/uart.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// UartPort
pub type UartPort = u32;
/// Bluetooth
pub const UartPort_Bluetooth: UartPort = 1;
/// Joy-Con(R)
pub const UartPort_JoyConR: UartPort = 2;
/// Joy-Con(L)
pub const UartPort_JoyConL: UartPort = 3;
/// MCU
pub const UartPort_MCU: UartPort = 4;

/// UartPortForDev
pub type UartPortForDev = u32;
/// Joy-Con(R)
pub const UartPortForDev_JoyConR: UartPortForDev = 1;
/// Joy-Con(L)
pub const UartPortForDev_JoyConL: UartPortForDev = 2;
/// Bluetooth
pub const UartPortForDev_Bluetooth: UartPortForDev = 3;

/// FlowControlMode
pub type UartFlowControlMode = u32;
/// None
pub const UartFlowControlMode_None: UartFlowControlMode = 0;
/// Hardware
pub const UartFlowControlMode_Hardware: UartFlowControlMode = 1;

/// PortEventType
pub type UartPortEventType = u32;
/// SendBufferEmpty
pub const UartPortEventType_SendBufferEmpty: UartPortEventType = 0;
/// SendBufferReady
pub const UartPortEventType_SendBufferReady: UartPortEventType = 1;
/// ReceiveBufferReady
pub const UartPortEventType_ReceiveBufferReady: UartPortEventType = 2;
/// ReceiveEnd
pub const UartPortEventType_ReceiveEnd: UartPortEventType = 3;

/// PortSession
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UartPortSession {
    /// IPortSession
    pub s: Service,
}

impl_zeroed_default!(UartPortSession);

unsafe extern "C" {
    /// Initialize uart.
    pub fn uartInitialize() -> Result;

    /// Exit uart.
    pub fn uartExit();

    /// Gets the Service object for the actual uart service session.
    pub fn uartGetServiceSession() -> *mut Service;

    /// HasPort
    ///
    /// # Arguments
    /// - `port` (in): `UartPort`
    /// - `out` (out): Output success flag.
    ///
    /// **Note:** Only available on [1.0.0-16.1.0].
    pub fn uartHasPort(port: UartPort, out: *mut bool) -> Result;

    /// HasPortForDev
    ///
    /// # Arguments
    /// - `port` (in): `UartPortForDev`
    /// - `out` (out): Output success flag.
    ///
    /// **Note:** Only available on [1.0.0-16.1.0].
    pub fn uartHasPortForDev(port: UartPortForDev, out: *mut bool) -> Result;

    /// IsSupportedBaudRate
    ///
    /// # Arguments
    /// - `port` (in): `UartPort`
    /// - `baud_rate` (in): BaudRate
    /// - `out` (out): Output success flag.
    ///
    /// **Note:** Only available on [1.0.0-16.1.0].
    pub fn uartIsSupportedBaudRate(port: UartPort, baud_rate: u32, out: *mut bool) -> Result;

    /// IsSupportedBaudRateForDev
    ///
    /// # Arguments
    /// - `port` (in): `UartPortForDev`
    /// - `baud_rate` (in): BaudRate
    /// - `out` (out): Output success flag.
    ///
    /// **Note:** Only available on [1.0.0-16.1.0].
    pub fn uartIsSupportedBaudRateForDev(port: UartPortForDev, baud_rate: u32, out: *mut bool) -> Result;

    /// IsSupportedFlowControlMode
    ///
    /// # Arguments
    /// - `port` (in): `UartPort`
    /// - `flow_control_mode` (in): `UartFlowControlMode`
    /// - `out` (out): Output success flag.
    ///
    /// **Note:** Only available on [1.0.0-16.1.0].
    pub fn uartIsSupportedFlowControlMode(port: UartPort, flow_control_mode: UartFlowControlMode, out: *mut bool) -> Result;

    /// IsSupportedFlowControlModeForDev
    ///
    /// # Arguments
    /// - `port` (in): `UartPortForDev`
    /// - `flow_control_mode` (in): `UartFlowControlMode`
    /// - `out` (out): Output success flag.
    ///
    /// **Note:** Only available on [1.0.0-16.1.0].
    pub fn uartIsSupportedFlowControlModeForDev(port: UartPortForDev, flow_control_mode: UartFlowControlMode, out: *mut bool) -> Result;

    /// Creates an `UartPortSession`.
    ///
    /// # Arguments
    /// - `s` (out): `UartPortSession`
    ///
    /// **Note:** Use `uartPortSessionOpenPort` or `uartPortSessionOpenPortForDev` before using any other cmds.
    pub fn uartCreatePortSession(s: *mut UartPortSession) -> Result;

    /// IsSupportedPortEvent
    ///
    /// # Arguments
    /// - `port` (in): `UartPort`
    /// - `port_event_type` (in): `UartPortEventType`
    /// - `out` (out): Output success flag.
    ///
    /// **Note:** Only available on [1.0.0-16.1.0].
    pub fn uartIsSupportedPortEvent(port: UartPort, port_event_type: UartPortEventType, out: *mut bool) -> Result;

    /// IsSupportedPortEventForDev
    ///
    /// # Arguments
    /// - `port` (in): `UartPortForDev`
    /// - `port_event_type` (in): `UartPortEventType`
    /// - `out` (out): Output success flag.
    ///
    /// **Note:** Only available on [1.0.0-16.1.0].
    pub fn uartIsSupportedPortEventForDev(port: UartPortForDev, port_event_type: UartPortEventType, out: *mut bool) -> Result;

    /// IsSupportedDeviceVariation
    ///
    /// # Arguments
    /// - `port` (in): `UartPort`
    /// - `device_variation` (in): DeviceVariation
    /// - `out` (out): Output success flag.
    ///
    /// **Note:** Only available on [7.0.0-16.1.0].
    pub fn uartIsSupportedDeviceVariation(port: UartPort, device_variation: u32, out: *mut bool) -> Result;

    /// IsSupportedDeviceVariationForDev
    ///
    /// # Arguments
    /// - `port` (in): `UartPortForDev`
    /// - `device_variation` (in): DeviceVariation
    /// - `out` (out): Output success flag.
    ///
    /// **Note:** Only available on [7.0.0-16.1.0].
    pub fn uartIsSupportedDeviceVariationForDev(port: UartPortForDev, device_variation: u32, out: *mut bool) -> Result;

    /// Close an `UartPortSession`.
    ///
    /// # Arguments
    /// - `s`: `UartPortSession`
    pub fn uartPortSessionClose(s: *mut UartPortSession);

    /// OpenPort
    ///
    /// # Arguments
    /// - `s`: `UartPortSession`
    /// - `out` (out): Output success flag.
    /// - `port` (in): `UartPort`
    /// - `baud_rate` (in): BaudRate
    /// - `flow_control_mode` (in): `UartFlowControlMode`
    /// - `device_variation` (in): [7.0.0+] DeviceVariation
    /// - `is_invert_tx` (in): [6.0.0+] IsInvertTx
    /// - `is_invert_rx` (in): [6.0.0+] IsInvertRx
    /// - `is_invert_rts` (in): [6.0.0+] IsInvertRts
    /// - `is_invert_cts` (in): [6.0.0+] IsInvertCts
    /// - `send_buffer` (in): Send buffer, must be 0x1000-byte aligned.
    /// - `send_buffer_length` (in): Send buffer size, must be 0x1000-byte aligned.
    /// - `receive_buffer` (in): Receive buffer, must be 0x1000-byte aligned.
    /// - `receive_buffer_length` (in): Receive buffer size, must be 0x1000-byte aligned.
    ///
    /// **Note:** This is not usable when the specified `UartPort` is already being used.
    pub fn uartPortSessionOpenPort(s: *mut UartPortSession, out: *mut bool, port: UartPort, baud_rate: u32, flow_control_mode: UartFlowControlMode, device_variation: u32, is_invert_tx: bool, is_invert_rx: bool, is_invert_rts: bool, is_invert_cts: bool, send_buffer: *mut c_void, send_buffer_length: u64, receive_buffer: *mut c_void, receive_buffer_length: u64) -> Result;

    /// OpenPortForDev
    ///
    /// # Arguments
    /// - `s`: `UartPortSession`
    /// - `out` (out): Output success flag.
    /// - `port` (in): `UartPortForDev`
    /// - `baud_rate` (in): BaudRate
    /// - `flow_control_mode` (in): `UartFlowControlMode`
    /// - `device_variation` (in): [7.0.0+] DeviceVariation
    /// - `is_invert_tx` (in): [6.0.0+] IsInvertTx
    /// - `is_invert_rx` (in): [6.0.0+] IsInvertRx
    /// - `is_invert_rts` (in): [6.0.0+] IsInvertRts
    /// - `is_invert_cts` (in): [6.0.0+] IsInvertCts
    /// - `send_buffer` (in): Send buffer, must be 0x1000-byte aligned.
    /// - `send_buffer_length` (in): Send buffer size, must be 0x1000-byte aligned.
    /// - `receive_buffer` (in): Receive buffer, must be 0x1000-byte aligned.
    /// - `receive_buffer_length` (in): Receive buffer size, must be 0x1000-byte aligned.
    ///
    /// **Note:** See the notes for `uartPortSessionOpenPort`.
    pub fn uartPortSessionOpenPortForDev(s: *mut UartPortSession, out: *mut bool, port: UartPortForDev, baud_rate: u32, flow_control_mode: UartFlowControlMode, device_variation: u32, is_invert_tx: bool, is_invert_rx: bool, is_invert_rts: bool, is_invert_cts: bool, send_buffer: *mut c_void, send_buffer_length: u64, receive_buffer: *mut c_void, receive_buffer_length: u64) -> Result;

    /// GetWritableLength
    ///
    /// # Arguments
    /// - `s`: `UartPortSession`
    /// - `out` (out): Output WritableLength.
    pub fn uartPortSessionGetWritableLength(s: *mut UartPortSession, out: *mut u64) -> Result;

    /// Send
    ///
    /// # Arguments
    /// - `s`: `UartPortSession`
    /// - `in_data` (in): Input data buffer.
    /// - `size` (in): Input data buffer size.
    /// - `out` (out): Output size.
    pub fn uartPortSessionSend(s: *mut UartPortSession, in_data: *const c_void, size: usize, out: *mut u64) -> Result;

    /// GetReadableLength
    ///
    /// # Arguments
    /// - `s`: `UartPortSession`
    /// - `out` (out): Output ReadableLength.
    pub fn uartPortSessionGetReadableLength(s: *mut UartPortSession, out: *mut u64) -> Result;

    /// Receive
    ///
    /// # Arguments
    /// - `s`: `UartPortSession`
    /// - `out_data` (out): Output data buffer.
    /// - `size` (in): Output data buffer size.
    /// - `out` (out): Output size.
    pub fn uartPortSessionReceive(s: *mut UartPortSession, out_data: *mut c_void, size: usize, out: *mut u64) -> Result;

    /// BindPortEvent
    ///
    /// # Arguments
    /// - `s`: `UartPortSession`
    /// - `port_event_type` (in): `UartPortEventType`
    /// - `threshold` (in): Threshold
    /// - `out` (out): Output success flag.
    /// - `out_event` (out): Output Event with autoclear=false.
    ///
    /// **Note:** The Event must be closed by the user after using `uartPortSessionUnbindPortEvent`.
    pub fn uartPortSessionBindPortEvent(s: *mut UartPortSession, port_event_type: UartPortEventType, threshold: s64, out: *mut bool, out_event: *mut Event) -> Result;

    /// UnbindPortEvent
    ///
    /// # Arguments
    /// - `s`: `UartPortSession`
    /// - `port_event_type` (in): `UartPortEventType`
    /// - `out` (out): Output success flag.
    pub fn uartPortSessionUnbindPortEvent(s: *mut UartPortSession, port_event_type: UartPortEventType, out: *mut bool) -> Result;
}
