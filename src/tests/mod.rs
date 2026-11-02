//! Host-side checks (`cargo +stable test --lib --target x86_64-unknown-linux-gnu`).
//!
//! These are unit tests of the library rather than integration tests so that the Switch-only
//! binary in `src/main.rs` is never built for the host.

mod generated_layout;
mod inline_ports;
