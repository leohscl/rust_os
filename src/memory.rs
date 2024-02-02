use x86_64::structures::paging::PageTable;
use x86_64::{PhysAddr, VirtAddr};

use crate::println;

pub unsafe fn translate_addr(addr: VirtAddr, physical_mem_offset: VirtAddr) -> Option<PhysAddr> {
    translate_addr_inner(addr, physical_mem_offset)
}

fn translate_addr_inner(addr: VirtAddr, physical_mem_offset: VirtAddr) -> Option<PhysAddr> {
    use x86_64::registers::control::Cr3;
    use x86_64::structures::paging::page_table::FrameError;

    let (l4_table_frame, _) = Cr3::read();

    let table_indexes = [
        addr.p4_index(),
        addr.p3_index(),
        addr.p2_index(),
        addr.p1_index(),
    ];
    let mut frame = l4_table_frame;

    for &index in &table_indexes {
        let virt = physical_mem_offset + frame.start_address().as_u64();
        let table_ptr: *const PageTable = virt.as_ptr();
        let table = unsafe { &*table_ptr };

        let entry = &table[index];
        frame = match entry.frame() {
            Ok(f) => f,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("Huge frames not supported"),
        }
    }
    Some(frame.start_address() + u64::from(addr.page_offset()))
}

pub unsafe fn active_level_4_table(physical_mem_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_mem_offset + phys.as_u64();
    let page_table_point: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_point
}
