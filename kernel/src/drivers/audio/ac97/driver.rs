extern crate alloc;

use crate::drivers::{audio::ac97::consts::*, util::{read_reg, write_reg}};

use alloc::string::*;

pub struct Ac97Driver<State> {
    raw_registers: *mut u32,
    _state: core::marker::PhantomData<State>,
}

impl Ac97Driver<Uninitialized> {
    pub fn new(base_address: *mut u32) -> Self {
        Self { raw_registers: base_address, _state: core::marker::PhantomData }
    }

    pub unsafe fn initialize(self) -> Result<Ac97Driver<Ready>, String> {
        let mut gcr = read_reg(self.raw_registers, AC97_GLOB_CNT);
        
        write_reg(self.raw_registers, AC97_GLOB_CNT, 0); // clear all settings and cold boot
        self.delay_ms(10);

        gcr |= GCR_COLD_RESET;

        write_reg(self.raw_registers, AC97_GLOB_CNT, gcr);
        self.delay_ms(10);

        let mut to = 10000;
            while (read_reg(self.raw_registers, AC97_GLOB_STA) & GSR_CODEC_READY) == 0 {
                to -= 1;
                if to == 0 {
                    return Err("device not ready after reset. dev link broken".to_string());
                }
                self.delay_us(10);
            }

        write_reg(self.raw_registers, AC97_CODEC_RESET, 0xFFFF);
        self.delay_ms(10);

        let reset_check = read_reg(self.raw_registers, AC97_CODEC_RESET);
        if reset_check == 0 || reset_check == 0xFFFF {
            return Err("cant communicate w codec after reset. dev link broken".to_string());
        }

        Ok(Ac97Driver { raw_registers: self.raw_registers, _state: core::marker::PhantomData })
    }
   
    fn delay_ms(&self, ms: u64) {
        for _ in 0..(ms * 50_000) { core::hint::spin_loop(); }
    }

    fn delay_us(&self, us: u64) {
        for _ in 0..(us * 50) { core::hint::spin_loop(); }
    }
}

impl Ac97Driver<Ready> {
    pub fn set_mute(&self, reg_offset: usize, mute: bool) {
        unsafe {
            let current = read_reg(self.raw_registers, reg_offset);
            let next = if mute {
                current | AC97_MUTE_BIT
            } else {
                current & !AC97_MUTE_BIT
            };
            write_reg(self.raw_registers, reg_offset, next);
        }
    }

    pub fn get_mute(&self, reg_offset: usize) -> bool {
        unsafe {
            let current = read_reg(self.raw_registers, reg_offset);
            (current & AC97_MUTE_BIT) != 0
        }
    }

    pub fn toggle_mute(&self, reg_offset: usize) {
        unsafe {
            let current = read_reg(self.raw_registers, reg_offset);
            let next = current ^ AC97_MUTE_BIT;
            write_reg(self.raw_registers, reg_offset, next);
        }
    }

    pub fn set_volume(&self, reg_offset: usize, left_percent: u8, right_percent: u8) {
        let left_attn = 31 - (left_percent.min(100) as u32 * 31 / 100);
        let right_attn = 31 - (right_percent.min(100) as u32 * 31 / 100);

        unsafe {
            let current_mute = read_reg(self.raw_registers, reg_offset) & AC97_MUTE_BIT;
            
            let value = current_mute | (left_attn << 8) | right_attn;
            write_reg(self.raw_registers, reg_offset, value);
        }
    }

    pub fn get_volume(&self, reg_offset: usize) -> (u8, u8) {
        unsafe {
            let current = read_reg(self.raw_registers, reg_offset);
            
            let left_attn = (current >> 8) & 0x1F;
            let right_attn = current & 0x1F;

            let left_percent = ((31 - left_attn) * 100 / 31) as u8;
            let right_percent = ((31 - right_attn) * 100 / 31) as u8;

            (left_percent, right_percent)
        }
    }
}
