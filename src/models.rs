use chrono::NaiveDateTime;

infer_schema!("koumakan.db");

#[derive(Serialize)]
#[derive(Queryable, Insertable)]
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

#[derive(Serialize)]
#[derive(Queryable, Insertable)]
#[table_name = "article"]
pub struct Aritcle {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub brief: Option<String>,
    pub content: Option<String>,
    pub typestring: Option<String>,
    pub labels: Option<String>,
    pub date: Option<NaiveDateTime>,
}
