use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

use crate::config::CONFIG;

type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Database {
    pool: DBPool,
}

impl Database {
    pub fn new() -> Self {
        let manager = ConnectionManager::<PgConnection>::new(&*CONFIG.database_url());
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("failed to create pool.");
        Database { pool }
    }

    pub fn get_connection(&self) -> r2d2::PooledConnection<ConnectionManager<PgConnection>> {
        self.pool
            .get()
            .expect("failed to get a database connection")
    }
}
