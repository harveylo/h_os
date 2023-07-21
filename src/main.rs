#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(h_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};
use h_os::{println, init, memory, };
use x86_64::VirtAddr;


// kernal main function is called outside kernal
// thus no signature checking is performed, use following macro to create an entry point
// wuth signature checking
entry_point!(kernel_main);

// #[no_mangle] // no name wrangling
pub  fn kernel_main(boot_into: &'static BootInfo) -> ! {
    println!("Hello, rust os World!");
    init();

    // If the print function does not turn off the interrupts,
    // the following statement may trigger deadlock
    // loop {
    //     for _i in 1..10000 {}
    //     print!("-");
    // }
    // unsafe{
    //     *(0xdeadbeaf as *mut u8) = 12;
    // }

    // manually invoke a breakpoint interrupt
    // x86_64::instructions::interrupts::int3();

    let offset = VirtAddr::new(boot_into.physical_memory_offset);
    let l4_table = unsafe { memory::active_4_level_pagetable(offset) };
    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);
        }
    }


    // conditional compilation
    #[cfg(test)]
    test_main();
    h_os::hlt_loop();
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