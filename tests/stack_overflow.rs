#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

use h_os::{serial_println, exit_qemu, gdt::{self}};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};



#[no_mangle]
pub extern "C" fn _start() -> !{
    serial_println!("Ready to begin STACK OVERFLOW test");
    gdt::init();
    init_test_idt();
    
    overflow();

    serial_println!("Should not reach here..");
    exit_qemu(h_os::QemuExitCode::Failed);
    loop {}
}

// will triger triple fault if stack overflow is not handled properly
#[allow(unconditional_recursion)]
fn overflow() {
    overflow();
    // avoid tail recursion optimization
    volatile::Volatile::new(2).read();
}

#[panic_handler]
fn panic(info : &PanicInfo) -> !{
    h_os::test_panic_handler(info);
}

lazy_static!{
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe{
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _errnum: u64) -> !{
    serial_println!("The DOUBLE PAGE FAULT exception is triggered");
    serial_println!("{:#?}", stack_frame);
    serial_println!("STACK OVERFLOW test ... \x1b[42m[OK]\x1b[0m");
    exit_qemu(h_os::QemuExitCode::Success);
    loop{}
}

fn init_test_idt() {
    IDT.load();
}