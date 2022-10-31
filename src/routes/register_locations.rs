use actix_web::{post, web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, serde::Deserialize)]
pub struct LocationData {
    location: String,
    prefecture_id: i32,
}

#[post("/register/locations")]
async fn register_locations(req: web::Json<LocationData>, pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        r#"INSERT INTO locations (id, location, prefecture_id)
    VALUES ($1, $2, $3)
    "#,
        Uuid::new_v4(),
        req.location,
        req.prefecture_id
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
