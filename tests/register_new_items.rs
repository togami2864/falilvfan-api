#![allow(clippy::bool_assert_comparison)]
use reqwest::header::CONTENT_TYPE;
use sqlx::postgres::types::PgInterval;
use test_helpers::TestApp;
use uuid::Uuid;

use crate::test_helpers::spawn_app;

mod test_helpers;

async fn insert_sample_album(app: &TestApp) -> Uuid {
    let album_id = Uuid::new_v4();
    let release_date =
        sqlx::types::chrono::NaiveDate::parse_from_str("2022/10/26", "%Y/%m/%d").unwrap();
    sqlx::query!(
        r#"INSERT INTO albums (album_id, album_name, spotify_id, release_date, is_single)
    VALUES ($1, $2, $3, $4, $5)
    "#,
        album_id,
        "Cocoon for the Golden Future",
        "05eS7MkETxSTk4UcyieA4s",
        release_date,
        false
    )
    .execute(&app.db_pool)
    .await
    .unwrap();
    album_id
}

async fn insert_sample_locations(app: &TestApp) -> Uuid {
    sqlx::query!(
        r#"INSERT INTO prefectures (id, prefecture)
    VALUES ($1, $2)
    "#,
        14,
        "神奈川県"
    )
    .execute(&app.db_pool)
    .await
    .unwrap();

    let location_id = Uuid::new_v4();
    sqlx::query!(
        r#"INSERT INTO locations (id, location, prefecture_id)
    VALUES($1, $2, $3)
        "#,
        location_id,
        "KT Zepp Yokohama",
        14
    )
    .execute(&app.db_pool)
    .await
    .unwrap();

    location_id
}

async fn insert_sample_tracks(app: &TestApp) -> Uuid {
    let album_id = insert_sample_album(app).await;
    let track_id = Uuid::new_v4();
    let dur = std::time::Duration::from_millis(188000);
    let dur = PgInterval::try_from(dur).unwrap();

    sqlx::query!(
        r#"INSERT INTO tracks (id, name, track_number, duration_ms, album_id, youtube_url)
    VALUES ($1, $2, $3, $4, $5, $6)
    "#,
        track_id,
        "Get Back the Hope",
        1,
        dur,
        album_id,
        "https://www.youtube.com/watch?v=aniw0eO_PlY&ab_channel=Fear%2CandLoathinginLasVegas"
    )
    .execute(&app.db_pool)
    .await
    .unwrap();

    track_id
}

#[tokio::test]
async fn return_200_for_register_new_album() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();
    let body = format!(
        r#"{{
"album_name": "{}",
"spotify_id": "{}",
"release_date": "{}",
"is_single": false
    }}"#,
        "Cocoon for the Golden Future", "05eS7MkETxSTk4UcyieA4s", "2022/10/26"
    );
    let response = client
        .post(format!("{}/register/album", &app.address))
        .header(CONTENT_TYPE, "application/json")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT album_name, spotify_id, is_single, release_date FROM albums")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved albums");

    assert_eq!(saved.album_name, "Cocoon for the Golden Future");
    assert_eq!(saved.spotify_id.len(), 22);
    assert_eq!(
        saved.release_date,
        sqlx::types::chrono::NaiveDate::parse_from_str("2022/10/26", "%Y/%m/%d").unwrap()
    );

    assert_eq!(saved.is_single, false);
}

#[tokio::test]
async fn return_200_for_register_new_location() {
    let app = spawn_app().await;

    sqlx::query!(
        r#"INSERT INTO prefectures (id, prefecture)
    VALUES ($1, $2)
    "#,
        14,
        "神奈川県"
    )
    .execute(&app.db_pool)
    .await
    .unwrap();

    let client = reqwest::Client::new();
    let body = format!(
        r#"{{"location": "KT Zepp Yokohama", "prefecture_id": {}}}"#,
        14
    );

    let response = client
        .post(format!("{}/register/locations", &app.address))
        .header(CONTENT_TYPE, "application/json")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT location, prefecture_id FROM locations")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved albums");

    assert_eq!(saved.location, "KT Zepp Yokohama");
    assert_eq!(saved.prefecture_id, 14);
}

#[tokio::test]
async fn return_200_for_register_new_track() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();
    let album_id = insert_sample_album(&app).await;

    let body = format!(
        r#"{{
"name": "{}",
"track_number": {},
"duration_ms": {},
"album_id": "{}",
"youtube_url": "{}"
    }}"#,
        "Get Back the Hope",
        1,
        188000,
        album_id,
        "https://www.youtube.com/watch?v=aniw0eO_PlY&ab_channel=Fear%2CandLoathinginLasVegas"
    );

    let response = client
        .post(format!("{}/register/tracks", &app.address))
        .header(CONTENT_TYPE, "application/json")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT name, track_number, duration_ms, youtube_url FROM tracks")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved albums");

    assert_eq!(saved.name, "Get Back the Hope");
    assert_eq!(saved.track_number, 1);
    // modify millisecond
    assert_eq!(saved.duration_ms.microseconds / 1000, 188000);
    assert_eq!(
        saved.youtube_url,
        "https://www.youtube.com/watch?v=aniw0eO_PlY&ab_channel=Fear%2CandLoathinginLasVegas"
    );
}

#[tokio::test]
async fn return_200_for_register_new_live() {
    let app = spawn_app().await;

    let location_id = insert_sample_locations(&app).await;
    let track_id = insert_sample_tracks(&app).await;

    let body = format!(
        r#"{{"location_id": "{}", "event_name":"FaLiLV Shuffle Tour 2022", "date": "2022/09/30", "is_festival": false, "setlist_data": [{{"track_id": "{}", "track_order": {}}}]
    }}"#,
        location_id, track_id, 1
    );

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/register/lives", &app.address))
        .header(CONTENT_TYPE, "application/json")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT event_name, date, is_festival FROM lives")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved albums");

    assert_eq!(saved.event_name, "FaLiLV Shuffle Tour 2022");
    assert_eq!(saved.date.to_string(), "2022-09-30");
    assert_eq!(saved.is_festival, false);
}
