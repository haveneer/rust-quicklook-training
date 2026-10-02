// Not part of the cargo workspace: a #[panic_handler] cannot coexist with std.
// Check it with: rustc --edition 2021 --crate-type lib -C panic=abort no_std/panic_handler.rs
#![no_std]

use core::panic::PanicInfo;

// Exactly one per final binary, mandatory as soon as std is absent (embedded, kernel, ...)
#[panic_handler]
fn on_panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        log_to_uart(location.file(), location.line()); // no println! without std
    }
    loop {} // never returns: halt, reset the board, blink a LED...
}

fn log_to_uart(_file: &str, _line: u32) {
    // write to a serial port register
}
