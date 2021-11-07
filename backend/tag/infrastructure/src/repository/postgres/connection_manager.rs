use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager as R2D2ConnectionManager;
use diesel::r2d2::{Pool, PooledConnection};
use anyhow::{bail, Error};
use std::time::Duration;

pub struct ConnectionManager {
    pool: Pool<R2D2ConnectionManager<PgConnection>>,
}

impl ConnectionManager {
    pub fn new(url: String, connection_timeout: Duration) -> Self {
        let m = R2D2ConnectionManager::<PgConnection>::new(url);
        let pool = Pool::<R2D2ConnectionManager<PgConnection>>::builder()
            .connection_timeout(connection_timeout)
            .build(m)
            .unwrap();
        ConnectionManager { pool }
    }

    pub fn connection(
        &self,
    ) -> Result<PooledConnection<R2D2ConnectionManager<PgConnection>>, Error> {
        match self.pool.get() {
            Ok(con) => Ok(con),
            Err(e) => bail!("{:#?}", e.to_string()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::repository::postgres::env::test_env_util;
    #[test]
    fn generate_connection_manager() {
        let cm =
            ConnectionManager::new(test_env_util::var("TAG_DATABASE_URL"), Duration::new(5, 0));
        assert!(cm.connection().is_ok());
    }
}
