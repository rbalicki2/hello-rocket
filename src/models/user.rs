use std::str;
use rocket::request::{FromParam,FromRequest,Request,Outcome};
use rocket::outcome::Outcome::{Success,Failure};
use rocket::http::{HeaderMap,Status};
use diesel::prelude::*;
use diesel::result::QueryResult;

use models::Id;
use models::OauthToken;
use models::UserRole;
use app::schema::{users,photos};
use app::db;
use app::errors::{Result, ResultExt, Error};

#[derive(Queryable, Serialize, Deserialize, Associations, Identifiable, Clone, Debug)]
#[has_many(photos)]
pub struct User {
  pub id: Id,
  pub name: String,
  pub role: UserRole,
}

impl<'a> FromParam<'a> for User {
  type Error = Error;
  fn from_param(param: &'a str) -> Result<User> {
    use app::schema::users::dsl::users;
    let conn = db::establish_connection();
    let id: i32 = str::parse::<i32>(param)
      .chain_err(|| "Invalid user ID")?;

    let user: User = users
      .find(id)
      .first::<User>(&conn)
      .chain_err(|| "Could not find user")?;

    Ok(user)
  }
}

#[table_name="users"]
#[derive(Insertable, Deserialize, Clone)]
pub struct NewUser {
  pub name: String,
//  pub role: UserRole,
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
  type Error = ();
  fn from_request(request: &'a Request<'r>) -> Outcome<Self, ()> {
    use app::schema::oauth_tokens::dsl;
    let conn = db::establish_connection();

    let headers: &HeaderMap = request.headers();
    let auth_header_opt: Option<&'a str> = headers.get_one("Authorization");

    match auth_header_opt {
      Some(auth) => {
        let actual_token: &str = &auth[7..];

        let oauth_token_result: QueryResult<(OauthToken, User)> = dsl::oauth_tokens.filter(dsl::token.eq(actual_token))
          .inner_join(users::table)
          .first::<(OauthToken, User)>(&conn);

        match oauth_token_result {
          Ok(v) => {
            Success(v.1)
          },
          Err(_) => Failure((Status::Unauthorized, ()))
        }
      },
      None => Failure((Status::Unauthorized, ()))
    }
  }
}
