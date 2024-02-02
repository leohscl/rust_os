#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]

use core::panic::PanicInfo;

#[allow(unused_imports)]
use rust_os::{println, serial_println, test_panic_handler};

use bootloader::{entry_point, BootInfo};
use x86_64::VirtAddr;

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World !");

    rust_os::init();
    println!("Did not crash");

    let physical_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let addresses_test = [
        0xb8000,
        0x201008,
        0x0100_0020_1a10,
        boot_info.physical_memory_offset,
    ];
    for adress in addresses_test {
        let virt = VirtAddr::new(adress);
        let phys = unsafe { rust_os::memory::translate_addr(virt, physical_mem_offset) };
        println!("{:?} -> {:?}", virt, phys);
    }
    // let level_4_table = unsafe { rust_os::memory::active_level_4_table(physical_mem_offset) };

    // for (i, entry) in level_4_table.iter().enumerate() {
    //     if !entry.is_unused() {
    //         println!("L4 entry: {}, {:?}", i, entry);
    //     }
    // }

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
