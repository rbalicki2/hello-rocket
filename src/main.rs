#![plugin(rocket_codegen)]
#![feature(plugin)]
#![recursion_limit = "1024"]

extern crate rocket;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;

extern crate r2d2;
extern crate r2d2_diesel;
extern crate dotenv;


#[macro_use]
extern crate error_chain;

pub mod app;
pub mod models;

fn main() {
  app::app().launch();
}