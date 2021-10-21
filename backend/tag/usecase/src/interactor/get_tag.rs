use crate::port::{GetTagInputData, GetTagOutputData, Port};
use anyhow::{Error, Result};
use derive_new::new;
use domain::repository::TagRepository;
use domain::vo::TagHash;

#[derive(new)]
pub struct GetTagInteractor<T: TagRepository> {
    tag_repository: T,
}

impl<T: TagRepository> Port<GetTagInputData, GetTagOutputData> for GetTagInteractor<T> {
    fn handle(&self, input: GetTagInputData) -> Result<GetTagOutputData, Error> {
        println!("test {:#?}", input);
        let tag = self.tag_repository.get(input.hash.parse::<TagHash>()?)?;
        Ok(GetTagOutputData {
            hash: tag.hash().clone().to_string(),
            name: tag.name().clone().to_string(),
            fusen_id: tag.fusen_id().clone().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::port::{GetTagInputData, GetTagOutputData, Port};
    use anyhow::{bail, Error, Result};
    use domain::aggregate::Tag;
    use domain::entity::TagBuilder;
    use domain::repository::TagRepository;
    use domain::vo::{FusenId, TagHash, TagName};
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
            let mut tags = self.tags.lock().unwrap();
            match tags.remove(&entity.hash()) {
                Some(_) => Ok(()),
                None => bail!("not found tag"),
            }
        }

        fn get(&self, hash: TagHash) -> Result<Tag, Error> {
            let tags = self.tags.lock().unwrap();
            match tags.get(&hash) {
                Some(tag) => Ok(tag.clone()),
                None => bail!("not found tag"),
            }
        }
    }

    #[test]
    fn test_succeeded() {
        let dummy_tag = get_dummy_tag("dummy_tag");
        let repository = TestTagRepository::new();
        repository.create(dummy_tag).unwrap();
        let sut = GetTagInteractor::new(repository);
        let input = GetTagInputData {
            hash: String::from_str("dummy_tag").unwrap(),
        };
        let output = sut.handle(input).unwrap();
        let expected_output = GetTagOutputData {
            hash: String::from_str("dummy_tag").unwrap(),
            name: String::from_str("dummy_tag").unwrap(),
            fusen_id: String::from_str("dummy_tag").unwrap(),
        };
        assert_eq!(output, expected_output);
        let not_expected_output = GetTagOutputData {
            hash: String::from_str("fake_dummy_tag").unwrap(),
            name: String::from_str("fake_dummy_tag").unwrap(),
            fusen_id: String::from_str("fake_dummy_tag").unwrap(),
        };
        assert_ne!(output, not_expected_output);
    }

    #[test]
    fn test_error_when_not_found_tag() {
        let dummy_tag = get_dummy_tag("dummy_tag");
        let repository = TestTagRepository::new();
        repository.create(dummy_tag).unwrap();
        let sut = GetTagInteractor::new(repository);
        let input = GetTagInputData {
            hash: String::from_str("not_registered_hash").unwrap(),
        };
        assert!(sut.handle(input).is_err());
    }
}
