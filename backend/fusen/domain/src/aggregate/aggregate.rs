use crate::entity::Entity;
use crate::entity::Fusen as EntityFusen;

pub trait AggregateRoot: Entity {}

pub type Fusen = EntityFusen;
impl AggregateRoot for Fusen {}
