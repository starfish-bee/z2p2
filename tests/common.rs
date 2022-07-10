use std::{error::Error, net::TcpListener};

use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use zero2prod2::{
    config::{get_configuration, DatabaseSettings},
    AppContext,
};

pub async fn spawn_app() -> Result<(String, PgPool), Box<dyn Error>> {
    let mut config = get_configuration().expect("failed to read config file");
    config.database.database_name = Uuid::new_v4().to_string();

    let listener = TcpListener::bind("127.0.0.1:0")?;
    let addr = listener.local_addr().unwrap().to_string();

    let db_pool = configure_database(&config.database).await;
    let db_pool_clone = db_pool.clone();

    tokio::spawn(async move {
        zero2prod2::run(AppContext { listener, db_pool })
            .await
            .expect("server crashed");
    });

    Ok((addr, db_pool_clone))
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");
    // Migrate database
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool
}
