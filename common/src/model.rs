use serde::{Deserialize, Serialize};

#[cfg(feature = "sqlx")]
use sqlx::FromRow;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(FromRow))]
pub struct Comment {
    pub id: i64,
    pub article_id: i64,
    pub name: String,
    pub email: String,
    pub website: String,
    pub content: String,
    pub avatar: String,
    #[serde(with = "my_date_format")]
    pub date: chrono::NaiveDateTime,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(FromRow))]
pub struct Article {
    pub id: i64,
    pub title: String,
    pub brief: String,
    pub content: String,
    pub category: String,
    pub labels: String,
    #[serde(with = "my_date_format")]
    pub date: chrono::NaiveDateTime,
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
