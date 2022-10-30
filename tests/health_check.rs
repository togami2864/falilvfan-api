use falilvfan::{configuration::get_configuration, startup::run};
use sqlx::{Connection, PgConnection};
use std::net::TcpListener;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn return_200_for_get_all_albums() {
    let app = spawn_app();
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_string = configuration.database.connection_string();

    let connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres");
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/albums", &app))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn return_200_for_get_album() {
    let app = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/album", &app))
        .query(&[("album_id", "aaaa")])
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn return_200_for_register_new_album() {
    let app = spawn_app();
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_string = configuration.database.connection_string();

    let connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres");
    let client = reqwest::Client::new();

    let body = "name=Cocoon%20for%20the%20Golden%20Future&spotify_id=05eS7MkETxSTk4UcyieA4s&is_single=false&release_date=20221026";
    let response = client
        .post(format!("{}/register/album", &app))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT album_name, spotify_id, is_single, release_date FROM albums")
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved albums");

    assert_eq!(saved.name, "Cocoon for the Golden Future");
    assert_eq!(saved.spotify_id.len(), 22);
    assert_eq!(saved.release_date, "20221026");
    assert_eq!(saved.is_single, false);
}
