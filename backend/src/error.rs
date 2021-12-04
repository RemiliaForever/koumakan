use axum::body::BoxBody;
use hyper::{Response, StatusCode};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    EnvError(#[from] dotenv::Error),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error(transparent)]
    MigrateError(#[from] sqlx::migrate::MigrateError),
    #[error(transparent)]
    HyperError(#[from] hyper::Error),
    #[error(transparent)]
    LettreError(#[from] lettre::error::Error),
    #[error(transparent)]
    SendMailError(#[from] lettre::transport::sendmail::Error),
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> Response<BoxBody> {
        match self {
            Self::SqlxError(e) => match e {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
        .into_response()
    }
}
