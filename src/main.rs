#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// This function is called on panic and is required to build.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// use C calling convention
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
