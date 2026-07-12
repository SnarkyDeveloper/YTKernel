#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

const COM1: u16 = 0x3F8;

unsafe fn outb(port: u16, value: u8) {
    unsafe {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") value,
            options(nomem, nostack, preserves_flags)
        );
    }
}

unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    unsafe {
        asm!(
            "in al, dx",
            out("al") value,
            in("dx") port,
            options(nomem, nostack, preserves_flags)
        );
    }
    value
}

unsafe fn init_serial() {
    unsafe {
        outb(COM1 + 1, 0x00);    // nu more interrupts
        outb(COM1 + 3, 0x80);    // baud rate divisor
        outb(COM1 + 0, 0x03);    // divisor = 3
        outb(COM1 + 1, 0x00); 
        outb(COM1 + 3, 0x03); 
        outb(COM1 + 2, 0xC7); 
        outb(COM1 + 4, 0x0B); 
    }
}

unsafe fn is_transmit_empty() -> bool {
    unsafe { (inb(COM1 + 5) & 0x20) != 0 }
}

unsafe fn print_char(c: u8) {
    unsafe {
        while !is_transmit_empty() {}
        outb(COM1, c);
    }
}

fn print_serial(string: &str) {
    for byte in string.bytes() {
        unsafe { print_char(byte); }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    unsafe { init_serial(); }
    
    print_serial("\n\r=== MELON IS GOONING SUCCESSFULY ===\n\r");
    print_serial("mommy asmr todo!\n\r");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
