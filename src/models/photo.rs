use models::Id;
use app::schema::photos;
use models::User;
use diesel;
use app::db;
use app::errors::{Result, ResultExt, Error};
use diesel::prelude::*;
use rocket::request::FromParam;

#[derive(Queryable, Associations, Serialize, Deserialize, Identifiable, Clone)]
#[belongs_to(User)]
pub struct Photo {
  pub id: Id,
  pub user_id: Id,
  pub url: String,
}

impl<'a> FromParam<'a> for Photo {
  type Error = Error;
  fn from_param(param: &'a str) -> Result<Photo> {
    use app::schema::photos::dsl::photos;
    let conn = db::establish_connection();
    let id: i32 = str::parse::<i32>(param)
      .chain_err(|| "Invalid photo ID")?;

    let photo: Photo = photos
      .find(id)
      .first::<Photo>(&conn)
      .chain_err(|| "Could not find photo")?;

    Ok(photo)
  }
}

#[derive(Deserialize, Clone)]
pub struct NewPhoto {
  pub url: String,
}

impl NewPhoto {
  pub fn to_insertable(&self, user: User) -> InsertablePhoto {
    InsertablePhoto {
      url: self.url.to_string(),
      user_id: user.id,
    }
  }
}

#[table_name="photos"]
#[derive(Insertable, Deserialize, Clone)]
pub struct InsertablePhoto {
  pub url: String,
  pub user_id: Id,
}

impl InsertablePhoto {
  pub fn insert(
    &self,
    conn: &db::DbConnection
  ) -> Result<Photo> {
    let photo: Photo = diesel::insert(self)
      .into(photos::table)
      .get_result(&**conn)
      .chain_err(|| "Could not insert photo")?;

    Ok(photo)
  }
}