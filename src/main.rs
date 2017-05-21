#![plugin(rocket_codegen)]
#![feature(plugin, custom_attribute, custom_derive)]
#![recursion_limit = "1024"]

// https://github.com/SergioBenitez/Rocket/issues/174
#![allow(unmounted_route)]

extern crate rocket;
extern crate rocket_contrib;

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;

#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate serde;

extern crate r2d2;
extern crate r2d2_diesel;
extern crate dotenv;

#[macro_use] extern crate error_chain;

#[macro_use] pub mod app;
pub mod models;
pub mod routes;

fn main() {
  app::app(routes::routes()).launch();
}