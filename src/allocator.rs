use core::alloc::{GlobalAlloc, Layout};
use good_memory_allocator::SpinLockedAllocator;

struct NonkernelAllocator {
    internal: SpinLockedAllocator,
}

impl NonkernelAllocator {
    const fn new() -> NonkernelAllocator {
        NonkernelAllocator {
            internal: SpinLockedAllocator::empty(),
        }
    }
}

// This leaks the address of ".data"
// That is, the first GB of the two for this PID
// It's also used at init
static mut CANARY: u64 = 0;

unsafe impl GlobalAlloc for NonkernelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if CANARY == 0 {
            let gb = unsafe { &CANARY } as *const u64 as u64;
            let gb = ((gb >> 30) + 1) << 30; // Get the next one
            self.internal.init(gb as usize, 1 << 29);
            CANARY = 1;
        }

        self.internal.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.internal.dealloc(ptr, layout);
    }
}

#[global_allocator]
static ALLOCATOR: NonkernelAllocator = NonkernelAllocator::new();
