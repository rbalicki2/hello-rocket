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

impl FromSqlRow<Text, Pg> for UserRole {
  fn build_from_row<T: Row<Pg>>(row: &mut T) -> Result<Self, Box<Error + Send + Sync>> {
    let the_error: Box<Error + Send + Sync> = Box::new(InvalidUserRoleError());
    let the_error_2: Box<Error + Send + Sync> = Box::new(InvalidUserRoleError());
    let the_error_3: Box<Error + Send + Sync> = Box::new(InvalidUserRoleError());

    let a: Result<Self, Box<Error + Send + Sync>> = row.take()
      .ok_or(the_error)
      .and_then(|val| from_utf8(val).map_err(|_| the_error_2))
      .and_then(|val| UserRole::from_string(val).ok_or(the_error_3));
    a
  }
}

impl AsExpression<Text> for UserRole {
  type Expression = Expression<SqlType=Text>;
  fn as_expression(self) -> Self::Expression {
    let s: String = self.to_string();
//    <String as AsExpression<Text>>::as_expression(s)
    AsExpression::<Text>::as_expression(s)
  }
}

impl<'a> AsExpression<Text> for &'a UserRole {
  type Expression = Expression<SqlType=Text>;
  fn as_expression(self) -> Self::Expression {
    let s: String = self.to_string();
//    <String as AsExpression<Text>>::as_expression(s)
    AsExpression::<Text>::as_expression(s)
  }
}

// Adding the following two takes us from 12 errors to 7:

impl AsExpression<Nullable<Text>> for UserRole {
  type Expression = Expression<SqlType=Nullable<Text>>;

  fn as_expression(self) -> Self::Expression {
    AsExpression::<Nullable<Text>>::as_expression(self.to_string())
  }
}

impl<'a> AsExpression<Nullable<Text>> for &'a UserRole {
  type Expression = Expression<SqlType=Nullable<Text>>;

  fn as_expression(self) -> Self::Expression {
    AsExpression::<Nullable<Text>>::as_expression(self.to_string())
  }
}
