use diesel::types::{Text,FromSqlRow};
use diesel::row::Row;
use std::error::Error;
use diesel::pg::Pg;
use std::str::from_utf8;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum UserRole {
  Admin,
  User
}

impl UserRole {
  fn from_string(s: &str) -> Option<UserRole> {
    match s {
      "ADMIN" => Some(UserRole::Admin),
      "USER" => Some(UserRole::User),
      _ => None
    }
  }
}

impl FromSqlRow<Text, Pg> for UserRole {
  fn build_from_row<T: Row<Pg>>(row: &mut T) -> Result<Self, Box<Error + Send + Sync>> {
    let raw_value = row.take().unwrap();
    let raw_str = from_utf8(raw_value);

    match raw_str {
      Ok(val) => Ok(UserRole::from_string(val).unwrap()),
      Err(_) => panic!("i dont know how to return yet")
    }
  }
}
