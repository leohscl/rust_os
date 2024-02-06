#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]

extern crate alloc;
use alloc::boxed::Box;
use core::panic::PanicInfo;
use x86_64::structures::paging::{Page, Translate};

use rust_os::{allocator, memory};
#[allow(unused_imports)]
use rust_os::{println, serial_println, test_panic_handler};

use bootloader::{entry_point, BootInfo};
use x86_64::VirtAddr;

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    let physical_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut offset_page_table = unsafe { memory::init(physical_mem_offset) };

    println!("Hello World !");
    rust_os::init();

    let addresses_test = [
        0xb8000,
        0x201008,
        0x0100_0020_1a10,
        boot_info.physical_memory_offset,
    ];
    for adress in addresses_test {
        let virt = VirtAddr::new(adress);
        let phys = offset_page_table.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }
    // let virtual_page = Page::from_start_address(VirtAddr::new(0)).unwrap();
    // let virtual_page = Page::from_start_address(VirtAddr::new(0xdeadbeaf000)).unwrap();
    // rust_os::memory::create_example_mapping(
    //     virtual_page,
    //     &mut offset_page_table,
    //     &mut boot_info_allocator,
    // );
    // let mut dummy_allocator = memory::EmptyFrameAllocator;
    // let page_ptr: *mut u64 = virtual_page.start_address().as_mut_ptr();
    // unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };
    let mut boot_info_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut offset_page_table, &mut boot_info_allocator)
        .expect("heap init failed");

    let x = Box::new(42);
    println!("x does not crash ! x = {}", x);

    rust_os::hlt_loop()
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
