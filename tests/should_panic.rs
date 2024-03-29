#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rust_os::{exit_qemu, hlt_loop, serial_println, QemuExitCode};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_panic();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    hlt_loop()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    hlt_loop()
}

fn should_panic() {
    panic!("Panicking !");
}
