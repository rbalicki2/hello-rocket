use diesel::types::{Text,FromSqlRow};
use diesel::row::Row;
use std::error::Error;
use diesel::pg::Pg;
use std::str::from_utf8;
use std::fmt;

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

#[derive(Debug)]
pub struct InvalidUserRoleError(i32);

impl Error for InvalidUserRoleError {
  fn description(&self) -> &str { "Invalid user role error" }
}

impl fmt::Display for InvalidUserRoleError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Invalid User Role")
  }
}

unsafe impl Send for InvalidUserRoleError {}
unsafe impl Sync for InvalidUserRoleError {}

impl FromSqlRow<Text, Pg> for UserRole {
  fn build_from_row<T: Row<Pg>>(row: &mut T) -> Result<Self, Box<Error + Send + Sync>> {
    let raw_value = row.take().unwrap();
    let raw_str: Option<UserRole> = from_utf8(raw_value)
      .ok()
      .and_then(UserRole::from_string);

    return raw_str.ok_or(Box::new(InvalidUserRoleError(1)));
  }
}
