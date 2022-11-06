use crate::helpers::insert_sample_album;
use crate::helpers::insert_sample_tracks;
use crate::helpers::spawn_app;

// TODO: fix unresolved
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TrackData {
    track_id: String,
    track_name: String,
    track_number: i32,
    youtube_url: String,
    album_id: String,
}

#[tokio::test]
async fn return_200_for_get_all_tracks() {
    let app = spawn_app().await;
    let album_id = insert_sample_album(&app).await;
    let _track_id = insert_sample_tracks(&app, &album_id).await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/tracks", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let json = response.text().await.unwrap();
    if let Err(e) = serde_json::from_str::<Vec<TrackData>>(&json) {
        panic!("{}", e);
    }
}
