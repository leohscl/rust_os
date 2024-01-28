#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]

use core::panic::PanicInfo;

use rust_os::{exit_qemu, println, serial_println, test_panic_handler, QemuExitCode};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World !");

    rust_os::init();

    println!("Did not crash");
    serial_println!("Did not crash");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("{}", info);
    println!("{}", info);
    loop {}
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
