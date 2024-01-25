#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use rust_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // println!("Hello World !");

    rust_os::init();

    x86_64::instructions::interrupts::int3();

    fn stack_overflow() {
        stack_overflow();
    }

    stack_overflow();

    // trigger a page fault
    // unsafe {
    //     *(0xdeadbeef as *mut u8) = 42;
    // }

    #[cfg(test)]
    test_main();

    println!("Did not crash");

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}
