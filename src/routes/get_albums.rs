use actix_web::{get, HttpRequest, HttpResponse};

#[get("/albums")]
async fn get_all_albums() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/album")]
async fn get_album(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}
