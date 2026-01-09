// kernel heap allocator

extern crate alloc;
use alloc::alloc::{GlobalAlloc, Layout};

#[global_allocator]
static ALLOCATOR: KernelAllocator = KernelAllocator::new(0x4087_D7FF, 10 * 1024);

pub struct KernelAllocator {
    heap: Heap,
}

impl KernelAllocator {
    pub const fn new(start: usize, size: usize) -> Self {
        Self {
            heap: Heap::new(start, size),
        }
    }
}

use crate::drivers::uart;
unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        uart::write_word(67);
        return self.heap.head_free.block_free_start_addr() as *mut u8;
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        todo!()
    }
}

pub struct Heap {
    pub start: usize,
    pub size: usize,
    pub head_free: FreeBlock, // todo: possibly make private not sure
}

impl Heap {
    pub const fn new(start: usize, size: usize) -> Self {
        Self {
            start: start,
            size: size,
            head_free: FreeBlock::new(size),
        }
    }
}

// linked list of freeblocks makes up heap
pub struct FreeBlock {
    size: usize,
    next: Option<&'static mut FreeBlock>,
}

impl FreeBlock {
    pub const fn new(size: usize) -> Self {
        Self {
            size: size,
            next: None,
        }
    }

    pub fn block_struct_start_addr(&self) -> usize {
        self as *const Self as usize
    }

    // returns the address of the first free byte out of this block.
    // since freeblocks are physically positioned at the beginning of the actual free space,
    // this will simply be the beginning of the freeblock struct + the size of the freeblock struct
    pub fn block_free_start_addr(&self) -> usize {
        self.block_struct_start_addr() + size_of::<Self>()
    }
}
