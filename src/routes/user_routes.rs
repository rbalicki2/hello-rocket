use diesel;
use rocket_contrib::JSON;
use rocket::State;
use diesel::prelude::*;
use itertools::Itertools;

use models::User;
use models::NewUser;
use models::Photo;
use models::LimitOffsetParam;

use app::errors::{Result, ResultExt};
use app::db;
use app::schema;

#[get("/users/<user>")]
pub fn get_user(user: User) -> JSON<User> {
  JSON(user)
}

#[post("/users", data="<user_data>")]
pub fn create_user(db_pool: State<db::ConnectionPool>, user_data: JSON<NewUser>) -> Result<JSON<User>> {
  // TODO figure out why you can't use a connection pool
  let conn: db::DbConnection = db_pool.get().chain_err(|| "Could not connect to DB")?;

  let user: NewUser = user_data.0;

  let returned_user: User = diesel::insert(&user)
    .into(schema::users::table)
    .get_result(&*conn)
    .chain_err(|| "Error inserting user")?;

  Ok(JSON(returned_user))
}

#[get("/users?<limit_offset>")]
pub fn get_users(db_pool: State<db::ConnectionPool>, limit_offset: LimitOffsetParam) -> Result<JSON<Vec<i32>>> {
  use app::schema::users::dsl::users;
  let conn: db::DbConnection = db_pool.get().chain_err(|| "Could not connect to DB")?;

  let limit: i64 = limit_offset.limit.unwrap_or(100) as i64;
  let offset: i64 = limit_offset.offset.unwrap_or(0) as i64;

  let users_response: Vec<i32> = users
    .left_outer_join(schema::photos::table)
    .load(&*conn)
    .chain_err(|| "could not load")?
    .iter()
//    .map(|x: (User, Photo)| 42)
    .collect();
//    .iter()

//    .group_by(|(user, photo)| user)
//    .map(|(user, photos)| user)
//    .collect();
//    .offset(offset)
//    .limit(limit)
//    .load::<User>(&*conn)
//    .chain_err(|| "Could not query users")?;

  Ok(JSON(users_response))
}