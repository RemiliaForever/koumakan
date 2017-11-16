use std::sync::RwLock;
use std::collections::BTreeMap;

use rocket::State;
use rocket::response::content::Xml;
use rocket_contrib::Json;
use diesel::prelude::*;
use chrono;
use chrono::DateTime;
use chrono::offset::Local;

use models::*;
use db::DbConn;
use rss::{Category, Channel, ChannelBuilder, ItemBuilder};

pub struct ALCache {
    archives: RwLock<BTreeMap<String, i32>>,
    labels: RwLock<BTreeMap<String, i32>>,
    rss: RwLock<Channel>,
}

impl ALCache {
    pub fn init_cache(conn: DbConn) -> ALCache {
        let cache = ALCache {
            archives: RwLock::new(BTreeMap::new()),
            labels: RwLock::new(BTreeMap::new()),
            rss: RwLock::new(
                ChannelBuilder::default()
                    .title("RemiliaForever's Blog")
                    .description("Welcome to Koumakan")
                    .link("https://blog.koumakan.cc")
                    .language(Some(String::from("zh-cn")))
                    .copyright(Some(String::from("CC BY-NC-SA 4.0")))
                    .webmaster(Some(String::from("remilia@koumakan.cc")))
                    .build()
                    .unwrap(),
            ),
        };
        cache.refresh_cache(&*conn);
        cache
    }

    pub fn refresh_cache(&self, conn: &SqliteConnection) {
        let result: Vec<Article> = article::table.load(conn).expect("error");
        let labels: &mut BTreeMap<String, i32> = &mut *self.labels.write().unwrap();
        let archives: &mut BTreeMap<String, i32> = &mut *self.archives.write().unwrap();
        let rss: &mut Channel = &mut *self.rss.write().unwrap();
        labels.clear();
        archives.clear();
        let mut items = Vec::new();
        for article in result {
            let labels_result: Vec<&str> = article.labels.split(",").collect();
            for label in labels_result {
                let count = labels.entry(String::from(label)).or_insert(0);
                *count += 1;
            }
            let archives_result: String = article.date.format("%Y-%m").to_string();
            let count = archives.entry(archives_result).or_insert(0);
            *count += 1;

            let mut category = Category::default();
            category.set_name(article.category);
            let offset = chrono::FixedOffset::east(8 * 3600);
            let datetime = DateTime::<Local>::from_utc(article.date - offset, offset);
            items.push(
                ItemBuilder::default()
                    .title(article.title)
                    .description(article.brief)
                    .link(format!(
                        "https://blog.koumakan.cc/article/{}",
                        article.id.unwrap()
                    ))
                    .categories(vec![category])
                    .pub_date(datetime.to_rfc2822())
                    .build()
                    .unwrap(),
            );
        }
        rss.set_items(items);
    }
}



#[get("/archive")]
fn get_archive(cache: State<ALCache>) -> Json<BTreeMap<String, i32>> {
    Json(cache.archives.read().unwrap().clone())
}

#[get("/labels")]
fn get_label(cache: State<ALCache>) -> Json<BTreeMap<String, i32>> {
    Json(cache.labels.read().unwrap().clone())
}

#[get("/rss")]
fn rss(cache: State<ALCache>) -> Xml<String> {
    Xml(cache.rss.read().unwrap().to_string())
}
