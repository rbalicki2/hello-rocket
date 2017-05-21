use rocket::Route;

mod user_routes;
mod photo_routes;

pub fn routes() -> Vec<Route> {
  return routes![
    user_routes::get_user,
    user_routes::create_user,
    user_routes::get_users,
    photo_routes::get_photos_for_user,
    photo_routes::create_new_photo_for_user,
    photo_routes::get_photo_for_user,
  ];
}
