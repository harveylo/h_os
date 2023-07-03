#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
mod vga_buffer;
mod serial;

use core::panic::PanicInfo;


// to be called when panic happens
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    println!("{}", info);
    loop {}
}

// conditional compilation
// the following function will only be compiled during tests
#[cfg(test)]
#[panic_handler]
fn panic(info : &PanicInfo) ->! {
    serial_println!("\x1b[41;5m[FAILED]\x1b[0m\n");
    serial_println!("\x1b[1;31m ERROR:\x1b[0m {}\n",info);
    exit_qemu(QemuExitCode::Failed);
    loop{}
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



// this attribute dedicates the function only exists in tests
#[cfg(test)]
// dyn keyword dedicates Trait object
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success);
}

// Add a test case
#[test_case]
fn trivial_assertion(){
    serial_print!("A trivial assertion...    ");
    assert_eq!(0,0);
    // vga_buffer::print_set_color(vga_buffer::Color::White, vga_buffer::Color::Green);
    // vga_buffer::print_resotre_default_color();
    // println!();
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

//? Testable Trait
pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self){
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("\x1b[42m[OK]\x1b[0m");
    }
}