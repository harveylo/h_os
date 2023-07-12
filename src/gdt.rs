
use x86_64::VirtAddr;
use x86_64::instructions::tables::load_tss;
use x86_64::registers::segmentation::{CS, Segment};
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use lazy_static::lazy_static;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

// Those initialization progress is done during runtime,
// but static variable must be evaluated during compilation time
// use lazy initialization to save the world
lazy_static!{
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            // allocation mechanism is not implemented yet
            // use static mut to simulate the stack
            // must use mut, otherwise bootloader will allocate this area into read-only page
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            // unsafe is necessary, for compiler taking this competable variable is not safe
            let stack_start = VirtAddr::from_ptr(unsafe {
                &STACK
            });
            let stack_end = stack_start+STACK_SIZE;
            stack_end
        };
        tss
    };
}
lazy_static!{
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        // kernel_code_segment automaticly get the current running kernel code segment descriptor
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selectors{code_selector,tss_selector})
    };
}

pub fn init() {
    GDT.0.load();
    unsafe{
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}