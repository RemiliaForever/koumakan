use md5;
use chrono;
use diesel;
use diesel::prelude::*;
use rocket_contrib::Json;
use lettre::{EmailAddress, EmailTransport, SimpleSendableEmail};
use lettre::SendmailTransport;

use db::DbConn;
use models::*;

#[get("/comments/<aid>")]
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
    // send email
    let domain = "koumakan.cc";
    let username = "remilia";
    let email = SimpleSendableEmail::new(
        EmailAddress::new(format!("Blog Notify <notify@{}>", domain)),
        vec![EmailAddress::new(format!("{}@{}", username, domain))],
        "New comment from blog".to_string(),
        format!(
            "You got one new comment.\n\n\
             article: https://blog.koumakan.cc/article/{}\n\n\
             comment: on {}\n\
             \tname: {}\n\
             \temail: {}\n\
             \twebsite: {}\n\
             \tcontent: {}\n\
             \n",
            cmt.article_id, cmt.date, cmt.name, cmt.email, cmt.website, cmt.content
        ),
    );
    let mut sender = SendmailTransport::new();
    println!("Send mail: {:?}", sender.send(&email));
}
