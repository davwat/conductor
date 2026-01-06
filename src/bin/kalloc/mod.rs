// kernel heap allocator
extern crate alloc;

use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

#[global_allocator]
static ALLOCATOR: KernelAllocator = KernelAllocator;

pub struct KernelAllocator;

unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        panic!("dealloc should be never called");
    }
}
