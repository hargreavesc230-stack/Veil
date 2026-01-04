// Minimal panic handler.

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // TODO: route panic details to serial/VGA consoles.
    loop {}
}
