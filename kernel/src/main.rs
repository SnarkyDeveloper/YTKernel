#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]

mod serial;
mod renderer;
mod memory;
mod drivers;

use core::mem::{size_of, offset_of};
use core::panic::PanicInfo;

use serial::*;
use renderer::*;

use crate::drivers::{Bar, check_device, scan_pci};
use crate::drivers::audio::ac97::consts::{AC97_MASTER_VOL, AC97_PCM_VOL};
use crate::drivers::audio::ac97::driver::Ac97Driver;
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
    _padding: u32, 
    pub kernel_start: u64,       // +52
    pub kernel_end: u64,         // +60
}

#[allow(dead_code)]
fn todos() {
    todo!("Write network stack - eh (AMD PCNet & rtl8139 later)");
    todo!("Add tls stack here");
    todo!("Add m3u8 parser (Should be easy)");
    todo!("Add video player - complex");
    todo!("Audio drivers - complex")
}

static IMAGE: &[u8] = include_bytes!("../assets/catgirl.png");
// static VIDEO: &[u8] = include_bytes!("../assets/badapple.mp4");

#[allow(dead_code)]
pub unsafe fn dump_memory_map(info: &BootInfo) {
    let mut ptr = info.memory_map;

    let end = info.memory_map.add(info.memory_map_size);
    print_u32("desc size=", core::mem::size_of::<memory::pmm::EfiMemoryDescriptor>() as u32);
    while ptr < end {
        let desc = &*(ptr as *const memory::pmm::EfiMemoryDescriptor);

        print_u32("\npages low: ", desc.number_of_pages as u32);
        print_u32(" & type: ", desc.ty);

        ptr = ptr.add(info.descriptor_size);
    }
}

#[allow(dead_code)]
fn print_info(info: &BootInfo) {

    print_u32("BootInfo size=", size_of::<BootInfo>() as u32);

    print_u32("off mmap=", offset_of!(BootInfo, memory_map) as u32);
    print_u32("off mapsz=", offset_of!(BootInfo, memory_map_size) as u32);
    print_u32("off descsz=", offset_of!(BootInfo, descriptor_size) as u32);
    print_u32("off descver=", offset_of!(BootInfo, descriptor_version) as u32);
    print_u32("off kstart=", offset_of!(BootInfo, kernel_start) as u32);
    print_u32("off kend=", offset_of!(BootInfo, kernel_end) as u32);
    print_u32("\nwidth: ", info.fb_width);
    print_u32("\nheight: ", info.fb_height);
    print_u32("\nstride: ", info.fb_stride);
    print_u32("\nformat: ", info.fb_format);
    
    print_u32("\nmem map: ", info.memory_map as u32);
    print_u32("\nmem map size: ", info.memory_map_size as u32);
    print_u32("\ndesc size: ", info.descriptor_size as u32);
    print_u32("\ndesc ver: ", info.descriptor_version as u32);
}


#[unsafe(no_mangle)]
pub extern "C" fn kernel_main(info_ptr: &'static BootInfo) -> ! {
    let info = unsafe { core::ptr::read(info_ptr) };
    unsafe { 
        init_serial(); 
    }
    
    // unsafe { dump_memory_map(&info); }
    // print_info(&info);

    unsafe {
        pmm::init(&info);
        pmm::reserve(info.kernel_start as usize, info.kernel_end as usize);
        if let Some(page) = pmm::alloc_page() {
            print_u32("Allocated page at: ", page as u32);
            print_serial("\n")
        } else {
            print_serial("\nFailed to allocate page");
        }

        let fb_start = info.fb_base as usize;

        let fb_size =
            info.fb_stride as usize *
            info.fb_height as usize *
            4;

        pmm::reserve(fb_start, fb_start + fb_size);
    }

    let convert = |hex: u32, op: u32| {
        let clamped_op = if op > 100 { 100 } else { op }; 
        let alpha = (clamped_op * 255) / 100;
        (alpha << 24) | (hex & 0x00FFFFFF)
    };

    if !info.fb_base.is_null() {
        unsafe {
        clear_screen(&info);
        // draw_png(&info, IMAGE, 50, 50);
        draw_string(&info, info.fb_width/4 + 10, info.fb_height/4 , "YTKernel", convert(0xCD201F, 0), 10);
        }
    }


   
    let mut base_address: *mut u32 = core::ptr::null_mut();

    let ac97_pci_device = scan_pci().into_iter().find(|e| {
        if e.class_code != 0x04 || e.subclass_code != 0x01 {
            return false;
        }

        let addr: *mut u32 = match e.bars[0] {
            Bar::Memory(addr) => {
                print_u32("Found AC97 device memory at: ", addr as u32);
                addr as *mut u32
            }
            Bar::Io(addr) => {
                print_u32("Found AC97 device IO at: ", addr as u32);
                addr as *mut u32
            }
            Bar::None => {
                print_serial("Found AC97 device with no BAR0\n");
                core::ptr::null_mut()
            }
        };

        if !addr.is_null() {
            base_address = addr;
            return true;
        }
        
        false
    });

// if !base_address.is_null() {
//     let mut ac97_driver = Ac97Driver::new(base_address);
//     unsafe {
//         let d = ac97_driver.initialize().unwrap();
//         d.set_volume(AC97_MASTER_VOL, 80, 80);
//         d.set_mute(AC97_MASTER_VOL, false);
//         d.set_volume(AC97_PCM_VOL, 100, 100);
//         d.set_mute(AC97_PCM_VOL, false);
//     }
// }


    loop {

    }
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
