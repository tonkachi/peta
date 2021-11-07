use super::models::{Tag as TagModel, TagData, TagFusenId, TagsFusendIdsData};
use super::schema;
use super::ConnectionManager;
use crate::domain::aggregate::Tag;
use crate::domain::entity::TagBuilder;
use crate::domain::repository::TagRepository;
use anyhow::{bail, Error};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager as R2D2ConnectionManager, PooledConnection};
use domain::vo::{FusenId, TagHash, TagName};
use std::str::FromStr;

macro_rules! from_str {
    ($t:ty, $s:expr) => {
        <$t>::from_str($s).unwrap()
    };
}

pub struct PostgresRepository {
    connection_manager: ConnectionManager,
}

impl PostgresRepository {
    pub fn new(cm: ConnectionManager) -> PostgresRepository {
        PostgresRepository {
            connection_manager: cm,
        }
    }

    fn connection(&self) -> Result<PooledConnection<R2D2ConnectionManager<PgConnection>>, Error> {
        self.connection_manager.connection()
    }

    fn create_with_conn(
        conn: &PooledConnection<R2D2ConnectionManager<PgConnection>>,
        entity: Tag,
    ) -> Result<(), Error> {
        // TODO: support transaction
        let insert_data = TagData {
            hash: entity.hash().to_string(),
            name: entity.name().to_string(),
        };
        diesel::insert_into(schema::tags::table)
            .values(&insert_data)
            .execute(conn)?;

        let fusen_ids: Vec<TagsFusendIdsData> = entity
            .fusen_ids()
            .iter()
            .map(|f| TagsFusendIdsData {
                tag_hash: entity.hash().to_string(),
                fusen_id: f.to_string(),
            })
            .collect();
        diesel::insert_into(schema::tags_fusen_ids::table)
            .values(fusen_ids)
            .execute(conn)?;
        Ok(())
    }

    fn delete_with_conn(
        conn: &PooledConnection<R2D2ConnectionManager<PgConnection>>,
        entity: Tag,
    ) -> Result<(), Error> {
        // TODO: support transaction
        let tag = Self::get_with_conn(conn, entity.hash().clone())?;
        let fusen_ids: Vec<String> = tag
            .fusen_ids()
            .iter()
            .map(|fusen_id| fusen_id.to_string())
            .collect();
        let filter = schema::tags_fusen_ids::table.filter(
            schema::tags_fusen_ids::tag_hash
                .eq(tag.hash().to_string())
                .and(schema::tags_fusen_ids::fusen_id.eq_any(fusen_ids)),
        );
        diesel::delete(filter).execute(conn)?;
        diesel::delete(schema::tags::table.filter(schema::tags::hash.eq(tag.hash().to_string())))
            .execute(conn)?;
        Ok(())
    }

    fn get_with_conn(
        conn: &PooledConnection<R2D2ConnectionManager<PgConnection>>,
        hash: TagHash,
    ) -> Result<Tag, Error> {
        //TODO: Option<Tag> を返したほうが良い？
        let results = schema::tags::table
            .filter(schema::tags::hash.eq(hash.to_string()))
            .left_join(
                schema::tags_fusen_ids::table
                    .on(schema::tags::hash.eq(schema::tags_fusen_ids::tag_hash)),
            )
            // MEMO:
            //.select((
            //    schema::tags::all_columns,
            //    //(schema::tags_fusen_ids::all_columns).nullable()
            //    //schema::tags::hash,
            //    //schema::tags::name,
            //    //schema::tags_fusen_ids::fusen_id.nullable(),
            //))
            .load::<(TagModel, Option<TagFusenId>)>(conn)?;

        if results.is_empty() {
            bail!("tag not found");
        }

        let first_data = match results.get(0) {
            Some(data) => data,
            None => bail!("tag not found"),
        };

        let tag = &first_data.0.clone();
        let mut fusen_ids: Vec<FusenId> = Vec::new();
        for data in results {
            let fusen_id = data.1;
            if let Some(fusen) = fusen_id {
                fusen_ids.push(from_str!(FusenId, &fusen.fusen_id))
            }
        }

        let tag = TagBuilder::default()
            .hash(from_str!(TagHash, &tag.hash))
            .name(from_str!(TagName, &tag.name))
            .fusen_ids(fusen_ids)
            .build()?;
        Ok(tag)
    }

