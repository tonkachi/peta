use crate::aggregate::AggregateRoot;
use crate::vo::Id;
use anyhow::{Error, Result};

pub trait CreateRepository<T>
where
    T: AggregateRoot,
{
    fn create(&self, entity: T) -> Result<(), Error>;
}

pub trait GetRepository<T>
where
    T: AggregateRoot,
{
    fn get(&self, id: Id<T>) -> Result<T, Error>;
}

pub trait DeleteRepository<T>
where
    T: AggregateRoot,
{
    fn delete(&self, entity: T) -> Result<(), Error>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;
    use crate::vo::Id;
    use anyhow::bail;
    use std::collections::HashMap;
    use std::hash::Hash;
    use std::sync::Arc;
    use std::sync::Mutex;

    #[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
    struct DummyEntity {
        id: Id<DummyEntity>,
    }
    impl AggregateRoot for DummyEntity {}
    impl Entity for DummyEntity {}

    struct DummyEntityRepository {
        store: Arc<Mutex<HashMap<Id<DummyEntity>, DummyEntity>>>,
    }
    impl DummyEntityRepository {
        fn new() -> Self {
            let m = HashMap::new();
            Self {
                store: Arc::new(Mutex::new(m)),
            }
        }
    }

    impl CreateRepository<DummyEntity> for DummyEntityRepository {
        fn create(&self, entity: DummyEntity) -> Result<(), Error> {
            let mut m = self.store.lock().unwrap();
            m.insert(entity.id.clone(), entity.clone());
            Ok(())
        }
    }

    impl GetRepository<DummyEntity> for DummyEntityRepository {
        fn get(&self, id: Id<DummyEntity>) -> Result<DummyEntity, Error> {
            let m = self.store.lock().unwrap();
            match m.get(&id.clone()) {
                Some(aggregate_root) => Ok(aggregate_root.clone()),
                None => bail!("not found entity"),
            }
        }
    }

    impl DeleteRepository<DummyEntity> for DummyEntityRepository {
        fn delete(&self, entity: DummyEntity) -> Result<(), Error> {
            let mut m = self.store.lock().unwrap();
            match m.remove(&entity.id.clone()) {
                Some(_) => Ok(()),
                None => bail!("not found entity"),
            }
        }
    }

    #[test]
    fn test_create_repository_for_entity() {
        let entity = DummyEntity {
            id: "hogehoge".parse::<Id<DummyEntity>>().unwrap(),
        };

        let sut = DummyEntityRepository::new();

        assert!(sut.create(entity).is_ok());
    }

    #[test]
    fn test_get_repository_for_entity() {
        let entity = DummyEntity {
            id: "hogehoge".parse::<Id<DummyEntity>>().unwrap(),
        };

        let sut = DummyEntityRepository::new();
        sut.create(entity).unwrap();

        assert!(sut
            .get("hogehoge".parse::<Id<DummyEntity>>().unwrap())
            .is_ok());
    }

    #[test]
    fn test_delete_repository_for_entity() {
        let entity = DummyEntity {
            id: "hogehoge".parse::<Id<DummyEntity>>().unwrap(),
        };

        let sut = DummyEntityRepository::new();
        sut.create(entity).unwrap();
        let target = sut
            .get("hogehoge".parse::<Id<DummyEntity>>().unwrap())
            .unwrap();

        assert!(sut.delete(target).is_ok());
    }
}
