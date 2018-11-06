use chrono::NaiveDateTime;
pub use crate::schema::*;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Queryable, Insertable)]
#[table_name = "comment"]
pub struct Comment {
    pub id: Option<i32>,
    pub article_id: i32,
    pub name: String,
    pub email: String,
    pub website: String,
    pub content: String,
    pub avatar: String,
    #[serde(with = "my_date_format")]
    pub date: NaiveDateTime,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Queryable, Insertable)]
#[table_name = "article"]
pub struct Article {
    pub id: Option<i32>,
    pub title: String,
    pub brief: String,
    pub content: String,
    pub category: String,
    pub labels: String,
    #[serde(with = "my_date_format")]
    pub date: NaiveDateTime,
}

mod my_date_format {
    use chrono::NaiveDateTime;
    use serde::{Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y年%m月%d日 %H:%M:%S";

    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&date.format(FORMAT).to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match NaiveDateTime::parse_from_str(&s, FORMAT) {
            Ok(s) => Ok(s),
            Err(_) => Ok(NaiveDateTime::from_timestamp(0, 0)),
        }
    }
}
