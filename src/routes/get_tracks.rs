use std::collections::HashMap;

use actix_web::{get, web, HttpResponse};
use sqlx::PgPool;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct TrackData {
    track_id: String,
    track_name: String,
    track_number: i32,
    youtube_url: String,
    album_id: String,
}

#[get("/tracks")]
async fn get_all_tracks(pool: web::Data<PgPool>) -> HttpResponse {
    let entities = match sqlx::query!(
        r#"SELECT id, name, track_number, youtube_url, album_id FROM tracks LIMIT 100"#
    )
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(fetch_result) => fetch_result,
        Err(e) => {
            println!("Failed to execute query: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let track_data_records = entities
        .into_iter()
        .map(|r| TrackData {
            track_id: r.id.to_string(),
            track_name: r.name,
            track_number: r.track_number,
            youtube_url: r.youtube_url,
            album_id: r.album_id.to_string(),
        })
        .collect::<Vec<_>>();

    let track_data_json = serde_json::to_value::<Vec<TrackData>>(track_data_records).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(track_data_json)
}

#[get("/admin/tracks/map")]
async fn get_track_id_map(pool: web::Data<PgPool>) -> HttpResponse {
    let entities = match sqlx::query!(r#"SELECT id, name FROM tracks"#)
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(fetch_result) => fetch_result,
        Err(e) => {
            println!("Failed to execute query: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let mut json_builder: HashMap<String, String> = HashMap::new();

    for record in entities.into_iter() {
        json_builder.insert(record.name, record.id.to_string());
    }

    let track_id_map = serde_json::to_value(json_builder).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(track_id_map)
}
