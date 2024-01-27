#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]

use core::panic::PanicInfo;

use rust_os::{println, test_panic_handler};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // println!("Hello World !");

    rust_os::init();

    println!("Did not crash");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info);
}
