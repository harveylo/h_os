#![no_std]
#![no_main]

use core::panic::PanicInfo;
use h_os::{QemuExitCode, exit_qemu, serial_println, hlt_loop};


#[no_mangle]
pub extern "C" fn _start() -> ! {
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