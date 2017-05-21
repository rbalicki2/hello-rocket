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

#[table_name="photos"]
#[derive(Insertable, Deserialize, Clone)]
pub struct NewPhoto {
  pub user_id: Id,
  pub url: String,
}
