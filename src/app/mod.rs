use rocket;
use rocket::State;
use diesel::prelude::*;
use r2d2::PooledConnection;
use r2d2_diesel::ConnectionManager;
use diesel::pg::PgConnection;

use dotenv::dotenv;

pub mod db;
pub mod schema;
pub mod errors;

use self::errors::*;

use models::User;

#[get("/")]
pub fn index(db_pool: State<db::ConnectionPool>) -> Result<String> {
  use self::schema::users::dsl::users;
  let conn: PooledConnection<ConnectionManager<PgConnection>> = db_pool.get().chain_err(|| "Could not establish connection")?;

  let results: Vec<User> = users.load::<User>(&*conn).chain_err(|| "Could not query DB")?;

  Ok(format!("Hello, world, {}", results.len()))
}

pub fn app() -> rocket::Rocket {
  dotenv().ok();
  rocket::ignite()
    .manage(db::establish_connection_pool())
    .mount("/", routes![index])
}
