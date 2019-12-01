use rocket_contrib::databases::diesel;

#[database("koumakan")]
pub struct DbConn(diesel::SqliteConnection);
