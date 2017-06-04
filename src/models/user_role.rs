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

// I create my own error because I couldn't figure out what kind
// of boxed error I could return from build_from_row otherwise.
#[derive(Debug)]
pub struct InvalidUserRoleError();

impl Error for InvalidUserRoleError {
  fn description(&self) -> &str { "Invalid user role error" }
}

impl fmt::Display for InvalidUserRoleError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Invalid User Role")
  }
}

// seem to do nothing...?
//unsafe impl Send for InvalidUserRoleError {}
//unsafe impl Sync for InvalidUserRoleError {}

impl FromSqlRow<Text, Pg> for UserRole {
  fn build_from_row<T: Row<Pg>>(row: &mut T) -> Result<Self, Box<Error + Send + Sync + 'static>> {
    let the_error: Box<InvalidUserRoleError> = Box::new(InvalidUserRoleError());

    let a: Result<Self, Box<Error + Send + Sync + 'static>> = row.take()
      // option
      .ok_or(the_error)
      // result
      .and_then(|val| from_utf8(val).map_err(|e| the_error))
      // result
      .and_then(|val| UserRole::from_string(val).ok_or(the_error));

    a
  }
}

//Compiling hello-rocket v0.1.0 (file:///Users/robertbalicki/Documents/code/hello-rocket)
//error[E0308]: mismatched types
//--> src/models/user_role.rs:47:63
//   |
//47 |       let a: Result<Self, Box<Error + Send + Sync + 'static>> = row.take()
//   |  _______________________________________________________________^
//48 | |       // option
//49 | |       .ok_or(the_error)
//50 | |       // result
//51 | |       .and_then(|val| from_utf8(val).map_err(|e| the_error))
//52 | |       // result
//53 | |       .and_then(|val| UserRole::from_string(val).ok_or(the_error));
//   | |__________________________________________________________________^ expected trait std::error::Error, found struct `models::user_role::InvalidUserRoleError`
//   |
//= note: expected type `std::result::Result<_, std::boxed::Box<std::error::Error + std::marker::Send + std::marker::Sync + 'static>>`
//found type `std::result::Result<_, std::boxed::Box<models::user_role::InvalidUserRoleError>>`
//= help: here are some functions which might fulfill your needs:
//- .unwrap()
//- .unwrap_err()