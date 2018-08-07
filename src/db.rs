use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::sqlite::SqliteConnection;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::ops::Deref;

type KoumakanPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn init() -> KoumakanPool {
    let manager = ConnectionManager::<SqliteConnection>::new("koumakan.db");
    KoumakanPool::builder()
        .max_size(3)
        .build(manager)
        .expect("db pool build failed.")
}

pub struct DbConn(pub PooledConnection<ConnectionManager<SqliteConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<KoumakanPool>>()?;
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
