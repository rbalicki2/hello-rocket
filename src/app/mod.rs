use rocket;
use rocket::Route;

use dotenv::dotenv;

pub mod db;
pub mod schema;
pub mod errors;
#[macro_use] pub mod field_names;

pub fn app(routes: Vec<Route>) -> rocket::Rocket {
  dotenv().ok();
  rocket::ignite()
    .manage(db::establish_connection_pool())
    .mount("/", routes)
}
