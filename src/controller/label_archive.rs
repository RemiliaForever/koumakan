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
    rss_feed: RwLock<Channel>,
    rss_fulltext: RwLock<Channel>,
}

impl ALCache {
    pub fn init_cache(conn: DbConn) -> ALCache {
        let rss_builder = ChannelBuilder::default()
            .title("RemiliaForever's Blog")
            .description("Welcome to Koumakan")
            .link("https://blog.koumakan.cc")
            .language(Some(String::from("zh-cn")))
            .copyright(Some(String::from("CC BY-NC-SA 4.0")))
            .webmaster(Some(String::from("remilia@koumakan.cc")))
            .build()
            .unwrap();
        let cache = ALCache {
            archives: RwLock::new(BTreeMap::new()),
            labels: RwLock::new(BTreeMap::new()),
            rss_feed: RwLock::new(rss_builder.clone()),
            rss_fulltext: RwLock::new(rss_builder.clone()),
        };
        cache.refresh_cache(&*conn);
        cache
    }

    pub fn refresh_cache(&self, conn: &SqliteConnection) {
        let labels: &mut BTreeMap<String, i32> = &mut *self.labels.write().unwrap();
        let archives: &mut BTreeMap<String, i32> = &mut *self.archives.write().unwrap();
        labels.clear();
        archives.clear();

        let rss_feed: &mut Channel = &mut *self.rss_feed.write().unwrap();
        let rss_fulltext: &mut Channel = &mut *self.rss_fulltext.write().unwrap();
        // less 20000 for help page
        let result: Vec<Article> = article::table
            .filter(article::id.gt(20000))
            .order(article::date.desc())
            .load(conn)
            .expect("error");
        let mut feed_items = Vec::new();
        let mut fulltext_items = Vec::new();
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
            let mut item_builder = ItemBuilder::default()
                .title(article.title)
                .link(format!(
                    "https://blog.koumakan.cc/article/{}",
                    article.id.unwrap()
                ))
                .categories(vec![category])
                .pub_date(datetime.to_rfc2822())
                .build()
                .unwrap();
            item_builder.set_description(article.brief.clone());
            feed_items.push(item_builder.clone());
            item_builder.set_description(format!("{}\n\n{}", article.brief, article.content));
            fulltext_items.push(item_builder);
        }
        let date = Local::now().to_rfc2822();
        rss_feed.set_items(feed_items);
        rss_feed.set_pub_date(date.clone());
        rss_feed.set_last_build_date(date.clone());
        rss_fulltext.set_items(fulltext_items);
        rss_fulltext.set_pub_date(date.clone());
        rss_fulltext.set_last_build_date(date.clone());
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

#[get("/rss/feed")]
fn rss_feed(cache: State<ALCache>) -> Xml<String> {
    Xml(cache.rss_feed.read().unwrap().to_string())
}

#[get("/rss/full")]
fn rss_full(cache: State<ALCache>) -> Xml<String> {
    Xml(cache.rss_fulltext.read().unwrap().to_string())
}
