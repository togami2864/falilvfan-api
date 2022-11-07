use actix_web::{get, web, HttpResponse};
use sqlx::PgPool;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
struct LocationData {
    location_id: String,
    location: String,
    prefecture_id: i32,
}

#[get("/locations")]
async fn fetch_all_locations(pool: web::Data<PgPool>) -> HttpResponse {
    let entities =
        match sqlx::query!(r#"SELECT id, location, prefecture_id FROM locations LIMIT 100"#)
            .fetch_all(pool.get_ref())
            .await
        {
            Ok(fetch_result) => fetch_result,
            Err(e) => {
                println!("Failed to execute query: {}", e);
                return HttpResponse::InternalServerError().finish();
            }
        };

    let location_data_records = entities
        .into_iter()
        .map(|r| LocationData {
            location_id: r.id.to_string(),
            location: r.location,
            prefecture_id: r.prefecture_id,
        })
        .collect::<Vec<_>>();

    let location_data_json =
        serde_json::to_value::<Vec<LocationData>>(location_data_records).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(location_data_json)
}
