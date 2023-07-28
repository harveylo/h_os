
use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::{
    structures::paging::{PageTable, page_table::FrameError, OffsetPageTable, Page, FrameAllocator, Size4KiB, PhysFrame, Mapper, },
    VirtAddr, PhysAddr,
};

/// Initialize a new OffsetPageTable.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
pub unsafe fn init(phy_addr_offset : VirtAddr) -> OffsetPageTable<'static> {
    let lv4_page_table = active_4_level_pagetable(phy_addr_offset);
    OffsetPageTable::new(lv4_page_table, phy_addr_offset)
}



/// Returns a mutable reference to the active level 4 table.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
unsafe fn active_4_level_pagetable(offset: VirtAddr)
    -> &'static mut PageTable
{
    let (pagetable_phy_addr, _) 
            = x86_64::registers::control::Cr3::read();
    let pagetable_vir_addr = offset + pagetable_phy_addr.start_address().as_u64();
    let table_ptr = pagetable_vir_addr.as_mut_ptr() as *mut PageTable;

    &mut *table_ptr
}




/// Allocate Physical Memory
/// FrameAllocator that returns usable frames from the bootloader's memory map
pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    /// Returns an iterator over the usable frames specified in the memory map.
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        //get all regions frrom the memory map
        let regions = self.memory_map.iter();
        let usable_regions = regions
            .filter(|r| r.region_type == MemoryRegionType::Usable);
        
        // map each region to its address range
        let addr_ranges = usable_regions
            .map(|r| r.range.start_addr()..r.range.end_addr());
        
        //transform to an iterator of frame start addresses
        let fram_addresses = addr_ranges
            .flat_map(|r| r.step_by(4096));
        
        // create PhysFrame types from the start addresses
        fram_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }

    /// Create a FrameAllocator from the passed memory map.
    ///
    /// This function is unsafe because the caller must guarantee that the passed
    /// memory map is valid. The main requirement is that all frames that are marked
    /// as `USABLE` in it are really unused.
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator { memory_map: (memory_map), next: (0) }
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}




// deprecated structures and functions

/// Translates the given virtual address to the mapped physical address, or
/// `None` if the address is not mapped.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `memory_offset`.
#[deprecated]
#[allow(deprecated)]
pub unsafe fn translate_addr_v2p(virt_addr : VirtAddr, memory_offset: VirtAddr) -> Option<PhysAddr> {
    translate_addr_inner_v2p(virt_addr,memory_offset)
}

/// Private function that is called by `translate_addr_v2p`.
///
/// This function is safe to limit the scope of `unsafe` because Rust treats
/// the whole body of unsafe functions as an unsafe block. This function must
/// only be reachable through `unsafe fn` from outside of this module.
#[deprecated]
fn translate_addr_inner_v2p(virt_addr : VirtAddr, memory_offset: VirtAddr) -> Option<PhysAddr> {
    let (lv4_table_frame, _) = x86_64::registers::control::Cr3::read();
    
    let mut cur_frame = lv4_table_frame;

    let indexes = [
        virt_addr.p4_index(),virt_addr.p3_index(),virt_addr.p2_index(),virt_addr.p1_index()
    ];

    for index in indexes {
        let table_vir_addr = memory_offset + cur_frame.start_address().as_u64();
        let table_ptr = table_vir_addr.as_mut_ptr();
        let table: &PageTable = unsafe{&*table_ptr};

        let entry = &table[index];

        cur_frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("Huge Page not Supported"),
        }
    }

    Some(cur_frame.start_address() + u64::from(virt_addr.page_offset()))
}

#[deprecated]
pub struct EmptyFrameAllocator;
#[allow(deprecated)]
unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        None
    }
}

#[deprecated]
pub fn create_mapping_to_vga(
    page: Page, 
    mapper: &mut OffsetPageTable, 
    frame_allocator: & mut impl FrameAllocator<Size4KiB>)
{
    use x86_64::structures::paging::PageTableFlags as Flags;
    let frame: PhysFrame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flag = Flags::PRESENT | Flags::WRITABLE;

    let map_to_result = unsafe {
        mapper.map_to(page, frame, flag, frame_allocator)
    };
    map_to_result.expect("map_to failed").flush();
}