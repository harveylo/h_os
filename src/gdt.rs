use x86_64::VirtAddr;
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
            // Stack allocation mechanism is not implemented yet
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
