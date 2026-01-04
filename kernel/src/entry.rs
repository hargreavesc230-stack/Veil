//! Kernel entry symbol.
//!
//! The linker script sets `_start` as the entry point; this file bridges the
//! ABI-level entry into Rust code.

use crate::kernel_main;

/// ABI entry point for the kernel.
///
/// # Safety and ABI
///
/// - Must use the C ABI because the bootloader jumps to a raw symbol.
/// - Assumes a valid stack has been set up by the loader.
/// - Must never return to the caller.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    kernel_main()
}
