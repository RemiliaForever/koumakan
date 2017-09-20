use diesel;
use serde_json;
use diesel::prelude::*;
use rocket_contrib::Json;

use std::collections::HashMap;

use db::DbConn;
use models::*;

#[post("/getArticleList", data = "<param>")]
fn get_article_list(conn: DbConn, param: Json<HashMap<String, String>>) -> Json<Vec<Aritcle>> {
    println!("{:?}", param);
    Json(article::table.load::<Aritcle>(&*conn).expect(
        "query error!",
    ))
}

#[post("/getArticle", data = "<param>")]
fn get_article(conn: DbConn, param: Json<HashMap<String, i32>>) -> Json<Aritcle> {
    Json(
        article::table.find(param["id"]).first::<Aritcle>(&*conn).expect("query error!"),
    )
}

#[post("/getArticleNav", data = "<param>")]
fn get_article_nav(conn: DbConn, param: Json<HashMap<String, i32>>) -> Json<serde_json::Value> {
    let current_id = param["id"];
    let art_pre: Result<String, _> =
        article::table.select(article::title).find(current_id - 1).first(&*conn);
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
    let art_next: Result<String, _> =
        article::table.select(article::title).find(current_id + 1).first(&*conn);
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
fn add_article(conn: DbConn, art: Json<Aritcle>) {
    let _ = diesel::insert(&*art).into(article::table).execute(&*conn);
}
