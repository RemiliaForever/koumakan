use chrono;
use diesel;
use diesel::prelude::*;
use lettre::{EmailTransport, SmtpTransport};
use lettre_email::EmailBuilder;
use md5;
use rocket_contrib::Json;

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
        comment.email = String::from("");
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
    println!("Send mail: {:?}", send_email(cmt.0));
}

fn send_email(cmt: Comment) -> Result<String, String> {
    // send email
    let domain = "koumakan.cc";
    let username = "remilia";
    let email = EmailBuilder::new()
        .from((format!("notify@{}", domain), "Blog Notifier"))
        .to(format!("{}@{}", username, domain))
        .subject("New comment from blog".to_string())
        .text(format!(
            r#"
You got one new comment.

article: https://blog.koumakan.cc/article/{}
comment on {}
name: {}
email: {}
website: {}
content:

{}
"#,
            cmt.article_id, cmt.date, cmt.name, cmt.email, cmt.website, cmt.content
        )).build()
        .map_err(|e| format!("build email error: {}", e))?;
    let sender = SmtpTransport::builder_unencrypted_localhost()
        .map_err(|e| format!("resolve server error: {}", e))?;
    sender
        .build()
        .send(&email)
        .map(|e| format!("smtp server response: {:?}", e))
        .map_err(|e| format!("smtp server error: {}", e))
}
