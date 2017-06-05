use rocket::request::{FromRequest,Request,Outcome};
use rocket::outcome::Outcome::{Success,Failure};
use rocket::http::{HeaderMap,Status};
use diesel::result::QueryResult;
use diesel::prelude::*;

use app::db;
use models::OauthToken;
use app::schema::users;
use models::User;
use models::UserRole;

#[derive(Debug)]
pub struct AuthenticatedUser(pub User);

impl<'a, 'r> FromRequest<'a, 'r> for AuthenticatedUser {
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
            Success(AuthenticatedUser(v.1))
          },
          Err(_) => Failure((Status::Unauthorized, ()))
        }
      },
      None => Failure((Status::Unauthorized, ()))
    }
  }
}

#[derive(Debug)]
pub struct AuthenticatedAdminUser(pub User);
impl <'a, 'r> FromRequest<'a, 'r> for AuthenticatedAdminUser {
  type Error = ();
  fn from_request(request: &'a Request<'r>) -> Outcome<Self, ()> {
    let user_outcome = AuthenticatedUser::from_request(request);

    match user_outcome {
      Success(auth_user) => {
        let user: User = auth_user.0;
        if user.role == UserRole::Admin {
          Success(AuthenticatedAdminUser(user))
        } else {
          Failure((Status::Forbidden, ()))
        }
      },
      _ => Failure((Status::Unauthorized, ())),
    }
  }
}

