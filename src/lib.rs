#![no_std]

use core::alloc::{GlobalAlloc, Layout as L};

pub struct Alleakator<P: GlobalAlloc> {
    p: P,
}

type B = *mut u8;

unsafe impl<P: GlobalAlloc> GlobalAlloc for Alleakator<P> {
    unsafe fn dealloc(&self, _: B, _: L) {
        //do absolutely nothing
    }
    unsafe fn alloc(&self, a: L) -> B {
        self.p.alloc(a)
    }
    unsafe fn realloc(&self, a: B, b: L, c: usize) -> B {
        self.p.realloc(a, b, c)
    }
    unsafe fn alloc_zeroed(&self, a: L) -> B {
        self.p.alloc_zeroed(a)
    }
}
