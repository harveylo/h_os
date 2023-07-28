#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(h_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use alloc::{boxed::Box, vec::Vec};
use bootloader::{entry_point, BootInfo};
use h_os::{hlt_loop, memory::{self, BootInfoFrameAllocator}, allocator::{self, HEAP_SIZE}};
use x86_64::VirtAddr;

extern crate alloc;


entry_point!(main);

fn main(boot_info: &'static BootInfo) -> !{
    // initialize os
    h_os::init();
    
    // initialize heap
    let offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe{memory::init(offset)};
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("Heap initialization error");
    
    test_main();

    hlt_loop()
}

#[test_case]
fn simple_allocation() {
    let h_1 = Box::new(42);
    let h_2 = Box::new("yes");

    assert_eq!(*h_1, 42);
    assert_eq!(*h_2,"yes");
}

#[test_case]
fn large_vec() {
    let n = 10000;
    let mut v = Vec::new();
    for i in 0..n {
        v.push(i);
    }
    assert_eq!(v.iter().sum::<u64>(), (n-1)*n/2);
}


#[test_case]
fn many_boxes(){
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x,i);
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    h_os::test_panic_handler(info)
}