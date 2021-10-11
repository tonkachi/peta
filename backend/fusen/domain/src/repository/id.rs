use crate::vo::Id;
use anyhow::{Error, Result};

pub trait IdRepository<T> {
    fn generate(&self) -> Result<Id<T>, Error>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;
    use crate::vo::Id;

    #[derive(Clone, Debug)]
    struct DummyEntity {}
    impl Entity for DummyEntity {}

    #[derive(Default)]
    struct MockIdRepository {}

    impl IdRepository<DummyEntity> for MockIdRepository {
        fn generate(&self) -> Result<Id<DummyEntity>, Error> {
            Ok("01F8MECHZX3TBDSZ7XRADM79XE"
                .parse::<Id<DummyEntity>>()
                .unwrap())
        }
    }

    #[test]
    fn test_id_repository_for_entity() {
        let sut = MockIdRepository::default();
        assert!(sut.generate().is_ok());
    }
}
