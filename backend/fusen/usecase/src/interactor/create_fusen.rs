use crate::port::{CreateFusenInputData, CreateFusenOutputData, Port};
use anyhow::{Error, Result};
use derive_new::new;
use domain::entity::{Fusen, FusenBuilder};
use domain::repository::IdRepository;
use domain::repository::{CreateRepository, DeleteRepository, GetRepository};
use domain::vo::{FusenNote, FusenTitle};

#[derive(new)]
pub struct CreateFusenInterractor<I, S>
where
    I: IdRepository<Fusen>,
    S: CreateRepository<Fusen> + GetRepository<Fusen> + DeleteRepository<Fusen>,
{
    id_repository: I,
    fusen_repository: S,
}

impl<I, S> Port<CreateFusenInputData, CreateFusenOutputData> for CreateFusenInterractor<I, S>
where
    I: IdRepository<Fusen>,
    S: CreateRepository<Fusen> + GetRepository<Fusen> + DeleteRepository<Fusen>,
{
    fn handle(&self, input: CreateFusenInputData) -> Result<CreateFusenOutputData, Error> {
        let id = self.id_repository.generate()?;

        let fusen = FusenBuilder::default()
            .id(id)
            .title(input.title.parse::<FusenTitle>()?)
            .note(input.note.parse::<FusenNote>()?)
            .build()
            .unwrap();

        self.fusen_repository.create(fusen.clone())?;
        match self.fusen_repository.get(fusen.id().clone()) {
            Ok(aggregate_root) => Ok(CreateFusenOutputData::new(aggregate_root)),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::bail;
    use domain::vo::Id;
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::sync::Mutex;

    #[derive(new)]
    struct MockIdRepository {}
    impl IdRepository<Fusen> for MockIdRepository {
        fn generate(&self) -> Result<Id<Fusen>, Error> {
            Ok("01F8MECHZX3TBDSZ7XRADM79XE".parse::<Id<Fusen>>().unwrap())
        }
    }

    struct MockFusenRepository {
        store: Arc<Mutex<HashMap<Id<Fusen>, Fusen>>>,
    }
    impl MockFusenRepository {
        fn new() -> Self {
            let m = HashMap::new();
            Self {
                store: Arc::new(Mutex::new(m)),
            }
        }
    }

    impl CreateRepository<Fusen> for MockFusenRepository {
        fn create(&self, entity: Fusen) -> Result<(), Error> {
            let mut m = self.store.lock().unwrap();
            m.insert(entity.id().clone(), entity.clone());
            Ok(())
        }
    }

    impl GetRepository<Fusen> for MockFusenRepository {
        fn get(&self, id: Id<Fusen>) -> Result<Fusen, Error> {
            let m = self.store.lock().unwrap();
            match m.get(&id.clone()) {
                Some(aggregate_root) => Ok(aggregate_root.clone()),
                None => bail!("not found entity"),
            }
        }
    }

    impl DeleteRepository<Fusen> for MockFusenRepository {
        fn delete(&self, entity: Fusen) -> Result<(), Error> {
            let mut m = self.store.lock().unwrap();
            match m.remove(&entity.id().clone()) {
                Some(_) => Ok(()),
                None => bail!("not found entity"),
            }
        }
    }

    #[test]
    fn test_create_fusen_handle() {
        let id_repository = MockIdRepository::new();
        let fusen_repository = MockFusenRepository::new();
        let sut = CreateFusenInterractor::new(id_repository, fusen_repository);

        assert_eq!(
            sut.handle(CreateFusenInputData::new(
                "title".to_string(),
                "note".to_string()
            ))
            .unwrap(),
            CreateFusenOutputData::new(
                FusenBuilder::default()
                    .id("01F8MECHZX3TBDSZ7XRADM79XE".parse::<Id<Fusen>>().unwrap())
                    .title("any".parse::<FusenTitle>().unwrap())
                    .note("any".parse::<FusenNote>().unwrap())
                    .build()
                    .unwrap()
            )
        );

        // ok
        assert!(sut
            .handle(CreateFusenInputData::new(
                "title".to_string(),
                "note".to_string()
            ))
            .is_ok());
        assert!(sut
            .handle(CreateFusenInputData::new(
                "Clean Architecture using Rust".to_string(),
                "クリーンアーキテクチャをRustで実装してみました〜！".to_string()
            ))
            .is_ok());

        // err
        assert!(sut
            .handle(CreateFusenInputData::new(
                "".to_string(),
                "hogehoge".to_string()
            ))
            .is_err());
    }
}
