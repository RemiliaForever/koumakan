extern crate rocket;
extern crate rocket_contrib;

use diesel::prelude::*;
use rocket_contrib::Template;

use db::DbConn;
use models::*;
use models::comment::dsl::*;

#[get("/")]
fn root(conn: DbConn) -> String {
    format!("{:?}\n", comment.load::<Comment>(&*conn))
}

#[get("/index")]
fn index() -> Template {
    Template::render("index", ("a", "a"))
}

#[get("/<all>")]
fn all(all: String) -> String {
    all
}
