#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct User {
  pub id: i32,
  pub name: String,
}
