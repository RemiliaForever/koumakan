mod article;
mod comment;
mod label_archive;
mod user;

//pub use self::label_archive::ALCache;

use actix_web::{web, Error, HttpResponse, ResponseError};

#[derive(Debug)]
pub struct ResError<T>
where
    T: std::fmt::Debug,
{
    cause: T,
}

impl<T> std::fmt::Display for ResError<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl<T> ResError<T>
where
    T: std::fmt::Debug + 'static,
{
    #[inline]
    pub fn new(e: T) -> ResError<T> {
        ResError { cause: e }
    }
}

impl<T> ResponseError for ResError<T> where T: std::fmt::Debug {}

#[inline]
pub fn effect_one(result: u64) -> Result<HttpResponse, Error> {
    if result == 1 {
        Ok(HttpResponse::Ok().finish())
    } else if result == 0 {
        Ok(HttpResponse::NotFound().finish())
    } else {
        Err(ResError::new(format!("unexpected effect raw: {}", result)).into())
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(article::get_article);
    cfg.service(article::get_article_nav);
    cfg.service(article::create_article);
    cfg.service(article::update_article);
    cfg.service(article::delete_article);

    cfg.service(comment::get_article_comments);
    cfg.service(comment::create_comment);

    cfg.service(user::get_login);
}
