#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(h_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;

use alloc::boxed::Box;
use bootloader::{BootInfo, entry_point,};
use h_os::{println, init, memory::{self, BootInfoFrameAllocator}, allocator, };
use x86_64::VirtAddr;


// kernal main function is called outside kernal
// thus no signature checking is performed, use following macro to create an entry point
// wuth signature checking
entry_point!(kernel_main);

// #[no_mangle] // no name wrangling
pub  fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello, rust os World!");
    init();

    let offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe{memory::init(offset)};
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("Heap initialization failed");

    let _x  = Box::new([1,2,3]);

    println!("Here it is");

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