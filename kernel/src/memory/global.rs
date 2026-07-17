use core::alloc::{GlobalAlloc, Layout};

use heapless::Vec;

use crate::memory::pmm;

struct KernelAllocator;

unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let pages_needed = (size + pmm::PAGE_SIZE - 1) / pmm::PAGE_SIZE;

        match pmm::alloc_cont_pages(pages_needed) {
            Some(phys_addr) => phys_addr as *mut u8,
            None => {
                core::ptr::null_mut() 
            }
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if ptr.is_null() {
            return;
        }

        let size = layout.size();
        let pages_needed = (size + pmm::PAGE_SIZE - 1) / pmm::PAGE_SIZE;

        let start_page = (ptr as usize) / pmm::PAGE_SIZE;

        for p in start_page..(start_page + pages_needed) {
            pmm::free_page(p);
        }
    }
}

#[global_allocator]
static ALLOCATOR: KernelAllocator = KernelAllocator;


