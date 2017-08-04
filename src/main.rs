#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate r2d2_diesel;
extern crate r2d2;
extern crate chrono;
extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::Template;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

mod db;
mod models;
mod controller;
use controller::*;

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

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
        .mount("/", routes![root, index, all])
        .mount("/static", routes![files])
        .catch(errors![not_found, server_error])
        .attach(Template::fairing())
        .manage(dbpool)
        .launch();

}
