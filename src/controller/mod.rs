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

pub fn get_routes() -> Vec<Route> {
    routes![
        article::get_article,
        article::get_article_nav,
        article::get_article_list,
        article::get_article_list_default,
        article::post_article,
        article::put_article,
        article::delete_article,
        user::login,
    ]
}
