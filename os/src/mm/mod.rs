//! Memory management implementation
//!
//! SV39 page-based virtual-memory architecture for RV64 systems, and
//! everything about memory management, like frame allocator, page table,
//! map area and memory set, is implemented here.
//!
//! Every task or process has a memory_set to control its virtual memory.

mod address;
mod frame_allocator;
mod heap_allocator;
mod memory_set;
mod page_table;

use address::VPNRange;
pub use address::{PhysAddr, PhysPageNum, StepByOne, VirtAddr, VirtPageNum};
use alloc::string::String;
pub use frame_allocator::{frame_alloc, frame_dealloc, FrameTracker};
pub use memory_set::remap_test;
pub use memory_set::{kernel_token, MapPermission, MemorySet, KERNEL_SPACE};
use page_table::PTEFlags;
pub use page_table::{
    translated_byte_buffer, translated_ref, translated_refmut, translated_str, PageTable,
    PageTableEntry, UserBuffer, UserBufferIterator,
};

use crate::task::current_task;

/// initiate heap allocator, frame allocator and kernel space
pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.exclusive_access().activate();
}

/// map memory
pub fn memory_map(
    start_va: VirtAddr,
    end_va: VirtAddr,
    map_perm: MapPermission,
) -> Result<(), String> {
    let current_task = current_task().unwrap();
    let mut inner = current_task.inner_exclusive_access();
    inner
        .memory_set
        .insert_framed_area_result(start_va, end_va, map_perm)
}

/// unmap memory
pub fn memory_unmap(start_va: VirtAddr, end_va: VirtAddr) -> Result<(), String> {
    let current_task = current_task().unwrap();
    let mut inner = current_task.inner_exclusive_access();
    inner.memory_set.remove_area_result(start_va, end_va)
}
