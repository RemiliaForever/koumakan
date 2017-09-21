use std::collections::HashMap;

use diesel;
use serde_json;
use chrono::*;
use diesel::prelude::*;
use rocket_contrib::Json;
use rocket::request::State;

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
fn get_article(conn: DbConn, param: Json<HashMap<String, i32>>) -> Json<Article> {
    let article = article::table.find(param["id"]).first(&*conn);
    Json(article.expect("error"))
}

#[post("/getArticleNav", data = "<param>")]
fn get_article_nav(conn: DbConn, param: Json<HashMap<String, i32>>) -> Json<serde_json::Value> {
    let current_id = param["id"];
    let art_pre: Result<String, _> = article::table
        .select(article::title)
        .find(current_id - 1)
        .first(&*conn);
    let art_nav_pre = match art_pre {
        Ok(s) => {
            json!({
                "id": current_id - 1,
                "title": s
            })
        }
        _ => {
            json!({
                "id": -1,
                "title": ""
            })
        }
    };
    let art_next: Result<String, _> = article::table
        .select(article::title)
        .find(current_id + 1)
        .first(&*conn);
    let art_nav_next = match art_next {
        Ok(s) => {
            json!({
                "id": current_id + 1,
                "title": s
            })
        }
        _ => {
            json!({
                "id": -1,
                "title": ""
            })
        }
    };
    Json(json!({
        "pre":art_nav_pre,
        "next":art_nav_next
    }))
}

#[post("/addArticle", data = "<art>")]
fn add_article(conn: DbConn, cache: State<ALCache>, art: Json<Article>) {
    diesel::insert(&*art)
        .into(article::table)
        .execute(&*conn)
        .expect("insert error");
    cache.refresh_cache(&*conn);
}

#[post("/updateArticle", data = "<art>")]
fn update_article(conn: DbConn, cache: State<ALCache>, art: Json<Article>) {
    diesel::insert(&*art)
        .into(article::table)
        .execute(&*conn)
        .expect("insert error");
    cache.refresh_cache(&*conn);
}
