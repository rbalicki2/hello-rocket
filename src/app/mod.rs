use rocket;
use rocket::State;
use rocket::response::Result;
use diesel::prelude::*;
use rocket_contrib::JSON;

use dotenv::dotenv;

pub mod db;
pub mod schema;
pub mod errors;

use models::User;

#[get("/")]
pub fn index(db_pool: State<db::ConnectionPool>) -> JSON<User> {
  use self::schema::users::dsl::users;
  // TODO handle errors here
  let conn: db::DbConnection = db_pool.get().unwrap();

  let users_query: User = users.first::<User>(&*conn).unwrap();

  JSON(users_query)
}

pub fn app() -> rocket::Rocket {
  dotenv().ok();
  rocket::ignite()
    .manage(db::establish_connection_pool())
    .mount("/", routes![index])
}
