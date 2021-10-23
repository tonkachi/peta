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
        let tag = self.tag_repository.get(input.hash.parse::<TagHash>()?)?;
        let fusen_ids = tag.fusen_ids().iter().map(|x| x.to_string()).collect();
        Ok(GetTagOutputData {
            hash: tag.hash().clone().to_string(),
            name: tag.name().clone().to_string(),
            fusen_ids,
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

        fn get_by_fusen_id(&self, fusen_id: FusenId) -> Result<Vec<Tag>, Error> {
            let tags = self.tags.lock().unwrap();
            let filtered_tags = tags
                .values()
                .filter(|&x| x.fusen_ids().contains(&fusen_id))
                .map(|x| x.clone())
                .collect();
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
    fn test_succeeded() {
        let dummy_tag = get_dummy_tag(&from_str!(String, "dummy_tag"));
        let repository = TestTagRepository::new();
        repository.create(dummy_tag).unwrap();
        let sut = GetTagInteractor::new(repository);
        let input = GetTagInputData {
            hash: from_str!(String, "dummy_tag"),
        };
        let output = sut.handle(input).unwrap();
        let expected_output = GetTagOutputData {
            hash: from_str!(String, "dummy_tag"),
            name: from_str!(String, "dummy_tag"),
            fusen_ids: vec![from_str!(String, "dummy_tag")],
        };
        assert_eq!(output, expected_output);
        let not_expected_output = GetTagOutputData {
            hash: from_str!(String, "fake_dummy_tag"),
            name: from_str!(String, "fake_dummy_tag"),
            fusen_ids: vec![from_str!(String, "fake_dummy_tag")],
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
