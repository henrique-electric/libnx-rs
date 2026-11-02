//! Rust port of the libnx headers (`include/switch.h` plus the BSD socket headers).
//!
//! Module layout mirrors `include/switch/`, and everything from it is re-exported at the crate root
//! like the C umbrella header. The BSD socket headers (`<sys/socket.h>`, `<netinet/in.h>`, ...), which
//! `switch.h` does not include either, live under [`bsd`]; the devkitA64 newlib types they need are
//! in [`newlib`]. Names are kept identical to libnx so the C documentation applies.
//! `extern` declarations link against `libnx.a`; `static inline` helpers are ported to Rust.
//!
//! Most of this crate is generated from the C headers by clang; see `tools/`.

#![cfg_attr(not(test), no_std)]
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

pub mod applets;
pub mod arm;
pub mod audio;
pub mod bsd;
pub mod crypto;
pub mod display;
pub mod kernel;
pub mod nacp;
pub mod newlib;
pub mod nro;
pub mod nvidia;
pub mod result;
pub mod runtime;
pub mod services;
pub mod sf;
pub mod types;

pub use applets::*;
pub use arm::*;
pub use audio::*;
pub use crypto::*;
pub use display::*;
pub use kernel::*;
pub use nacp::*;
pub use nro::*;
pub use nvidia::*;
pub use result::*;
pub use runtime::*;
pub use services::*;
pub use sf::*;
pub use types::*;

// Lets the tests use `libnx::...` paths like an external user would.
#[cfg(test)]
extern crate self as libnx;
#[cfg(test)]
mod tests;
