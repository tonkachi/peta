use crate::vo::ValueObject;
use anyhow::Error;
use std::str::FromStr;
use std::string::ToString;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TagHash(String);

impl ValueObject for TagHash {}

impl FromStr for TagHash {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl ToString for TagHash {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}
