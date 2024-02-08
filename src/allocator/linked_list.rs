use core::{alloc::GlobalAlloc, mem, ptr::null_mut};

struct ListNode {
    size: usize,
    next: Option<&'static mut ListNode>,
}

impl ListNode {
    const fn new(size: usize) -> Self {
        ListNode { size, next: None }
    }

    fn start_addr(&self) -> usize {
        self as *const Self as usize
    }

    fn end_addr(&self) -> usize {
        self.start_addr() + self.size
    }
}

pub struct LinkedListAllocator {
    head: ListNode,
}

impl LinkedListAllocator {
    pub const fn new() -> Self {
        Self {
            head: ListNode::new(0),
        }
    }

    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.add_free_region(heap_start, heap_size);
    }

    unsafe fn add_free_region(&mut self, addr: usize, size: usize) {
        assert_eq!(align_up(addr, mem::align_of::<ListNode>()), addr);
        assert!(size >= mem::size_of::<ListNode>());

        let mut node = ListNode::new(size);
        node.next = self.head.next.take();
        let node_ptr = addr as *mut ListNode;
        node_ptr.write(node);
        self.head.next = Some(&mut *node_ptr);
    }

    fn find_region(&mut self, size: usize, align: usize) -> Option<(&'static mut ListNode, usize)> {
        let mut node = &mut self.head;
        while let Some(ref mut region) = node.next {
            if let Ok(alloc_start) = Self::alloc_from_region(&region, size, align) {
                let next = region.next.take();
                let ret = (node.next.take().unwrap(), alloc_start);
                node.next = next;
                return Some(ret);
            } else {
                node = node.next.as_mut().unwrap();
            }
        }
        None
    }

    fn alloc_from_region(node: &ListNode, size: usize, align: usize) -> Result<usize, ()> {
        let alloc_start = align_up(node.start_addr(), align);
        let alloc_end = alloc_start.checked_add(size).ok_or(())?;

        if node.end_addr() < alloc_end {
            return Err(());
        }
        let excess_size = node.end_addr() - alloc_end;
        if excess_size > 0 && excess_size < mem::size_of::<ListNode>() {
            return Err(());
        }
        Ok(alloc_start)
    }

    fn size_align(layout: core::alloc::Layout) -> (usize, usize) {
        let layout = layout
            .align_to(mem::align_of::<ListNode>())
            .expect("adjusting alignment failed")
            .pad_to_align();
        let size = layout.size().max(mem::size_of::<ListNode>());
        (size, layout.align())
    }
}

use super::{align_up, Locked};
unsafe impl GlobalAlloc for Locked<LinkedListAllocator> {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let mut linked = self.lock();
        let (size, align) = LinkedListAllocator::size_align(layout);
        if let Some((region, start)) = linked.find_region(size, align) {
            let alloc_end = region.start_addr() + size;
            let excess = region.end_addr() - alloc_end;
            if excess > 0 {
                linked.add_free_region(alloc_end, excess);
            }
            start as *mut u8
        } else {
            null_mut()
        }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        let (size, _) = LinkedListAllocator::size_align(layout);
        self.lock().add_free_region(ptr as usize, size);
    }
}
