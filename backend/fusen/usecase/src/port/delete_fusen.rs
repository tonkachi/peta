use crate::port::{InputData, OutputData};
use derive_new::new;

#[derive(new, Clone, Debug, PartialEq)]
pub struct DeleteFusenInputData {
    pub id: String,
}

impl InputData for DeleteFusenInputData {}

#[derive(new, Clone, Debug, PartialEq)]
pub struct DeleteFusenOutputData {}

impl OutputData for DeleteFusenOutputData {}
