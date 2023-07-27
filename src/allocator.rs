use core::{alloc::GlobalAlloc, ptr::null_mut};

use x86_64::{structures::paging::{Mapper, Size4KiB, FrameAllocator, mapper::MapToError, Page, PageTableFlags}, VirtAddr};


pub const HEAP_START: usize = 0x4242_4242_0000;
pub const HEAP_SIZE: usize = 1024*1024; // 1MB, increase this if needed


pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) ->  Result<(), MapToError<Size4KiB>>{
    let heap_start_addr = VirtAddr::new(HEAP_START as u64);
    let heap_end_addr = VirtAddr::new((HEAP_START+HEAP_SIZE-1usize)as u64);
    let start_page = Page::containing_address(heap_start_addr);
    let end_page = Page::containing_address(heap_end_addr);

    let page_range = Page::range_inclusive(start_page, end_page);

    for page in page_range {
        let frame = frame_allocator.allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;

        let flags = PageTableFlags::WRITABLE | PageTableFlags::PRESENT;
        unsafe {mapper.map_to(page, frame, flags, frame_allocator)?.flush();}
    }
    Ok(())
}

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