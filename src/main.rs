#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(h_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};
use h_os::{println, init, memory::{self}, };
use x86_64::{VirtAddr, structures::paging::Page};


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
    // let l4_table = unsafe { memory::active_4_level_pagetable(offset) };
    // for (i, entry) in l4_table.iter().enumerate() {
    //     if !entry.is_unused() {
    //         println!("L4 Entry {}: {:?}", i, entry);
    //         let l3phy = entry.addr();
    //         let l3vir = offset + l3phy.as_u64();
    //         let l3tptr = l3vir.as_mut_ptr() as *mut PageTable;
    //         let l3table = unsafe {&mut *l3tptr};
    //         for (j, l3entry) in l3table.iter().enumerate(){
    //             if !l3entry.is_unused(){
    //                 println!("L3 Entry {} from L4 entry {}: {:?}",j,i,l3entry);
    //             }
    //         }
    //     }
    // }
    let mut mapper = unsafe{memory::init(offset)};
    
    // let addreses = [
    //     // VGA identity mapping
    //     0xb8000,
    //     // some code page
    //     0x201008,
    //     // some stack page
    //     0x0100_0020_1a10,
    //     // virtual address mapped to physical address 0
    //     boot_into.physical_memory_offset,
    // ];

    // for address in addreses {
    //     let vir_addr = VirtAddr::new(address);
    //     let phy_addr = mapper.translate_addr(vir_addr);
    //     println!("{:?} -> {:?}", vir_addr, phy_addr);
    // }

    let mut frame_allocator = memory::EmptyFrameAllocator;

    let page = Page::containing_address(VirtAddr::new(0));
    
    memory::create_mapping_to_vga(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe{ page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

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