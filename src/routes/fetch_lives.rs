use actix_web::{get, web, HttpResponse};
use sqlx::PgPool;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
struct LiveData {
    live_id: String,
    location_id: String,
    date: String,
    is_festival: bool,
    event_name: String,
}

#[get("/lives")]
async fn fetch_all_lives(pool: web::Data<PgPool>) -> HttpResponse {
    let entities = match sqlx::query!(
        r#"SELECT live_id, location_id, date, is_festival, event_name FROM lives ORDER BY date DESC"#
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

    let live_data_records = entities
        .into_iter()
        .map(|r| LiveData {
            live_id: r.live_id.to_string(),
            location_id: r.location_id.to_string(),
            date: r.date.to_string(),
            is_festival: r.is_festival,
            event_name: r.event_name,
        })
        .collect::<Vec<_>>();

    let live_data_json = serde_json::to_value::<Vec<LiveData>>(live_data_records).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(live_data_json)
}
