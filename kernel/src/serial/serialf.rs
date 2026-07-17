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

pub unsafe fn init_serial() {
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

pub unsafe fn print_char(c: u8) {
    unsafe {
        while !is_transmit_empty() {}
        outb(COM1, c);
    }
}

pub fn print_serial(string: &str) {
    for byte in string.bytes() {
        unsafe { print_char(byte); }
    }
}


pub fn print_u64(prefix: &str, mut n: u64) {
    print_serial(prefix);
    
    if n == 0 {
        unsafe { print_char(b'0'); }
        return;
    }

    let mut storage: u64 = 0;
    let mut digits = 0;

    while n > 0 {
        storage = (storage << 8) | ((b'0' + (n % 10) as u8) as u64);
        n /= 10;
        digits += 1;
    }

    unsafe {
        for _ in 0..digits {
            let c = (storage & 0xFF) as u8;
            print_char(c);
            storage >>= 8;
        }
    }
}

pub fn print_u32(prefix: &str, n: u32) {
    print_u64(prefix, n as u64)
}
