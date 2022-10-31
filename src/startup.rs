use std::net::TcpListener;

use crate::routes::{get_album, get_all_albums, health_check, register_album, register_locations};
use actix_web::web;
use actix_web::{dev::Server, App, HttpServer};
use sqlx::PgPool;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .service(health_check)
            .service(get_all_albums)
            .service(get_album)
            .service(register_album)
            .service(register_locations)
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
