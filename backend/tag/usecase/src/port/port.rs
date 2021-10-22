use anyhow::{Error, Result};

pub trait InputData {}
pub trait OutputData {}

pub trait Port<Input: InputData, Output: OutputData> {
    fn handle(&self, input: Input) -> Result<Output, Error>;
}
