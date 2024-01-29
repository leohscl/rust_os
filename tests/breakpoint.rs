#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use rust_os::hlt_loop;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    rust_os::init();
    test_main();
    hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info);
}

#[test_case]
fn test_println() {
    x86_64::instructions::interrupts::int3();
}
