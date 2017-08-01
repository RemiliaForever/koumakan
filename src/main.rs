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

mod db;
use db::DbConn;
mod models;

use chrono::prelude::*;
use diesel::prelude::*;
use self::models::*;
use self::models::comment::dsl::*;

#[get("/")]
fn index(conn: DbConn) -> String {
    let m = Comment {
        id: None,
        article_id: Some(1),
        name: Some(String::from("RemiliaForever")),
        avatar: Some(String::from("asdf")),
        email: Some(String::from("remilia@koumakan.cc")),
        website: None,
        content: Some(String::from("content")),
        date: Some(Local.ymd(2017, 1, 1).and_hms(0, 0, 0).naive_local()),
    };
    println!("{:?}", diesel::insert(&m).into(comment).execute(&*conn));
    format!("{:?}\n", comment.load::<Comment>(&*conn))
}

fn main() {
    let server = rocket::ignite();
    let dbpool = db::init();
    server.mount("/", routes![index]).manage(dbpool).launch();
}
