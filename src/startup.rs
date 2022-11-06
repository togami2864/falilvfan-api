use std::net::TcpListener;

use crate::routes::{
    get_album, get_all_albums, get_all_locations, get_all_tracks, get_track_id_map, health_check,
    register_album, register_lives, register_locations, register_tracks,
};
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::web;
use actix_web::{dev::Server, App, HttpServer};
use sqlx::PgPool;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .service(health_check)
            .service(get_all_albums)
            .service(get_album)
            .service(register_album)
            .service(register_locations)
            .service(register_tracks)
            .service(register_lives)
            .service(get_all_tracks)
            .service(get_track_id_map)
            .service(get_all_locations)
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
