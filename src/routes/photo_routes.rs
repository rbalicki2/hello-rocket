use diesel;
use rocket_contrib::JSON;
use rocket::State;
use diesel::prelude::*;

use models::Photo;
use models::NewPhoto;
use models::User;
use models::Id;
use models::LimitOffsetParam;

use app::errors::{Result, ResultExt};
use app::db;
use app::schema;

#[get("/users/<user>/photos")]
pub fn get_photos_for_user(
  db_pool: State<db::ConnectionPool>,
  user: User
) -> Result<JSON<Vec<Photo>>> {
  use app::schema::users::dsl::users;
  use app::schema::photos::dsl::{photos,user_id};
  let conn: db::DbConnection = db_pool.get().chain_err(|| "Could not connect to DB")?;

  // TODO do proper ORM stuff like user.get_photos
  let photos_vec: Vec<Photo> = photos
    .filter(user_id.eq(user.id))
    .load::<Photo>(&*conn)
    .chain_err(|| "Error loading photos")?;

  Ok(JSON(photos_vec))
}

