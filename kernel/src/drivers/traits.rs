pub trait CustomDriver: Send + Sync {
    type Config;
    type Error;

    fn init(config: Self::Config) -> Result<Self, Self::Error> 
    where 
        Self: Sized;

    fn reset(&self) -> Result<(), Self::Error>;

    fn shutdown(&mut self) -> Result<(), Self::Error>;
}
