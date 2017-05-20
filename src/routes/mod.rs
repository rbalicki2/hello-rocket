use rocket::Route;

mod user_routes;

pub fn routes() -> Vec<Route> {
  return routes![user_routes::get_user, user_routes::create_user];
}
