use std::sync::RwLock;
use std::collections::BTreeMap;

use rocket::State;
use rocket_contrib::Json;
use diesel::prelude::*;

use models::*;
use db::DbConn;
use rss::{Channel, ChannelBuilder, ItemBuilder};

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
                    .link("https://koumakan.cc")
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
            items.push(
                ItemBuilder::default()
                    .title(article.title)
                    .description(article.brief)
                    .link(format!(
                        "https://koumakan.cc/article/{}",
                        article.id.unwrap()
                    ))
                    .build()
                    .unwrap(),
            );
        }
        rss.set_items(items);
    }
}



#[post("/getArchive")]
fn get_archive(cache: State<ALCache>) -> Json<BTreeMap<String, i32>> {
    Json(cache.archives.read().unwrap().clone())
}

#[post("/getLabel")]
fn get_label(cache: State<ALCache>) -> Json<BTreeMap<String, i32>> {
    Json(cache.labels.read().unwrap().clone())
}

#[get("/rss")]
fn rss(cache: State<ALCache>) -> String {
    cache.rss.read().unwrap().to_string()
}
