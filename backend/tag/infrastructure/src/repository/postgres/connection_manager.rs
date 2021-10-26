use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager as R2D2ConnectionManager;
use diesel::r2d2::{Pool, PooledConnection};
use r2d2::Error;
use std::default::Default;
use std::time::Duration;

pub struct Config {
    pub url: String,
    pub connection_timeout: std::time::Duration,
    // etc
    // TODO: remove pub and implement getter
}

impl Default for Config {
    fn default() -> Config {
        Config {
            url: String::from(""),
            connection_timeout: Duration::new(5, 0),
        }
    }
}

pub struct ConnectionManager {
    pool: Pool<R2D2ConnectionManager<PgConnection>>,
}

impl ConnectionManager {
    pub fn new(config: Config) -> Self {
        let m = R2D2ConnectionManager::<PgConnection>::new(config.url);
        let pool = Pool::<R2D2ConnectionManager<PgConnection>>::builder()
            .connection_timeout(config.connection_timeout)
            .build(m)
            .unwrap();
        ConnectionManager { pool }
    }

    pub fn connection(
        &self,
    ) -> Result<PooledConnection<R2D2ConnectionManager<PgConnection>>, Error> {
        self.pool.get()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::repository::postgres::env::test_env_util;
    use std::default::Default;
    #[test]
    fn generate_connection_manager() {
        let config = Config {
            url: test_env_util::var("TAG_DATABASE_URL"),
            ..Default::default()
        };
        let cm = ConnectionManager::new(config);
        // test not error occured
        assert!(cm.connection().is_ok());
    }
}
