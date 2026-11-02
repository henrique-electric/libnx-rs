//! `include/switch/runtime`

#![allow(clippy::module_inception)]

pub mod btdev;
pub mod devices;
pub mod diag;
pub mod env;
pub mod hosversion;
pub mod nxlink;
pub mod pad;
pub mod resolver;
pub mod ringcon;
pub mod util;

pub use btdev::*;
pub use devices::*;
pub use diag::*;
pub use env::*;
pub use hosversion::*;
pub use nxlink::*;
pub use pad::*;
pub use resolver::*;
pub use ringcon::*;
pub use util::*;
