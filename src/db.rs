extern crate r2d2;

use r2d2_diesel::ConnectionManager;
use diesel::sqlite::SqliteConnection;

use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn init() -> Pool {
    let config = r2d2::Config::builder().pool_size(3).build();
    let manager = ConnectionManager::<SqliteConnection>::new("koumakan.db");
    r2d2::Pool::new(config, manager).expect("db pool")
}

pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<SqliteConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
