//! `include/switch/crypto`

#![allow(clippy::module_inception)]

pub mod aes;
pub mod aes_cbc;
pub mod aes_ctr;
pub mod aes_xts;
pub mod cmac;
pub mod crc;
pub mod hmac;
pub mod sha1;
pub mod sha256;

pub use aes::*;
pub use aes_cbc::*;
pub use aes_ctr::*;
pub use aes_xts::*;
pub use cmac::*;
pub use crc::*;
pub use hmac::*;
pub use sha1::*;
pub use sha256::*;
