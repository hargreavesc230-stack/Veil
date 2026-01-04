//! Kernel panic handler.
//!
//! This is intentionally minimal until early I/O exists.

use core::panic::PanicInfo;

/// Kernel panic handler.
///
/// # Safety and ABI
///
/// The panic handler must never return. It is a last-resort path when the
/// kernel state is unknown.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // TODO: route panic details to serial/VGA consoles.
    loop {}
}
