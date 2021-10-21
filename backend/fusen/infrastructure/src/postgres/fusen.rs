use crate::postgres::models::*;
use crate::postgres::schema::fusens;
use crate::postgres::DbPool;
use anyhow::{Error, Result};
use diesel::prelude::*;
use domain::entity::*;
use domain::repository::*;
use domain::vo::*;

#[derive(Clone)]
pub struct FusenRepository {
    connections: DbPool,
}

impl FusenRepository {
    pub fn new(connections: DbPool) -> Self {
        Self { connections }
    }

    fn create_with_conn(&self, conn: &PgConnection, aggregate: Fusen) -> Result<(), Error> {
        diesel::insert_into(fusens::table)
            .values(&NewFusenModel {
                id: aggregate.id().clone().to_string(),
                title: aggregate.title().clone().to_string(),
                note: aggregate.note().clone().to_string(),
            })
            .execute(conn)?;

        Ok(())
    }

    fn get_with_conn(&self, conn: &PgConnection, id: Id<Fusen>) -> Result<Fusen, Error> {
        let fusen = fusens::table
            .select((fusens::id, fusens::title, fusens::note))
            .filter(fusens::id.eq(id.to_string()))
            .first::<FusenModel>(conn)?;

        Ok(FusenBuilder::default()
            .id(fusen.id.parse::<Id<Fusen>>().unwrap())
            .title(fusen.title.parse::<FusenTitle>().unwrap())
            .note(fusen.note.parse::<FusenNote>().unwrap())
            .build()?)
    }

    fn delete_with_conn(&self, conn: &PgConnection, aggregate: Fusen) -> Result<(), Error> {
        diesel::delete(fusens::table.filter(fusens::id.eq(aggregate.id().to_string())))
            .execute(conn)?;

        Ok(())
    }
}

impl CreateRepository<Fusen> for FusenRepository {
    fn create(&self, aggregate: Fusen) -> Result<(), Error> {
        let conn = self.connections.pool().get()?;

        self.create_with_conn(&conn, aggregate)
    }
}

impl GetRepository<Fusen> for FusenRepository {
    fn get(&self, id: Id<Fusen>) -> Result<Fusen, Error> {
        let conn = self.connections.pool().get()?;

        self.get_with_conn(&conn, id)
    }
}

impl DeleteRepository<Fusen> for FusenRepository {
    fn delete(&self, aggregate: Fusen) -> Result<(), Error> {
        let conn = self.connections.pool().get()?;

        self.delete_with_conn(&conn, aggregate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Once;
    use ulid::Ulid;

    static INIT: Once = Once::new();

    pub fn init(connections: DbPool) {
        INIT.call_once(|| {
            connections.init().unwrap();
        });
    }

    #[test]
    fn test_fusen_repository_create() {
        let database_url = "postgres://postgres:postgres@localhost/peta_test";
        let connections = DbPool::new(database_url);

        init(connections.clone());

        let conn = connections.pool().get().unwrap();
        conn.test_transaction::<_, Error, _>(|| {
            let entity = FusenBuilder::default()
                .id(Ulid::new().to_string().parse::<Id<Fusen>>().unwrap())
                .title("title".parse::<FusenTitle>().unwrap())
                .note("note".parse::<FusenNote>().unwrap())
                .build()
                .unwrap();

            let sut = FusenRepository::new(connections);

            assert!(sut.create_with_conn(&conn, entity).is_ok());

            Ok(())
        });
    }

    #[test]
    fn test_fusen_repository_get() {
        let database_url = "postgres://postgres:postgres@localhost/peta_test";
        let connections = DbPool::new(database_url);

        init(connections.clone());

        let conn = connections.pool().get().unwrap();
        conn.test_transaction::<_, Error, _>(|| {
            let entity = FusenBuilder::default()
                .id(Ulid::new().to_string().parse::<Id<Fusen>>().unwrap())
                .title("title".parse::<FusenTitle>().unwrap())
                .note("note".parse::<FusenNote>().unwrap())
                .build()
                .unwrap();

            let sut = FusenRepository::new(connections);
            sut.create_with_conn(&conn, entity.clone()).unwrap();

            assert!(sut.get_with_conn(&conn, entity.id().clone()).is_ok());

            Ok(())
        });
    }

    #[test]
    fn test_fusen_repository_delete() {
        let database_url = "postgres://postgres:postgres@localhost/peta_test";
        let connections = DbPool::new(database_url);

        init(connections.clone());

        let conn = connections.pool().get().unwrap();
        conn.test_transaction::<_, Error, _>(|| {
            let entity = FusenBuilder::default()
                .id(Ulid::new().to_string().parse::<Id<Fusen>>().unwrap())
                .title("title".parse::<FusenTitle>().unwrap())
                .note("note".parse::<FusenNote>().unwrap())
                .build()
                .unwrap();

            let sut = FusenRepository::new(connections);
            sut.create_with_conn(&conn, entity.clone()).unwrap();

            assert!(sut.delete_with_conn(&conn, entity).is_ok());

            Ok(())
        });
    }
}
