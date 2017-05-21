use models::Id;
use app::schema::{photos};
use models::User;

#[derive(Queryable, Associations, Serialize, Deserialize, Identifiable, Clone)]
#[belongs_to(User)]
pub struct Photo {
  pub id: Id,
  pub user_id: Id,
  pub url: String,
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
