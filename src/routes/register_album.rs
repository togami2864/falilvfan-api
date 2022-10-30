use actix_web::{post, HttpResponse};

#[post("/register/album")]
async fn register_album() -> HttpResponse {
    HttpResponse::Ok().finish()
}
