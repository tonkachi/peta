use crate::vo::ValueObject;
use anyhow::Error;
use std::str::FromStr;
use std::string::ToString;

#[derive(Clone, Debug)]
pub struct Name(String);

impl ValueObject for Name {}

impl FromStr for Name {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl ToString for Name {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}
