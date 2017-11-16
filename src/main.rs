#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#![recursion_limit="128"]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate r2d2_diesel;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate rocket;
extern crate rocket_contrib;

extern crate chrono;
extern crate md5;
extern crate rss;

mod db;
mod models;
mod controller;

fn main() {
    use rocket::fairing::AdHoc;

    let server = rocket::ignite();
    let pool = db::init();
    let cache = controller::ALCache::init_cache(db::DbConn(pool.get().unwrap()));
    server
        .mount("/", controller::get_routes())
        .attach(AdHoc::on_attach(|server| {
            let token = String::from(server.config().get_str("token").unwrap());
            Ok(server.manage(token))
        }))
        .manage(pool)
        .manage(cache)
        .launch();
}
