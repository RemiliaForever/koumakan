use diesel::prelude::*;
use rocket_contrib::Json;
use rocket::State;

use std::sync::RwLock;
use std::collections::HashMap;

use models::*;
use db::DbConn;

pub struct ALCache {
    archives: RwLock<HashMap<String, i32>>,
    labels: RwLock<HashMap<String, i32>>,
}

pub fn init_cache(conn: DbConn) -> ALCache {
    ALCache {
        archives: RwLock::new(HashMap::new()),
        labels: RwLock::new(HashMap::new()),
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
