use crate::aggregate::Tag;
use crate::vo::TagHash;
use anyhow::{Error, Result};

pub trait TagRepository {
    fn create(&self, entity: Tag) -> Result<(), Error>;
    fn delete(&self, entity: Tag) -> Result<(), Error>;
    fn get(&self, hash: TagHash) -> Result<Tag, Error>;
    // TODO: fn get_by_fusen_id(&self, fusend_id: FusenId) -> Result<Vec<Tag>, Error>;
    // TODO: remove fusenid from tag
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aggregate::Tag;
    use crate::entity::TagBuilder;
    use crate::vo::{FusenId, TagHash, TagName};
    use anyhow::{bail, Error};
    use std::collections::HashMap;
    use std::str::FromStr;
    use std::sync::Mutex;

    fn get_dummy_tag(dummy_data: &str) -> Tag {
        let hash = TagHash::from_str(dummy_data).unwrap();
        let name = TagName::from_str(dummy_data).unwrap();
        let fusen_id = FusenId::from_str(dummy_data).unwrap();
        TagBuilder::default()
            .hash(hash)
            .name(name)
            .fusen_id(fusen_id)
            .build()
            .unwrap()
    }

    struct TestTagRepository {
        tags: Box<Mutex<HashMap<TagHash, Tag>>>,
    }

    impl TestTagRepository {
        fn new() -> Self {
            let m = HashMap::new();
            Self {
                tags: Box::new(Mutex::new(m)),
            }
        }
    }

    impl TagRepository for TestTagRepository {
        fn create(&self, entity: Tag) -> Result<(), Error> {
            let mut tags = self.tags.lock().unwrap();
            tags.insert(entity.hash().clone(), entity.clone());
            Ok(())
        }

        fn delete(&self, entity: Tag) -> Result<(), Error> {
            print!("::: {:#?}", entity);
            let mut m = self.tags.lock().unwrap();
            match m.remove(&entity.hash().clone()) {
                Some(_) => Ok(()),
                None => bail!("not found tag"),
            }
        }

        fn get(&self, hash: TagHash) -> Result<Tag, Error> {
            let tags = self.tags.lock().unwrap();
            match tags.get(&hash.clone()) {
                Some(tag) => Ok(tag.clone()),
                None => bail!("not found tag"),
            }
        }
    }

    #[test]
    fn test_create_repository() {
        let repo = TestTagRepository::new();
        let dummy_tag = get_dummy_tag("dummy");
        assert!(repo.create(dummy_tag).is_ok());
    }

    #[test]
    fn test_delete_repository() {
        let repo = TestTagRepository::new();
        let dummy_tag = get_dummy_tag("dummy");
        repo.create(dummy_tag.clone()).unwrap();
        assert!(repo.delete(dummy_tag).is_ok());
    }

    #[test]
    fn test_get_repository() {
        let repo = TestTagRepository::new();
        let dummy_tag = get_dummy_tag("dummy");
        repo.create(dummy_tag.clone()).unwrap();
        assert!(repo.get(dummy_tag.hash().clone()).is_ok());
    }
}
