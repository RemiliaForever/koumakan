use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use lettre::AsyncTransport;
use sqlx::SqlitePool;

use crate::{
    error::Error,
    util::{GetOrDefault, JSON},
};
use common::Comment;

pub async fn get_article_comments(
    Extension(db): Extension<SqlitePool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, Error> {
    let mut comments = sqlx::query_as!(
        Comment,
        "SELECT * FROM comment WHERE article_id = ? ORDER BY comment.date DESC",
        id
    )
    .fetch_all(&db)
    .await?;
    // mask user's email
    for comment in &mut comments {
        comment.email = "".to_owned();
    }
    Ok(Json(comments))
}

pub async fn create_comment(
    Extension(db): Extension<SqlitePool>,
    Path(id): Path<i64>,
    Json(body): Json<JSON>,
) -> Result<impl IntoResponse, Error> {
    let email: String = body.get_or_default("email");
    let hash = md5::compute(email.trim());
    let mut comment = Comment {
        id: 0,
        article_id: id,
        name: body.get_or_default("name"),
        email,
        website: body.get_or_default("website"),
        content: body.get_or_default("content"),
        avatar: format!("https://www.gravatar.com/avatar/{:x}?s=56d=identicon", hash,),
        date: chrono::Local::now().naive_local(),
    };

    // check
    if comment.name.is_empty() || comment.email.is_empty() || comment.content.is_empty() {
        return Ok((StatusCode::BAD_REQUEST, "name or content is empty"));
    }

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
    .execute(&db)
    .await?
    .last_insert_rowid();
    comment.id = result;

    match send_email(&comment).await {
        Ok(_) => log::info!("Send mail: Ok"),
        Err(e) => log::error!("Send mail: {}", e),
    };
    Ok((StatusCode::OK, ""))
}

async fn send_email(comment: &Comment) -> Result<(), Error> {
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
        .body(body)?;
    lettre::AsyncSendmailTransport::new().send(email).await?;

    Ok(())
}
