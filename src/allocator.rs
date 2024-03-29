use core::alloc::GlobalAlloc;

use core::ptr::null_mut;

pub mod bump;
pub mod fixed_size;
pub mod linked_list;

use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024;

// use linked_list_allocator::LockedHeap;
// use self::linked_list::LinkedListAllocator;

pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Self {
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}

use fixed_size::FixedSizeAllocator;
#[global_allocator]
static ALLOCATOR: Locked<FixedSizeAllocator> = Locked::new(FixedSizeAllocator::new());
// static ALLOCATOR: Locked<LinkedListAllocator> = Locked::new(LinkedListAllocator::new());
// static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());

struct DummyAllocator;

unsafe impl GlobalAlloc for DummyAllocator {
    unsafe fn alloc(&self, _layout: core::alloc::Layout) -> *mut u8 {
        null_mut()
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        panic!("Should never dealloc !")
    }
}

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE as u64 - 1u64;
        let heap_start_page: Page<Size4KiB> = Page::containing_address(heap_start);
        let heap_end_page: Page<Size4KiB> = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)?.flush();
        }
    }

    unsafe { ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE) };

    Ok(())
}

pub fn align_up(current_adress: usize, align: usize) -> usize {
    (current_adress + align - 1) & !(align - 1)
}
