use core::{alloc::GlobalAlloc, ptr::null_mut};



pub struct Dummy;

unsafe impl GlobalAlloc for Dummy {
    unsafe fn alloc(&self, _layout: core::alloc::Layout) -> *mut u8 {
        null_mut()
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        panic!("dealloc is not implemented yet")
    }
}

#[global_allocator]
static ALLOCATOR: Dummy = Dummy;