    fn get_by_fusen_id_with_conn(
        conn: &PooledConnection<R2D2ConnectionManager<PgConnection>>,
        fusen_id: FusenId,
    ) -> Result<Vec<Tag>, Error> {
        let results = schema::tags_fusen_ids::table
            .filter(schema::tags_fusen_ids::fusen_id.eq(fusen_id.to_string()))
            .inner_join(schema::tags::table)
            .load::<(TagFusenId, TagModel)>(conn)?;
        results
            .iter()
            .map(|data| {
                let tag_data = &data.1;
                // TODO: SQL performance tuning
                Self::get_with_conn(conn, from_str!(TagHash, &tag_data.hash))
            })
            .collect()
    }

    fn update_tag_with_conn(
        conn: &PooledConnection<R2D2ConnectionManager<PgConnection>>,
        entity: Tag,
    ) -> Result<(), Error> {
        // TODO: support transaction
        let current_tag = Self::get_with_conn(conn, entity.hash().clone())?;
        let mut delete_fusen_ids: Vec<String> = Vec::new();
        let mut insert_fusen_ids: Vec<TagsFusendIdsData> = Vec::new();

        for fusen_id in entity.fusen_ids() {
            if !current_tag.fusen_ids().contains(fusen_id) {
                insert_fusen_ids.push(TagsFusendIdsData {
                    tag_hash: current_tag.hash().to_string(),
                    fusen_id: fusen_id.to_string(),
                });
            }
        }

        for fusen_id in current_tag.fusen_ids() {
            if !entity.fusen_ids().contains(fusen_id) {
                delete_fusen_ids.push(fusen_id.to_string())
            }
        }

        let filter = schema::tags_fusen_ids::table.filter(
            schema::tags_fusen_ids::tag_hash
                .eq(current_tag.hash().to_string())
                .and(schema::tags_fusen_ids::fusen_id.eq_any(delete_fusen_ids)),
        );
        diesel::delete(filter).execute(conn)?;
        diesel::insert_into(schema::tags_fusen_ids::table)
            .values(insert_fusen_ids)
            .execute(conn)?;

        let tag_filter =
            schema::tags::table.filter(schema::tags::hash.eq(current_tag.hash().to_string()));
        diesel::update(tag_filter)
            .set(schema::tags::name.eq(entity.name().to_string()))
            .execute(conn)?;

        Ok(())
    }
}

impl TagRepository for PostgresRepository {
    fn create(&self, entity: Tag) -> Result<(), Error> {
        let conn = self.connection().unwrap();
        Self::create_with_conn(&conn, entity)
    }

    fn delete(&self, entity: Tag) -> Result<(), Error> {
        let conn = self.connection().unwrap();
        Self::delete_with_conn(&conn, entity)
    }

    fn get(&self, hash: TagHash) -> Result<Tag, Error> {
        let conn = self.connection().unwrap();
        Self::get_with_conn(&conn, hash)
    }

    fn get_by_fusen_id(&self, fusen_id: FusenId) -> Result<Vec<Tag>, Error> {
        let conn = self.connection().unwrap();
        Self::get_by_fusen_id_with_conn(&conn, fusen_id)
    }

    fn update_tag(&self, entity: Tag) -> Result<(), Error> {
        let conn = self.connection().unwrap();
        Self::update_tag_with_conn(&conn, entity)
    }
}

#[cfg(test)]
mod test {
    use super::ConnectionManager;
    use super::*;
    use crate::repository::postgres::env::test_env_util;
    use diesel::result::Error as DError;
    use domain::vo::{FusenId, TagHash, TagName};
    use std::time::Duration;

    fn get_conn() -> PooledConnection<R2D2ConnectionManager<PgConnection>> {
        let cm = ConnectionManager::new(
            test_env_util::var("TAG_TEST_DATABASE_URL"),
            Duration::new(5, 0),
        );
        let repo = PostgresRepository::new(cm);
        repo.connection().unwrap()
    }

    fn assert_tag_eq(t1: Tag, t2: Tag) {
        assert_eq!(t1, t2);
        assert_eq!(t1.name(), t2.name());
        assert_eq!(t1.fusen_ids().len(), t2.fusen_ids().len());
        for fusen_id in t1.fusen_ids() {
            assert!(t2.fusen_ids().contains(fusen_id));
        }
    }

