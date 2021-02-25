use actix_web::{delete, get, post, put, web, HttpResponse};
use log::debug;
use serde_derive::Deserialize;
use sqlx::SqlitePool;

use crate::{
    common::ResError,
    controller::{effect_one, user::check_login, ALCache},
};
use common::Article;

#[get("/article/{id}")]
async fn get_article(
    pool: web::Data<SqlitePool>,
    param: web::Path<i32>,
) -> Result<HttpResponse, ResError> {
    let article = match sqlx::query_as!(Article, "SELECT * FROM article WHERE id = ?", *param)
        .fetch_optional(&**pool)
        .await?
    {
        Some(result) => result,
        None => Err(ResError::from(http::StatusCode::NOT_FOUND))?,
    };
    debug!("body: {:?}", article);
    Ok(HttpResponse::Ok().body(bincode::serialize(&article)?))
}

#[get("/article/{id}/nav")]
async fn get_article_nav(
    pool: web::Data<SqlitePool>,
    param: web::Path<i32>,
) -> Result<HttpResponse, ResError> {
    let pre = *param - 1;
    let art_pre = sqlx::query_as!(Article, "SELECT * FROM article WHERE id = ?", pre)
        .fetch_optional(&**pool)
        .await?;

    let next = *param + 1;
    let art_next = sqlx::query_as!(Article, "SELECT * FROM article WHERE id = ?", next)
        .fetch_optional(&**pool)
        .await?;

    let result = vec![art_pre, art_next];
    Ok(HttpResponse::Ok().body(bincode::serialize(&result)?))
}

#[derive(Default, Debug, Deserialize)]
pub struct ArticleQueryParam {
    filter: Option<String>,
    value: Option<String>,
    pagesize: Option<i64>,
    offset: Option<i64>,
}

#[get("/articles")]
async fn get_article_list(
    pool: web::Data<SqlitePool>,
    param: web::Query<ArticleQueryParam>,
) -> Result<HttpResponse, ResError> {
    let param = param.into_inner();
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
        ).fetch_all(&**pool).await?,
        "label" => {
            let search_value = format!("%,{},%",value);
            sqlx::query_as!(Article,
                "SELECT * FROM article WHERE id > 20000 AND category||','||labels||',' LIKE ? ORDER BY date DESC LIMIT ? OFFSET ?",
                search_value,
                pagesize,
                offset,
            ).fetch_all(&**pool).await?},
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
            ).fetch_all(&**pool).await?
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
            ).fetch_all(&**pool).await?
        },
        "" => sqlx::query_as!(Article,
            "SELECT * FROM article WHERE id > 20000 ORDER BY date DESC LIMIT ? OFFSET ?",
            pagesize,
            offset,
        ).fetch_all(&**pool).await?,
        _ => panic!("error typestring"),
    };
    let result = result
        .into_iter()
        .map(|mut a| {
            a.content = "".to_owned();
            a
        })
        .collect::<Vec<Article>>();
    debug!("body: {:#?}", result);
    Ok(HttpResponse::Ok().body(bincode::serialize(&result)?))
}

#[post("/article")]
async fn create_article(
    pool: web::Data<SqlitePool>,
    cache: web::Data<ALCache>,
    req: web::HttpRequest,
    body: web::Bytes,
) -> Result<HttpResponse, ResError> {
    check_login(req)?;

    let mut article = bincode::deserialize::<Article>(&body)?;
    article.date = chrono::Local::now().naive_local();
    let result = sqlx::query!(
        "INSERT INTO article VALUES (0, ?, ?, ?, ?, ?, ?)",
        article.title,
        article.brief,
        article.content,
        article.category,
        article.labels,
        article.date,
    )
    .execute(&**pool)
    .await?
    .last_insert_rowid();
    article.id = result;

    cache.dirty();
    debug!("body: {:?}", article);
    Ok(HttpResponse::Ok().body(bincode::serialize(&article)?))
}

#[put("/article/{id}")]
async fn update_article(
    pool: web::Data<SqlitePool>,
    cache: web::Data<ALCache>,
    param: web::Path<i32>,
    req: web::HttpRequest,
    body: web::Bytes,
) -> Result<HttpResponse, ResError> {
    check_login(req)?;
    let mut article = bincode::deserialize::<Article>(&body)?;
    article.date = chrono::Local::now().naive_local();
    let result = sqlx::query!(
        "UPDATE article SET title = ?, brief = ?, content = ?, category = ?, labels = ?, date = ? WHERE id = ?",
        article.title,
        article.brief,
        article.content,
        article.category,
        article.labels,
        article.date,
        *param
    )
        .execute(&**pool)
        .await?
        .rows_affected();

    cache.dirty();
    effect_one(result)
}

#[delete("/article/{id}")]
async fn delete_article(
    pool: web::Data<SqlitePool>,
    cache: web::Data<ALCache>,
    param: web::Path<i32>,
    req: web::HttpRequest,
) -> Result<HttpResponse, ResError> {
    check_login(req)?;
    let result = sqlx::query!("DELETE FROM article WHERE id = ?", *param)
        .execute(&**pool)
        .await?
        .rows_affected();

    cache.dirty();
    effect_one(result)
}
