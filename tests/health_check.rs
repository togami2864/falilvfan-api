#![allow(clippy::bool_assert_comparison)]
use falilvfan::{
    configuration::{get_configuration, DatabaseSettings},
    startup::run,
};
use reqwest::header::CONTENT_TYPE;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration(false).expect("Failed to read configuration.");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(&configuration.database).await;

    let server = run(listener, connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate te database");
    connection_pool
}

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
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/albums", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn return_200_for_get_album() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/album", &app.address))
        .query(&[("album_id", "aaaa")])
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(200, response.status().as_u16());
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
