use falilvfan::routes::{AlbumData, GetAlbumRes};

use crate::helpers::insert_sample_album;
use crate::helpers::insert_sample_tracks;
use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn return_200_for_get_all_albums() {
    let app = spawn_app().await;
    insert_sample_album(&app).await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/albums", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
    let json = response.json::<String>().await.unwrap();
    if let Err(e) = serde_json::from_str::<Vec<AlbumData>>(&json) {
        panic!("{}", e);
    }
}

#[tokio::test]
async fn return_200_for_get_album() {
    let app = spawn_app().await;
    let album_id = insert_sample_album(&app).await;
    let _track_id = insert_sample_tracks(&app, &album_id).await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/album", &app.address))
        .query(&[("album_id", format!("{}", album_id))])
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let json = response.text().await.unwrap();
    if let Err(e) = serde_json::from_str::<GetAlbumRes>(&json) {
        panic!("{}", e);
    }
}
