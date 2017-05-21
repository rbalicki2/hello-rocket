use std::str;
use rocket::request::FromParam;
use diesel::prelude::*;

use models::Id;
use app::schema::{users,photos};
use app::db;
use app::errors::{Result, ResultExt, Error};

#[derive(Queryable, Serialize, Deserialize, Associations, Identifiable, Clone)]
#[has_many(photos)]
pub struct User {
  pub id: Id,
  pub name: String,
}

impl<'a> FromParam<'a> for User {
  type Error = Error;
  fn from_param(param: &'a str) -> Result<User> {
    use app::schema::users::dsl::users;
    let conn = db::establish_connection();
    let id: i32 = str::parse::<i32>(param)
      .chain_err(|| "Invalid user ID")?;

    let user: User = users
      .find(id)
      .first::<User>(&conn)
      .chain_err(|| "Could not find user")?;

    Ok(user)
  }
}

#[table_name="users"]
#[derive(Insertable, Deserialize, Clone)]
pub struct NewUser {
  pub name: String,
}
