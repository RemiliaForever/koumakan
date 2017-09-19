use chrono;
use diesel;
use diesel::prelude::*;
use rocket_contrib::Json;

use db::DbConn;
use models;
use models::*;
use models::comment::dsl::*;
use models::article::dsl::*;

#[post("/getArticleList", data = "<param>")]
fn get_article_list(conn: DbConn, param: String) -> Json<Vec<Aritcle>> {
    println!("{}", param);
    Json(article.load::<Aritcle>(&*conn).expect("query error!"))
}

#[post("/getArchive")]
fn get_archive(conn: DbConn) -> Json<Vec<Comment>> {
    Json(comment.load::<Comment>(&*conn).expect("query error!"))
}

#[post("/getLabel")]
fn get_label(conn: DbConn) -> Json<Vec<Comment>> {
    Json(comment.load::<Comment>(&*conn).expect("query error!"))
}



#[post("/getArticle")]
fn get_article(conn: DbConn) -> Json<Aritcle> {
    Json(Aritcle {
        id: Some(1),
        title: String::from("title"),
        brief: String::from("asfd"),
        content: String::from("asdf"),
        typestring: String::from("IT"),
        labels: String::from("asfd,asfd"),
        date: chrono::Local::now().naive_local(),
    })
}

#[post("/getArticleNav")]
fn get_article_nav(conn: DbConn) -> String {
    json!({
        "pre":{
            "id": 1,
            "title": "asdf"
        },
        "next":{
            "id": 3,
            "title": "asdf"
        }
    }).to_string()
}

#[post("/getComment")]
fn get_comment(conn: DbConn) -> Json<Vec<Comment>> {
    Json(vec![
         Comment {
             id: Some(3),
             article_id: 4,
             name: String::from("test"),
             avatar: String::from("asfd"),
             email: String::from("afd@asdf"),
             website: String::from("asdf"),
             content: String::from("asfd"),
             date: chrono::Local::now().naive_local(),
         },
    ])
}

#[post("/addComment", data = "<cmt>")]
fn add_comment(conn: DbConn, cmt: Json<Comment>) {
    let _ = diesel::insert(&cmt.0).into(comment).execute(&*conn);
}
