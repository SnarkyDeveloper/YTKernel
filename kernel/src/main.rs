#![no_std]
#![no_main]

mod serial;

use core::panic::PanicInfo;
use core::fmt::Write;

use heapless::String;
use serial::*;

#[repr(C)]
pub struct BootInfo {
    pub framebuffer: *mut u32,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: u32,
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
    unsafe { init_serial(); }
    todo!("Fix info not sending from the assembly offsets");
    todos();

    print_serial("\n\r=== MELON IS GOONING SUCCESSFULY ===\n\r");
    print_serial("mommy asmr todo!\n\r");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
