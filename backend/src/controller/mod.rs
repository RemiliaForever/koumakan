mod article;
mod comment;
mod label_archive;
mod user;

use axum::{
    error_handling::HandleErrorLayer,
    handler::Handler,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    AddExtensionLayer,
    BoxError,
    Json,
    Router,
};
use tower::{layer::layer_fn, timeout::TimeoutLayer, ServiceBuilder};

async fn not_found() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(StatusCode::NOT_FOUND.canonical_reason()),
    )
}

pub fn router(db: sqlx::SqlitePool) -> Router {
    let public_route = Router::new()
        .route("/articles", get(article::get_article_list))
        .route("/article/:id", get(article::get_article))
        .route("/article/:id/nav", get(article::get_article_nav))
        .route("/article/:id/comment", get(comment::get_article_comments))
        .route("/archive", get(label_archive::get_archive))
        .route("/label", get(label_archive::get_label))
        .route("/rss/feed", get(label_archive::get_rss_feed))
        .route("/rss/full", get(label_archive::get_rss_full));
    let private_route = Router::new()
        .route("/login", get(user::get_login))
        .route("/article", post(article::create_article))
        .route("/article/:id", put(article::update_article))
        .route("/article/:id", delete(article::delete_article))
        .route("/article/:id/comment", post(comment::create_comment))
        .layer(layer_fn(|inner| user::Authorization { inner }));
    let app = Router::new()
        .merge(public_route)
        .merge(private_route)
        .fallback(not_found.into_service())
        .layer(AddExtensionLayer::new(db))
        .layer(AddExtensionLayer::new(label_archive::ALCache::new()))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(async move |_: BoxError| {
                    StatusCode::REQUEST_TIMEOUT
                }))
                .layer(TimeoutLayer::new(std::time::Duration::from_secs(10))),
        );

    app
}
