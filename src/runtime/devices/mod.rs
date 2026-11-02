//! `include/switch/runtime/devices`

#![allow(clippy::module_inception)]

pub mod console;
pub mod fs_dev;
pub mod romfs_dev;
pub mod socket;
pub mod usb_comms;

pub use console::*;
pub use fs_dev::*;
pub use romfs_dev::*;
pub use socket::*;
pub use usb_comms::*;
