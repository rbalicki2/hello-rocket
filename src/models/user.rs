use models::Id;
use app::schema::*;

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct GetUser {
  pub id: Id,
  pub name: String,
}

#[table_name="users"]
#[derive(Insertable, Deserialize, Clone)]
pub struct PostUser {
  pub name: String,
}
