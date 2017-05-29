use diesel;
use rocket_contrib::JSON;
use rocket::State;
use diesel::prelude::*;

use models::User;
use models::NewUser;
use models::LimitOffsetParam;

use app::errors::{Result, ResultExt};
use app::db;
use app::schema;
use query_param_group::{NamedFields, QueryParamGroup};

#[get("/users/<user>")]
pub fn get_user(user: User) -> JSON<User> {
  JSON(user)
}

#[post("/users", data="<user_data>")]
pub fn create_user(db_pool: State<db::ConnectionPool>, user_data: JSON<NewUser>) -> Result<JSON<User>> {
  // TODO figure out why you can't use a connection pool
  let conn: db::DbConnection = db_pool.get().chain_err(|| "Could not connect to DB")?;

  let user: NewUser = user_data.0;

  let returned_user: User = diesel::insert(&user)
    .into(schema::users::table)
    .get_result(&*conn)
    .chain_err(|| "Error inserting user")?;

  Ok(JSON(returned_user))
}

#[derive(FromForm, Debug)]
pub struct UserNameParam {
  pub name: Option<String>,
}

impl NamedFields for UserNameParam {
  const FIELDS: &'static [&'static str] = &["name"];
}

#[get("/users?<query_params>")]
pub fn get_users(
  db_pool: State<db::ConnectionPool>,
  query_params: QueryParamGroup<(LimitOffsetParam, UserNameParam)>
) -> Result<JSON<Vec<User>>> {
  use app::schema::users::dsl;
  let conn: db::DbConnection = db_pool.get().chain_err(|| "Could not connect to DB")?;

  let qp = query_params.get_owned();
  let limit: i64 = qp.0.limit.unwrap_or(100) as i64;
  let offset: i64 = qp.0.offset.unwrap_or(0) as i64;

  let users_response = dsl::users
    .offset(offset)
    .limit(limit);

  let users_response: Vec<User> = qp.1.name
    .map_or(
      users_response.load::<User>(&*conn),
      |name| users_response.filter(dsl::name.eq(name)).load::<User>(&*conn)
    )
    .chain_err(|| "Could not query users")?;

  Ok(JSON(users_response))
}