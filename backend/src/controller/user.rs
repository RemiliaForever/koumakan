use std::io::Read;

use rocket::data::Data;
use rocket::http::{Cookie, Cookies, Status};
use rocket::State;

#[post("/login", data = "<pass>")]
pub fn login(mut cookies: Cookies, token: State<String>, pass: Data) -> Status {
    let mut password = String::new();
    let _ = pass.open().read_to_string(&mut password);
    if *token == password {
        cookies.add_private(Cookie::new("isLogin", "true"));
        Status::Ok
    } else {
        Status::Unauthorized
    }
}

#[get("/login")]
pub fn check_login(mut cookies: Cookies) -> Status {
    match cookies.get_private("isLogin") {
        Some(_) => Status::Ok,
        None => Status::Unauthorized,
    }
}
