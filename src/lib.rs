use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[get("/album")]
async fn get_album(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

#[get("/albums")]
async fn get_all_albums() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(health_check)
            .service(get_all_albums)
            .service(get_album)
    })
    .listen(listener)?
    .run();
    Ok(server)
}
