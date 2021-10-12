use crate::vo::ValueObject;
use anyhow::Error;
use std::str::FromStr;
use std::string::ToString;

#[derive(Clone, Debug)]
pub struct FusenId(String);

impl ValueObject for FusenId {}

impl FromStr for FusenId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl ToString for FusenId {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}
