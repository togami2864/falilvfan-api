use falilvfan::routes::LocationData;

use crate::helpers::insert_sample_locations;
use crate::helpers::spawn_app;

#[tokio::test]
async fn return_200_for_get_all_locations() {
    let app = spawn_app().await;
    let _ = insert_sample_locations(&app).await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/locations", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let json = response.text().await.unwrap();
    if let Err(e) = serde_json::from_str::<Vec<LocationData>>(&json) {
        panic!("{}", e);
    }
}
