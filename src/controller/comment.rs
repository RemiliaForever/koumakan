use md5;
use chrono;
use diesel;
use diesel::prelude::*;
use rocket_contrib::Json;

use std::collections::HashMap;

use db::DbConn;
use models::*;

#[post("/getComments", data = "<param>")]
fn get_comments(conn: DbConn, param: Json<HashMap<String, String>>) -> Json<Vec<Comment>> {
    let id = param["id"].parse::<i32>().expect("error");
    let mut comments = comment::table
        .filter(comment::article_id.eq(id))
        .order(comment::date)
        .load::<Comment>(&*conn)
        .expect("error");
    // mask user's email
    for comment in &mut comments {
        comment.email = String::from("")
    }
    Json(comments)
}

#[post("/addComment", data = "<cmt>")]
fn add_comment(conn: DbConn, mut cmt: Json<Comment>) -> Json<&'static str> {
    // caculate avatar
    cmt.avatar = format!(
        "https://www.gravatar.com/avatar/{:x}?s=56d=identicon",
        md5::compute(cmt.email.trim())
    );
    cmt.date = chrono::Local::now().naive_local();
    let _ = diesel::insert(&*cmt).into(comment::table).execute(&*conn);
    Json("finished")
}
