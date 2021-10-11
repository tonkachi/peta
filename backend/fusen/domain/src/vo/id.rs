use crate::vo::ValueObject;
use anyhow::Error;
use std::hash::Hash;
use std::hash::Hasher;
use std::marker::PhantomData;
use std::str::FromStr;
use std::string::ToString;

#[derive(Default, Clone, Debug, Eq)]
pub struct Id<T> {
    value: String,

    _phantom: PhantomData<T>,
}

impl<T> ValueObject for Id<T> {}

impl<T> FromStr for Id<T> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            value: s.to_string(),
            _phantom: PhantomData,
        })
    }
}

impl<T> ToString for Id<T> {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug)]
    struct Entity {}

    #[derive(Clone, Debug)]
    struct AnotherEntity {}

    #[derive(Clone, Debug)]
    enum EnumEntity {}

    #[test]
    fn test_id() {
        assert!("01F8MECHZX3TBDSZ7XRADM79XE".parse::<Id::<Entity>>().is_ok());
        assert!("01F8MECHZX3TBDSZ7XRADM79XE"
            .parse::<Id::<AnotherEntity>>()
            .is_ok());
        assert!("01F8MECHZX3TBDSZ7XRADM79XE"
            .parse::<Id::<EnumEntity>>()
            .is_ok());
    }

    #[test]
    fn test_id_to_string() {
        assert_eq!(
            "01F8MECHZX3TBDSZ7XRADM79XE"
                .parse::<Id::<Entity>>()
                .unwrap()
                .to_string(),
            "01F8MECHZX3TBDSZ7XRADM79XE".to_string()
        );
    }

    #[test]
    fn test_id_eq() {
        let id = "01F8MECHZX3TBDSZ7XRADM79XE".parse::<Id<Entity>>().unwrap();
        let cloned = id.clone();

        let another = "01F8MECHZX3TBDSZ7XRADM79XF".parse::<Id<Entity>>().unwrap();

        assert_eq!(id, cloned);
        assert_ne!(id, another);
    }
}
