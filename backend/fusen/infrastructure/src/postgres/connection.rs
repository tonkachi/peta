use anyhow::{Error, Result};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use diesel_migrations::embed_migrations;
use r2d2::Pool;

embed_migrations!("../migrations");

#[derive(Clone)]
pub struct DbPool(Pool<ConnectionManager<PgConnection>>);

impl DbPool {
    pub fn new(database_url: &str) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder().build_unchecked(manager);

        Self(pool)
    }

    pub fn init(&self) -> Result<(), Error> {
        let conn = self.pool().get()?;

        embedded_migrations::run_with_output(&conn, &mut std::io::stdout())?;

        Ok(())
    }

    pub fn pool(&self) -> Pool<ConnectionManager<PgConnection>> {
        self.0.clone()
    }
}
