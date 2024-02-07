#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;

use rust_os::hlt_loop;
use rust_os::serial_print;

use bootloader::{entry_point, BootInfo};
use rust_os::{allocator, memory};
use x86_64::VirtAddr;

entry_point!(main);

#[no_mangle]
fn main(boot_info: &'static BootInfo) -> ! {
    rust_os::init();
    let physical_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut offset_page_table = unsafe { memory::init(physical_mem_offset) };
    let mut boot_info_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut offset_page_table, &mut boot_info_allocator)
        .expect("heap init failed");

    serial_print!("Testing heap allocation...");
    test_main();
    hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info);
}

use alloc::boxed::Box;

#[test_case]
fn simple_allocation() {
    let heap_value_1 = Box::new(2);
    let heap_value_2 = Box::new(3);
    assert_eq!(*heap_value_1, 2);
    assert_eq!(*heap_value_2, 3);
}

use alloc::vec::Vec;

#[test_case]
fn large_allocation() {
    let mut vec_basic = Vec::new();
    let n = 500;
    for x in 0..n {
        vec_basic.push(x);
    }
    assert_eq!(vec_basic.iter().sum::<u64>(), n * (n - 1) / 2);
}

use rust_os::allocator::HEAP_SIZE;
#[test_case]
fn many_boxes() {
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}

#[test_case]
fn many_boxes_long_lived() {
    let long_lived = Box::new(1);
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
    assert_eq!(*long_lived, 1);
}
