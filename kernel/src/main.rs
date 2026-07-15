#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]

mod serial;
mod renderer;
mod memory;

use core::panic::PanicInfo;

use serial::*;
use renderer::*;

use crate::memory::pmm;

#[repr(C)]
pub struct BootInfo {
    pub fb_base: *mut u32,      // +0
    pub fb_width: u32,          // +8
    pub fb_height: u32,         // +12
    pub fb_stride: u32,         // +16
    pub fb_format: u32,         // +20

    pub memory_map: *const u8,  // +24
    pub memory_map_size: usize, // +32
    pub descriptor_size: usize, // +40
    pub descriptor_version: u32, // +48

    pub kernel_start: u64,       // +52
    pub kernel_end: u64,         // +60
}

fn todos() {
    todo!("Write network stack - eh (AMD PCNet & rtl8139 later)");
    todo!("Add tls stack here");
    todo!("Add m3u8 parser (Should be easy)");
    todo!("Add video player - complex");
    todo!("Audio drivers - complex")
}

static IMAGE: &[u8] = include_bytes!("../assets/catgirl.png");

pub unsafe fn dump_memory_map(info: &BootInfo) {
    let mut ptr = info.memory_map;

    let end = info.memory_map.add(info.memory_map_size);

    while ptr < end {
        let desc = &*(ptr as *const memory::pmm::EfiMemoryDescriptor);

        print_u32("\ntype: ", desc.ty);
        // print_u32("pages low: ", desc.number_of_pages as u32);

        ptr = ptr.add(info.descriptor_size);
    }
}

#[unsafe(no_mangle)]
pub extern "sysv64" fn kernel_main(info: &'static BootInfo) -> ! {
    unsafe { 
        init_serial(); 
    }
    // print_u32("main size=", info.memory_map_size as u32);
    // print_u32("kernel start: ", info.kernel_start as u32);
    // print_u32("kernel end: ", info.kernel_end as u32);
    // unsafe { dump_memory_map(info); }
    unsafe {
        pmm::init(info);
        pmm::reserve(info.kernel_start as usize, info.kernel_end as usize);
        if let Some(page) = pmm::alloc_page() {
            // print_u32("Allocated page at: ", page as u32);
        } else {
            print_serial("Failed to allocate page\n");
        }

        // let fb_start = info.fb_base as usize;
        //
        // let fb_size =
        //     info.fb_stride as usize *
        //     info.fb_height as usize *
        //     4;
        //
        // pmm::reserve(fb_start, fb_start + fb_size);
    }
    // print_u32("mem add up: ", (info.memory_map_size + info.descriptor_size + info.descriptor_version as usize) as u32);
    // print_u32("mem map: ", info.memory_map_size as u32);
    // print_u32("desc size: ", info.descriptor_size as u32);
    // print_u32("desc ver: ", info.descriptor_version as u32);
    // print_u32("width: ", info.fb_width);
    // print_u32("height: ", info.fb_height);
    // print_u32("stride: ", info.fb_stride);
    // print_u32("format: ", info.fb_format);

    // if !info.fb_base.is_null() {
    //     unsafe {
    //     
    //     let total_pixels = (info.fb_stride * info.fb_height) as usize;
    //     for i in 0..total_pixels {
    //         *info.fb_base.add(i) = 0x00121214;
    //     } 
    //     // draw_image(info, IMAGE, 50, 50);
    //     draw_string(info, 50, 50, "ily rust <3", 0x80D34516, 10);
    //     }
    // }

    loop {}
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
