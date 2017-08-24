extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;

use diesel::prelude::*;
use rocket_contrib::Json;

use db::DbConn;
use models::*;
use models::comment::dsl::*;

#[get("/")]
fn root(conn: DbConn) -> Json<Vec<Comment>> {
    Json(comment.load::<Comment>(&*conn).expect("query error!"))
}

#[get("/<all>")]
fn all(all: String) -> String {
    all
}
