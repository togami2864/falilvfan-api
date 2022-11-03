use std::str::FromStr;

use actix_web::{get, web, HttpResponse};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct AlbumData {
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
        .map(|r| AlbumData {
            album_id: r.album_id.to_string(),
            album_name: r.album_name,
            spotify_id: r.spotify_id,
            is_single: r.is_single,
            release_date: r.release_date.to_string(),
        })
        .collect::<Vec<_>>();

    let album_data_json = serde_json::to_string::<Vec<AlbumData>>(&album_data_records).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(album_data_json)
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TrackData {
    track_id: String,
    album_id: String,
    track_name: String,
    duration_ms: i64,
    youtube_url: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AlbumMeta {
    album_id: String,
    album_name: String,
    spotify_id: String,
    release_date: String,
    is_single: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GetAlbumRes {
    tracks: Vec<TrackData>,
    album: AlbumMeta,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct QueryParams {
    album_id: String,
}

#[get("/album")]
async fn get_album(params: web::Query<QueryParams>, pool: web::Data<PgPool>) -> HttpResponse {
    let album_id = Uuid::from_str(&params.album_id).unwrap();

    let album_meta = match sqlx::query!("SELECT album_id, album_name, spotify_id, release_date, is_single FROM albums WHERE albums.album_id = $1", album_id)
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(res) => AlbumMeta {
            album_id: album_id.to_string(),
            album_name: res.album_name,
            spotify_id: res.spotify_id,
            release_date: res.release_date.to_string(),
            is_single: res.is_single,
        },
        Err(e) => {
            println!("Failed to execute query: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let album_meta_json = serde_json::to_value::<AlbumMeta>(album_meta).unwrap();

    let entities = match sqlx::query!("SELECT id, album_id, name, duration_ms, youtube_url FROM tracks WHERE tracks.album_id = $1", album_id)
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(fetch_result) => fetch_result,
        Err(e) => {
            println!("Failed to execute query: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let all_tracks_in_album = entities
        .into_iter()
        .map(|r| TrackData {
            track_id: r.id.to_string(),
            album_id: r.album_id.to_string(),
            track_name: r.name,
            duration_ms: r.duration_ms.microseconds / 1000,
            youtube_url: r.youtube_url,
        })
        .collect::<Vec<_>>();
    let tracks_json = serde_json::to_value::<Vec<TrackData>>(all_tracks_in_album).unwrap();

    let get_album_response = json!({
        "tracks": tracks_json,
        "album": album_meta_json
    });

    HttpResponse::Ok()
        .content_type("application/json")
        .json(get_album_response)
}
