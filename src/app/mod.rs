use rocket;
use rocket::State;
use diesel::prelude::*;
use diesel;
use rocket_contrib::JSON;

use dotenv::dotenv;

pub mod db;
pub mod schema;
pub mod errors;

use models::GetUser;
use models::PostUser;
use models::Id;

use self::errors::{Result, Error, ErrorKind, ResultExt};

#[get("/users/<id>")]
pub fn get_user(db_pool: State<db::ConnectionPool>, id: Id) -> Result<JSON<GetUser>> {
  use self::schema::users::dsl::users;
  let conn: db::DbConnection = db_pool.get().chain_err(|| "Could not connect to DB")?;

  let user: GetUser = users
    .find(id)
    .first::<GetUser>(&*conn)
    .chain_err(|| "Could not find user")?;

  Ok(JSON(user))
}

#[post("/users", data="<user_data>")]
pub fn create_user(db_pool: State<db::ConnectionPool>, user_data: JSON<PostUser>) -> Result<JSON<GetUser>> {
//  use self::schema::users::dsl::users;
//  let conn: db::DbConnection = db_pool.get().chain_err(|| "Could not connect to DB")?;
  // TODO use the pooled connection. But:
  // the trait `diesel::Connection` is not implemented for
//   `r2d2::PooledConnection<r2d2_diesel::ConnectionManager<diesel::pg::PgConnection>>`
  let conn = db::establish_connection();

  let user: PostUser = user_data.0;

  let returned_user: GetUser = diesel::insert(&user)
    .into(schema::users::table)
    .get_result(&conn)
    .chain_err(|| "Error inserting user")?;

  Ok(JSON(returned_user))
}

pub fn app() -> rocket::Rocket {
  dotenv().ok();
  rocket::ignite()
    .manage(db::establish_connection_pool())
    .mount("/", routes![get_user, create_user])
}
