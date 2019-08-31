use core::alloc::{GlobalAlloc, Layout};

pub struct Alleakator<PA: GlobalAlloc> {
    parent_allocator: PA,
}

unsafe impl<PA: GlobalAlloc> GlobalAlloc for Alleakator<PA> {
    unsafe fn dealloc(&self, _: *mut u8, _: Layout) {
        //do absolutely nothing
    }
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.parent_allocator.alloc(layout)
    }
    unsafe fn realloc(&self) {}
    unsafe fn alloc_zeroed(&self)
}