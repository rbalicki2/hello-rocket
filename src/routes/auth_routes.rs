use rocket_contrib::JSON;
use rocket::State;
use diesel::prelude::*;

use models::User;
use models::LoginCredential;
use models::InsertableOauthToken;
use models::OauthToken;

use app::errors::{Result, ResultExt};
use app::db;

#[post("/login", data="<login_credentials>")]
pub fn login(
  db_pool: State<db::ConnectionPool>,
  login_credentials: JSON<LoginCredential>
) -> Result<JSON<OauthToken>> {
  use app::schema::oauth_tokens::dsl::oauth_tokens;
  use app::schema::users::dsl;

  let name: String = login_credentials.into_inner().name;

  let conn: db::DbConnection = db_pool.get().chain_err(|| "Could not connect to DB")?;

  let user: User = dsl::users.filter(dsl::name.eq(name))
    .first::<User>(&*conn)
    .chain_err(|| "Could not query or find user")?;

  let oauth_token: OauthToken = InsertableOauthToken::create_from_user(user)
    .insert(&conn)
    .chain_err(|| "Could not create oauth token")?;

  Ok(JSON(oauth_token))
}

