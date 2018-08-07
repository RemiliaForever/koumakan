use rocket::http::{Cookie, Cookies};
use rocket::State;

#[post("/login", data = "<pass>")]
fn login(mut cookies: Cookies, token: State<String>, pass: String) -> &'static str {
    if *token == pass {
        cookies.add_private(Cookie::new("isLogin", "true"));
        "Login Success"
    } else {
        "Validate Error"
    }
}

#[get("/login")]
fn check_login(mut cookies: Cookies) {
    cookies.get_private("isLogin").expect("Validate Error");
}
