use diesel;
use serde_json;
use diesel::prelude::*;
use rocket::http::Cookies;
use rocket_contrib::Json;
use rocket::request::State;
use chrono::Local;

use db::DbConn;
use models::*;
use super::ALCache;

#[get("/articles/<id>")]
fn get_article(conn: DbConn, id: i32) -> Json<Option<Article>> {
    let mut article = article::table
        .filter(article::id.eq(id))
        .load::<Article>(&*conn)
        .expect("error");
    Json(article.pop())
}

#[get("/articles/<id>/nav")]
fn get_article_nav(conn: DbConn, id: i32) -> Json<serde_json::Value> {
    use diesel::result::Error;
    fn art_to_nav(result: Result<String, Error>, art_id: i32) -> serde_json::Value {
        match result {
            Ok(s) => json!({"id": art_id, "title": s }),
            _ => json!({"id": -1, "title": "没有了"}),
        }
    }
    let art_pre: Result<String, _> = article::table
        .select(article::title)
        .filter(article::id.eq(id - 1))
        .first(&*conn);
    let art_next: Result<String, _> = article::table
        .select(article::title)
        .filter(article::id.eq(id + 1))
        .first(&*conn);
    Json(json!({
        "pre":art_to_nav(art_pre, id - 1),
        "next":art_to_nav(art_next, id + 1)
    }))
}

#[derive(Default)]
#[derive(FromForm)]
struct ArticleQueryParam {
    filter: Option<String>,
    value: Option<String>,
    pagesize: Option<i64>,
    offset: Option<i64>,
}

#[get("/articles")]
fn get_article_list_default(conn: DbConn) -> Json<Vec<Article>> {
    get_article_list(conn, ArticleQueryParam::default())
}

#[get("/articles?<param>")]
fn get_article_list(conn: DbConn, param: ArticleQueryParam) -> Json<Vec<Article>> {
    let value = match param.value {
        Some(s) => s,
        None => String::from(""),
    };
    let pagesize = match param.pagesize {
        Some(p) => p,
        None => 10,
    };
    let offset = match param.offset {
        Some(o) => o,
        None => 0,
    };
    let query = match param.filter {
        Some(s) => match s.as_ref() {
            "category" => article::table
                .filter(article::category.eq(value))
                .filter(article::id.gt(20000))
                .order(article::date.desc())
                .limit(pagesize)
                .offset(offset)
                .load::<Article>(&*conn),
            "label" => article::table
                .filter(
                    article::category
                        .concat(",")
                        .concat(article::labels)
                        .concat(",")
                        .like(format!("%,{},%", value)),
                )
                .filter(article::id.gt(20000))
                .order(article::date.desc())
                .limit(pagesize)
                .offset(offset)
                .load::<Article>(&*conn),
            "archive" => {
                let date: Vec<&str> = value.split("-").collect();
                let year = date[0].parse::<u32>().unwrap_or(2000);
                let month = date[1].parse::<u32>().unwrap_or(1);
                article::table
                    .filter(
                        article::date
                            .ge(format!("{:04}-{:02}", year, month))
                            .and(article::date.lt(format!("{:04}-{:02}", year, month + 1))),
                    )
                    .filter(article::id.gt(20000))
                    .order(article::date.desc())
                    .limit(pagesize)
                    .offset(offset)
                    .load::<Article>(&*conn)
            }
            "search" => article::table
                .filter(
                    article::title
                        .like(format!("%{}%", value))
                        .or(article::brief.like(format!("%{}%", value)))
                        .or(article::category.like(format!("%{}%", value)))
                        .or(article::labels.like(format!("%{}%", value))),
                )
                .filter(article::id.gt(20000))
                .order(article::date.desc())
                .limit(pagesize)
                .offset(offset)
                .load::<Article>(&*conn),
            _ => panic!("error typestring"),
        },
        _ => article::table
            .filter(article::id.gt(20000))
            .order(article::date.desc())
            .limit(pagesize)
            .offset(offset)
            .load::<Article>(&*conn),
    };
    let mut result = query.expect("error");
    // mask content
    for a in &mut result {
        a.content = String::from("")
    }
    Json(result)
}

#[post("/articles", data = "<article>")]
fn post_article(
    mut cookies: Cookies,
    conn: DbConn,
    cache: State<ALCache>,
    mut article: Json<Article>,
) -> &'static str {
    cookies.get_private("isLogin").expect("Validate Error");
    article.date = Local::now().naive_local();
    diesel::insert_into(article::table)
        .values(&*article)
        .execute(&*conn)
        .expect("insert error");
    cache.refresh_cache(&*conn);
    "Success"
}

#[put("/articles", data = "<article>")]
fn put_article(
    mut cookies: Cookies,
    conn: DbConn,
    cache: State<ALCache>,
    article: Json<Article>,
) -> &'static str {
    cookies.get_private("isLogin").expect("Validate Error");
    // article.date = Local::now().naive_local();
    diesel::update(article::table.filter(article::id.eq(article.id)))
        .set((
            article::title.eq(&article.title),
            article::brief.eq(&article.brief),
            article::content.eq(&article.content),
            article::category.eq(&article.category),
            article::labels.eq(&article.labels),
            article::date.eq(&article.date),
        ))
        .execute(&*conn)
        .expect("update error");
    cache.refresh_cache(&*conn);
    "Success"
}

#[delete("/articles/<id>")]
fn delete_article(
    mut cookies: Cookies,
    conn: DbConn,
    cache: State<ALCache>,
    id: i32,
) -> &'static str {
    cookies.get_private("isLogin").expect("Validate Error");
    diesel::delete(article::table.filter(article::id.eq(id)))
        .execute(&*conn)
        .expect("delete error");
    cache.refresh_cache(&*conn);
    "Success"
}
