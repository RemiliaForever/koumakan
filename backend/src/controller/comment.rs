use actix_web::{get, post, web, HttpResponse};
use lettre::Transport;
use sqlx::SqlitePool;

use crate::controller::ResError;
use common::Comment;

#[get("/article/{id}/comment")]
async fn get_article_comments(
    pool: web::Data<SqlitePool>,
    param: web::Path<i32>,
) -> Result<HttpResponse, ResError> {
    let mut comments = sqlx::query_as!(
        Comment,
        "SELECT * FROM comment WHERE article_id = ? ORDER BY comment.date DESC",
        *param
    )
    .fetch_all(&**pool)
    .await?;
    // mask user's email
    for comment in &mut comments {
        comment.email = "".to_owned();
    }
    Ok(HttpResponse::Ok().body(bincode::serialize(&comments)?))
}

#[post("/article/{id}/comment")]
async fn create_comment(
    pool: web::Data<SqlitePool>,
    param: web::Path<i64>,
    body: web::Bytes,
) -> Result<HttpResponse, ResError> {
    let mut comment = bincode::deserialize::<Comment>(&body)?;
    comment.article_id = *param;
    // caculate avatar
    comment.avatar = format!(
        "https://www.gravatar.com/avatar/{:x}?s=56d=identicon",
        md5::compute(comment.email.trim())
    );
    comment.date = chrono::Local::now().naive_local();
    let result = sqlx::query!(
        "INSERT INTO comment VALUES (NULL, ?, ?, ?, ?, ?, ?, ?)",
        comment.article_id,
        comment.name,
        comment.email,
        comment.website,
        comment.content,
        comment.avatar,
        comment.date
    )
    .execute(&**pool)
    .await?
    .last_insert_rowid();
    comment.id = result;

    log::info!("Send mail: {:?}", send_email(&comment));
    Ok(HttpResponse::Ok().body(bincode::serialize(&comment)?))
}

fn send_email(comment: &Comment) -> Result<String, String> {
    let from = "Blog Notifier <notify@koumakan.cc>";
    let to = "RemiliaForever <remilia@kouamkan.cc>";
    let subject = "New comment from blog";
    let body = format!(
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
        comment.article_id,
        comment.date,
        comment.name,
        comment.email,
        comment.website,
        comment.content
    );
    let email = lettre::Message::builder()
        .from(from.parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .body(body)
        .map_err(|e| e.to_string())?;
    lettre::SendmailTransport::new()
        .send(&email)
        .map(|_r| "Ok".to_owned())
        .map_err(|e| e.to_string())
}
