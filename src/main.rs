#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]

use core::panic::PanicInfo;

use rust_os::hlt_loop;
#[allow(unused_imports)]
use rust_os::{println, serial_println, test_panic_handler};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World !");

    rust_os::init();

    println!("Did not crash");
    serial_println!("Did not crash");
    let ptr = 0x2052bc as *mut u8;
    let res = unsafe { *ptr };
    println!("{}", res);
    println!("Read worked !");
    unsafe {
        *ptr = 42;
    }
    println!("Write worked !");
    hlt_loop()
    // exit_qemu(QemuExitCode::Success);
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
    rust_os::hlt_loop()
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
