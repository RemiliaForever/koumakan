#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

mod controller;
mod db;
mod models;
mod schema;

fn main() {
    use rocket::fairing::AdHoc;

    let server = rocket::ignite();
    let pool = db::init();
    let cache = controller::ALCache::init_cache(db::DbConn(
        pool.get().expect("init database connection pool failed!"),
    ));
    server
        .mount("/", controller::get_routes())
        .attach(AdHoc::on_attach(|server| {
            let token = String::from(
                server
                    .config()
                    .get_str("token")
                    .expect("init server token failed!"),
            );
            Ok(server.manage(token))
        }))
        .manage(pool)
        .manage(cache)
        .launch();
}
