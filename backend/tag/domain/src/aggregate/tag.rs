use super::aggregate_root::AggregateRoot;
use crate::entity::Tag as TagEntity;

pub type Tag = TagEntity;

impl AggregateRoot for Tag {}
