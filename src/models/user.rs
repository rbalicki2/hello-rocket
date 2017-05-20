use models::Id;

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct User {
  pub id: Id,
  pub name: String,
}
