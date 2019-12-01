#[cfg(feature = "orm")]
#[macro_use]
extern crate diesel;

mod model;
#[cfg(feature = "orm")]
pub mod schema;

pub use model::{Article, Comment};
