#![allow(clippy::bool_assert_comparison)]
use reqwest::header::CONTENT_TYPE;

use crate::helpers::{
    insert_sample_album, insert_sample_locations, insert_sample_tracks, spawn_app,
};

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
        .post(format!("{}/admin/album/register", &app.address))
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
        .post(format!("{}/admin/location/register", &app.address))
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
"trackNumber": {},
"durationMs": {},
"albumId": "{}",
"youtubeUrl": "{}"
    }}"#,
        "Get Back the Hope",
        1,
        188000,
        album_id,
        "https://www.youtube.com/watch?v=aniw0eO_PlY&ab_channel=Fear%2CandLoathinginLasVegas"
    );

    let response = client
        .post(format!("{}/admin/track/register", &app.address))
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
    let album_id = insert_sample_album(&app).await;
    let track_id = insert_sample_tracks(&app, &album_id).await;

    let body = format!(
        r#"{{"locationId": "{}", "eventName":"FaLiLV Shuffle Tour 2022", "date": "2022/09/30", "isFestival": false, "setlistData": [{{"trackId": "{}", "trackOrder": {}, "trackName": "{}"}}]
    }}"#,
        location_id, track_id, 1, "Get Back the Hope"
    );

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/admin/live/register", &app.address))
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
