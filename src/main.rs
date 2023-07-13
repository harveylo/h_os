#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(h_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use h_os::{println, init};

#[no_mangle] // no name wrangling
pub extern "C" fn _start() -> ! {
    println!("Hello, rust os World!");
    init();
    // unsafe{
    //     *(0xdeadbeaf as *mut u8) = 12;
    // }

    // manually invoke a breakpoint interrupt
    // x86_64::instructions::interrupts::int3();


    // conditional compilation
    #[cfg(test)]
    test_main();
    loop {}
}




// to be called when panic happens
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info : &PanicInfo) -> !{
    h_os::test_panic_handler(info);
}