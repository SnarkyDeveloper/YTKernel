use crate::drivers::traits::CustomDriver;

pub trait AudioDriver : CustomDriver {
    type Error;
    fn set_sample_rate(&self, channel: u8, sample_rate: u32) -> Result<(), <Self as AudioDriver>::Error>;
    fn write_samples(&self, channel: u8, samples: &[u8]) -> Result<(), <Self as AudioDriver>::Error>;
    
    fn read_samples(&self, channel: u8, buffer: &mut [u8]) -> Result<(), <Self as AudioDriver>::Error>;
}
