use std::net::TcpListener;

use falilvfan::configuration::get_configuration;
use falilvfan::startup::run;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let is_dev = matches!(args.get(1), Some(v) if v == "dev");

    let configuration = get_configuration(is_dev).expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
