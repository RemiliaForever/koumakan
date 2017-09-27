use std::collections::HashMap;

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
use super::get_or_null;


#[post("/getArticleList", data = "<param>")]
fn get_article_list(conn: DbConn, param: Json<HashMap<String, String>>) -> Json<Vec<Article>> {
    let typestring = get_or_null(&param, "typestring");
    let value = get_or_null(&param, "param");
    let pagesize = get_or_null(&param, "pagesize").parse::<i64>().unwrap_or(10);
    let offset = get_or_null(&param, "offset").parse::<i64>().unwrap_or(0);
    let query = match typestring.as_ref() {
        "type" => {
            article::table
                .filter(article::typestring.eq(value))
                .order(article::date)
                .limit(pagesize)
                .offset(offset)
                .load::<Article>(&*conn)
        }
        "label" => {
            article::table
                .filter(
                    article::typestring
                        .concat(",")
                        .concat(article::labels)
                        .concat(",")
                        .like(format!("%,{},%", value)),
                )
                .order(article::date)
                .limit(pagesize)
                .offset(offset)
                .load::<Article>(&*conn)
        }
        "archive" => {
            let date: Vec<&str> = value.split("-").collect();
            let year = date[0].parse::<u32>().unwrap_or(2000);
            let month = date[1].parse::<u32>().unwrap_or(1);
            article::table
                .filter(article::date.ge(format!("{:04}-{:02}", year, month)).and(
                    article::date.lt(format!("{:04}-{:02}", year, month + 1)),
                ))
                .order(article::date)
                .limit(pagesize)
                .offset(offset)
                .load::<Article>(&*conn)
        }
        "search" => {
            article::table
                .filter(
                    article::title
                        .like(format!("%{}%", value))
                        .or(article::brief.like(format!("%{}%", value)))
                        .or(article::typestring.like(format!("%{}%", value)))
                        .or(article::labels.like(format!("%{}%", value))),
                )
                .order(article::date)
                .limit(pagesize)
                .offset(offset)
                .load::<Article>(&*conn)
        }
        _ => {
            article::table
                .order(article::date)
                .limit(pagesize)
                .offset(offset)
                .load::<Article>(&*conn)
        }
    };
    let mut result = query.expect("error");
    // mask content
    for a in &mut result {
        a.content = String::from("")
    }
    Json(result)
}

#[post("/getArticle", data = "<param>")]
fn get_article(conn: DbConn, param: Json<HashMap<String, String>>) -> Json<Article> {
    let article = match param["id"].parse::<i32>() {
        Ok(id) => article::table.find(id).first(&*conn).expect("error"),
        Err(_) => Article {
            id: Some(-1),
            title: String::default(),
            brief: String::default(),
            typestring: String::default(),
            labels: String::default(),
            content: String::default(),
            date: Local::now().naive_local(),
        },
    };
    Json(article)
}

#[post("/getArticleNav", data = "<param>")]
fn get_article_nav(conn: DbConn, param: Json<HashMap<String, String>>) -> Json<serde_json::Value> {
    use diesel::result::Error;
    fn art_to_nav(result: Result<String, Error>, art_id: i32) -> serde_json::Value {
        match result {
            Ok(s) => json!({"id": art_id, "title": s }),
            _ => json!({"id": -1, "title": "没有了"}),
        }
    }

    let current_id = param["id"].parse::<i32>().expect("error");
    let art_pre: Result<String, _> = article::table
        .select(article::title)
        .find(current_id - 1)
        .first(&*conn);
    let art_next: Result<String, _> = article::table
        .select(article::title)
        .find(current_id + 1)
        .first(&*conn);
    Json(json!({
        "pre":art_to_nav(art_pre, current_id - 1),
        "next":art_to_nav(art_next, current_id + 1)
    }))
}

#[post("/addArticle", data = "<art>")]
fn add_article(mut cookies: Cookies, conn: DbConn, cache: State<ALCache>, art: Json<Article>) {
    cookies.get_private("isLogin").expect("Validate Error");
    diesel::insert(&*art)
        .into(article::table)
        .execute(&*conn)
        .expect("insert error");
    cache.refresh_cache(&*conn);
}

#[post("/updateArticle", data = "<art>")]
fn update_article(mut cookies: Cookies, conn: DbConn, cache: State<ALCache>, art: Json<Article>) {
    cookies.get_private("isLogin").expect("Validate Error");
    diesel::update(article::table.filter(article::id.eq(art.id)))
        .set((
            article::title.eq(&art.title),
            article::brief.eq(&art.brief),
            article::content.eq(&art.content),
            article::typestring.eq(&art.typestring),
            article::labels.eq(&art.labels),
            article::date.eq(&art.date),
        ))
        .execute(&*conn)
        .expect("update error");
    cache.refresh_cache(&*conn);
}

#[post("/deleteArticle", data = "<param>")]
fn delete_article(mut cookies: Cookies, conn: DbConn, cache: State<ALCache>, param: String) {
    cookies.get_private("isLogin").expect("Validate Error");
    let id = param.parse::<i32>().expect("wrong parameter");
    diesel::delete(article::table.filter(article::id.eq(id)))
        .execute(&*conn)
        .expect("delete error");
    cache.refresh_cache(&*conn);
}
