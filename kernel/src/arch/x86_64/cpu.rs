//! CPU-level helpers for x86_64.
//!
//! These wrap privileged instructions and must be used with care.

use core::arch::asm;

/// Halt the current CPU until the next interrupt.
pub fn halt() {
    // Safety: requires privileged execution (CPL0).
    unsafe {
        asm!("hlt", options(nomem, nostack, preserves_flags));
    }
}

/// Enable maskable interrupts.
pub fn enable_interrupts() {
    // Safety: requires privileged execution (CPL0).
    unsafe {
        asm!("sti", options(nomem, nostack, preserves_flags));
    }
}

/// Disable maskable interrupts.
pub fn disable_interrupts() {
    // Safety: requires privileged execution (CPL0).
    unsafe {
        asm!("cli", options(nomem, nostack, preserves_flags));
    }
}
