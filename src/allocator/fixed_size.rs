use core::{
    alloc::GlobalAlloc,
    mem,
    ptr::{null_mut, NonNull},
};
use linked_list_allocator::Heap;

use super::Locked;

const BLOCK_SIZES: [usize; 8] = [16, 32, 64, 128, 256, 512, 1024, 2048];

struct ListNode {
    next: Option<&'static mut ListNode>,
}

pub struct FixedSizeAllocator {
    heads: [Option<&'static mut ListNode>; 8],
    fallback_allocator: Heap,
}
impl FixedSizeAllocator {
    pub const fn new() -> FixedSizeAllocator {
        const EMPTY: Option<&'static mut ListNode> = None;
        FixedSizeAllocator {
            heads: [EMPTY; 8],
            fallback_allocator: Heap::empty(),
        }
    }

    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.fallback_allocator.init(heap_start, heap_size);
    }

    unsafe fn fallback_alloc(&mut self, layout: core::alloc::Layout) -> *mut u8 {
        match self.fallback_allocator.allocate_first_fit(layout) {
            Ok(ptr) => ptr.as_ptr(),
            Err(_) => null_mut(),
        }
    }
}

fn list_index(layout: &core::alloc::Layout) -> Option<usize> {
    let required_block_size = layout.size().max(layout.align());
    BLOCK_SIZES.iter().position(|&s| s >= required_block_size)
}

unsafe impl GlobalAlloc for Locked<FixedSizeAllocator> {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let mut allocator = self.lock();
        if let Some(index_block) = list_index(&layout) {
            match allocator.heads[index_block].take() {
                Some(node) => {
                    allocator.heads[index_block] = node.next.take();
                    (node as *mut ListNode) as *mut u8
                }
                None => {
                    let block_size = BLOCK_SIZES[index_block];
                    let block_align = block_size;
                    let layout =
                        core::alloc::Layout::from_size_align(block_size, block_align).unwrap();
                    allocator.fallback_alloc(layout)
                }
            }
        } else {
            allocator.fallback_alloc(layout)
        }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        let mut allocator = self.lock();
        if let Some(index_block) = list_index(&layout) {
            let new_node = ListNode {
                next: allocator.heads[index_block].take(),
            };

            assert!(mem::size_of::<ListNode>() <= BLOCK_SIZES[index_block]);
            assert!(mem::align_of::<ListNode>() <= BLOCK_SIZES[index_block]);
            let new_node_ptr = ptr as *mut ListNode;
            new_node_ptr.write(new_node);
            allocator.heads[index_block] = Some(&mut *new_node_ptr);
        } else {
            let ptr = NonNull::new(ptr).unwrap();
            allocator.fallback_allocator.deallocate(ptr, layout);
        }
    }
}
