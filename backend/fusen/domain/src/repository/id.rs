use crate::vo::Id;
use anyhow::{Error, Result};

pub trait IdRepository {
    fn generate<T>(&self) -> Result<Id<T>, Error>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vo::Id;

    #[derive(Clone, Debug)]
    struct DummyEntity {}

    #[derive(Default)]
    struct MockIdRepository {}

    impl IdRepository for MockIdRepository {
        fn generate<T>(&self) -> Result<Id<T>, Error> {
            Ok("01F8MECHZX3TBDSZ7XRADM79XE".parse::<Id<T>>().unwrap())
        }
    }

    #[test]
    fn test_id_repository_for_entity() {
        let sut = MockIdRepository::default();
        assert!(sut.generate::<DummyEntity>().is_ok());
    }
}
