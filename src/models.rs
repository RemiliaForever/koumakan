use chrono::NaiveDateTime;

infer_schema!("koumakan.db");

#[derive(Serialize, Deserialize)]
#[derive(Queryable, Insertable)]
#[table_name = "comment"]
pub struct Comment {
    pub id: Option<i32>,
    pub article_id: i32,
    pub name: String,
    pub avatar: String,
    pub email: String,
    pub website: String,
    pub content: String,
    #[serde(with = "my_date_format")]
    pub date: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
#[derive(Queryable, Insertable)]
#[table_name = "article"]
pub struct Aritcle {
    pub id: Option<i32>,
    pub title: String,
    pub brief: String,
    pub content: String,
    pub typestring: String,
    pub labels: String,
    #[serde(with = "my_date_format")]
    pub date: NaiveDateTime,
}

mod my_date_format {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y年%m月%d日 %H:%M:%S";

    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}", date.format(FORMAT)))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}
