#![plugin(rocket_codegen)]
#![feature(plugin, custom_attribute)]
#![recursion_limit = "1024"]

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

pub mod app;
pub mod models;

fn main() {
  app::app().launch();
}