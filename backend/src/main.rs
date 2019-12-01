#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod catcher;
mod controller;
mod db;
mod model;

fn main() {
    let server = rocket::ignite();
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
