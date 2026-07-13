#![no_std]
#![no_main]

mod serial;
mod renderer;

use core::panic::PanicInfo;
use core::fmt::Write;

use heapless::String;
use serial::*;
use renderer::*;

#[repr(C)]
pub struct BootInfo {
    pub fb_base: *mut u32,  
    pub fb_width: u32,
    pub fb_height: u32,
    pub fb_stride: u32,
    pub fb_format: u32
}


fn todos() {
    todo!("Write network stack - eh (AMD PCNet & rtl8139 later)");
    todo!("Add tls stack here");
    todo!("Add m3u8 parser (Should be easy)");
    todo!("Add video player - complex");
    todo!("Audio drivers - complex")
}

#[unsafe(no_mangle)]
pub extern "sysv64" fn kernel_main(info: &'static BootInfo) -> ! {
    unsafe { 
        init_serial(); 
    }

    if !info.fb_base.is_null() {
        unsafe {
        
        let total_pixels = (info.fb_stride * info.fb_height) as usize;
        for i in 0..total_pixels {
            *info.fb_base.add(i) = 0x00121214;
        } 
        draw_string(info, 50, 50, "lesbians <3", 0x00F5A9B8, 10);
        }
    }

    loop {}
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
