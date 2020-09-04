#[macro_use]
extern crate log;

use actix_web::{middleware, App, HttpServer};
use sqlx::SqlitePool;

mod controller;

#[actix_rt::main]
async fn main() -> Result<(), String> {
    dotenv::dotenv().ok();
    env_logger::init();

    let db_pool = SqlitePool::connect(&dotenv::var("DATABASE_URL").unwrap())
        .await
        .map_err(|e| e.to_string())?;
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .map_err(|e| e.to_string())?;

    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .data(controller::ALCache::init_cache())
            .wrap(middleware::Logger::default())
            .configure(controller::init)
    })
    .bind(format!(
        "{}:{}",
        dotenv::var("HOST").unwrap(),
        dotenv::var("PORT").unwrap()
    ))
    .map_err(|e| e.to_string())?
    .run()
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}
