#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate serde_derive;

extern crate r2d2_diesel;
extern crate r2d2;
extern crate chrono;
extern crate rocket;
extern crate rocket_contrib;


mod db;
mod models;
mod controller;
use controller::*;


#[error(404)]
fn not_found() -> String {
    String::from("404")
}

#[error(500)]
fn server_error() -> String {
    String::from("500")
}

fn main() {
    let server = rocket::ignite();
    let dbpool = db::init();
    server
        .mount("/", routes![root, all])
        .catch(errors![not_found, server_error])
        .manage(dbpool)
        .launch();

}
