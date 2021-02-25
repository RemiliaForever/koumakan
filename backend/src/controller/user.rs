use actix_web::{get, web, HttpResponse};

use crate::controller::ResError;

pub fn check_login(req: web::HttpRequest) -> Result<HttpResponse, ResError> {
    if let Some(h) = req.headers().get("Authorization") {
        if let Ok(token) = h.to_str() {
            if dotenv::var("TOKEN").unwrap() == token {
                Ok(HttpResponse::Ok().finish())
            } else {
                Err(http::StatusCode::FORBIDDEN)?
            }
        } else {
            Err(http::StatusCode::BAD_REQUEST)?
        }
    } else {
        Err(http::StatusCode::UNAUTHORIZED)?
    }
}

#[get("/login")]
async fn get_login(req: web::HttpRequest) -> Result<HttpResponse, ResError> {
    check_login(req)
}
