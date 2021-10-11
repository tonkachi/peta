#[allow(clippy::module_inception)]
mod entity;

mod fusen;

pub use self::entity::Entity;
pub use self::fusen::{Fusen, FusenBuilder};
