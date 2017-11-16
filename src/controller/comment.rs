use md5;
use chrono;
use diesel;
use diesel::prelude::*;
use rocket;
use rocket_contrib::Json;

use std::collections::HashMap;

use db::DbConn;
use models::*;

#[get("/comments/aid/<aid>")]
fn get_comments(conn: DbConn, aid: i32) -> Json<Vec<Comment>> {
    let mut comments = comment::table
        .filter(comment::article_id.eq(aid))
        .order(comment::date)
        .load::<Comment>(&*conn)
        .expect("error");
    // mask user's email
    for comment in &mut comments {
        comment.email = String::from("")
    }
    Json(comments)
}

#[post("/comments", data = "<cmt>")]
fn add_comment(conn: DbConn, mut cmt: Json<Comment>) -> rocket::response::status::NoContent {
    // caculate avatar
    cmt.avatar = format!(
        "https://www.gravatar.com/avatar/{:x}?s=56d=identicon",
        md5::compute(cmt.email.trim())
    );
    cmt.date = chrono::Local::now().naive_local();
    let _ = diesel::insert(&*cmt).into(comment::table).execute(&*conn);
    rocket::response::status::NoContent {}
}
