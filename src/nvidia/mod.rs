//! `include/switch/nvidia`

#![allow(clippy::module_inception)]

pub mod address_space;
pub mod channel;
pub mod fence;
pub mod gpu;
pub mod gpu_channel;
pub mod graphic_buffer;
pub mod ioctl;
pub mod map;
pub mod types;

pub use address_space::*;
pub use channel::*;
pub use fence::*;
pub use gpu::*;
pub use gpu_channel::*;
pub use graphic_buffer::*;
pub use ioctl::*;
pub use map::*;
pub use types::*;
