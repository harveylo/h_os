use linked_list_allocator::LockedHeap;
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

        // map_to function needs a frame_allocator to allocate physical memory
        // for createing page tables if necessary
        unsafe {mapper.map_to(page, frame, flags, frame_allocator)?.flush();}
    }

    // initialize the allocator
    // initialization must happen after mapping all heap pages
    // A allocator must log all those memory the has been allocated to
    // get a valid memory region and deallocate them correctly
    unsafe{
        ALLOCATOR.lock().init(HEAP_START as *mut u8, HEAP_SIZE);
    }
    Ok(())
}

// A allocator is used to allocate memory(Both virtual memory and physical memory) for 
// objects during runtime
#[global_allocator]
// must initialize allocator after this call
// empty() does not initialize the allocator with any necessary information
static ALLOCATOR: LockedHeap = LockedHeap::empty();