//! `include/switch/display`

#![allow(clippy::module_inception)]

pub mod binder;
pub mod buffer_producer;
pub mod framebuffer;
pub mod native_window;
pub mod parcel;
pub mod types;

pub use binder::*;
pub use buffer_producer::*;
pub use framebuffer::*;
pub use native_window::*;
pub use parcel::*;
pub use types::*;
