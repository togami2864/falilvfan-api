use std::net::TcpListener;

use actix_web::{dev::Server, App, HttpServer};

use crate::routes::{get_album, get_all_albums, health_check};

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
