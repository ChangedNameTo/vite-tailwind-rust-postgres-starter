use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use std::env;

/// Type alias for a Diesel connection pool
pub type DbPool = Pool<ConnectionManager<PgConnection>>;

/// Establishes a connection pool for Diesel
pub fn establish_connection() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(5) // Equivalent to `max_connections(5)`
        .build(manager)
        .expect("Failed to create pool.")
}
