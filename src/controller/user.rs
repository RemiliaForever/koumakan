use rocket::State;
use rocket::http::{Cookie, Cookies};

#[post("/login", data = "<pass>")]
fn login(mut cookies: Cookies, token: State<String>, pass: String) -> &'static str {
    if &*token == &pass {
        cookies.add_private(Cookie::new("isLogin", "true"));
        "Login Success"
    } else {
        "Validate Error"
    }
}
