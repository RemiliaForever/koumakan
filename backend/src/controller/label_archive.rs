use std::{
    collections::BTreeMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use tokio::sync::RwLock;

use actix_web::{get, web, HttpResponse};
use chrono::{offset::Local, DateTime};
use rss::{Category, Channel, ChannelBuilder, ItemBuilder};
use sqlx::SqlitePool;

use crate::controller::ResError;
use common::Article;

fn render_markdown(input: &str) -> String {
    let parser = pulldown_cmark::Parser::new_ext(input, pulldown_cmark::Options::all());
    let mut output = String::with_capacity(input.len() * 3 / 2);
    pulldown_cmark::html::push_html(&mut output, parser);
    output
}

pub struct ALCache {
    archives: Arc<RwLock<BTreeMap<String, i32>>>,
    labels: Arc<RwLock<BTreeMap<String, i32>>>,
    rss_feed: Arc<RwLock<Channel>>,
    rss_fulltext: Arc<RwLock<Channel>>,
    is_dirty: Arc<AtomicBool>,
}

impl ALCache {
    pub fn init_cache() -> ALCache {
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
            archives: Arc::new(RwLock::new(BTreeMap::new())),
            labels: Arc::new(RwLock::new(BTreeMap::new())),
            rss_feed: Arc::new(RwLock::new(rss_builder.clone())),
            rss_fulltext: Arc::new(RwLock::new(rss_builder.clone())),
            is_dirty: Arc::new(AtomicBool::new(true)),
        };
        cache
    }

    pub async fn refresh_cache(&self, pool: &SqlitePool) -> Result<(), ResError> {
        let labels = &mut *self.labels.write().await;
        let archives = &mut *self.archives.write().await;
        labels.clear();
        archives.clear();

        let rss_feed = &mut *self.rss_feed.write().await;
        let rss_fulltext = &mut *self.rss_fulltext.write().await;
        // filter help page
        let result = sqlx::query_as!(
            Article,
            "SELECT * FROM article WHERE id > 20000 ORDER BY date DESC"
        )
        .fetch_all(pool)
        .await?;
        let mut feed_items = Vec::new();
        let mut fulltext_items = Vec::new();
        for article in result {
            let labels_result: Vec<&str> = article.labels.split(",").collect();
            labels_result
                .into_iter()
                .map(|label| {
                    labels
                        .entry(label.to_owned())
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                })
                .count();
            let archives_result: String = article.date.format("%Y-%m").to_string();
            archives
                .entry(archives_result)
                .and_modify(|count| *count += 1)
                .or_insert(1);

            let mut category = Category::default();
            category.set_name(article.category);
            let offset = chrono::FixedOffset::east(8 * 3600);
            let datetime = DateTime::<Local>::from_utc(article.date - offset, offset);
            let mut item_builder = ItemBuilder::default()
                .title(article.title)
                .link(format!("https://blog.koumakan.cc/article/{}", article.id))
                .categories(vec![category])
                .pub_date(datetime.to_rfc2822())
                .build()
                .unwrap();
            item_builder.set_description(article.brief.clone());
            feed_items.push(item_builder.clone());
            item_builder.set_description(format!(
                "{}\n\n{}",
                article.brief,
                render_markdown(&article.content)
            ));
            fulltext_items.push(item_builder);
        }
        let date = Local::now().to_rfc2822();
        rss_feed.set_items(feed_items);
        rss_feed.set_pub_date(date.clone());
        rss_feed.set_last_build_date(date.clone());
        rss_fulltext.set_items(fulltext_items);
        rss_fulltext.set_pub_date(date.clone());
        rss_fulltext.set_last_build_date(date.clone());

        Ok(())
    }

    #[inline]
    pub fn dirty(&self) {
        self.is_dirty.store(true, Ordering::Relaxed);
    }

    #[inline]
    pub async fn check_dirty(&self, pool: &SqlitePool) -> Result<(), ResError> {
        if self.is_dirty.load(Ordering::Relaxed) {
            self.refresh_cache(pool).await?;
            self.is_dirty.store(false, Ordering::Relaxed);
        }
        Ok(())
    }
}

#[get("/archive")]
async fn get_archive(
    pool: web::Data<SqlitePool>,
    cache: web::Data<ALCache>,
) -> Result<HttpResponse, ResError> {
    cache.check_dirty(&**pool).await?;
    let result = cache.archives.read().await;
    Ok(HttpResponse::Ok().body(bincode::serialize(&*result)?))
}

#[get("/label")]
async fn get_label(
    pool: web::Data<SqlitePool>,
    cache: web::Data<ALCache>,
) -> Result<HttpResponse, ResError> {
    cache.check_dirty(&**pool).await?;
    let result = cache.labels.read().await;
    Ok(HttpResponse::Ok().body(bincode::serialize(&*result)?))
}

#[get("/rss/feed")]
async fn get_rss_feed(
    pool: web::Data<SqlitePool>,
    cache: web::Data<ALCache>,
) -> Result<HttpResponse, ResError> {
    cache.check_dirty(&**pool).await?;
    Ok(HttpResponse::Ok()
        .insert_header(("Content-Type", "application/rss+xml; charset=utf-8"))
        .body(cache.rss_feed.read().await.to_string()))
}

#[get("/rss/full")]
async fn get_rss_full(
    pool: web::Data<SqlitePool>,
    cache: web::Data<ALCache>,
) -> Result<HttpResponse, ResError> {
    cache.check_dirty(&**pool).await?;
    Ok(HttpResponse::Ok()
        .insert_header(("Content-Type", "application/rss+xml; charset=utf-8"))
        .body(cache.rss_fulltext.read().await.to_string()))
}
