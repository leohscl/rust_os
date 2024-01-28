#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use lazy_static::lazy_static;

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use rust_os::serial_print;
use rust_os::serial_println;

use rust_os::exit_qemu;
use rust_os::QemuExitCode;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("Testing double fault exception...\t");

    rust_os::gdt::init();
    TEST_IDT.load();
    // rust_os::init();

    stack_overflow();
    // panic!("Execution continued after stack overflow");
    loop {}
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
    volatile::Volatile::new(0).read();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info);
}

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(rust_os::gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
