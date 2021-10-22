use super::port::{InputData, OutputData};

#[derive(Default, Debug, PartialEq)]
pub struct GetTagInputData {
    pub hash: String,
}

impl InputData for GetTagInputData {}

#[derive(Default, Debug, PartialEq)]
pub struct GetTagOutputData {
    pub hash: String,
    pub name: String,
    pub fusen_id: String,
}

impl OutputData for GetTagOutputData {}
