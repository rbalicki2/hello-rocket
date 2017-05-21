use rocket_contrib::JSON;
use rocket::State;
use diesel::prelude::*;

use models::Photo;
use models::NewPhoto;
use models::User;

use app::errors::{Result,ResultExt};
use app::db;

#[get("/users/<user>/photos")]
pub fn get_photos_for_user(
  db_pool: State<db::ConnectionPool>,
  user: User
) -> Result<JSON<Vec<Photo>>> {
  use app::schema::photos::dsl::{photos,user_id};
  let conn: db::DbConnection = db_pool.get().chain_err(|| "Could not connect to DB")?;

  // TODO do proper ORM stuff like user.get_photos
  let photos_vec: Vec<Photo> = photos
    .filter(user_id.eq(user.id))
    .load::<Photo>(&*conn)
    .chain_err(|| "Error loading photos")?;

  Ok(JSON(photos_vec))
}

#[post("/users/<user>/photos", data="<photo_data>")]
pub fn create_new_photo_for_user(
  db_pool: State<db::ConnectionPool>,
  user: User,
  photo_data: JSON<NewPhoto>
) -> Result<JSON<Photo>> {
  let conn: db::DbConnection = db_pool.get().chain_err(|| "Could not connect to DB")?;
  let insertable_photo = photo_data.0.to_insertable(user);
  let returned_photo: Photo = insertable_photo.insert(&conn)?;

  Ok(JSON(returned_photo))
}

#[get("/users/<user>/photos/<photo>")]
pub fn get_photo_for_user(
  user: User,
  photo: Photo
) -> Result<JSON<Photo>> {
  if photo.user_id != user.id {
    return Err("Photo not found")?;
  }
  Ok(JSON(photo))
}
