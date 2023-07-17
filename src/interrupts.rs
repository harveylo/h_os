use spin::Mutex;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::hlt_loop;
use crate::print;
use crate::println;
use crate::gdt;
use spin;
use pic8259::ChainedPics;

pub const PIC_1_OFFSET: u8 = 0x20;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET+8;
pub static PICS: spin::Mutex<ChainedPics> = spin::Mutex::new(unsafe {
    ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)
});

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex{
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize{
        usize::from(self as u8)
    }
}

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
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_handler);
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
    hlt_loop()
}

extern "x86-interrupt" fn timer_handler(_stack_frame: InterruptStackFrame){
    // print!(".");
    unsafe{
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn keyboard_handler (_stack_frame: InterruptStackFrame){
    use x86_64::instructions::port::{PortGeneric, ReadWriteAccess, Port};
    // println!("\nScancode from keyboard: {scan_code}");

    use pc_keyboard::{Keyboard,layouts,HandleControl,ScancodeSet1,DecodedKey};
    lazy_static!(
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key,ScancodeSet1>> = Mutex::new(
            Keyboard::new(ScancodeSet1::new(), layouts::Us104Key, HandleControl::Ignore)
        );
    );
    let mut ps2_port: PortGeneric<u8, ReadWriteAccess> = Port::new(0x60);
    let scan_code = unsafe {
        ps2_port.read()
    };

    let mut keyboard = KEYBOARD.lock();

    if let Ok(Some(key_event)) = keyboard.add_byte(scan_code){
        if let Some(key) = keyboard.process_keyevent(key_event){
            match key {
                DecodedKey::Unicode(character) => print!("{character}"),
                DecodedKey::RawKey(key) => print!("{:#?}",key),
            }
        }
    }

    unsafe{
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}