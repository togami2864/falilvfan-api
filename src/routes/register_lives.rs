use std::str::FromStr;

use actix_web::{post, web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, serde::Deserialize)]
pub struct SetListData {
    track_id: String,
    track_order: i32,
}

#[derive(Debug, serde::Deserialize)]
pub struct LivesData {
    location_id: String,
    date: String,
    is_festival: bool,
    setlist_data: Vec<SetListData>,
}

#[post("/register/lives")]
async fn register_lives(req: web::Json<LivesData>, pool: web::Data<PgPool>) -> HttpResponse {
    let live_id = Uuid::new_v4();
    let location_id = Uuid::from_str(&req.location_id).unwrap();
    let date = sqlx::types::chrono::NaiveDate::parse_from_str(&req.date, "%Y/%m/%d").unwrap();

    if let Err(e) = sqlx::query!(
        r#"INSERT INTO lives (live_id, location_id, date, is_festival)
    VALUES ($1, $2, $3, $4)
    "#,
        live_id,
        location_id,
        date,
        req.is_festival
    )
    .execute(pool.get_ref())
    .await
    {
        println!("Failed to execute query: {}", e);
        return HttpResponse::InternalServerError().finish();
    };

    for track_data in req.setlist_data.iter() {
        let setlist_id = Uuid::new_v4();
        let track_id = Uuid::from_str(&track_data.track_id).unwrap();
        match sqlx::query!(
            r#"INSERT INTO setlists (id, live_id, track_id, track_order) VALUES($1, $2, $3, $4)"#,
            setlist_id,
            live_id,
            track_id,
            track_data.track_order
        )
        .execute(pool.get_ref())
        .await
        {
            Err(e) => {
                println!("Failed to execute query: {}", e);
                return HttpResponse::InternalServerError().finish();
            }
            _ => continue,
        };
    }
    HttpResponse::Ok().finish()
}
