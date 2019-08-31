#![no_std]

use core::alloc::{GlobalAlloc as A, Layout as L};

pub struct Alleakator<P: A> {
    p: P,
}

type B = *mut u8;

unsafe impl<P: A> A for Alleakator<P> {
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
