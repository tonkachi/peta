use crate::entity::Entity;
use crate::vo::{FusenNote, FusenTitle, Id};
use derive_builder::Builder;
use getset::{Getters, Setters};

#[derive(Clone, Debug, Getters, Setters, Builder, Eq)]
#[builder(setter(into))]
pub struct Fusen {
    #[builder(pattern = "immutable")]
    #[getset(get = "pub")]
    id: Id<Fusen>,

    #[getset(get = "pub", set = "pub")]
    title: FusenTitle,
    #[getset(get = "pub", set = "pub")]
    note: FusenNote,
}

impl Entity for Fusen {}

impl PartialEq for Fusen {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vo::{FusenNote, FusenTitle, Id};

    #[test]
    fn test_fusen() {
        assert!(FusenBuilder::default()
            .id("01F8MECHZX3TBDSZ7XRADM79XE".parse::<Id::<Fusen>>().unwrap())
            .title("title".parse::<FusenTitle>().unwrap())
            .note("note".parse::<FusenNote>().unwrap())
            .build()
            .is_ok());
    }

    #[test]
    fn test_fusen_eq() {
        let before = FusenBuilder::default()
            .id("01F8MECHZX3TBDSZ7XRADM79XE".parse::<Id<Fusen>>().unwrap())
            .title("title".parse::<FusenTitle>().unwrap())
            .note("note".parse::<FusenNote>().unwrap())
            .build()
            .unwrap();

        let mut after = before.clone();
        after.set_title("mod_title".parse::<FusenTitle>().unwrap());

        let another = FusenBuilder::default()
            .id("01F8MECHZX3TBDSZ7XRADM79XF".parse::<Id<Fusen>>().unwrap())
            .title("title".parse::<FusenTitle>().unwrap())
            .note("note".parse::<FusenNote>().unwrap())
            .build()
            .unwrap();

        assert_eq!(before, after);
        assert_ne!(before, another);
        assert_ne!(after, another);
    }
}
