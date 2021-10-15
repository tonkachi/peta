use anyhow::{Error, Result};
use domain::repository::IdRepository as Repository;
use domain::vo::Id;
use ulid::Ulid;

#[derive(Default)]
pub struct IdRepository {}

impl Repository for IdRepository {
    fn generate<T>(&self) -> Result<Id<T>, Error> {
        Ulid::new().to_string().as_str().parse::<Id<T>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::aggregate::AggregateRoot;
    use domain::entity::Entity;
    use domain::vo::Id;

    #[derive(Default, Clone, Debug, PartialEq, Eq)]
    struct DummyEntity {
        id: Id<DummyEntity>,
    }
    impl AggregateRoot for DummyEntity {}
    impl Entity for DummyEntity {}

    #[test]
    fn test_id_repository() {
        let sut = IdRepository::default();

        assert!(sut.generate::<DummyEntity>().is_ok());
    }
}
