use crate::port::{InputData, OutputData};
use derive_new::new;
use domain::entity::Fusen;

#[derive(new, Clone, Debug, PartialEq)]
pub struct GetFusenInputData {
    pub id: String,
}

impl InputData for GetFusenInputData {}

#[derive(new, Clone, Debug, PartialEq)]
pub struct GetFusenOutputData {
    pub fusen: Fusen,
}

impl OutputData for GetFusenOutputData {}