    fn set_db_data(conn: &PooledConnection<R2D2ConnectionManager<PgConnection>>) -> Vec<Tag> {
        let tag = TagBuilder::default()
            .hash(from_str!(TagHash, "test_tag_hash"))
            .name(from_str!(TagName, "test_tag"))
            .fusen_ids(vec![from_str!(FusenId, "f1"), from_str!(FusenId, "f2")])
            .build()
            .unwrap();

        let tag2 = TagBuilder::default()
            .hash(from_str!(TagHash, "test_tag_hash_2"))
            .name(from_str!(TagName, "test_tag2"))
            .fusen_ids(vec![from_str!(FusenId, "f1"), from_str!(FusenId, "f3")])
            .build()
            .unwrap();

        let tag3 = TagBuilder::default()
            .hash(from_str!(TagHash, "test_tag_hash_3"))
            .name(from_str!(TagName, "test_tag3"))
            .fusen_ids(vec![from_str!(FusenId, "f1"), from_str!(FusenId, "f3")])
            .build()
            .unwrap();
        let tags = vec![tag.clone(), tag2.clone(), tag3.clone()];
        for t in tags {
            PostgresRepository::create_with_conn(&conn, t).unwrap();
        }
        vec![tag, tag2, tag3]
    }

    #[test]
    fn create() {
        let conn = get_conn();
        conn.test_transaction::<_, DError, _>(|| {
            let new_tag = TagBuilder::default()
                .hash(from_str!(TagHash, "test_tag_hash"))
                .name(from_str!(TagName, "test_tag"))
                .fusen_ids(vec![
                    from_str!(FusenId, "fusenid1"),
                    from_str!(FusenId, "fusenid2"),
                ])
                .build()
                .unwrap();
            PostgresRepository::create_with_conn(&conn, new_tag.clone()).unwrap();
            let tag = PostgresRepository::get_with_conn(&conn, new_tag.hash().clone()).unwrap();
            assert_tag_eq(new_tag, tag);
            Ok(())
        });
    }

    #[test]
    fn delete() {
        let conn = get_conn();
        conn.test_transaction::<_, DError, _>(|| {
            let tags = set_db_data(&conn);
            assert!(PostgresRepository::delete_with_conn(&conn, tags[0].clone()).is_ok());
            assert!(PostgresRepository::get_with_conn(&conn, tags[0].hash().clone()).is_err());
            Ok(())
        });
    }

    #[test]
    fn get() {
        let conn = get_conn();
        conn.test_transaction::<_, DError, _>(|| {
            let tags = set_db_data(&conn);
            for t in tags {
                let selected_tag =
                    PostgresRepository::get_with_conn(&conn, t.hash().clone()).unwrap();
                assert_tag_eq(selected_tag, t.clone());
            }
            Ok(())
        });
    }

    #[test]
    fn get_by_fusen_id() {
        let conn = get_conn();
        conn.test_transaction::<_, DError, _>(|| {
            let tags = set_db_data(&conn);
            let target_fusen_id = "f3";
            let test_tags: Vec<Tag> = tags
                .iter()
                .filter(|t| t.fusen_ids().contains(&from_str!(FusenId, target_fusen_id)))
                .map(|t| t.clone())
                .collect();

            let fusen_id = from_str!(FusenId, target_fusen_id);
            let selected_tags =
                PostgresRepository::get_by_fusen_id_with_conn(&conn, fusen_id).unwrap();

            assert_eq!(selected_tags.len(), test_tags.len());
            test_tags.iter().for_each(|tt| {
                for t in &selected_tags {
                    if t.hash() == tt.hash() {
                        assert_tag_eq(t.clone(), tt.clone());
                    }
                }
            });
            Ok(())
        });
    }

    #[test]
    fn update_tag() {
        let conn = get_conn();
        conn.test_transaction::<_, DError, _>(|| {
            let _tags = set_db_data(&conn);
            let update_target_hash = "test_tag_hash_2";
            let new_tag = TagBuilder::default()
                .hash(from_str!(TagHash, update_target_hash))
                .name(from_str!(TagName, "update name"))
                .fusen_ids(vec![
                    from_str!(FusenId, "f4"),
                    from_str!(FusenId, "f3"),
                    from_str!(FusenId, "f5"),
                ])
                .build()
                .unwrap();
            assert!(PostgresRepository::update_tag_with_conn(&conn, new_tag.clone()).is_ok());
            let selected_tag =
                PostgresRepository::get_with_conn(&conn, from_str!(TagHash, &update_target_hash))
                    .unwrap();
            assert_tag_eq(selected_tag, new_tag.clone());
            Ok(())
        });
    }
}
