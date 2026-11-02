//! `include/switch/kernel`

#![allow(clippy::module_inception)]

pub mod barrier;
pub mod condvar;
pub mod detect;
pub mod event;
pub mod jit;
pub mod levent;
pub mod mutex;
pub mod random;
pub mod rwlock;
pub mod semaphore;
pub mod shmem;
pub mod svc;
pub mod thread;
pub mod tmem;
pub mod uevent;
pub mod utimer;
pub mod virtmem;
pub mod wait;

pub use barrier::*;
pub use condvar::*;
pub use detect::*;
pub use event::*;
pub use jit::*;
pub use levent::*;
pub use mutex::*;
pub use random::*;
pub use rwlock::*;
pub use semaphore::*;
pub use shmem::*;
pub use svc::*;
pub use thread::*;
pub use tmem::*;
pub use uevent::*;
pub use utimer::*;
pub use virtmem::*;
pub use wait::*;
