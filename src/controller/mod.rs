mod article;
mod comment;
mod label_archive;
mod user;

use rocket::Route;
pub use self::label_archive::ALCache;

use rocket_contrib::Json;
use std::collections::HashMap;
fn get_or_null(param: &Json<HashMap<String, String>>, key: &str) -> String {
    match param.get(key) {
        Some(s) => s.clone(),
        None => String::from(""),
    }
}

pub fn get_api_routes() -> Vec<Route> {
    routes![
        label_archive::get_archive,
        label_archive::get_label,
        article::get_article_list,
        article::get_article,
        article::get_article_nav,
        article::add_article,
        article::update_article,
        article::delete_article,
        comment::get_comments,
        comment::add_comment,
        user::login,
    ]
}

pub fn get_root_routes() -> Vec<Route> {
    routes![label_archive::rss]
}
