use std::str::FromStr;

use actix_web::{post, web, HttpResponse};
use sqlx::{postgres::types::PgInterval, PgPool};
use uuid::Uuid;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct TrackData {
    name: String,
    track_number: i32,
    duration_ms: i32,
    album_id: String,
    youtube_url: String,
}

#[post("/register/tracks")]
async fn register_tracks(req: web::Json<TrackData>, pool: web::Data<PgPool>) -> HttpResponse {
    let dur = std::time::Duration::from_millis(req.duration_ms as u64);
    let dur = PgInterval::try_from(dur).unwrap();
    let album_id = Uuid::from_str(&req.album_id).unwrap();

    match sqlx::query!(
        r#"INSERT INTO tracks (name, track_number, duration_ms, album_id, youtube_url)
    VALUES ($1, $2, $3, $4, $5)
    "#,
        req.name,
        req.track_number,
        dur,
        album_id,
        req.youtube_url
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
