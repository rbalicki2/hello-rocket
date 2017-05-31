use diesel;
use std::str;

use rocket::request::{FromForm};
use chrono::NaiveDateTime;
use chrono::prelude::UTC;
use diesel::prelude::*;

use models::User;
use models::Id;

use app::db;
use app::schema::{users,oauth_tokens};
use app::errors::{Result, ResultExt, Error};


#[derive(Deserialize, Clone, Debug)]
pub struct LoginCredential {
  pub name: String,
  // TODO password or something
}

#[derive(Queryable, Deserialize, Serialize, Associations, Identifiable, Debug, Clone)]
#[belongs_to(User)]
#[serde(rename_all = "camelCase")]
pub struct OauthToken {
  pub id: i32,
  pub user_id: Id,
  pub expires_at: NaiveDateTime,
  pub token: String,
}

#[table_name="oauth_tokens"]
#[derive(Insertable, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InsertableOauthToken {
  pub user_id: Id,
  pub expires_at: NaiveDateTime,
  pub token: String,
}

impl InsertableOauthToken {
  pub fn create_from_user(user: User) -> InsertableOauthToken {
    InsertableOauthToken {
      user_id: user.id,
      expires_at: UTC::now().naive_utc(),
      token: "secret".to_string(),
    }
  }

  pub fn insert(
    &self,
    conn: &db::DbConnection
  ) -> Result<OauthToken> {
    let oauth_token: OauthToken = diesel::insert(self)
      .into(oauth_tokens::table)
      .get_result(&**conn)
      .chain_err(|| "Could not create oauth token")?;

    Ok(oauth_token)
  }
}

