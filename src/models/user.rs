use models::Id;
use app::schema::users;

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct User {
  pub id: Id,
  pub name: String,
}

#[table_name="users"]
#[derive(Insertable, Deserialize, Clone)]
pub struct NewUser {
  pub name: String,
}
