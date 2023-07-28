#![no_std]
#![no_main]

use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};
use h_os::{QemuExitCode, exit_qemu, serial_println, hlt_loop};


entry_point!(main);

fn main(_boot_info: &'static BootInfo) -> ! {
    should_fail();
    serial_println!("test does not panic!");
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    hlt_loop();
}


fn should_fail() {
    use h_os::serial_print;
    serial_print!("should fail here ... ");
    assert_eq!(0, 1);
}