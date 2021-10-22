use crate::aggregate::Tag;
use crate::vo::{FusenId, TagHash};
use anyhow::{Error, Result};

pub trait TagRepository {
    fn create(&self, entity: Tag) -> Result<(), Error>;
    fn delete(&self, entity: Tag) -> Result<(), Error>;
    fn get(&self, hash: TagHash) -> Result<Tag, Error>;
    fn get_by_fusen_id(&self, fsend_id: FusenId) -> Result<Vec<Tag>, Error>;
    fn update_tag(&self, entity: Tag) -> Result<(), Error>;
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

    macro_rules! from_str {
        ($t:ty, $s:expr) => {
            <$t>::from_str($s).unwrap()
        };
    }

    fn get_dummy_tag(dummy_data: &str) -> Tag {
        let hash = from_str!(TagHash, dummy_data);
        let name = from_str!(TagName, dummy_data);
        let fusen_id = from_str!(FusenId, dummy_data);
        TagBuilder::default()
            .hash(hash)
            .name(name)
            .fusen_ids(vec![fusen_id])
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
            if tags.get(entity.hash()).is_some() {
                bail!("tag is already exists")
            }
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

        fn get_by_fusen_id(&self, fusen_id: FusenId) -> Result<Vec<Tag>, Error> {
            let tags = self.tags.lock().unwrap();
            let mut filtered_tags: Vec<Tag> = Vec::new();
            for tag in tags.values() {
                if tag.fusen_ids().contains(&fusen_id) {
                    filtered_tags.push(tag.clone());
                }
            }
            Ok(filtered_tags)
        }

        fn update_tag(&self, entity: Tag) -> Result<(), Error> {
            let mut tags = self.tags.lock().unwrap();
            if tags.get(entity.hash()).is_none() {
                bail!("not found tag")
            }
            tags.insert(entity.hash().clone(), entity.clone());
            Ok(())
        }
    }

    #[test]
    fn create() {
        let repo = TestTagRepository::new();
        let dummy_tag = get_dummy_tag("dummy");
        assert!(repo.create(dummy_tag).is_ok());
    }

    #[test]
    fn delete() {
        let repo = TestTagRepository::new();
        let dummy_tag = get_dummy_tag("dummy");
        repo.create(dummy_tag.clone()).unwrap();
        assert!(repo.delete(dummy_tag).is_ok());
    }

    #[test]
    fn get() {
        let repo = TestTagRepository::new();
        let dummy_tag = get_dummy_tag("dummy");
        repo.create(dummy_tag.clone()).unwrap();
        assert!(repo.get(dummy_tag.hash().clone()).is_ok());
    }

    #[test]
    fn get_by_fusen_id() {
        let sut = TestTagRepository::new();
        let tag1_fusen_id_1_and_2 = TagBuilder::default()
            .hash(from_str!(TagHash, "tag1"))
            .name(from_str!(TagName, "tag1"))
            .fusen_ids(vec![from_str!(FusenId, "f1"), from_str!(FusenId, "f2")])
            .build()
            .unwrap();
        let tag2_fusen_id_1 = TagBuilder::default()
            .hash(from_str!(TagHash, "tag2"))
            .name(from_str!(TagName, "tag2"))
            .fusen_ids(vec![from_str!(FusenId, "f1")])
            .build()
            .unwrap();
        let tag3_fusen_id_3 = TagBuilder::default()
            .hash(from_str!(TagHash, "tag3"))
            .name(from_str!(TagName, "tag3"))
            .fusen_ids(vec![from_str!(FusenId, "f3")])
            .build()
            .unwrap();

        sut.create(tag1_fusen_id_1_and_2.clone()).unwrap();
        sut.create(tag2_fusen_id_1.clone()).unwrap();
        sut.create(tag3_fusen_id_3.clone()).unwrap();

        let tags_with_f1 = sut.get_by_fusen_id(from_str!(FusenId, "f1")).unwrap();
        for tag in tags_with_f1 {
            assert!(vec![tag1_fusen_id_1_and_2.clone(), tag2_fusen_id_1.clone()].contains(&tag));
        }

        let tags_with_f2 = sut.get_by_fusen_id(from_str!(FusenId, "f2")).unwrap();
        assert_eq!(tags_with_f2, vec![tag1_fusen_id_1_and_2.clone()])
    }

    #[test]
    fn update_tag() {
        let sut = TestTagRepository::new();
        let dummy_tag = get_dummy_tag("dummy");
        sut.create(dummy_tag.clone()).unwrap();

        // get by dummy hash before update
        let tag_before_update = sut.get(from_str!(TagHash, "dummy")).unwrap();
        assert_eq!(tag_before_update.name(), dummy_tag.name());

        let new_tag = TagBuilder::default()
            .hash(from_str!(TagHash, "dummy"))
            .name(from_str!(TagName, "tag1"))
            .fusen_ids(vec![from_str!(FusenId, "f1"), from_str!(FusenId, "f2")])
            .build()
            .unwrap();
        sut.update_tag(new_tag.clone()).unwrap();

        let tag_after_update = sut.get(from_str!(TagHash, "dummy")).unwrap();
        // tag.hash field is used when compare tag entity so in thid test compare tag.name field
        assert_ne!(tag_after_update.name(), tag_before_update.name());
        assert_eq!(tag_after_update.name(), new_tag.name());

        // fail when tag is not found
        let not_found_tag = get_dummy_tag("not_found_tag");
        assert!(sut.update_tag(not_found_tag.clone()).is_err());
    }
}
