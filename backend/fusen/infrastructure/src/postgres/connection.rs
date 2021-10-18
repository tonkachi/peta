use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;

pub fn establish_connections(database_url: &str) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder().build_unchecked(manager)
}
