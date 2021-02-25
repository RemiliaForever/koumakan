mod article;
mod comment;
mod label_archive;
mod user;

use actix_web::{web, HttpResponse};

pub use self::label_archive::ALCache;
use crate::common::ResError;

#[inline]
pub fn effect_one(result: u64) -> Result<HttpResponse, ResError> {
    if result == 1 {
        Ok(HttpResponse::Ok().finish())
    } else if result == 0 {
        Ok(HttpResponse::NotFound().finish())
    } else {
        Err(ResError::new(format!("unexpected effect raw: {}", result)).into())
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(article::get_article_list);
    cfg.service(article::get_article);
    cfg.service(article::get_article_nav);
    cfg.service(article::create_article);
    cfg.service(article::update_article);
    cfg.service(article::delete_article);

    cfg.service(comment::get_article_comments);
    cfg.service(comment::create_comment);

    cfg.service(label_archive::get_archive);
    cfg.service(label_archive::get_label);
    cfg.service(label_archive::get_rss_feed);
    cfg.service(label_archive::get_rss_full);

    cfg.service(user::get_login);
}
