use crate::port::{DeleteFusenInputData, DeleteFusenOutputData, Port};
use anyhow::{Error, Result};
use derive_new::new;
use domain::entity::*;
use domain::repository::*;
use domain::vo::*;

#[derive(new)]
pub struct DeleteFusenInteractor<S>
where
    S: GetRepository<Fusen> + DeleteRepository<Fusen>,
{
    fusen_repository: S,
}

impl<S> Port<DeleteFusenInputData, DeleteFusenOutputData> for DeleteFusenInteractor<S>
where
    S: GetRepository<Fusen> + DeleteRepository<Fusen>,
{
    fn handle(&self, input: DeleteFusenInputData) -> Result<DeleteFusenOutputData, Error> {
        let id = input.id.parse::<Id<Fusen>>()?;

        let fusen = self.fusen_repository.get(id)?;
        self.fusen_repository.delete(fusen)?;

        Ok(DeleteFusenOutputData::new())
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
    fn test_delete_fusen_handle() {
        let fusen_repository = MockFusenRepository::new();
        let fusen_a = FusenBuilder::default()
            .id("01F8MECHZX3TBDSZ7XRADM79XE".parse::<Id<Fusen>>().unwrap())
            .title("any".parse::<FusenTitle>().unwrap())
            .note("any".parse::<FusenNote>().unwrap())
            .build()
            .unwrap();
        let fusen_b = FusenBuilder::default()
            .id("01F8MECHZX3TBDSZ7XRADM79XF".parse::<Id<Fusen>>().unwrap())
            .title(
                "Clean Architecture using Rust"
                    .parse::<FusenTitle>()
                    .unwrap(),
            )
            .note(
                "クリーンアーキテクチャをRustで実装してみました〜！"
                    .parse::<FusenNote>()
                    .unwrap(),
            )
            .build()
            .unwrap();

        fusen_repository.create(fusen_a.clone()).unwrap();
        fusen_repository.create(fusen_b.clone()).unwrap();

        let sut = DeleteFusenInteractor::new(fusen_repository);

        assert_eq!(
            sut.handle(DeleteFusenInputData::new(fusen_a.id().to_string()))
                .unwrap(),
            DeleteFusenOutputData::new()
        );

        // ok
        assert!(sut
            .handle(DeleteFusenInputData::new(fusen_b.id().to_string()))
            .is_ok());

        // err
        assert!(sut
            .handle(DeleteFusenInputData::new("NOTFOUND_ID".to_string()))
            .is_err());
    }
}
