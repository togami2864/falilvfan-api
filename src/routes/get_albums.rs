use actix_web::{get, web, HttpRequest, HttpResponse};
use sqlx::PgPool;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AlbumDataRes {
    album_id: String,
    album_name: String,
    spotify_id: String,
    release_date: String,
    is_single: bool,
}

#[get("/albums")]
async fn get_all_albums(pool: web::Data<PgPool>) -> HttpResponse {
    let entities = match sqlx::query!(r#"SELECT * FROM albums LIMIT 100"#)
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(fetch_result) => fetch_result,
        Err(e) => {
            println!("Failed to execute query: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let album_data_records = entities
        .into_iter()
        .map(|r| AlbumDataRes {
            album_id: r.album_id.to_string(),
            album_name: r.album_name,
            spotify_id: r.spotify_id,
            is_single: r.is_single,
            release_date: r.release_date.to_string(),
        })
        .collect::<Vec<_>>();

    let album_data_json = serde_json::to_string::<Vec<AlbumDataRes>>(&album_data_records).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(album_data_json)
}

#[get("/album")]
async fn get_album(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}
