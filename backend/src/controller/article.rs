use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use sqlx::{Done, SqlitePool};

use crate::controller::{effect_one, user::check_login, ALCache, ResError};
use common::Article;

#[get("/article/{id}")]
async fn get_article(
    pool: web::Data<SqlitePool>,
    param: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let article = sqlx::query_as!(Article, "SELECT * FROM article WHERE id = ?", param.0)
        .fetch_one(&**pool)
        .await
        .map_err(ResError::new)?;
    Ok(HttpResponse::Ok().body(bincode::serialize(&article).map_err(ResError::new)?))
}

#[get("/article/{id}/nav")]
async fn get_article_nav(
    pool: web::Data<SqlitePool>,
    param: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let pre = param.0 - 1;
    let art_pre = sqlx::query_as!(Article, "SELECT * FROM article WHERE id = ?", pre)
        .fetch_optional(&**pool)
        .await
        .map_err(ResError::new)?;

    let next = param.0 + 1;
    let art_next = sqlx::query_as!(Article, "SELECT * FROM article WHERE id = ?", next)
        .fetch_optional(&**pool)
        .await
        .map_err(ResError::new)?;

    let result = vec![art_pre, art_next];
    Ok(HttpResponse::Ok().body(bincode::serialize(&result).map_err(ResError::new)?))
}

// #[derive(Default)]
// #[derive(FromForm)]
// pub struct ArticleQueryParam {
//     filter: Option<String>,
//     value: Option<String>,
//     pagesize: Option<i64>,
//     offset: Option<i64>,
// }
//
// #[get("/articles?<param..>")]
// pub fn get_article_list(conn: DbConn, param: Form<ArticleQueryParam>) -> Json<Vec<Article>> {
//     let value = match param.value {
//         Some(ref s) => s.clone(),
//         None => String::from(""),
//     };
//     let pagesize = match param.pagesize {
//         Some(ref p) => p.clone(),
//         None => 10,
//     };
//     let offset = match param.offset {
//         Some(ref o) => o.clone(),
//         None => 0,
//     };
//     let query = match param.filter {
//         Some(ref s) => match s.as_ref() {
//             "category" => article::table
//                 .filter(article::category.eq(value))
//                 .filter(article::id.gt(20000))
//                 .order(article::date.desc())
//                 .limit(pagesize)
//                 .offset(offset)
//                 .load::<Article>(&*conn),
//             "label" => article::table
//                 .filter(
//                     article::category
//                         .concat(",")
//                         .concat(article::labels)
//                         .concat(",")
//                         .like(format!("%,{},%", value)),
//                 )
//                 .filter(article::id.gt(20000))
//                 .order(article::date.desc())
//                 .limit(pagesize)
//                 .offset(offset)
//                 .load::<Article>(&*conn),
//             "archive" => {
//                 let date: Vec<&str> = value.split('-').collect();
//                 let year = date[0].parse::<u32>().unwrap_or(2000);
//                 let month = date[1].parse::<u32>().unwrap_or(1);
//                 article::table
//                     .filter(
//                         article::date
//                             .ge(format!("{:04}-{:02}", year, month))
//                             .and(article::date.lt(format!("{:04}-{:02}", year, month + 1))),
//                     )
//                     .filter(article::id.gt(20000))
//                     .order(article::date.desc())
//                     .limit(pagesize)
//                     .offset(offset)
//                     .load::<Article>(&*conn)
//             }
//             "search" => article::table
//                 .filter(
//                     article::title
//                         .like(format!("%{}%", value))
//                         .or(article::brief.like(format!("%{}%", value)))
//                         .or(article::category.like(format!("%{}%", value)))
//                         .or(article::labels.like(format!("%{}%", value))),
//                 )
//                 .filter(article::id.gt(20000))
//                 .order(article::date.desc())
//                 .limit(pagesize)
//                 .offset(offset)
//                 .load::<Article>(&*conn),
//             _ => panic!("error typestring"),
//         },
//         _ => article::table
//             .filter(article::id.gt(20000))
//             .order(article::date.desc())
//             .limit(pagesize)
//             .offset(offset)
//             .load::<Article>(&*conn),
//     };
//     let result = query.expect("error");
//     // mask content
//     Json(
//         result
//             .into_iter()
//             .map(|mut a| {
//                 a.content = "".to_owned();
//                 a
//             })
//             .collect(),
//     )
// }

#[post("/article")]
async fn create_article(
    pool: web::Data<SqlitePool>,
    cache: web::Data<ALCache>,
    req: web::HttpRequest,
    body: web::Bytes,
) -> Result<HttpResponse, Error> {
    check_login(req)?;

    let mut article = bincode::deserialize::<Article>(&body).map_err(ResError::new)?;
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
    .await
    .map_err(ResError::new)?
    .last_insert_rowid();
    article.id = Some(result);

    cache.dirty();
    Ok(HttpResponse::Ok().body(bincode::serialize(&article).map_err(ResError::new)?))
}

#[put("/article/{id}")]
async fn update_article(
    pool: web::Data<SqlitePool>,
    cache: web::Data<ALCache>,
    param: web::Path<i32>,
    req: web::HttpRequest,
    body: web::Bytes,
) -> Result<HttpResponse, Error> {
    check_login(req)?;
    let mut article = bincode::deserialize::<Article>(&body).map_err(ResError::new)?;
    article.date = chrono::Local::now().naive_local();
    let result = sqlx::query!(
        "UPDATE article SET title = ?, brief = ?, content = ?, category = ?, labels = ?, date = ? WHERE id = ?",
        article.title,
        article.brief,
        article.content,
        article.category,
        article.labels,
        article.date,
        param.0
    )
    .execute(&**pool)
    .await
    .map_err(ResError::new)?
    .rows_affected();

    cache.dirty();
    effect_one(result)
}

#[delete("/article/<id>")]
async fn delete_article(
    pool: web::Data<SqlitePool>,
    cache: web::Data<ALCache>,
    param: web::Path<i32>,
    req: web::HttpRequest,
) -> Result<HttpResponse, Error> {
    check_login(req)?;
    let result = sqlx::query!("DELETE FROM article WHERE id = ?", param.0)
        .execute(&**pool)
        .await
        .map_err(ResError::new)?
        .rows_affected();

    cache.dirty();
    effect_one(result)
}
