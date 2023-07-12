use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::println;
use crate::gdt;

use lazy_static::lazy_static;
lazy_static!{
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        // unsafe because the validity is secured by caller
        unsafe{
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}
pub fn init_idt(){
    IDT.load();
}

// exception handlers
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame){
    println!("BREAKPOINT EXCEPTION CREATED\n{:#?}",stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, error_num: u64) -> !{
    println!("DOUBLE FAULT EXCEPTION TRIGGERED\n{:#?}\n with error_num: {}", stack_frame,error_num);
    // panic!("DOUBLE FAULT EXCEPTION TRIGGERED\n{:#?}\n with error_num: {}", stack_frame,error_num);
    loop {}
}