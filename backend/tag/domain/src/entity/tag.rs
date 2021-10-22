use crate::entity::Entity;
use crate::vo::{FusenId, TagHash, TagName};
use derive_builder::Builder;
use getset::{Getters, Setters};

#[derive(Clone, Debug, Getters, Setters, Builder, Eq)]
#[builder(setter(into))]
pub struct Tag {
    #[builder(pattern = "immutable")]
    #[getset(get = "pub")]
    hash: TagHash,

    #[getset(get = "pub", set = "pub")]
    name: TagName,

    #[getset(get = "pub", set = "pub")]
    fusen_ids: Vec<FusenId>,
}

impl Entity for Tag {}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}
