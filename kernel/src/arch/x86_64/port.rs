//! Port I/O helpers for x86_64.
//!
//! Port access touches hardware directly and is inherently unsafe.

use core::arch::asm;

/// Read a byte from an I/O port.
///
/// # Safety
///
/// The caller must ensure the port is valid and the device access is safe.
pub unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    // Safety: the caller guarantees the port access is valid.
    unsafe {
        asm!(
            "in al, dx",
            in("dx") port,
            out("al") value,
            options(nomem, nostack, preserves_flags)
        );
    }
    value
}

/// Write a byte to an I/O port.
///
/// # Safety
///
/// The caller must ensure the port is valid and the device access is safe.
pub unsafe fn outb(port: u16, value: u8) {
    // Safety: the caller guarantees the port access is valid.
    unsafe {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") value,
            options(nomem, nostack, preserves_flags)
        );
    }
}
