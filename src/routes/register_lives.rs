use std::str::FromStr;

use actix_web::{post, web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, serde::Deserialize)]
pub struct LivesData {
    location_id: String,
    date: String,
    is_festival: bool,
}

#[post("/register/lives")]
async fn register_lives(req: web::Json<LivesData>, pool: web::Data<PgPool>) -> HttpResponse {
    let location_id = Uuid::from_str(&req.location_id).unwrap();
    let date = sqlx::types::chrono::NaiveDate::parse_from_str(&req.date, "%Y/%m/%d").unwrap();
    match sqlx::query!(
        r#"INSERT INTO lives (location_id, date, is_festival)
    VALUES ($1, $2, $3)
    "#,
        location_id,
        date,
        req.is_festival
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
