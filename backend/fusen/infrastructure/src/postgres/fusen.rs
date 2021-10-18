use crate::postgres::establish_connections;
use crate::postgres::models::*;
use crate::postgres::schema::fusens;
use anyhow::{Error, Result};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use domain::entity::*;
use domain::repository::*;
use domain::vo::*;
use r2d2::Pool;

pub struct FusenRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl FusenRepository {
    pub fn default() -> Self {
        Self {
            pool: establish_connections(),
        }
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
        let conn = self.pool.get()?;

        self.create_with_conn(&conn, aggregate)
    }
}

impl GetRepository<Fusen> for FusenRepository {
    fn get(&self, id: Id<Fusen>) -> Result<Fusen, Error> {
        let conn = self.pool.get()?;

        self.get_with_conn(&conn, id)
    }
}

impl DeleteRepository<Fusen> for FusenRepository {
    fn delete(&self, aggregate: Fusen) -> Result<(), Error> {
        let conn = self.pool.get()?;

        self.delete_with_conn(&conn, aggregate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ulid::Ulid;

    fn establish_connection() -> PgConnection {
        let database_url = "postgres://postgres:postgres@localhost/peta_test";
        PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url))
    }

    #[test]
    fn test_fusen_repository_create() {
        let conn = establish_connection();
        conn.test_transaction::<_, Error, _>(|| {
            let entity = FusenBuilder::default()
                .id(Ulid::new().to_string().parse::<Id<Fusen>>().unwrap())
                .title("title".parse::<FusenTitle>().unwrap())
                .note("note".parse::<FusenNote>().unwrap())
                .build()
                .unwrap();

            let sut = FusenRepository::default();

            assert!(sut.create_with_conn(&conn, entity.clone()).is_ok());

            Ok(())
        });
    }

    #[test]
    fn test_fusen_repository_get() {
        let conn = establish_connection();
        conn.test_transaction::<_, Error, _>(|| {
            let entity = FusenBuilder::default()
                .id(Ulid::new().to_string().parse::<Id<Fusen>>().unwrap())
                .title("title".parse::<FusenTitle>().unwrap())
                .note("note".parse::<FusenNote>().unwrap())
                .build()
                .unwrap();

            let sut = FusenRepository::default();
            sut.create_with_conn(&conn, entity.clone()).unwrap();

            assert!(sut.get_with_conn(&conn, entity.id().clone()).is_ok());

            Ok(())
        });
    }

    #[test]
    fn test_fusen_repository_delete() {
        let conn = establish_connection();
        conn.test_transaction::<_, Error, _>(|| {
            let entity = FusenBuilder::default()
                .id(Ulid::new().to_string().parse::<Id<Fusen>>().unwrap())
                .title("title".parse::<FusenTitle>().unwrap())
                .note("note".parse::<FusenNote>().unwrap())
                .build()
                .unwrap();

            let sut = FusenRepository::default();
            sut.create_with_conn(&conn, entity.clone()).unwrap();

            assert!(sut.delete_with_conn(&conn, entity.clone()).is_ok());

            Ok(())
        });
    }
}
