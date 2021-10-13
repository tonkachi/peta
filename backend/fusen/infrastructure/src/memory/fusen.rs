use anyhow::bail;
use anyhow::{Error, Result};
use domain::entity::Fusen;
use domain::repository::{CreateRepository, DeleteRepository, GetRepository};
use domain::vo::Id;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

pub struct FusenRepository {
    store: Arc<Mutex<HashMap<Id<Fusen>, Fusen>>>,
}

impl FusenRepository {
    #[allow(dead_code)]
    pub fn default() -> Self {
        Self {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl CreateRepository<Fusen> for FusenRepository {
    fn create(&self, aggregate: Fusen) -> Result<(), Error> {
        let mut m = self.store.lock().unwrap();
        m.insert(aggregate.id().clone(), aggregate.clone());
        Ok(())
    }
}

impl GetRepository<Fusen> for FusenRepository {
    fn get(&self, id: Id<Fusen>) -> Result<Fusen, Error> {
        let m = self.store.lock().unwrap();
        match m.get(&id) {
            Some(aggregate) => Ok(aggregate.clone()),
            None => bail!("not found entity"),
        }
    }
}

impl DeleteRepository<Fusen> for FusenRepository {
    fn delete(&self, aggregate: Fusen) -> Result<(), Error> {
        let mut m = self.store.lock().unwrap();
        match m.remove(aggregate.id()) {
            Some(_) => Ok(()),
            None => bail!("not found entity"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::aggregate::AggregateRoot;
    use domain::entity::Entity;
    use domain::entity::FusenBuilder;
    use domain::vo::FusenNote;
    use domain::vo::FusenTitle;
    use domain::vo::Id;

    #[derive(Default, Clone, Debug, PartialEq, Eq)]
    struct DummyEntity {
        id: Id<DummyEntity>,
    }
    impl AggregateRoot for DummyEntity {}
    impl Entity for DummyEntity {}

    #[test]
    fn test_create_repository_for_entity() {
        let entity = FusenBuilder::default()
            .id("01F8MECHZX3TBDSZ7XRADM79XE".parse::<Id<Fusen>>().unwrap())
            .title("title".parse::<FusenTitle>().unwrap())
            .note("note".parse::<FusenNote>().unwrap())
            .build()
            .unwrap();

        let sut = FusenRepository::default();

        assert!(sut.create(entity).is_ok());
    }

    #[test]
    fn test_get_repository_for_entity() {
        let entity = FusenBuilder::default()
            .id("01F8MECHZX3TBDSZ7XRADM79XE".parse::<Id<Fusen>>().unwrap())
            .title("title".parse::<FusenTitle>().unwrap())
            .note("note".parse::<FusenNote>().unwrap())
            .build()
            .unwrap();

        let sut = FusenRepository::default();
        sut.create(entity).unwrap();

        assert!(sut
            .get("01F8MECHZX3TBDSZ7XRADM79XE".parse::<Id<Fusen>>().unwrap())
            .is_ok());
    }

    #[test]
    fn test_delete_repository_for_entity() {
        let entity = FusenBuilder::default()
            .id("01F8MECHZX3TBDSZ7XRADM79XE".parse::<Id<Fusen>>().unwrap())
            .title("title".parse::<FusenTitle>().unwrap())
            .note("note".parse::<FusenNote>().unwrap())
            .build()
            .unwrap();

        let sut = FusenRepository::default();
        sut.create(entity).unwrap();
        let target = sut
            .get("01F8MECHZX3TBDSZ7XRADM79XE".parse::<Id<Fusen>>().unwrap())
            .unwrap();

        assert!(sut.delete(target).is_ok());
    }
}
