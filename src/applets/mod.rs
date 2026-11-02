//! `include/switch/applets`

#![allow(clippy::module_inception)]

pub mod album_la;
pub mod error;
pub mod friends_la;
pub mod hid_la;
pub mod libapplet;
pub mod mii_la;
pub mod nfp_la;
pub mod nifm_la;
pub mod pctlauth;
pub mod psel;
pub mod swkbd;
pub mod web;

pub use album_la::*;
pub use error::*;
pub use friends_la::*;
pub use hid_la::*;
pub use libapplet::*;
pub use mii_la::*;
pub use nfp_la::*;
pub use nifm_la::*;
pub use pctlauth::*;
pub use psel::*;
pub use swkbd::*;
pub use web::*;
