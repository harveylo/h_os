#![no_std]

#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

// `alloc` crate ships with Rust compiler as part of std library
// so no need to add this dependency in Cargo.toml
// add extern crate statement, specifying that the compiler should try to include it
// should add alloc to build-std lsit too
extern crate alloc;

use core::panic::PanicInfo;

#[cfg(test)]
use bootloader::{entry_point,BootInfo};

pub mod serial;
pub mod vga_buffer;
pub mod interrupts;
pub mod gdt;
pub mod memory;
pub mod allocator;
// mod pit_8254;


pub fn init() {
    interrupts::init_idt();
    gdt::init();
    unsafe{
        interrupts::PICS.lock().initialize();
        // pit_8254::PIT::new(1).init();
    }
    x86_64::instructions::interrupts::enable();
}

//? Testable Trait
pub trait Testable {
    fn run(&self) -> ();
}

// generic implementation
// implemented the Testable trait for all types that has Fn() Trait
impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self){
        serial_print!("{}...\t", core::any::type_name::<T>());
        // For the self object has implemented the Fn trait, it can be called by using just ()
        self();
        serial_println!("\x1b[42m[OK]\x1b[0m");
    }
}

// dyn keyword dedicates Trait object
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("\x1b[41;5m[FAILED]\x1b[0m\n");
    serial_println!("\x1b[1;31m ERROR:\x1b[0m {}\n",info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

#[cfg(test)]
entry_point!(kernel_test_main);
// Entry Point for `cargo test`
#[cfg(test)]
pub fn kernel_test_main(_boot_info: &'static BootInfo) -> ! {


    init();
    test_main();
    hlt_loop();
}

#[test_case]
fn test_breakpoint(){
    x86_64::instructions::interrupts::int3();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    test_panic_handler(info)
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode){
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

// Using hlt instruction can make the cpu sleep until next interrupt
// So it's a little more eco-friedly than merely a infinite loop
pub fn hlt_loop() -> !{
    loop{
        x86_64::instructions::hlt();
    }
}