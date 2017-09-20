use diesel;
use diesel::prelude::*;
use rocket_contrib::Json;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};

use std::collections::HashMap;

use db::DbConn;
use models::*;


pub struct ALCache {
    pub archives: HashMap<String, i32>,
    pub labels: HashMap<String, i32>,
}

pub fn init_cache() -> ALCache {
    ALCache {
        archives: HashMap::new(),
        labels: HashMap::new(),
    }
}


#[post("/getArchive")]
fn get_archive(cache: State<ALCache>) {
    println!("{:?} {:?}", cache.archives, cache.labels);
}

#[post("/getLabel")]
fn get_label() {}
