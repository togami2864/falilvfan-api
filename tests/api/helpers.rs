use std::net::TcpListener;

use falilvfan::{
    configuration::{get_configuration, DatabaseSettings},
    startup::run,
};
use sqlx::{postgres::types::PgInterval, Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
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

pub async fn insert_sample_album(app: &TestApp) -> Uuid {
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

pub async fn insert_sample_locations(app: &TestApp) -> Uuid {
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

pub async fn insert_sample_tracks(app: &TestApp, album_id: &Uuid) -> Uuid {
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
