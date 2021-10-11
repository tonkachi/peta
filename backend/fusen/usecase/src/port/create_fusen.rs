use crate::port::{InputData, OutputData};
use derive_new::new;
use domain::entity::Fusen;

#[derive(new, Clone, Debug, PartialEq)]
pub struct CreateFusenInputData {
    pub title: String,
    pub note: String,
}

impl InputData for CreateFusenInputData {}

#[derive(new, Clone, Debug, PartialEq)]
pub struct CreateFusenOutputData {
    pub fusen: Fusen,
}

impl OutputData for CreateFusenOutputData {}
