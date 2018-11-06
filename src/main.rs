#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

mod catcher;
mod controller;
mod db;
mod model;
mod schema;

fn main() {
    let server = rocket::ignite();
    // let pool = db::init();
    let token = String::from(
        server
            .config()
            .get_str("token")
            .expect("init server token failed!"),
    );
    server
        .mount("/", controller::get_routes())
        .register(catcher::get_catchers())
        .attach(db::DbConn::fairing())
        .manage(token)
        .manage(controller::ALCache::init_cache())
        .launch();
}
