extern crate diesel;
extern crate chrono;

use chrono::NaiveDateTime;

infer_schema!("koumakan.db");

#[derive(Queryable, Insertable, Debug)]
#[table_name = "comment"]
pub struct Comment {
    pub id: Option<i32>,
    pub article_id: Option<i32>,
    pub name: Option<String>,
    pub avatar: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub content: Option<String>,
    pub date: Option<NaiveDateTime>,
}
