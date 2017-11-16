mod article;
mod comment;
mod label_archive;
mod user;

use rocket::Route;
pub use self::label_archive::ALCache;

pub fn get_routes() -> Vec<Route> {
    routes![
        article::get_article,
        article::get_article_nav,
        article::get_article_list,
        article::get_article_list_default,
        article::post_article,
        article::put_article,
        article::delete_article,
        comment::get_comments,
        comment::post_comments,
        label_archive::get_label,
        label_archive::get_archive,
        label_archive::rss,
        user::login,
    ]
}
