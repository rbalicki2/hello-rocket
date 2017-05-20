use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use r2d2;
use r2d2_diesel;
use std;

use r2d2::PooledConnection;
use r2d2_diesel::ConnectionManager;

pub type ConnectionPool = r2d2::Pool<r2d2_diesel::ConnectionManager<diesel::pg::PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn establish_connection_pool() -> ConnectionPool {
  let config = r2d2::Config::default();
  let connection_manager = r2d2_diesel::ConnectionManager::<PgConnection>::new(std::env::var("DATABASE_URL").unwrap());
  r2d2::Pool::new(config, connection_manager).unwrap()
}

pub fn establish_connection() -> PgConnection {
  // TODO handle errors better
  let database_url = std::env::var("DATABASE_URL").unwrap();
  PgConnection::establish(&database_url).unwrap()
}
