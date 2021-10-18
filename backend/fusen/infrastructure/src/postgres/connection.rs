use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;
use std::env;

pub fn establish_connections() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    r2d2::Pool::builder().build_unchecked(manager)
}
