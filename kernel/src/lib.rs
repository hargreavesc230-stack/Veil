#![no_std]
#![no_main]

//! Veil kernel crate.
//!
//! The ABI entry (`_start`) lives in `entry.rs` and forwards into `kernel_main`.
//! The Multiboot2 header must reside in the first 32 KiB of the binary, so it
//! is placed in a dedicated section and kept by the linker.

pub mod arch;
mod entry;
mod multiboot;
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
