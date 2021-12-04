use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::SqlitePool;

use crate::{
    controller::label_archive::ALCache,
    error::Error,
    util::{GetOrDefault, JSON},
};
use common::Article;

pub async fn get_article(
    Extension(db): Extension<SqlitePool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, Error> {
    let article = sqlx::query_as!(Article, "SELECT * FROM article WHERE id = ?", id)
        .fetch_one(&db)
        .await?;
    Ok(Json(article))
}

pub async fn get_article_nav(
    Extension(db): Extension<SqlitePool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, Error> {
    let pre = id - 1;
    let art_pre = sqlx::query_as!(Article, "SELECT * FROM article WHERE id = ?", pre)
        .fetch_optional(&db)
        .await?;

    let next = id + 1;
    let art_next = sqlx::query_as!(Article, "SELECT * FROM article WHERE id = ?", next)
        .fetch_optional(&db)
        .await?;

    let result = vec![art_pre, art_next];
    Ok(Json(result))
}

#[derive(serde::Deserialize)]
pub struct ArticleQueryParam {
    filter: Option<String>,
    value: Option<String>,
    pagesize: Option<i64>,
    offset: Option<i64>,
}

pub async fn get_article_list(
    Extension(db): Extension<SqlitePool>,
    Query(param): Query<ArticleQueryParam>,
) -> Result<impl IntoResponse, Error> {
    let filter = match param.filter {
        Some(filter) => filter,
        None => String::new(),
    };
    let value = match param.value {
        Some(value) => value,
        None => String::new(),
    };
    let pagesize = match param.pagesize {
        Some(pagesize) => pagesize,
        None => 10,
    };
    let offset = match param.offset {
        Some(offset) => offset,
        None => 0,
    };
    let result = match filter.as_ref() {
        "category" => sqlx::query_as!(Article,
            "SELECT * FROM article WHERE id > 20000 AND category = ? ORDER BY date DESC LIMIT ? OFFSET ?",
            value,
            pagesize,
            offset,
        ).fetch_all(&db).await?,
        "label" => {
            let search_value = format!("%,{},%",value);
            sqlx::query_as!(Article,
                "SELECT * FROM article WHERE id > 20000 AND category||','||labels||',' LIKE ? ORDER BY date DESC LIMIT ? OFFSET ?",
                search_value,
                pagesize,
                offset,
            ).fetch_all(&db).await?},
        "archive" => {
            let date: Vec<&str> = value.split('-').collect();
            let year = date[0].parse::<u32>().unwrap_or(2000);
            let month = date[1].parse::<u32>().unwrap_or(1);
            let start_date = format!("{:04}-{:02}", year, month);
            let end_date = format!("{:04}-{:02}", year, month + 1);
            sqlx::query_as!(Article,
                "SELECT * FROM article WHERE id > 20000 AND date >= ? AND date < ? ORDER BY date DESC LIMIT ? OFFSET ?",
                start_date,
                end_date,
                pagesize,
                offset,
            ).fetch_all(&db).await?
        }
        "search" => {
            let search_value =
                format!("%{}%", value);
            sqlx::query_as!(Article,
                "SELECT * FROM article WHERE id > 20000 AND ( title LIKE ? OR brief LIKE ? OR category LIKE ? OR labels LIKE ? ) ORDER BY date DESC LIMIT ? OFFSET ?",
                search_value,
                search_value,
                search_value,
                search_value,
                pagesize,
                offset,
            ).fetch_all(&db).await?
        },
        "" => sqlx::query_as!(Article,
            "SELECT * FROM article WHERE id > 20000 ORDER BY date DESC LIMIT ? OFFSET ?",
            pagesize,
            offset,
        ).fetch_all(&db).await?,
        _ => return Ok((StatusCode::BAD_REQUEST, "Bad filter type").into_response()),
    };
    let result = result
        .into_iter()
        .map(|mut a| {
            a.content = "".to_owned();
            a
        })
        .collect::<Vec<Article>>();
    Ok((StatusCode::OK, Json(result)).into_response())
}

pub async fn create_article(
    Extension(db): Extension<SqlitePool>,
    Extension(cache): Extension<ALCache>,
    Json(body): Json<JSON>,
) -> Result<impl IntoResponse, Error> {
    let article = Article {
        id: 0,
        title: body.get_or_default("title"),
        brief: body.get_or_default("brief"),
        content: body.get_or_default("content"),
        category: body.get_or_default("category"),
        labels: body.get_or_default("labels"),
        date: chrono::Local::now().naive_local(),
    };
    let result = sqlx::query!(
        "INSERT INTO article VALUES (0, ?, ?, ?, ?, ?, ?)",
        article.title,
        article.brief,
        article.content,
        article.category,
        article.labels,
        article.date,
    )
    .execute(&db)
    .await?
    .last_insert_rowid();

    cache.dirty();
    Ok((StatusCode::OK, Json(result)))
}

pub async fn update_article(
    Extension(db): Extension<SqlitePool>,
    Extension(cache): Extension<ALCache>,
    Path(id): Path<i32>,
    Json(body): Json<JSON>,
) -> Result<impl IntoResponse, Error> {
    let article = Article {
        id: 0,
        title: body.get_or_default("title"),
        brief: body.get_or_default("brief"),
        content: body.get_or_default("content"),
        category: body.get_or_default("category"),
        labels: body.get_or_default("labels"),
        date: chrono::Local::now().naive_local(),
    };
    let result = sqlx::query!(
        "UPDATE article SET title = ?, brief = ?, content = ?, category = ?, labels = ?, date = ? WHERE id = ?",
        article.title,
        article.brief,
        article.content,
        article.category,
        article.labels,
        article.date,
        id,
    ).execute(&db)
    .await?
    .rows_affected();

    if result == 0 {
        Ok(StatusCode::NOT_FOUND.into_response())
    } else {
        cache.dirty();
        Ok(().into_response())
    }
}

pub async fn delete_article(
    Extension(db): Extension<SqlitePool>,
    Extension(cache): Extension<ALCache>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, Error> {
    sqlx::query!("DELETE FROM article WHERE id = ?", id)
        .execute(&db)
        .await?;
    cache.dirty();

    Ok(())
}
