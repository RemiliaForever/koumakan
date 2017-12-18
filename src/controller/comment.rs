use md5;
use chrono;
use diesel;
use diesel::prelude::*;
use rocket_contrib::Json;
use rocket::http::Cookies;

use db::DbConn;
use models::*;

#[get("/comments/<aid>")]
fn get_comments(mut cookies: Cookies, conn: DbConn, aid: i32) -> Json<Vec<Comment>> {
    let mut comments = comment::table
        .filter(comment::article_id.eq(aid))
        .order(comment::date)
        .load::<Comment>(&*conn)
        .expect("error");

    if let None = cookies.get_private("isLogin") {
        // mask user's email
        for comment in &mut comments {
            comment.email = String::from("")
        }
    }
    Json(comments)
}

#[post("/comments", data = "<cmt>")]
fn post_comments(conn: DbConn, mut cmt: Json<Comment>) {
    // caculate avatar
    cmt.avatar = format!(
        "https://www.gravatar.com/avatar/{:x}?s=56d=identicon",
        md5::compute(cmt.email.trim())
    );
    cmt.date = chrono::Local::now().naive_local();
    diesel::insert_into(comment::table)
        .values(&*cmt)
        .execute(&*conn)
        .expect("insert error");
}
