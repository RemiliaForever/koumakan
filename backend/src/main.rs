#[macro_use]
extern crate log;

use actix_web::{middleware, App, HttpServer};
use anyhow::Result;
use sqlx::SqlitePool;

mod catcher;
mod controller;

#[actix_rt::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let db_pool = SqlitePool::connect(&dotenv::var("DATABASE_URL").unwrap()).await?;
    sqlx::migrate!("./migrations").run(&db_pool).await?;

    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .wrap(middleware::Logger::default())
            .configure(controller::init)
    })
    .bind(format!(
        "{}:{}",
        dotenv::var("HOST").unwrap(),
        dotenv::var("PORT").unwrap()
    ))?
    .run()
    .await?;

    Ok(())
}
