#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(h_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
mod vga_buffer;
mod serial;

use core::panic::PanicInfo;
use h_os::println;


// to be called when panic happens
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    println!("{}", info);
    loop {}
}


#[no_mangle] // no name wrangling
pub extern "C" fn _start() -> ! {
    println!("Hello, rust os World!");
    vga_buffer::print_heart();
    vga_buffer::print_hollow_smile();
    println!("this is {}", 3/5);
    serial_println!(3);
    // conditional compilation
    #[cfg(test)]
    test_main();
    loop {}
}




// Add test cases
#[test_case]
fn trivial_assertion(){
    assert_eq!(0,0);
    // vga_buffer::print_set_color(vga_buffer::Color::White, vga_buffer::Color::Green);
    // vga_buffer::print_resotre_default_color();
    // println!();
}



