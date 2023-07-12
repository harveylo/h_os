#![no_std]

#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

pub mod serial;
pub mod vga_buffer;
pub mod interrupts;
pub mod gdt;



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
    loop{}
}


// Entry Point for `cargo test`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    loop{}
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

pub fn init() {
    interrupts::init_idt();
    gdt::init();
}