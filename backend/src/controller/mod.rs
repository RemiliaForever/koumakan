mod article;
//mod comment;
//mod label_archive;
mod user;

//pub use self::label_archive::ALCache;

use actix_web::{web, ResponseError};

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
    pub fn new(e: T) -> ResError<T> {
        ResError { cause: e }
    }
}

impl<T> ResponseError for ResError<T> where T: std::fmt::Debug {}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(article::get_article);
    cfg.service(article::get_article_nav);

    cfg.service(user::get_login);
}
