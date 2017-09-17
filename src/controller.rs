use diesel::prelude::*;
use rocket_contrib::Json;
use chrono;

use db::DbConn;
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
        title: Some(String::from("title")),
        brief: Some(String::from("asfd")),
        content: Some(String::from("asdf")),
        typestring: Some(String::from("IT")),
        labels: Some(String::from("asfd,asfd")),
        date: Some(chrono::Local::now().naive_local()),
    })
}
#[post("/getComment")]
fn get_comment(conn: DbConn) -> Json<Vec<Comment>> {
    Json(vec![
        Comment {
            id: Some(3),
            article_id: Some(4),
            name: Some(String::from("test")),
            avatar: Some(String::from("asfd")),
            email: Some(String::from("afd@asdf")),
            website: Some(String::from("asdf")),
            content: Some(String::from("asfd")),
            date: Some(chrono::Local::now().naive_local()),
        },
    ])
}
