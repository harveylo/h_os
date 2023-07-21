
use x86_64::{
    structures::paging::PageTable,
    VirtAddr,
};

/// Returns a mutable reference to the active level 4 table.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
pub unsafe fn active_4_level_pagetable(offset: VirtAddr)
    -> &'static mut PageTable
{
    let (pagetable_phy_addr, _) 
            = x86_64::registers::control::Cr3::read();
    let pagetable_vir_addr = offset + pagetable_phy_addr.start_address().as_u64();
    let table_ptr = pagetable_vir_addr.as_mut_ptr() as *mut PageTable;

    &mut *table_ptr
}