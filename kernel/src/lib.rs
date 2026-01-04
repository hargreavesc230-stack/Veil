#![no_std]
#![no_main]

//! Veil kernel crate.
//!
//! The ABI entry (`_start`) lives in `entry.rs` and forwards into `kernel_main`.
//! This separation keeps the Rust-level entry independent from linker details.

pub mod arch;
mod entry;
mod panic;

/// Rust-level entry, separated from the ABI-level `_start`.
///
/// This must never return.
pub fn kernel_main() -> ! {
    // TODO: initialize early subsystems once boot is defined.
    loop {
        crate::arch::x86_64::halt();
    }
}
