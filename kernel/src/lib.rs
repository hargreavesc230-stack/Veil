#![no_std]

// Kernel crate root; boot entry will be wired later.

pub mod arch;
mod panic;

pub fn kernel_main() {
    // TODO: initialize early subsystems once boot is defined.
}
