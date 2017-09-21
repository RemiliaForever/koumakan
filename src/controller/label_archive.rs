use std::sync::RwLock;
use std::collections::HashMap;

use rocket::State;
use rocket_contrib::Json;
use chrono::NaiveDateTime;
use diesel::prelude::*;

use models::*;
use db::DbConn;

pub struct ALCache {
    archives: RwLock<HashMap<String, i32>>,
    labels: RwLock<HashMap<String, i32>>,
}

impl ALCache {
    pub fn init_cache(conn: DbConn) -> ALCache {
        let cache = ALCache {
            archives: RwLock::new(HashMap::new()),
            labels: RwLock::new(HashMap::new()),
        };
        cache.refresh_cache(&*conn);
        cache
    }

    pub fn refresh_cache(&self, conn: &SqliteConnection) {
        let result: Vec<(String, NaiveDateTime)> = article::table
            .select((article::labels, article::date))
            .load(conn)
            .expect("error");
        let labels: &mut HashMap<String, i32> = &mut *self.labels.write().unwrap();
        let archives: &mut HashMap<String, i32> = &mut *self.archives.write().unwrap();
        labels.clear();
        archives.clear();
        for (article_labels, article_date) in result {
            let labels_result: Vec<&str> = article_labels.split(",").collect();
            for label in labels_result {
                let count = labels.entry(String::from(label)).or_insert(0);
                *count += 1;
            }
            let archives_result: String = article_date.format("%Y-%m").to_string();
            let count = archives.entry(archives_result).or_insert(0);
            *count += 1;
        }
    }
}



#[post("/getArchive")]
fn get_archive(cache: State<ALCache>) -> Json<HashMap<String, i32>> {
    Json(cache.archives.read().unwrap().clone())
}

#[post("/getLabel")]
fn get_label(cache: State<ALCache>) -> Json<HashMap<String, i32>> {
    Json(cache.labels.read().unwrap().clone())
}
