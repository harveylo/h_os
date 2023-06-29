#![no_std]
#![no_main]
mod vga_buffer;

use core::panic::PanicInfo;

// to be called when panic happens
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop {}
}


#[no_mangle] // no name wrangling
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}
