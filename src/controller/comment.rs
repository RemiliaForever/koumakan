use md5;
use chrono;
use diesel;
use diesel::prelude::*;
use rocket_contrib::Json;

use std::collections::HashMap;

use db::DbConn;
use models::*;

#[post("/getComments", data = "<param>")]
fn get_comments(conn: DbConn, param: Json<HashMap<String, i32>>) -> Json<Vec<Comment>> {
    let comments = comment::table
        .filter(comment::article_id.eq(param["id"]))
        .order(comment::date)
        .load::<Comment>(&*conn)
        .expect("error!");
    Json(comments)
}

#[post("/addComment", data = "<cmt>")]
fn add_comment(conn: DbConn, mut cmt: Json<Comment>) {
    // caculate avatar
    cmt.avatar = format!(
        "https://www.gravatar.com/avatar/{:x}?d=identicon",
        md5::compute(cmt.email.trim())
    );
    // get current datetime
    cmt.date = chrono::Local::now().naive_local();
    let _ = diesel::insert(&*cmt).into(comment::table).execute(&*conn);
}
