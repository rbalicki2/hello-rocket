use rocket;
use rocket::State;
use diesel::prelude::*;
use rocket_contrib::JSON;

use dotenv::dotenv;

pub mod db;
pub mod schema;
pub mod errors;

use models::User;
use models::Id;

use self::errors::{Result, Error, ErrorKind, ResultExt};

#[get("/<id>")]
pub fn index(db_pool: State<db::ConnectionPool>, id: Id) -> Result<JSON<User>> {
  use self::schema::users::dsl::users;
  // TODO handle errors here
//  let conn: db::DbConnection = db_pool.get().unwrap();
  let conn: db::DbConnection = db_pool.get().chain_err(|| "Could not connect to DB")?;

  let users_query: User = users
    .find(id)
    .first::<User>(&*conn)
    .chain_err(|| "Could not find user")?;

  Ok(JSON(users_query))
}

pub fn app() -> rocket::Rocket {
  dotenv().ok();
  rocket::ignite()
    .manage(db::establish_connection_pool())
    .mount("/", routes![index])
}
