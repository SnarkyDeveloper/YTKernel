// audio consts
pub const AC97_GLOB_CNT: usize   = 0x100 + 0x2C; 
pub const AC97_GLOB_STA: usize   = 0x100 + 0x30; 

pub const AC97_CODEC_RESET: usize = 0x00;

pub const GCR_COLD_RESET: u32 = 1 << 1;
pub const GCR_WARM_RESET: u32 = 1 << 2;
pub const GSR_CODEC_READY: u32 = 1 << 8;

pub const AC97_RESET: usize       = 0x00;
pub const AC97_MASTER_VOL: usize   = 0x02;
pub const AC97_MIC_VOL: usize      = 0x0E;
pub const AC97_PCM_VOL: usize      = 0x18;
pub const AC97_RECORD_SEL: usize   = 0x1A;

pub const AC97_MUTE_BIT: u32 = 1 << 15;

pub struct Uninitialized;
pub struct Ready;

