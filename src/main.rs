#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
mod vga_buffer;

use core::panic::PanicInfo;


// to be called when panic happens
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
    // conditional compilation
    #[cfg(test)]
    test_main();
    loop {}
}



// this attribute dedicates the function only exists in tests
#[cfg(test)]
// dyn keyword dedicates Trait object
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}