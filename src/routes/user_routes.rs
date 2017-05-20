use diesel;
use rocket_contrib::JSON;
use rocket::State;
use diesel::prelude::*;

use models::User;
use models::NewUser;
use models::Id;

use app::errors::{Result, Error, ErrorKind, ResultExt};
use app::db;
use app::schema;

#[get("/users/<id>")]
pub fn get_user(db_pool: State<db::ConnectionPool>, id: Id) -> Result<JSON<User>> {
  use app::schema::users::dsl::users;
  let conn: db::DbConnection = db_pool.get().chain_err(|| "Could not connect to DB")?;

  let user: User = users
    .find(id)
    .first::<User>(&*conn)
    .chain_err(|| "Could not find user")?;

  Ok(JSON(user))
}

#[post("/users", data="<user_data>")]
pub fn create_user(user_data: JSON<NewUser>) -> Result<JSON<User>> {
  // TODO figure out why you can't use a connection pool
  let conn = db::establish_connection();

  let user: NewUser = user_data.0;

  let returned_user: User = diesel::insert(&user)
    .into(schema::users::table)
    .get_result(&conn)
    .chain_err(|| "Error inserting user")?;

  Ok(JSON(returned_user))
}