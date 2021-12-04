#![feature(async_closure)]

mod controller;
mod error;
mod util;

use std::str::FromStr;

use sqlx::SqlitePool;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), error::Error> {
    dotenv::dotenv().ok();
    init_log();
    let db = init_db().await?;
    serve_web(db).await
}

async fn shutdown_signal() {
    use std::io;
    use tokio::signal::unix::SignalKind;

    async fn terminate() -> io::Result<()> {
        tokio::signal::unix::signal(SignalKind::terminate())?
            .recv()
            .await;
        Ok(())
    }

    tokio::select! {
        _ = terminate() => {},
        _ = tokio::signal::ctrl_c() => {},
    }
    log::warn!("signal received, starting graceful shutdown")
}

fn init_log() {
    use env_logger::fmt::Color;
    use std::io::Write;

    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            let mut style = buf.style();
            match record.level() {
                log::Level::Trace => style.set_color(Color::Cyan),
                log::Level::Debug => style.set_color(Color::Blue),
                log::Level::Info => style.set_color(Color::Green),
                log::Level::Warn => style.set_color(Color::Yellow),
                log::Level::Error => style.set_color(Color::Red).set_bold(true),
            };
            writeln!(
                buf,
                "[{}|{:<5}][{}:{}:{}] {}",
                style.value(chrono::Local::now().format("%Y-%m-%d %H:%M:%S.%6f")),
                style.value(record.level()),
                style.value(record.module_path().unwrap_or("unknown")),
                style.value(record.file_static().unwrap_or("unknown")),
                style.value(record.line().unwrap_or(0)),
                style.value(&record.args()),
            )
        })
        .init();
}

async fn init_db() -> Result<SqlitePool, error::Error> {
    let options = sqlx::sqlite::SqliteConnectOptions::from_str(&dotenv::var("DATABASE_URL")?)?
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .synchronous(sqlx::sqlite::SqliteSynchronous::Normal);
    let db_pool = sqlx::sqlite::SqlitePool::connect_with(options).await?;
    sqlx::migrate!("./migrations").run(&db_pool).await?;
    Ok(db_pool)
}

async fn serve_web(db: SqlitePool) -> Result<(), error::Error> {
    let addr = std::net::SocketAddr::from_str(&format!(
        "{}:{}",
        dotenv::var("HOST").unwrap(),
        dotenv::var("PORT").unwrap()
    ))
    .unwrap();

    log::info!("start listen on {}", addr);

    axum::Server::bind(&addr)
        .serve(controller::router(db).into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}
