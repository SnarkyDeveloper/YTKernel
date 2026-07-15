use crate::serial::print_u32;


// 4kb page size
pub const PAGE_SIZE: usize = 4096;
pub const EFI_CONVENTIONAL_MEMORY: u32 = 7;
const BITMAP_SIZE: usize = 16 * 1024 * 1024;
static mut BITMAP: [u8; BITMAP_SIZE] = [0; BITMAP_SIZE];

#[repr(C)]
pub struct EfiMemoryDescriptor {
    pub ty: u32,
    pub pad: u32,
    pub physical_start: u64,
    pub virtual_start: u64,
    pub number_of_pages: u64,
    pub attribute: u64,
}

pub unsafe fn init(info: &crate::BootInfo) {
    for i in 0..BITMAP_SIZE {
        BITMAP[i] = 0xFF;
    }

    let mut ptr = info.memory_map as usize;
    let end = ptr + info.memory_map_size;

    while ptr < end {
        let desc = &*(ptr as *const EfiMemoryDescriptor);

        if desc.ty == EFI_CONVENTIONAL_MEMORY {
            let start = desc.physical_start as usize / PAGE_SIZE;
            let end_page = start + desc.number_of_pages as usize;

            for page in start..end_page {
                mark_free(page);
            }
        }

        ptr += info.descriptor_size;
    }
}

pub unsafe fn reserve(start: usize, end: usize) {
    let first = start / PAGE_SIZE;
    let last = (end + PAGE_SIZE - 1) / PAGE_SIZE;

    for page in first..last {
        mark_used(page);
    }
}

pub unsafe fn alloc_page() -> Option<usize> {
    let bytes = BITMAP_SIZE;

    for byte in 0..bytes {
        if BITMAP[byte] != 0xFF {
            for bit in 0..8 {
                let page = byte * 8 + bit;

                if !is_used(page) {
                    mark_used(page);
                    return Some(page * PAGE_SIZE);
                }
            }
        }
    }

    None
}

pub unsafe fn free_page(addr: usize) {
    mark_free(addr / PAGE_SIZE);
}

unsafe fn mark_used(page: usize) {
    BITMAP[page / 8] |= 1 << (page % 8);
}

unsafe fn mark_free(page: usize) {
    BITMAP[page / 8] &= !(1 << (page % 8));
}

unsafe fn is_used(page: usize) -> bool {
    (BITMAP[page / 8] & (1 << (page % 8))) != 0
}
