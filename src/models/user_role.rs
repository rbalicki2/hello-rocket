use diesel::types::{Text,FromSqlRow,Nullable};
use diesel::row::Row;
use diesel::expression::{Expression,AsExpression};
use diesel::expression::helper_types::AsExprOf;
use std::error::Error;
use diesel::pg::Pg;
use std::str::from_utf8;
use std::fmt;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
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

impl fmt::Display for UserRole {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", match *self {
      UserRole::Admin => "ADMIN",
      UserRole::User => "USER",
    })
  }
}

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

impl FromSqlRow<Text, Pg> for UserRole {
  fn build_from_row<T: Row<Pg>>(row: &mut T) -> Result<Self, Box<Error + Send + Sync>> {
    fn error() -> Box<Error + Send + Sync> {
      Box::new(InvalidUserRoleError())
    }

    let a: Result<Self, Box<Error + Send + Sync>> = row.take()
      .ok_or_else(error)
      .and_then(|val| from_utf8(val).map_err(|_| error()))
      .and_then(|val| UserRole::from_string(val).ok_or_else(error));
    a
  }
}

impl AsExpression<Nullable<Text>> for UserRole {
  type Expression = AsExprOf<String, Nullable<Text>>;

  fn as_expression(self) -> Self::Expression {
    AsExpression::<Nullable<Text>>::as_expression(self.to_string())
  }
}

impl<'a> AsExpression<Nullable<Text>> for &'a UserRole {
  type Expression = AsExprOf<String, Nullable<Text>>;

  fn as_expression(self) -> Self::Expression {
    AsExpression::<Nullable<Text>>::as_expression(self.to_string())
  }
}
