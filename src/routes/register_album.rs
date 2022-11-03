use actix_web::{post, web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AlbumData {
    album_name: String,
    spotify_id: String,
    release_date: String,
    is_single: bool,
}

#[post("/register/album")]
async fn register_album(req: web::Json<AlbumData>, pool: web::Data<PgPool>) -> HttpResponse {
    let release_date =
        sqlx::types::chrono::NaiveDate::parse_from_str(&req.release_date, "%Y/%m/%d").unwrap();

    match sqlx::query!(
        r#"INSERT INTO albums (album_id, album_name, spotify_id, release_date, is_single)
    VALUES ($1, $2, $3, $4, $5)
    "#,
        Uuid::new_v4(),
        req.album_name,
        req.spotify_id,
        release_date,
        req.is_single
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
