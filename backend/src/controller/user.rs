use actix_web::{get, web, HttpResponse};

pub fn check_login(req: web::HttpRequest) -> Result<HttpResponse, HttpResponse> {
    if let Some(h) = req.headers().get("Authorization") {
        if let Ok(token) = h.to_str() {
            if dotenv::var("TOKEN").unwrap() == token {
                Ok(HttpResponse::Ok().finish())
            } else {
                Err(HttpResponse::Forbidden().finish())
            }
        } else {
            Err(HttpResponse::BadRequest().finish())
        }
    } else {
        Err(HttpResponse::Unauthorized().finish())
    }
}

#[get("/login")]
async fn get_login(req: web::HttpRequest) -> Result<HttpResponse, HttpResponse> {
    check_login(req)
}
