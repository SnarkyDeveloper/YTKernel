use core::arch::asm;
use heapless::Vec;

pub const MAX_PCI_DEVICES: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PciAddress {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bar {
    Memory(u64),
    Io(u16), 
    None,
}

#[derive(Debug, Clone)]
pub struct PciDevice {
    pub address: PciAddress,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class_code: u8,
    pub subclass_code: u8,
    pub prog_if: u8,
    pub revision_id: u8,
    pub bars: [Bar; 6],
    pub interrupt_line: u8,
}

#[inline(always)]
pub unsafe fn outl(port: u16, value: u32) {
    asm!(
        "out dx, eax",
        in("dx") port,
        in("eax") value,
        options(nomem, nostack, preserves_flags)
    );
}

#[inline(always)]
pub unsafe fn inl(port: u16) -> u32 {
    let value: u32;
    asm!(
        "in eax, dx",
        out("eax") value,
        in("dx") port,
        options(nomem, nostack, preserves_flags)
    );
    value
}

pub unsafe fn pci_read_u32(bus: u8, device: u8, function: u8, offset: u8) -> u32 {
    let address = (1 << 31)
        | ((bus as u32) << 16)
        | ((device as u32) << 11)
        | ((function as u32) << 8)
        | ((offset as u32) & 0xFC);

    outl(0xCF8, address);
    inl(0xCFC)
}

pub fn check_device(bus: u8, device: u8, function: u8) -> Option<PciDevice> {
    unsafe {
        let id_reg = pci_read_u32(bus, device, function, 0x00);
        let vendor_id = id_reg as u16;
        let device_id = (id_reg >> 16) as u16;

        // empty slot
        if vendor_id == 0xFFFF || vendor_id == 0x0000 {
            return None;
        }

        let class_reg = pci_read_u32(bus, device, function, 0x08);
        let revision_id = (class_reg & 0xFF) as u8;
        let prog_if = ((class_reg >> 8) & 0xFF) as u8;
        let subclass_code = ((class_reg >> 16) & 0xFF) as u8;
        let class_code = ((class_reg >> 24) & 0xFF) as u8;

        let mut bars = [Bar::None; 6];
        for i in 0..6 {
            let offset = 0x10 + (i * 4);
            let bar_val = pci_read_u32(bus, device, function, offset);
            
            if bar_val != 0 {
                if (bar_val & 1) == 1 {
                    bars[i as usize] = Bar::Io((bar_val & 0xFFFC) as u16);
                } else {
                    let memory_type = (bar_val >> 1) & 0x03;
                    if memory_type == 2 && i < 5 {
                        let next_bar_val = pci_read_u32(bus, device, function, offset + 4);
                        let full_address = ((next_bar_val as u64) << 32) | ((bar_val & 0xFFFFFFF0) as u64);
                        bars[i as usize] = Bar::Memory(full_address);
                    } else {
                        bars[i as usize] = Bar::Memory((bar_val & 0xFFFFFFF0) as u64);
                    }
                }
            }
        }

        let intr_reg = pci_read_u32(bus, device, function, 0x3C);
        let interrupt_line = (intr_reg & 0xFF) as u8;

        Some(PciDevice {
            address: PciAddress { bus, device, function },
            vendor_id,
            device_id,
            class_code,
            subclass_code,
            prog_if,
            revision_id,
            bars,
            interrupt_line,
        })
    }
}

pub fn scan_pci_bus() -> Vec<PciDevice, MAX_PCI_DEVICES> {
    let mut devices = Vec::new();

    for bus in 0..=255 {
        for device in 0..32 {
            if let Some(pci_dev) = check_device(bus, device, 0) {
                let _ = devices.push(pci_dev);
                
                unsafe {
                    let header_reg = pci_read_u32(bus, device, 0, 0x0C);
                    let header_type = ((header_reg >> 16) & 0xFF) as u8;
                    
                    // muttifunc?
                    if (header_type & 0x80) != 0 {
                        for function in 1..8 {
                            if let Some(func_dev) = check_device(bus, device, function) {
                                let _ = devices.push(func_dev);
                            }
                        }
                    }
                }
            }
        }
    }
    devices
}
